use std::collections::HashMap;

//~ pub fn hashmap2cqlmap(map:HashMap<&str,&str>) -> Vec<u8> {
    //~ let mut bytes = Vec::<u8>::new();
    //~ bytes.write_be_u16(map.len() as u16); //one as short indicating one k/v in map
    //~ for (key,value) in map.iter() {
        //~ bytes.write_be_u16(key.len() as u16); //one short indicating length of k
    //~ bytes.write_str(key[]);
    //~ bytes.write_be_u16(value.len() as u16); //one short indicating length of v
    //~ bytes.write_str(value[]); 
    //~ }
    //~ bytes
//~ }

pub type CqlStringMap = Vec<u8>;
 
///for any type that implements CqlTransportTypeBuilder, it must have a build() function that converts
///from type T to type U, and that it is up to the implementation of each type CqlTransportType what its
///respective types for T and U are.
pub trait CqlTransportTypeBuilder<T,U> {
    fn to_cql_type(&self) -> U;
}

///A builder for the CqlStringMap type must take as "T" type HashMap<String,String> and
///produce a "U" as a CqlStringMap
impl CqlTransportTypeBuilder<HashMap<String,String>,CqlStringMap> for HashMap<String,String> {
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
                bytes
            } 
        }
    }
}
