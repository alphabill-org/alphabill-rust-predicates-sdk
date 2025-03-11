/*!
Some basic CBOR handling support.
*/
use crate::memory;

/**
parses the memory buffer pointed by handle on the host side as CBOR
and returns it's content. The CBOR should contain "plain old data"
types only!
 */
pub fn parse<'a>(handle: u64) -> &'a [u8] {
    let addr = unsafe { _parse(handle) };
    memory::byte_array(addr)
}

/* *
cbor_parse_array assumes that `handle` points to CBOR encoded array and decodes it to
raw CBOR elements. Each item in the array is added to host's variable list and it's
handle is added to the vector returned by the function.
* /
pub fn cbor_parse_array(handle: u64) -> Vec<u64> {
    let addr = unsafe { _cbor_parse_array(handle) };
    let input = memory::byte_array(addr);
    let mut p = decoder::Decoder::new(input);
    let mut r = Vec::new();
    (0..p.uint32()).for_each(|_| {
        r.push(p.uint64());
    });
    r
}*/

#[link(wasm_import_module = "cbor")]
extern "C" {
    #[link_name = "parse"]
    fn _parse(handle: u64) -> u64;

    //#[link_name = "cbor_parse_array"]
    //fn _cbor_parse_array(handle: u64) -> u64;
}
