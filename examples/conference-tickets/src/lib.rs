/*!
# Transferable Conference Tickets

Predicates for a token type of the "conference X ticket" example use-case.

## Business rules
 - A conference ticket may be bought at an "early-bird" price `P1` up to a given
   date `D1` and at a "regular" price `P2 > P1` after that.
 - A conference ticket may be transferred to another person up to a deadline
   `D2 > D1`; at `D2`, the conference organizer prints the participant materials
   (including badges and other personalized items) and no changes are allowed any more.
 - If the ticket was originally bought as "early-bird" and is being transferred
   after `D1`, it must first be "upgraded" to "regular" by paying the conference
   organizer the price difference `P2-P1`.
*/

#![no_std]

extern crate alloc;
#[cfg(any(
    feature = "type-bearer",
    feature = "token-update-data",
    feature = "token-bearer",
))]
use alloc::vec::Vec;

use alphabill::{evaluation_ctx, predicate_result};

#[cfg(any(feature = "token-update-data", feature = "token-bearer"))]
use alphabill::api::SignedByResult;
#[cfg(any(
    feature = "type-bearer",
    feature = "type-update-data",
    feature = "token-bearer"
))]
use alphabill::txsystem::token::nft;

#[cfg(any(
    feature = "type-bearer",
    feature = "token-update-data",
    feature = "token-bearer",
))]
use alphabill::{
    cbor,
    decoder::{self, Value},
    error::Error,
};

/**
Bearer predicate invariant clause for token type.

Evaluated when NFT is transferred to a new bearer (transfer NFT tx is executed).
Implements business rule:

    token data is "early-bird" and date <= D1
    or
    token data is "regular" and date <= D2

## Arguments
Predicate doesn't expect any arguments (IOW the `InvariantPredicateSignatures` value
in the "transfer NFT" transaction for the predicate must be empty).

## Returns
 - `0`: predicate evaluates to "true";
 - `1`: predicate evaluates to "false";
 - `0xnn02`: failed to load unit's data;
 - `3`: token data field "data" not assigned;
 - `0xnn05`: failed to load tx order;
 - `0xnn0c`: failed to load config arguments;
*/
#[cfg(feature = "type-bearer")]
#[unsafe(no_mangle)]
pub extern "C" fn type_bearer() -> u64 {
    let cfg = match Config::load() {
        Ok(c) => c,
        Err(err) => predicate_result!(err.chain(0xc)),
    };
    let txo = match evaluation_ctx::tx_order() {
        Ok(txo) => txo,
        Err(err) => predicate_result!(err.chain(5)),
    };
    match nft::TokenData::load(&txo.unit_id, false) {
        Ok(data) => {
            let status = match data.data {
                Some(v) => v,
                _ => return 3,
            };
            let cur_dt = evaluation_ctx::current_time();
            if (status == b"early-bird" && cur_dt <= cfg.early_bird_date)
                || (status == b"regular" && cur_dt <= cfg.transferrable_until)
            {
                predicate_result!(true)
            }
            predicate_result!(false)
        }
        Err(err) => predicate_result!(err.chain(2)),
    }
}

/**
Data update predicate invariant clause for token type.

Evaluated when NFT data is updated (update NFT tx is executed).
Implements business rule:

    old data is "early-bird" and new data is "regular"

## Arguments
No arguments. Also doesn't access the "configuration parameters".

## Returns:
 - `0`: predicate evaluated to "true";
 - `2`: new value of the token `data` is not "regular";
 - `0xnn03`: failed to read transaction attributes;
 - `4`: transaction is not of "update NFT" type;
 - `5`: old value of the token `data` is not "early-bird";
 - `0xnn06`: failed to read NFT data from current state;
 - `0a`: the "data" field of the tx attributes (new state) is unassigned;
 - `0b`: the "data" field of the unit (current state) is unassigned;
 - `0xnn0d`: failed to load tx order;
 */
