#[phase(plugin)]
extern crate lazy_static;

use std::collections::HashMap;

use cql_transport_types::CqlTransportTypeBuilder;
use cql_transport_types::consistency::Consistency;
use cql_transport_types::query::QueryFlags;
use cql_transport_types::CqlResult;
use cql_transport_types::CqlStringMap;



lazy_static! {
    static ref STARTUP_BODY:CqlStringMap = {
        let mut map:HashMap<String,String> = HashMap::<String,String>::new();
        map.insert("CQL_VERSION".to_string(),"3.0.0".to_string());
        map.to_cql_type()
    };
}

#[repr(C, packed)]
#[derive(Show)]
pub struct Body<'b> {pub bytes:Vec<u8>}

pub trait BodyBuilder {
    fn build_startup(bytes:Vec<u8>) -> Self;
    fn build_query(bytes:Vec<u8>, query:String, consistency:Consistency, flags:QueryFlags) -> Self;
    fn len(&self) -> u32;
    fn get_results(&self) -> CqlResult;
}


impl<'b> BodyBuilder for Body<'b> {
    fn build_startup(mut bytes:Vec<u8>) -> Self {
        debug!("body bytes {}: ", bytes[]);
        bytes.push_all(STARTUP_BODY.bytes[]);
        Body{bytes:bytes}
    }

    fn build_query(mut bytes:Vec<u8>, query:String, consistency:Consistency, flags:QueryFlags) -> Self {
        bytes.push_all(query.to_cql_type().bytes[]);
        bytes.write_be_u16(consistency as u16).unwrap();
        bytes.write_u8(flags as u8).unwrap();
        debug!("query bytes: {}", bytes[]);
        debug!("query bytes len: {}", bytes.len());
        Body{bytes:bytes}
    }

    fn len(&self) -> u32 {
        self.bytes.len() as u32
    }

    fn get_results(&self) -> CqlResult {
        panic!();
    }

}



