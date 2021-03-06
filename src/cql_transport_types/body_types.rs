use std::collections::HashMap;
use std::str::from_utf8;

pub struct CqlStringMap {pub bytes:Vec<u8>}


///for any type that implements CqlTransportTypeBuilder, it must have a build() function that converts
///from type T to type U, and that it is up to the implementation of each type CqlTransportType what its
///respective types for T and U are.
pub trait CqlTransportTypeBuilder<NATIVET,CQLT> {
    fn to_cql_type(&self) -> CQLT;
}

pub trait CqlTransportTypeSerializer<CQLT,NATIVET> {
    fn to_native_type(&self) -> NATIVET;
}

impl CqlTransportTypeSerializer<Self,HashMap<String,String>> for CqlStringMap {
    fn to_native_type(&self) -> HashMap<String,String> {
        let map = HashMap::new();
        map
    }
}

///A builder for the CqlStringMap type must take as "T" type HashMap<String,String> and
///produce a "U" as a CqlStringMap
impl CqlTransportTypeBuilder<Self,CqlStringMap> for HashMap<String,String> {
    fn to_cql_type(&self) -> CqlStringMap {
        let mut bytes = Vec::<u8>::new();
        match bytes.write_be_u16(self.len() as u16) { //one as short indicating one k/v in map
            Err(_) => panic!("Couldn't write a short to a byte array!!!"),
            Ok(_) => {
                for (key,value) in self.iter() {
                    bytes.write_be_u16(key.len() as u16).unwrap(); //one short indicating length of k
                    bytes.write_str(key[]).unwrap();
                    bytes.write_be_u16(value.len() as u16).unwrap(); //one short indicating length of v
                    bytes.write_str(value[]).unwrap();
                }
                CqlStringMap{bytes:bytes}
            }
        }
    }
}

pub struct CqlLongString{pub bytes:Vec<u8>}

impl CqlTransportTypeSerializer<Self,String> for CqlLongString {
    fn to_native_type(&self) -> String {
        let (_,bytes) = self.bytes.split_at(4); //discarding the size for now
        match from_utf8(bytes) {
            Err(err) => panic!("couldn't extract a String from a CqlLongString: {}",err),
            Ok(value) => value.to_string()
        }
    }
}

///A builder for the CqlStringMap type must take as "T" type HashMap<String,String> and
///produce a "U" as a CqlStringMap
impl CqlTransportTypeBuilder<Self,CqlLongString> for String {
    fn to_cql_type(&self) -> CqlLongString {
        let mut bytes = Vec::<u8>::new();
        bytes.write_be_u32(self.len() as u32).unwrap();
        bytes.write_str(self[]).unwrap();
        CqlLongString{bytes:bytes}
    }
}
