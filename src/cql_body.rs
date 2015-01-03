use raw_byte_utils::Utils;

#[repr(C, packed)]
#[deriving(Copy,Show)]
pub struct Body<'a> {
    pub bytes:&'a Vec<u8>
}

const CQL_VERSION_LEN:u16 = 11;
const CQL_VERSION:&'static str = "CQL_VERSION";
const CQL_3_0_0_LEN:u16 = 5;
const CQL_3_0_0:&'static str = "3.0.0";

impl<'b> Body<'b> {
    pub fn build_startup(bytes:&'b mut Vec<u8>) -> Body<'b>{unsafe{
        //crude hack
        bytes.push_all(&[0x00, 0x01]); //one as short indicating one k/v in map
        bytes.push_all(&[0x00, 0x0b]); //one as short indicating one k/v in map
        bytes.push_all(CQL_VERSION[].as_bytes()); //k
        bytes.push_all(&[0x00, 0x05]); //one short indicating length of v
        bytes.push_all(CQL_3_0_0[].as_bytes()); //k
        debug!("body bytes {}: ", bytes[]);
        Body{bytes:bytes}
    }}
}
