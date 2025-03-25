#[cfg(any(
    feature = "nft-create-type",
    feature = "nft-mint-token",
    feature = "nft-transfer",
    feature = "nft-update"
))]
use alloc::{string::String, vec::Vec};
extern crate alloc;

#[cfg(any(
    feature = "nft-create-type",
    feature = "nft-mint-token",
    feature = "nft-transfer",
    feature = "nft-update"
))]
use crate::{
    decoder::{self},
    error::error_code,
    evaluation_ctx,
};

use crate::{error::Error, txsystem::TxOrder};

#[cfg(feature = "nft-create-type")]
const PAYLOAD_TYPE_NFT_CREATE_TYPE: u32 = 2;
#[cfg(feature = "nft-mint-token")]
const PAYLOAD_TYPE_NFT_MINT: u32 = 4;
#[cfg(feature = "nft-transfer")]
const PAYLOAD_TYPE_NFT_TRANSFER: u32 = 6;
#[cfg(feature = "nft-update")]
const PAYLOAD_TYPE_NFT_UPDATE: u32 = 12;

/// different NFT token tx attributes
pub enum TxKind {
    #[cfg(feature = "nft-create-type")]
    CreateType(CreateType),
    #[cfg(feature = "nft-mint-token")]
    Mint(Mint),
    #[cfg(feature = "nft-transfer")]
    Transfer(Transfer),
    #[cfg(feature = "nft-update")]
    Update(Update),
}

pub fn tx_attributes(txo: &TxOrder) -> Result<TxKind, Error> {
    #[cfg(any(
        feature = "nft-create-type",
        feature = "nft-mint-token",
        feature = "nft-transfer",
        feature = "nft-update"
    ))]
    // right now we have just one version of all the attribute structs so we can
    // load the data here - as soon as some struct needs to support multiple
    // versions we have to load the data for concrete type... see Mint::load()
    let data = evaluation_ctx::tx_attributes(txo, 1);
    match txo.typ {
        #[cfg(feature = "nft-create-type")]
        PAYLOAD_TYPE_NFT_CREATE_TYPE => Ok(TxKind::CreateType(CreateType::from(data)?)),
        #[cfg(feature = "nft-mint-token")]
        PAYLOAD_TYPE_NFT_MINT => Ok(TxKind::Mint(Mint::from(data)?)),
        #[cfg(feature = "nft-transfer")]
        PAYLOAD_TYPE_NFT_TRANSFER => Ok(TxKind::Transfer(Transfer::from(data)?)),
        #[cfg(feature = "nft-update")]
        PAYLOAD_TYPE_NFT_UPDATE => Ok(TxKind::Update(Update::from(data)?)),
        _ => Err(Error::new(0xFF)),
    }
}

#[cfg(feature = "nft-create-type")]
#[cfg_attr(test, derive(PartialEq, Debug))]
#[derive(Default)]
pub struct CreateType {
    pub symbol: Option<String>,
    pub name: Option<String>,
    /// parent type of the type, empty for root type
    /// NB! use Some(empty vec) to distinguish between missing
    /// value and empty value!
    pub type_id: Option<Vec<u8>>,
}

#[cfg(feature = "nft-create-type")]
impl CreateType {
    pub fn from(input: Vec<u8>) -> Result<Self, Error> {
        let mut ct = Self::default();
        for fld in decoder::TagValueIter::new(&input) {
            match fld {
                (1, v) => ct.symbol = v.try_into().or_else(error_code(1))?,
                (2, v) => ct.name = v.try_into().or_else(error_code(2))?,
                (3, v) => ct.type_id = v.try_into().or_else(error_code(3))?,
                _ => (), // unknown field to us, ignore
            }
        }
        Ok(ct)
    }
}

#[cfg(feature = "nft-mint-token")]
#[cfg_attr(test, derive(PartialEq, Debug))]
#[derive(Default)]
pub struct Mint {
    pub type_id: Option<Vec<u8>>,
    pub name: Option<String>,
    pub uri: Option<String>,
    pub data: Option<Vec<u8>>,
    pub nonce: Option<u64>,
}

#[cfg(feature = "nft-mint-token")]
impl Mint {
    // not really needed right now as we have version 1 of all the structs
    // but this is how to do it when we need to support multiple versions?
    pub fn load(txo: &TxOrder) -> Result<Self, Error> {
        let data = evaluation_ctx::tx_attributes(txo, 1);
        Self::from(data)
    }

    pub fn from(input: Vec<u8>) -> Result<Self, Error> {
        let mut r = Self::default();
        for fld in decoder::TagValueIter::new(&input) {
            match fld {
                (1, v) => r.name = v.try_into().or_else(error_code(1))?,
                (2, v) => r.uri = v.try_into().or_else(error_code(2))?,
                (3, v) => r.data = v.try_into().or_else(error_code(3))?,
                (4, v) => r.nonce = v.try_into().or_else(error_code(4))?,
                (5, v) => r.type_id = v.try_into().or_else(error_code(5))?,
                _ => (), // unknown field to us, ignore
            }
        }
        Ok(r)
    }
}

