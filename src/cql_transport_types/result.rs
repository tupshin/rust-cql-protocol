#[repr(C,packed)]
#[allow(non_camel_case_types)]
#[derive(Copy,Show)]
pub enum ResultType {
    VOID=0x0001,
    ROWS=0x0002,
    SET_KEYSPACE=0x0003,
    PREPARED=0x0004,
    SCHEMA_CHANGE=0x0005
}

#[derive(Copy,Show)]
pub struct VoidResult;
pub struct RowsResult {pub bytes:Vec<u8>}
pub struct SetKeyspaceResult {pub bytes:Vec<u8>}
pub struct PreparedResult {pub bytes:Vec<u8>}
pub struct SchemaChangeResult {pub bytes:Vec<u8>}

#[repr(C,packed)]
#[allow(non_camel_case_types)]
pub enum CqlResult {
    VOID(VoidResult),
    ROWS(RowsResult),
    SET_KEYSPACE(SetKeyspaceResult),
    PREPARED(PreparedResult),
    SCHEMA_CHANGE(SchemaChangeResult)
}
