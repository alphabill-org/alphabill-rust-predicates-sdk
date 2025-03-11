#![no_std]

extern crate alloc;
use alloc::vec::Vec;

use alphabill::{
    api::SignedByResult,
    cbor,
    decoder::{self, Value},
    error::Error,
    evaluation_ctx, predicate_result,
};

/**
Time locked owner predicate.

The same as P2PKH but with additional "locked until" date ie the owner is only
recognized after that date.

## Returns
 - `0`: predicate evaluates to "true";
 - `0x0101`: predicate evaluates to "false" because current time is not past unlock time;
 - `0x0801`: false because P2PKH but evaluated to false (likely wrong owner proof);
 - `0x0901`: false because evaluating P2PKH returned error (likely invalid owner proof);
 - `0xnn05`: failed to load tx order;
 - `0xnn0c`: failed to load configuration;
*/
#[no_mangle]
pub extern "C" fn time_lock() -> u64 {
    let cfg = match Config::load() {
        Ok(c) => c,
        Err(err) => predicate_result!(err.chain(0xc)),
    };
    if evaluation_ctx::current_time() < cfg.locked_until {
        predicate_result!(false, 1)
    }

    let txo = match evaluation_ctx::tx_order() {
        Ok(txo) => txo,
        Err(err) => predicate_result!(err.chain(5)),
    };
    match txo.signed_by(&cfg.pkh) {
        SignedByResult::True => predicate_result!(true),
        SignedByResult::False => predicate_result!(false, 8),
        SignedByResult::Error => predicate_result!(false, 9),
    }
}

struct Config {
    /// unit owner (pkh) is recognized only after this date
    locked_until: u64,
    /// public key hash of the owner.
    pkh: Vec<u8>,
}

impl Config {
    fn load() -> Result<Self, Error> {
        // the config BLOB is created at the same time the predicate is created
        // so we can be reasonably sure it is correct (ie right amount of items
        // in correct order and of type) so we do not use tagged encoding, just
        // read positional values...
        // TODO: instead of parsing CBOR each time save the arg as TV encoded
        // blob to begin with!
        let input = cbor::parse(evaluation_ctx::HANDLE_CONFIG);
        Self::from(input)
    }

    fn from(input: &[u8]) -> Result<Self, Error> {
        let mut p = decoder::Decoder::new(input);
        let v = p.value();
        if let Value::Array(items) = v {
            if items.len() != 2 {
                return Err(Error::new(7));
            }
            return Ok(Config {
                locked_until: match items[0] {
                    Value::U64(v) => v,
                    Value::U32(v) => v.into(),
                    _ => return Err(Error::new(1)),
                },
                pkh: match &items[1] {
                    Value::Bytes(v) => v.to_vec(),
                    _ => return Err(Error::new(2)),
                },
            });
        }
        Err(Error::new(6))
    }
}
