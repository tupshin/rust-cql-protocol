use std::mem;
use std::raw;


pub unsafe fn raw_byte_repr<'a, T>(ptr: &'a T) -> &'a [u8] {
    mem::transmute(raw::Slice{
    data: ptr as *const _ as *const u8,
        len: mem::size_of::<T>(),
    })
}

