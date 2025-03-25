/*!
Some basic CBOR handling support.
*/
use crate::memory;

use crate::evaluation_ctx::ABHandle;

const FLAG_ARRAY: u32 = 0;
const FLAG_STRUCT: u32 = 1;

/**
Parse CBOR encoded data.

To load the data into predicate memory the [`serialize_obj`] and [`Decoder`] can
be used.

When the CBOR encoded buffer is not a single struct (Alphabill serializes
structs to CBOR as arrays) then the [`parse_array`] can be used to get handle
of each individual element.

[`Decoder`]: crate::decoder::Decoder
[`serialize_obj`]: crate::evaluation_ctx::serialize_obj
*/
pub fn parse(handle: ABHandle) -> ABHandle {
    let addr = unsafe { _parse(handle, FLAG_STRUCT) };
    let (addr, size) = memory::unpack_pointer(addr);
    let data = addr as *const ABHandle;
    let handles = unsafe { core::slice::from_raw_parts(data, size / size_of::<ABHandle>()) };
    handles[0]
}

/**
Parses CBOR encoded array and returns handle to each element.

This function acts the same way as [parse] except when the `handle` points
to (CBOR encoded) array - then it registers each (decoded) array item as
individual host variable and returns handle to it.
*/
pub fn parse_array<'a>(handle: ABHandle) -> &'a [ABHandle] {
    let ptr = unsafe { _parse(handle, FLAG_ARRAY) };
    let (addr, size) = memory::unpack_pointer(ptr);
    let data = addr as *const ABHandle;
    unsafe { core::slice::from_raw_parts(data, size / size_of::<ABHandle>()) }
}

/**
Split CBOR encoded array into "chunks", ie each array item is registered as a new
host variable.

Unlike [`parse_array`] the array items are not decoded, the new variables contain
CBOR encoded data so ie [`create_obj`] could be used to create "native Alphabill"
objects based on the data.

[`create_obj`]: crate::evaluation_ctx::create_obj
*/
pub fn chunks<'a>(handle: ABHandle) -> &'a [ABHandle] {
    let ptr = unsafe { _chunks(handle) };
    let (addr, size) = memory::unpack_pointer(ptr);
    let data = addr as *const ABHandle;
    unsafe { core::slice::from_raw_parts(data, size / size_of::<ABHandle>()) }
}

#[link(wasm_import_module = "cbor")]
unsafe extern "C" {
    #[link_name = "parse"]
    fn _parse(handle: ABHandle, flag: u32) -> u64;

    #[link_name = "chunks"]
    fn _chunks(handle: ABHandle) -> u64;
}
