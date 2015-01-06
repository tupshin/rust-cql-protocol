use cql_transport_types::CqlLongString;

#[repr(C,packed)]
#[allow(non_camel_case_types)]
#[derive(Copy,Show)]
pub enum QueryFlags {
    NONE=0x00,
    VALUES=0x01,
    SKIP_METADATA=0x02,
    PAGE_SIZE=0x04,
    WITH_PAGING_STATE=0x08,
    WITH_SERIAL_CONSISTENCY=0x10,
    WITH_DEFAULT_TIMESTAMP=0x20,
    WITH_NAMES_FOR_VALUES=0x40
}

pub type Query = CqlLongString; 
