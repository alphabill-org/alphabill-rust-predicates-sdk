#![no_std]

use alphabill::{evaluation_ctx, predicate_result};

/**
TODO: description of the predicate

## Returns
 - `0`: predicate evaluated to "true";
 - `1`: predicate evaluated to "false";
*/
#[unsafe(no_mangle)]
pub extern "C" fn {{predicate_name}}() -> u64 {
    let txo = evaluation_ctx::tx_order();
    predicate_result!(true)
}