#[cfg(feature = "nft-transfer")]
#[cfg_attr(test, derive(PartialEq, Debug))]
#[derive(Default)]
pub struct Transfer {
    pub type_id: Option<Vec<u8>>,
    pub counter: Option<u64>,
}

#[cfg(feature = "nft-transfer")]
impl Transfer {
    pub fn from(input: Vec<u8>) -> Result<Self, Error> {
        let mut r = Self::default();
        for fld in decoder::TagValueIter::new(&input) {
            match fld {
                (1, v) => r.type_id = v.try_into().or_else(error_code(fld.0))?,
                (2, v) => r.counter = v.try_into().or_else(error_code(fld.0))?,
                _ => (), // unknown field to us, ignore
            }
        }
        Ok(r)
    }
}

/**
Data of the "update NFT token" transaction.
*/
#[cfg(feature = "nft-update")]
#[cfg_attr(test, derive(PartialEq, Debug))]
#[derive(Default)]
pub struct Update {
    pub data: Option<Vec<u8>>,
    pub counter: Option<u64>,
}

#[cfg(feature = "nft-update")]
impl Update {
    pub fn from(input: Vec<u8>) -> Result<Self, Error> {
        let mut r = Self::default();
        for fld in decoder::TagValueIter::new(&input) {
            match fld {
                (1, v) => r.data = v.try_into().or_else(error_code(1))?,
                (2, v) => r.counter = v.try_into().or_else(error_code(2))?,
                _ => (), // unknown field to us, ignore
            }
        }
        Ok(r)
    }
}

/**
Data of an NFT token.

Unit data can be loaded using [`unit_data`] function and then creating the
data structure with [`TokenData::from`] method.

[`unit_data`]: crate::evaluation_ctx::unit_data
*/
#[cfg(feature = "nft-token-data")]
#[cfg_attr(test, derive(PartialEq, Debug))]
#[derive(Default)]
pub struct TokenData {
    pub type_id: Option<Vec<u8>>,
    pub name: Option<String>,
    pub uri: Option<String>,
    pub data: Option<Vec<u8>>,
    pub counter: Option<u64>,
    /// locked status of the token, non-zero value means locked
    pub locked: Option<u64>,
}

#[cfg(feature = "nft-token-data")]
impl TokenData {
    pub const TAG_VER: u8 = 1;

    pub fn load(unit_id: &Vec<u8>, committed: bool) -> Result<Self, Error> {
        let ud = evaluation_ctx::unit_data(unit_id, committed, Self::TAG_VER);
        Self::from(ud)
    }

    pub fn from(input: Vec<u8>) -> Result<Self, Error> {
        let mut r = Self::default();
        for fld in decoder::TagValueIter::new(&input) {
            match fld {
                (1, v) => r.type_id = v.try_into().or_else(error_code(fld.0))?,
                (2, v) => r.name = v.try_into().or_else(error_code(fld.0))?,
                (3, v) => r.uri = v.try_into().or_else(error_code(fld.0))?,
                (4, v) => r.data = v.try_into().or_else(error_code(fld.0))?,
                (5, v) => r.counter = v.try_into().or_else(error_code(fld.0))?,
                (6, v) => r.locked = v.try_into().or_else(error_code(fld.0))?,
                _ => (), // unknown field to us, ignore
            }
        }
        Ok(r)
    }
}

/**
Data of an NFT token type.
*/
#[cfg(feature = "nft-type-data")]
#[cfg_attr(test, derive(PartialEq, Debug))]
#[derive(Default)]
pub struct TypeData {
    /// parent type of the type, empty means "root type"
    pub parent_id: Option<Vec<u8>>,
    pub symbol: Option<String>,
    pub name: Option<String>,
}

#[cfg(feature = "nft-type-data")]
impl TypeData {
    pub const TAG_VER: u8 = 1;

    pub fn load(unit_id: &Vec<u8>, committed: bool) -> Result<Self, Error> {
        let ud = evaluation_ctx::unit_data(unit_id, committed, Self::TAG_VER);
        Self::from(ud)
    }

    pub fn from(input: Vec<u8>) -> Result<Self, Error> {
        let mut r = Self::default();
        for fld in decoder::TagValueIter::new(&input) {
            match fld {
                (1, v) => r.parent_id = v.try_into().or_else(error_code(fld.0))?,
                (2, v) => r.symbol = v.try_into().or_else(error_code(fld.0))?,
                (3, v) => r.name = v.try_into().or_else(error_code(fld.0))?,
                _ => (), // unknown field to us, ignore
            }
        }
        Ok(r)
    }
}

// test module generated by Go backend
#[cfg(test)]
mod nft_test;
