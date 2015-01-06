#[repr(C,packed)]
#[allow(non_camel_case_types)]
#[derive(Copy,Show)]
pub enum Consistency {
    ANY=0x0000,
    ONE=0x0001,
    TWO=0x0002,
    THREE=0x0003,
    QUORUM=0x0004,
    ALL=0x0005,
    LOCAL_QUORUM=0x0006,
    EACH_QUORUM=0x0007,
    SERIAL=0x0008,
    LOCAL_SERIAL=0x0009,
    LOCAL_ONE=0x000A
}
