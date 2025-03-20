#![no_std]

use alphabill::{
    api::{SignedByResult, signed_by_pkh},
    cbor,
    decoder::{Decoder, Value},
    error::Error,
    evaluation_ctx, predicate_result,
};

/**
Bearer predicate requiring m out of n signatures.

## Prerequisites
### Configuration
When the predicate is created the "configuration" must be CBOR encoded array where
the first item is number of signatures required (unsigned integer) followed by one
or more Public Key Hash (byte slice), ie

    [ threshold, pkh1, pkh2, ... ]

It's users responsibility to make sure that `0 < threshold <= count(pkh)`.
Up to 255 PKH-s may be provided (IOW predicate supports up to 255 signatures).

When predicate is created with invalid configuration it's behavior is undefined.

### Authorization Proof
The AuthProof of the transaction order must be CBOR array of P2PKH signatures
in the same order as the PKH-s in the configuration. Missing signature must be
represented by nil (`0xf6` in CBOR). The signatures should be encoded as raw byte
buffers (ie not typed arrays but opaque buffers).

## Returns
 - `0`: predicate evaluates to "true" (ie there is at least threshold valid signatures);
 - `1`: predicate evaluates to "false" (not enough valid signatures);
 - `0x0101`: predicate evaluates to "false" because the `threshold > len(pkh)` IOW invalid conf;
 - `0x0c`: failed to load threshold configuration;
 - `0x1c`: number of proofs does not equal to number of PKH-s;
*/
#[unsafe(no_mangle)]
pub extern "C" fn multi_sig() -> u64 {
    // load configuration
    let cfg = cbor::parse_array(evaluation_ctx::HANDLE_CONFIG);
    // read the required signature count value
    let mut p = Decoder::from_handle(cfg[0]);
    let mut threshold = match p.value() {
        Value::U64(v) => v as u8,
        Value::U32(v) => v as u8,
        _ => predicate_result!(Error::new(0x0C)),
    };

    let pkh_handles = &cfg[1..];
    // prepare the proof(s)
    let proof_handles = cbor::parse_array(evaluation_ctx::HANDLE_ARGUMENT);
    if proof_handles.len() != pkh_handles.len() {
        predicate_result!(Error::new(0x1C));
    }

    // iterate over pkh/proofs and verify
    let mut invalid_allowed = pkh_handles.len() - (threshold as usize);
    for (i, pkh) in pkh_handles.iter().enumerate() {
        match signed_by_pkh(evaluation_ctx::HANDLE_TX_ORDER, *pkh, proof_handles[i]) {
            SignedByResult::True => {
                threshold -= 1;
                if 0 == threshold {
                    predicate_result!(true)
                }
            }
            _ => {
                // if it's no longer possible to get threshold valid signatures break early
                if 0 == invalid_allowed {
                    predicate_result!(false)
                }
                invalid_allowed -= 1;
            }
        };
    }
    // actually should never end up here... it must be that threshold is greater than
    // the count of PKH/proofs or zero (and got overflow)?
    predicate_result!(false, 1)
}