#[cfg(feature = "type-update-data")]
#[unsafe(no_mangle)]
pub extern "C" fn type_update_data() -> u64 {
    let txo = match evaluation_ctx::tx_order() {
        Ok(txo) => txo,
        Err(err) => predicate_result!(err.chain(0x0d)),
    };
    // new data is "regular"...
    match nft::tx_attributes(&txo) {
        Ok(attr) => match attr {
            nft::TxKind::Update(data) => {
                if let Some(d) = data.data {
                    if d != b"regular" {
                        predicate_result!(false, 2)
                    }
                } else {
                    return 0x0a;
                }
            }
            _ => return 4,
        },
        Err(err) => predicate_result!(err.chain(3)),
    }
    // ...and old data is "early-bird"
    match nft::TokenData::load(&txo.unit_id, false) {
        Ok(data) => match data.data {
            Some(v) => {
                if v != b"early-bird" {
                    predicate_result!(false, 5)
                }
            }
            _ => return 0x0b,
        },
        Err(err) => predicate_result!(err.chain(6)),
    }

    predicate_result!(true)
}

/**
Token bearer predicate.

Implements business rule:

    transaction is signed by the conference organizer
        or
    token data is "early-bird" and receipt of payment of P1 to the conference organizer
    and the reference-number in the payment order is the hash of the pair (1, token ID)
        or
    token data is "regular" and receipt of payment of P2 to the conference organizer
    and the reference-number in the payment order is the hash of the pair (1, token ID)

## Arguments
The `TokenCreationPredicateSignatures` of the "mint NFT token" transaction must be CBOR encoded array
of arrays where each (child) array contains of two objects: transaction record and transaction proof
(in that order) of the payment to the conference organizer. The Alphabill CLI wallet has a `proof-output`
flag which saves the transfer proof(s) in this format.

## Returns
 - `0`: predicate evaluates to "true";
 - `1`: predicate evaluates to "false";
 - `0x0801`: false because Bearer predicate is P2PKH and evaluated to false;
 - `0xnn02`: failed to load token attributes;
 - `3`: the data field is missing (token attributes);
 - `0xnn05`: failed to load tx order;
 - `0xnn0c`: failed to load configuration;
*/
#[cfg(feature = "token-bearer")]
#[unsafe(no_mangle)]
pub extern "C" fn token_bearer() -> u64 {
    let cfg = match Config::load() {
        Ok(c) => c,
        Err(err) => predicate_result!(err.chain(0xc)),
    };
    let mut txo = match evaluation_ctx::tx_order() {
        Ok(txo) => txo,
        Err(err) => predicate_result!(err.chain(5)),
    };

    match txo.signed_by(&cfg.pkh) {
        SignedByResult::True => predicate_result!(true),
        SignedByResult::False => predicate_result!(false, 8),
        _ => (),
    }

    let status = match nft::TokenData::load(&txo.unit_id, false) {
        Ok(data) => match data.data {
            Some(v) => v,
            _ => return 3,
        },
        Err(err) => predicate_result!(err.chain(2)),
    };

    // the reference-number in the payment order is the hash of (1, token ID)
    let mut nonce_data: Vec<u8> = alloc::vec![0x01];
    nonce_data.append(&mut txo.unit_id);
    let transferred = check_money_transfer(&cfg.pkh, nonce_data);

    if (status == b"early-bird" && transferred == cfg.early_bird_price)
        || (status == b"regular" && transferred == cfg.regular_price)
    {
        predicate_result!(true)
    }
    predicate_result!(false)
}

