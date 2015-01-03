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
pub type Body<'b> = Vec<u8>;

pub trait BodyBuilder {
    fn build_startup(bytes:Vec<u8>) -> Self;
    fn build_query(bytes:Vec<u8>, query:String) -> Self;
    fn len(&self) -> u32;
}


impl<'b> BodyBuilder for Body<'b> {
    fn build_startup(mut bytes:Vec<u8>) -> Self {
        debug!("body bytes {}: ", bytes[]);
        bytes.push_all(STARTUP_BODY[]);
        bytes
    }

    fn build_query(bytes:Vec<u8>, query:String) -> Self {
        debug!("body bytes {}: ", bytes[]);
        let cql_string = query.to_cql_type();
       cql_string
    }

    fn len(&self) -> u32 {
        self.len() as u32
    }
}



