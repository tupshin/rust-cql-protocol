use cql_header::Header;
use cql_body::Body;
use cql_stream::CqlStream;
use cql_body::BodyBuilder;
use cql_error::CqlError;
use cql_error::TransportErrorCode;
use cql_transport_types::CqlLongString;
use cql_transport_types::Consistency;
use cql_transport_types::QueryFlags;
use cql_transport_types::ResultType;
use cql_transport_types::CqlResult;

use std::str::from_utf8;

use std::io::IoResult;
use std::num::Int;
use std::mem;
pub use cql_header::Opcode;


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

    pub fn build_query<'a>(bytes:Vec<u8>, query:String, consistency:Consistency, flags:QueryFlags) ->  Frame<'a> {
        let body:Body = BodyBuilder::build_query(bytes,query, consistency, flags);
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
                bytes.push_all(parts.body.bytes[]);
                bytes
            }
        }
    }

    pub fn get_opcode(&self) -> Opcode {
        self.get_header().opcode
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

    pub fn get_body<'a>(&'a self) -> &Body {
        match self{
            &Frame::Bytes(ref bytes) => {
                let body = bytes[10..].as_ptr() as *const Body;
                unsafe{&*body}
            }
            &Frame::Parts(ref parts) => {
                &parts.body
            }
        }
    }

    pub fn get_error(&self) -> CqlError {
        let body = self.get_body();
        match self.get_header().opcode {
            Opcode::ERROR => unsafe{
                let (err_code_slice,message_slice) = self.get_body().bytes.split_at(4);
                let message_slice = message_slice.as_ptr() as *const CqlLongString;
                let ref message_slice:CqlLongString = *message_slice;
                CqlError{error_code:TransportErrorCode::UNAVAILABLE_EXCEPTION,error_msg:message_slice.bytes.to_string()}
            },
            _ => panic!("get_error called on a non-error frame")
        }
    }

    pub fn get_results(&self) -> CqlError {
        let body = self.get_body();
        match self.get_header().opcode {
            Opcode::RESULT => match body.get_results() {
                CqlResult::VOID(_) => {panic!()},
                CqlResult::ROWS(rows_result) => {panic!()},
                CqlResult::SET_KEYSPACE(set_keyspace_result) => {panic!()},
                CqlResult::PREPARED(prepared_result) => {panic!()},
                CqlResult::SCHEMA_CHANGE(schema_change) => {panic!()}
            },
            _ => panic!("get_error called on a non-error frame")
        }
    }

    pub fn send_frame(self, stream:&mut CqlStream) -> IoResult<()> {
        stream.write_frame(self)
    }
}

