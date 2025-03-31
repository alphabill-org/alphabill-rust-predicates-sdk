#![no_std]

use alphabill::{
    api::{SignedByResult, signed_by_pkh},
    cbor,
    decoder::{Decoder, Value},
    evaluation_ctx, predicate_result,
};

/**
Time locked bearer predicate.

The same as P2PKH but with additional "locked until" date ie the owner is only
recognized after that date.

## Prerequisites
### Configuration
When the predicate is created the "configuration" must be provided as CBOR encoded
array where the first item is locked until date (unsigned integer) followed by the
Public Key Hash of the bearer (byte slice), ie

    [ locked_until_date, pkh ]

### Authorization Proof
The AuthProof of the transaction order must be the same as P2PKH predicate template
uses.

## Returns
 - `0`: predicate evaluates to "true";
 - `0xff01`: predicate evaluates to "false" because current time is not past unlock time;
 - `0xnn01`: false because evaluating P2PKH returned false or error;
 - `0x0c`: failed to load locked_until date from configuration (not uint64?);
*/
#[unsafe(no_mangle)]
pub extern "C" fn time_lock() -> u64 {
    let cfg_handles = cbor::parse_array(evaluation_ctx::HANDLE_CONFIG);
    // first handle refers to the locked until date
    let mut p = Decoder::from_handle(cfg_handles[0]);
    let locked_until = match p.value() {
        Value::U64(v) => v,
        Value::U32(v) => v as u64,
        _ => predicate_result!(false, 0x0c),
    };

    if evaluation_ctx::current_time() < locked_until {
        predicate_result!(false, 0xff)
    }

    match signed_by_pkh(
        evaluation_ctx::HANDLE_TX_ORDER,
        cfg_handles[1],
        evaluation_ctx::HANDLE_ARGUMENT,
    ) {
        SignedByResult::True => predicate_result!(true),
        err => predicate_result!(false, err as u64),
    }
}
