use cql_frame::Frame;
use cql_header::Header;

use std::io::IoResult;
use std::io::TcpStream;
use std::io::net::ip::ToSocketAddr;

use std::sync::{Once, ONCE_INIT};
static INIT: Once = ONCE_INIT;

static mut HEADER_RESPONSE_BUF: [u8;9] =  [0u8,..9];

pub trait CqlStream {
    //fn connect_cql(&mut self, addr: &str) -> IoResult<&CqlStream>;
   // fn startup(&mut self) -> IoResult<()>;
    fn write_frame(&mut self,frame:Frame) -> IoResult<()>;
    fn get_next_frame(&mut self) -> IoResult<&Frame>;
    fn match_len<'a>(&'a mut self, mut frame:Frame<'a>) -> IoResult<Frame<'a>>;
   // fn query(&mut self, query:String) ->  IoResult<()>;
}

impl CqlStream for TcpStream {
    fn write_frame(&mut self,frame:Frame) -> IoResult<()> {
       self.write(frame.as_bytes().as_slice())
    }

    fn get_next_frame<'a>(&'a mut self) -> IoResult<&'a Frame> {unsafe{
        match self.read(HEADER_RESPONSE_BUF.as_mut_slice()) {
            Err(err) => panic!("failed to read frame: {}", err),
            Ok(_) => {
                let response = HEADER_RESPONSE_BUF.as_ptr() as *const Header;
                let response = *response;
                //construct a Frame presized to hold all of the header (which has already been written) plus
                //room for all the bytes based on the expected body size
                let frame:Frame = Header::frame_it(response);
        
                //get a reference to a mutable slice of just the body bytes of the frame (everything after byte 9)
                panic!();
 //               self.match_len(frame)
            }
        }
    }}

    fn match_len<'a>(&'a mut self, mut frame:Frame<'a>) -> IoResult<Frame<'a>> {
        debug!("match_len says: {}", frame.get_header().body_length);
        let len = frame.get_header().body_length.length as uint;
        match len {
            0...8 => {
                debug!("short header: {}",len);
                Ok(frame)
            },
            9 => {
                debug!("header size frame: {}",len);
                Ok(frame)
            },
            _ => {
                debug!("got some body: {}",len);
                //panic!("frame size: {}", frame.len());

                let mut bytes = frame.as_bytes();
                let mut slice = bytes[mut 10..len];
        
                match self.read(slice) {
                    Err(err) => panic!("failed to read body: {}",err),
                    Ok(_) => Ok(frame)
                }
            }
        }
    }

}