/**
Data update predicate invariant clause for token.

Evaluated when NFT data is updated (update NFT tx is executed).
Implements business rule:

    signed by the conference organizer
        or
    date <= D2 and receipt of payment of P2-P1 to the conference organizer and the nonce in
    the payment order is the hash of the pair (2, token ID)

## Arguments
The `DataUpdateSignatures` of the "update NFT token" transaction must be CBOR encoded array
of arrays where each (child) array contains of two objects: transaction record and transaction proof
(in that order) of the payment to the conference organizer. The Alphabill CLI wallet has a `proof-output`
flag which saves the transfer proof(s) in this format.

## Returns:
 - `0`: predicate evaluated to "true";
 - `1`: current date is past the D2 date;
 - `0x0801`: false because Bearer predicate is P2PKH and evaluated to false;
 - `7`: transferred amount doesn't equal to `P2 - P1`;
 - `0xnn0c`: failed to load configuration;
 - `0xnn0d`: failed to load tx order;
 */
#[cfg(feature = "token-update-data")]
#[unsafe(no_mangle)]
pub extern "C" fn token_update_data() -> u64 {
    let cfg = match Config::load() {
        Ok(c) => c,
        Err(err) => predicate_result!(err.chain(0xc)),
    };
    let mut txo = match evaluation_ctx::tx_order() {
        Ok(txo) => txo,
        Err(err) => predicate_result!(err.chain(0x0d)),
    };

    match txo.signed_by(&cfg.pkh) {
        SignedByResult::True => predicate_result!(true),
        SignedByResult::False => predicate_result!(false, 8),
        _ => (),
    }

    // date <= D2
    let cur_dt = evaluation_ctx::current_time();
    if cur_dt > cfg.transferrable_until {
        predicate_result!(false, 1)
    }
    // receipt of payment of P2-P1 to the conference organizer
    // the nonce in the payment order is the hash of the pair (2, token ID)
    let mut nonce_data: Vec<u8> = alloc::vec![0x02];
    nonce_data.append(&mut txo.unit_id);
    let transferred = check_money_transfer(&cfg.pkh, nonce_data);
    if transferred != cfg.regular_price - cfg.early_bird_price {
        predicate_result!(false, 7)
    }
    predicate_result!(true)
}

/**
checks that the money transfer transaction(s) sent in as predicate argument
is valid and returns the amount transferred.
*/
#[cfg(any(feature = "token-update-data", feature = "token-bearer"))]
fn check_money_transfer(pkh: &[u8], nonce_data: Vec<u8>) -> u64 {
    let ref_no = alphabill::api::digest_sha256(nonce_data);
    alphabill::api::amount_transferred(evaluation_ctx::HANDLE_ARGUMENT, pkh, Some(&ref_no))
}

#[cfg(any(
    feature = "token-update-data",
    feature = "token-bearer",
    feature = "type-bearer"
))]
struct Config {
    /// after this date early-bird price ends
    early_bird_date: u64,
    /// tickets can be transferred until this date
    transferrable_until: u64,
    /// early-bird price
    early_bird_price: u64,
    /// regular ticket price
    regular_price: u64,
    /// public key hash of the organizer.
    /// At this time only the p2pkh template is supported so we store
    /// the hash rather than public key
    pkh: Vec<u8>,
}

#[cfg(any(
    feature = "token-update-data",
    feature = "token-bearer",
    feature = "type-bearer"
))]
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
            if items.len() != 5 {
                return Err(Error::new(7));
            }
            return Ok(Config {
                early_bird_date: match items[0] {
                    Value::U64(v) => v,
                    Value::U32(v) => v.into(),
                    _ => return Err(Error::new(1)),
                },
                transferrable_until: match items[1] {
                    Value::U64(v) => v,
                    Value::U32(v) => v.into(),
                    _ => return Err(Error::new(2)),
                },
                early_bird_price: match items[2] {
                    Value::U64(v) => v,
                    Value::U32(v) => v.into(),
                    _ => return Err(Error::new(3)),
                },
                regular_price: match items[3] {
                    Value::U64(v) => v,
                    Value::U32(v) => v.into(),
                    _ => return Err(Error::new(4)),
                },
                pkh: match &items[4] {
                    Value::Bytes(v) => v.to_vec(),
                    _ => return Err(Error::new(5)),
                },
            });
        }
        Err(Error::new(6))
    }
}
