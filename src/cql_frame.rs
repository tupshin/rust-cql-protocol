use cql_header::Header;
use cql_body::Body;
use cql_stream::CqlStream;
use cql_body::BodyBuilder;

use std::io::IoResult;


#[deriving(Show)]
pub enum Frame<'a> {
    Bytes(Vec<u8>),
    Parts(FrameParts<'a>)
}

#[deriving(Show)]
pub struct FrameParts<'a> {
    header:Header,
    body:Body<'a>
}

impl<'b> Frame<'b> {

    pub fn build_startup<'a>(bytes:Vec<u8>) ->  Frame<'a> {
        let header = Header::build_startup();
        
        let body:Body = BodyBuilder::build_startup(bytes);
        Frame::Parts(FrameParts{header:header,body:body})
    }

    pub fn build_query<'a>(bytes:Vec<u8>, query:String) ->  Frame<'a> {
        let body:Body = BodyBuilder::build_query(bytes,query);
        let header = Header::build_query(body.len() as u32);
        Frame::Parts(FrameParts{header:header,body:body})
    }

    pub fn len(&self) -> u32 {
        match self {
            &Frame::Bytes(ref bytes) => bytes.len() as u32,
            &Frame::Parts(ref parts) => parts.header.len() as u32 + parts.body.len() as u32
        }
    }

    pub fn as_bytes<'a>(&'a self) -> Vec<u8> {
        match self{
            &Frame::Bytes(ref bytes) => bytes.clone(), 
            &Frame::Parts(ref parts) => {
                let mut bytes = Vec::<u8>::new();
                bytes.push_all(parts.header.to_bytes()[]);
                bytes.push_all(parts.body[]);
                bytes
            }
        }
    }

    pub fn get_header<'a>(&'a self) -> Header {
        match self{
            &Frame::Bytes(ref bytes) => {
                let header = bytes[0..9].as_ptr() as *const Header;
                unsafe{*header}
            }
            &Frame::Parts(ref parts) => {
                parts.header
            }
        }
    }

    pub fn send_frame(self, stream:&mut CqlStream) -> IoResult<()> {
        stream.write_frame(self)
    }
}

