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

When an proof is not nil and turns out to be invalid (verifying the P2PKH returns
either false or error) the predicate returns "false" even when there is enough
valid signatures to satisfy the threshold (IOW invalid signature is interpreted as veto).

## Returns
 - `0`: predicate evaluates to "true" (ie there is at least threshold valid signatures);
 - `1`: predicate evaluates to "false" (too many signatures missing, early exit);
 - `0xnn01`: false because P2PKH evaluates to "false" or error;
 - `0xff01`: false because not enough positive votes (invalid threshold?);
 - `0x0c`: failed to load threshold configuration;
 - `0x1c`: number of proofs does not equal to number of PKH-s;
*/
#[unsafe(no_mangle)]
pub extern "C" fn multi_sig() -> u64 {
    // load configuration
    let cfg = cbor::parse_array(evaluation_ctx::HANDLE_CONFIG);
    // read the required signature count value
    let mut p = Decoder::from_handle(cfg[0]);
    let threshold = match p.value() {
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
    let mut missing_allowed = pkh_handles.len() - (threshold as usize);
    let mut valid_signatures: u8 = 0;
    for (i, pkh) in pkh_handles.iter().enumerate() {
        match signed_by_pkh(evaluation_ctx::HANDLE_TX_ORDER, *pkh, proof_handles[i]) {
            SignedByResult::True => valid_signatures += 1,
            SignedByResult::NilProof => {
                // if it's no longer possible to get threshold valid signatures break early
                if 0 == missing_allowed {
                    predicate_result!(false)
                }
                missing_allowed -= 1;
            }
            err => predicate_result!(false, err as u64),
        };
    }

    if valid_signatures >= threshold {
        predicate_result!(true)
    }
    predicate_result!(false, 0xff)
}
