#[phase(plugin)]
extern crate lazy_static;

use std::collections::HashMap;

use cql_transport_types::CqlTransportTypeBuilder;



lazy_static! {
    static ref STARTUP_BODY:Vec<u8> = {
        let mut map:HashMap<String,String> = HashMap::<String,String>::new();
        map.insert("CQL_VERSION".to_string(),"3.0.0".to_string());
        //let body = Body{bytes:&map.to_cql_type()};
        map.to_cql_type()
    };
}

#[repr(C, packed)]
#[deriving(Copy,Show)]
pub struct Body<'a> {
    pub bytes:&'a Vec<u8>
}

impl<'b> Body<'b> {
    pub fn build_startup(bytes:&'b mut Vec<u8>) -> Body<'b>{
        debug!("body bytes {}: ", bytes[]);
        bytes.push_all((STARTUP_BODY[]));
        Body{bytes:bytes}
    }

    pub fn len(&self) -> u16 {
        self.bytes.len() as u16
    }
}



