#[phase(plugin, link)]
extern crate log;

use std::fmt;
use std::fmt::Show;
use std::num::Int;

use raw_byte_utils::*;

use cql_frame::Frame;

#[repr(C, packed)]
#[deriving(Show,Copy)]
pub struct Header {
    pub version:Version,
    pub flags:Flags,
    pub stream:StreamId,
    pub opcode:Opcode,
    pub body_length:Length
}

#[deriving(Copy,Show)]
pub enum Version {DEFAULT=0x03}

#[repr(u8, packed)]
#[deriving(Copy,Show)]
pub enum Flags {NONE=0x00,COMPRESSION=0x01,TRACING=0x02,ALL=0x03}

#[repr(u16, packed)]
pub type StreamId = i16;

#[repr(u8, packed)]
#[deriving(Copy,Show)] #[allow(non_camel_case_types)]
pub enum Opcode {
    ERROR=0x00,
    STARTUP=0x01,
    READY=0x02,
    AUTHENTICATE=0x03,
    OPTIONS=0x05,
    SUPPORTED=0x06,
    QUERY=0x07,
    RESULT=0x08,
    PREPARE=0x09,
    EXECUTE=0x0A,
    REGISTER=0x0B,
    EVENT=0x0C,
    BATCH=0x0D,
    AUTH_CHALLENGE=0x0E,
    AUTH_RESPONSE=0x0F,
    AUTH_SUCCESS=0x10
}

#[repr(C, packed)]
#[deriving(Copy)]
pub struct Length {pub length:u32}

impl fmt::Show for Length {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", Int::from_be(self.length))
  }
}

impl Header {
    pub fn build_startup() -> Header {
        let version:Version=Version::DEFAULT;
        let flags=Flags::NONE;
        let opcode=Opcode::STARTUP;
        let stream:i16=IdGen::new_id();
        let length=Length{length:Int::from_be(22)};
        Header{version:version,flags:flags,opcode:opcode,stream:stream,body_length:length}
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut vec = Vec::<u8>::new();
        vec.push_all(unsafe{raw_byte_repr(self)});
        vec
    }

    pub fn frame_it<'a>(header:Header, bytes:&'a mut Vec<u8>)  -> Frame<'a> {
        debug!("header: {}", header);
        let size = header.body_length.length as uint;
        debug!("body size {}",size);
        bytes.push_all(unsafe{raw_byte_repr(&header)});
        bytes.resize(size + 9 , 0);
        Frame::Bytes(bytes)
    }
}

pub trait IdGen {
  /// Dumb approach to get increasing and unique ids
  fn new_id() -> Self;
}

static mut STREAM_ID : i16 = 0;

impl IdGen for i16 {
  fn new_id() -> Self {unsafe{
    STREAM_ID +=1;
    STREAM_ID
  }}
}
