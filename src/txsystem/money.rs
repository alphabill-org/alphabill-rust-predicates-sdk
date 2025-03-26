/*!
Money transaction system APIs and data structures.
*/

#[cfg(any(feature = "money-transfer", feature = "money-split"))]
extern crate alloc;
#[cfg(any(feature = "money-transfer", feature = "money-split"))]
use alloc::vec::Vec;

#[cfg(any(feature = "money-transfer", feature = "money-split"))]
use crate::{error::error_code, evaluation_ctx};

use crate::{error::Error, txsystem::TxOrder};

pub const SYSTEM_ID: u32 = 1;

#[cfg(feature = "money-transfer")]
const PAYLOAD_TYPE_TRANSFER: u32 = 1;
#[cfg(feature = "money-split")]
const PAYLOAD_TYPE_SPLIT: u32 = 2;

/// different money tx attributes
pub enum TxKind {
    #[cfg(feature = "money-transfer")]
    Transfer(Transfer),
    #[cfg(feature = "money-split")]
    Split(Split),
}

pub fn tx_attributes(txo: &TxOrder) -> Result<TxKind, Error> {
    #[cfg(any(feature = "money-transfer", feature = "money-split"))]
    let data = evaluation_ctx::tx_attributes(txo.handle, 1);
    match txo.typ {
        #[cfg(feature = "money-transfer")]
        PAYLOAD_TYPE_TRANSFER => Ok(TxKind::Transfer(Transfer::from(data)?)),
        #[cfg(feature = "money-split")]
        PAYLOAD_TYPE_SPLIT => Ok(TxKind::Split(Split::from(data)?)),
        _ => Err(Error::new(1)),
    }
}

#[cfg(feature = "money-transfer")]
#[derive(Default)]
pub struct Transfer {
    pub value: u64,
    pub counter: u64,
}

#[cfg(feature = "money-transfer")]
impl Transfer {
    pub fn from(input: Vec<u8>) -> Result<Self, Error> {
        let mut r = Self::default();
        for fld in crate::decoder::TagValueIter::new(&input) {
            match fld {
                (1, v) => r.value = v.try_into().or_else(error_code(1))?,
                (2, v) => r.counter = v.try_into().or_else(error_code(2))?,
                _ => (), // unknown field to us, ignore
            }
        }
        Ok(r)
    }
}

#[cfg(feature = "money-split")]
#[derive(Default)]
pub struct Split {
    //TargetUnits    []*TargetUnit - todo: add field
    pub remaining_value: u64,
    pub counter: u64,
}

#[cfg(feature = "money-split")]
impl Split {
    pub fn from(input: Vec<u8>) -> Result<Self, Error> {
        let mut r = Self::default();
        for fld in crate::decoder::TagValueIter::new(&input) {
            match fld {
                (1, v) => r.remaining_value = v.try_into().or_else(error_code(1))?,
                (2, v) => r.counter = v.try_into().or_else(error_code(2))?,
                _ => (), // unknown field to us, ignore
            }
        }
        Ok(r)
    }
}
