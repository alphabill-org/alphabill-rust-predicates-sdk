/*!
Provides access to the evaluation context of the predicate.
*/

extern crate alloc;
use alloc::vec::Vec;

use crate::{error::Error, memory, txsystem::TxOrder};

/// handle to host variable
pub type ABHandle = u32;

/// handle of the transaction order which triggered the predicate
pub const HANDLE_TX_ORDER: ABHandle = 1;
/// handle of the predicate argument (extracted from the proof).
pub const HANDLE_ARGUMENT: ABHandle = 2;
/// handle of the argument provided when predicate record was created
pub const HANDLE_CONFIG: ABHandle = 3;

/**
Returns current round number of the host transaction system shard.
*/
pub fn current_round() -> u64 {
    unsafe { _current_round() }
}

/**
Returns Unix time, the number of seconds elapsed since January 1, 1970 UTC.
*/
pub fn current_time() -> u64 {
    unsafe { _current_time() }
}

/**
Returns the transaction order which triggered the predicate.
*/
pub fn tx_order() -> Result<TxOrder, Error> {
    TxOrder::from_handle(HANDLE_TX_ORDER)
}

/**
Returns the raw data of the attributes of the given tx order.

Transaction system specific parsing has to be used to extract the data structure,
ie [`ft::tx_attributes`], [`nft::tx_attributes`] or [`money::tx_attributes`].

[`ft::tx_attributes`]: crate::txsystem::token::ft::tx_attributes
[`nft::tx_attributes`]: crate::txsystem::token::nft::tx_attributes
[`money::tx_attributes`]: crate::txsystem::money::tx_attributes
*/
pub fn tx_attributes(txo: &TxOrder, version: u8) -> Vec<u8> {
    let addr = unsafe { _tx_attributes(txo.handle, version) };
    memory::load_bytes(addr)
}

/**
Returns raw data of given unit.

Unit must belong into the same shard the node which evaluates the predicate belongs to.

Returned data can be used as input for a constructor of specific data structure.
*/
pub fn unit_data(unit_id: &Vec<u8>, committed: bool, version: u8) -> Vec<u8> {
    let p = unit_id.as_ptr();
    let addr = memory::pack_pointer((p as usize) as u32, unit_id.len());
    let dp = unsafe { _unit_data(addr, committed, version) };
    memory::load_bytes(dp)
}

/**
Creates variable in the host execution environment.

The variable lives only in the context of the current program.
# Arguments
 * `data` - raw bytes to use as the value of the variable;

Returns handle of the variable;

If the buffer represents some complex data type the [create_obj] api can be
used to "convert" it to "complex structure".
*/
pub fn add_var(data: Vec<u8>) -> ABHandle {
    let p = data.as_ptr();
    let addr = memory::pack_pointer((p as usize) as u32, data.len());
    unsafe { _add_var(addr) }
}

/**
Creates object in the host execution environment.

The object lives only in the context of the current program.
# Arguments
 * `type_id` - identifier of the native Alphabill type;
 * `handle` - the native serialized representation (CBOR) of the object;

Returns handle of the object;
*/
pub fn create_obj(type_id: u16, handle: ABHandle) -> ABHandle {
    // todo: there should also be tx system id param?
    unsafe { _create_obj_h(type_id as u32, handle) }
}

/**
Export object from host side into predicate's memory.

Given handle of a (host side) obj the obj is serialized into memory and the address of
it is returned. The [decoder] can then be used to parse it.

[decoder]: crate::decoder::Decoder
*/
pub fn serialize_obj(handle: ABHandle, version: u8) -> u64 {
    unsafe { _serialize_obj(handle, version) }
}

#[link(wasm_import_module = "context")]
unsafe extern "C" {
    #[link_name = "add_var"]
    fn _add_var(addr: u64) -> ABHandle;

    #[link_name = "create_obj_h"]
    fn _create_obj_h(type_id: u32, handle: ABHandle) -> ABHandle;

    #[link_name = "serialize_obj"]
    fn _serialize_obj(handle: ABHandle, version: u8) -> u64;

    #[link_name = "tx_attributes"]
    fn _tx_attributes(handle: ABHandle, version: u8) -> u64;

    #[link_name = "current_round"]
    fn _current_round() -> u64;

    #[link_name = "now"]
    fn _current_time() -> u64;

    #[link_name = "unit_data"]
    fn _unit_data(unit_id: u64, committed: bool, version: u8) -> u64;
}
