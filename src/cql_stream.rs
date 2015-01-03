use cql_frame::Frame;
use cql_header::Header;
use raw_byte_utils::*;

use std::io::IoResult;
use std::io::TcpStream;
use std::io::IoError;
use std::io::IoErrorKind;


//~ use std::sync::{Once, ONCE_INIT};
//~ static INIT: Once = ONCE_INIT;

static mut HEADER_RESPONSE_BUF: [u8;9] =  [0u8,..9];

pub trait CqlStream {
    fn write_frame(&mut self,frame:Frame) -> IoResult<()>;
    fn get_next_frame<'a>(&'a mut self, bytes:Vec<u8>) -> IoResult<Frame<'a>>;
    fn match_len<'a>(&'a mut self, mut frame:Frame<'a>) -> IoResult<Frame<'a>>;
    fn build_inbound_frame<'a>(&mut self, header:Header, mut bytes:Vec<u8>)  -> Frame<'a>;
   // fn query(&mut self, query:String) ->  IoResult<()>;
}

impl CqlStream for TcpStream {
    fn write_frame(&mut self,frame:Frame) -> IoResult<()> {
        debug!("writing frame of length: {}", frame.as_bytes().len());
       self.write(frame.as_bytes().as_slice())
    }

    fn get_next_frame<'a>(&'a mut self, bytes: Vec<u8>) -> IoResult<Frame<'a>> {unsafe{
        //read the next 9 bytes off the stream and stick them into a mutable slice of our buffer
        match self.read(HEADER_RESPONSE_BUF.as_mut_slice()) {
            Err(err) => panic!("failed to read frame: {}", err),
            Ok(_) => {
                let response_header = HEADER_RESPONSE_BUF.as_ptr() as *const Header;
                let response_header:Header = *response_header;
                //construct a Frame presized to hold all of the header (which has already been written) plus
                //room for all the bytes based on the expected body size
                let frame:Frame = self.build_inbound_frame(response_header, bytes);
                //get a reference to a mutable slice of just the body bytes of the frame (everything after byte 9)
                let body_slice = self.match_len(frame);
                body_slice

        
            }
        }
    }}

    fn build_inbound_frame<'a>(&mut self, header:Header, mut bytes:Vec<u8>)  -> Frame<'a> {
        debug!("header: {}", header);
        let size = header.get_body_len();
        debug!("header's claimed body size {}",size);
        bytes.push_all(unsafe{raw_byte_repr(&header)});
        bytes.resize(size as uint + 9 , 0);
        match size {
            0 => Frame::Bytes(bytes),
            _ => {
                match self.read(&mut*bytes[9u..8u+size as uint]) { //FIXME wtf math
                    Err(err) => panic!("failed to read body: {}", err),
                    Ok(_) => Frame::Bytes(bytes)
                }
            }
        }
    }

    fn match_len<'a>(&'a mut self, frame:Frame<'a>) -> IoResult<Frame<'a>> {
        debug!("match_len says: {}", frame.get_header().body_length);
        let len = frame.len() as uint;
        match len {
            0...8 => {
                debug!("short header: {}",len);
                Err(IoError{
                    kind:IoErrorKind::EndOfFile,
                    desc:"frame less than 9 bytes",
                    detail:Some(format!("Only {} bytes were returned.",len))
                    })
            },
            9 => {
                debug!("header size frame: {}",len);
                Ok(frame)
            },
            _ => {
                debug!("got some body: {}",len);
                //panic!("frame size: {}", frame.len());

                let mut bytes = frame.as_bytes();
                debug!("bytes buf size is {}",bytes.len());
                let (_,body) = bytes.split_at_mut(9);
                debug!("body buf size is {}",body.len());
                match self.read(body) {
                    Err(err) => panic!("failed to read body: {}",err),
                    Ok(_) => {debug!("why you no get here");Ok(frame)}
                }
            }
        }
    }

}
