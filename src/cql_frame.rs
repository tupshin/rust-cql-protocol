use cql_header::Header;
use cql_body::Body;
use cql_stream::CqlStream;

use std::io::IoResult;


#[deriving(Copy)]
pub enum Frame<'a> {
    Bytes(&'a Vec<u8>),
    Parts(FrameParts<'a>)
}

#[deriving(Copy)]
pub struct FrameParts<'a> {
    header:Header,
    body:Body<'a>
}

impl<'b> Frame<'b> {

    pub fn build_startup<'a>(bytes:&'a Vec<u8>) ->  Frame<'a> {
        let header = Header::build_startup();
        let body = Body::build_startup(bytes);
        Frame::Parts(FrameParts{header:header,body:body})
    }

    pub fn as_bytes<'a>(&'a self) -> Vec<u8> {
        match self{
            &Frame::Bytes(bytes) => bytes.clone(), 
            &Frame::Parts(parts) => {
                let mut bytes = Vec::<u8>::new();
                bytes.push_all(parts.header.to_bytes()[]);
                bytes.push_all(parts.body.bytes[]);
                bytes
            }
        }
    }

    pub fn get_header<'a>(&'a self) -> &'a Header {
        match self{
            &Frame::Bytes(ref bytes) => {
                let header = bytes[0..9].as_ptr() as *const Header;
                unsafe{&*header}
            }
            &Frame::Parts(ref parts) => {
                &parts.header
            }
        }
    }



    pub fn send_frame(self, stream:&mut CqlStream) -> IoResult<()> {
        stream.write_frame(self)
    }
}

