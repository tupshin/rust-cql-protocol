#[repr(C, packed)]
#[deriving(Copy)]
pub struct Body<'a> {
    pub bytes:&'a Vec<u8>
}

impl<'b> Body<'b> {
    pub fn build_startup(bytes:&'b Vec<u8>) -> Body<'b>{
     
        //Frame{
        //FIXME. this should resize back to just header length, instead it should directly write the bytes into the correct slice positions
        //framed_header.bytes.resize(9, 0);
        //bytes.push_all(&[0x00, 0x01]); //one as short indicating one k/v in map
        //let mut kv = HashMap::<String,String>::new();
        //kv.insert("CQL_VERSION".to_string(),"3_0_0".to_string());
        //~ bytes.push_all(CQL_VERSION_LEN); //one short indicating length of k
        //~ bytes.push_all(CQL_VERSION); //k
        //~ bytes.push_all(&[0x00, 0x05]); //one short indicating length of v
        //~ bytes.push_all(CQL_3_0_0); //k
        //~ let cqs = 
        //~ foo.write_to(foo);
        //let writer:Writer<CqlString> = Writer::new();
        //let ck = CqlString::write_to
//panic!();
        Body{bytes:bytes}
    }
}
