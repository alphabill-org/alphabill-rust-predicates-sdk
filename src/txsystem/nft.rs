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

#[cfg(test)]
mod tests {

    use alloc::{string::ToString, vec};

    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn CreateType_from_1() {
        // test generated by Go backend!
        let data = vec![
            0x1, 0x4, 0x2, 0x0, 0x0, 0x0, 0x41, 0x42, 0x2, 0x4, 0xa, 0x0, 0x0, 0x0, 0x74, 0x65,
            0x73, 0x74, 0x20, 0x74, 0x6f, 0x6b, 0x65, 0x6e,
        ];
        assert_eq!(
            CreateType::from(data).unwrap(),
            CreateType {
                name: Some("test token".to_string()),
                symbol: Some("AB".to_string()),
                type_id: None,
            }
        );
        let data = vec![
            0x1, 0x4, 0x6, 0x0, 0x0, 0x0, 0x41, 0x42, 0x2d, 0x4e, 0x46, 0x54, 0x2, 0x4, 0xb, 0x0,
            0x0, 0x0, 0x66, 0x75, 0x6e, 0x6b, 0x79, 0x20, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x3, 0x1,
            0x6, 0x0, 0x0, 0x0, 0x1, 0x2, 0x3, 0x8, 0x9, 0x0,
        ];
        assert_eq!(
            CreateType::from(data).unwrap(),
            CreateType {
                name: Some("funky token".to_string()),
                symbol: Some("AB-NFT".to_string()),
                type_id: Some(vec![0x1, 0x2, 0x3, 0x8, 0x9, 0x0]),
            }
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn Mint_from_1() {
        // test generated by Go backend!
        let data = vec![
            0x1, 0x4, 0xa, 0x0, 0x0, 0x0, 0x74, 0x65, 0x73, 0x74, 0x20, 0x74, 0x6f, 0x6b, 0x65,
            0x6e, 0x4, 0x2, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x5, 0x1, 0x4, 0x0, 0x0, 0x0,
            0x8, 0x7, 0x6, 0x5,
        ];
        assert_eq!(
            Mint::from(data).unwrap(),
            Mint {
                name: Some("test token".to_string()),
                uri: None,
                data: None,
                nonce: Some(1),
                type_id: Some(vec![0x8, 0x7, 0x6, 0x5]),
            }
        );
        let data = vec![
            0x1, 0x4, 0xa, 0x0, 0x0, 0x0, 0x74, 0x65, 0x73, 0x74, 0x20, 0x74, 0x6f, 0x6b, 0x65,
            0x6e, 0x2, 0x4, 0xe, 0x0, 0x0, 0x0, 0x61, 0x62, 0x3a, 0x2f, 0x2f, 0x6e, 0x66, 0x74,
            0x2f, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x3, 0x1, 0x5, 0x0, 0x0, 0x0, 0x64, 0x61, 0x74,
            0x61, 0x21, 0x4, 0x2, 0xe8, 0x3, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x5, 0x1, 0x3, 0x0, 0x0,
            0x0, 0xff, 0xff, 0xff,
        ];
        assert_eq!(
            Mint::from(data).unwrap(),
            Mint {
                name: Some("test token".to_string()),
                uri: Some("ab://nft/token".to_string()),
                data: Some(vec![0x64, 0x61, 0x74, 0x61, 0x21]),
                nonce: Some(1000),
                type_id: Some(vec![0xff, 0xff, 0xff]),
            }
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn Transfer_from_1() {
        // test generated by Go backend!
        let data = vec![
            0x1, 0x1, 0x4, 0x0, 0x0, 0x0, 0x8, 0x7, 0x6, 0x5, 0x2, 0x2, 0x7, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0,
        ];
        assert_eq!(
            Transfer::from(data).unwrap(),
            Transfer {
                counter: Some(7),
                type_id: Some(vec![0x8, 0x7, 0x6, 0x5]),
            }
        );
        let data = vec![
            0x1, 0x1, 0x4, 0x0, 0x0, 0x0, 0x8, 0x7, 0x6, 0x5, 0x2, 0x2, 0x7, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0,
        ];
        assert_eq!(
            Transfer::from(data).unwrap(),
            Transfer {
                counter: Some(7),
                type_id: Some(vec![0x8, 0x7, 0x6, 0x5]),
            }
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn Update_from_1() {
        // test generated by Go backend!
        let data = vec![
            0x1, 0x1, 0x0, 0x0, 0x0, 0x0, 0x2, 0x2, 0x6, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ];
        assert_eq!(
            Update::from(data).unwrap(),
            Update {
                counter: Some(6),
                data: Some(Vec::new()),
            }
        );
        let data = vec![
            0x1, 0x1, 0xd, 0x0, 0x0, 0x0, 0x6e, 0x65, 0x77, 0x20, 0x64, 0x61, 0x74, 0x61, 0x20,
            0x68, 0x65, 0x72, 0x65, 0x2, 0x2, 0x7, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ];
        assert_eq!(
            Update::from(data).unwrap(),
            Update {
                counter: Some(7),
                data: Some(vec![
                    0x6e, 0x65, 0x77, 0x20, 0x64, 0x61, 0x74, 0x61, 0x20, 0x68, 0x65, 0x72, 0x65
                ]),
            }
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn TokenData_from_1() {
        // test generated by Go backend!
        let data = vec![
            0x1, 0x1, 0x3, 0x0, 0x0, 0x0, 0x1, 0x5, 0x0, 0x5, 0x2, 0x5a, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x6, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ];
        assert_eq!(
            TokenData::from(data).unwrap(),
            TokenData {
                type_id: Some(vec![0x1, 0x5, 0x0]),
                name: None,
                uri: None,
                data: None,
                counter: Some(90),
                locked: Some(0),
            }
        );
        let data = vec![
            0x1, 0x1, 0x1, 0x0, 0x0, 0x0, 0x1, 0x2, 0x4, 0x9, 0x0, 0x0, 0x0, 0x68, 0x6f, 0x74,
            0x20, 0x73, 0x74, 0x75, 0x66, 0x66, 0x3, 0x4, 0x7, 0x0, 0x0, 0x0, 0x66, 0x6f, 0x6f,
            0x2f, 0x62, 0x61, 0x72, 0x4, 0x1, 0x3, 0x0, 0x0, 0x0, 0x9, 0x1, 0x1, 0x5, 0x2, 0x5a,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x6, 0x2, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ];
        assert_eq!(
            TokenData::from(data).unwrap(),
            TokenData {
                type_id: Some(vec![0x1]),
                name: Some("hot stuff".to_string()),
                uri: Some("foo/bar".to_string()),
                data: Some(vec![0x9, 0x1, 0x1]),
                counter: Some(90),
                locked: Some(1),
            }
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn TypeData_from_1() {
        // test generated by Go backend!
        let data = vec![
            0x2, 0x4, 0x1, 0x0, 0x0, 0x0, 0x41, 0x3, 0x4, 0x5, 0x0, 0x0, 0x0, 0x61, 0x62, 0x63,
            0x64, 0x65,
        ];
        assert_eq!(
            TypeData::from(data).unwrap(),
            TypeData {
                parent_id: None,
                symbol: Some("A".to_string()),
                name: Some("abcde".to_string()),
            }
        );
        let data = vec![
            0x1, 0x1, 0x4, 0x0, 0x0, 0x0, 0xff, 0x0, 0x7f, 0x80, 0x2, 0x4, 0x3, 0x0, 0x0, 0x0,
            0x6f, 0x68, 0x21, 0x3, 0x4, 0x6, 0x0, 0x0, 0x0, 0x71, 0x77, 0x65, 0x72, 0x74, 0x79,
        ];
        assert_eq!(
            TypeData::from(data).unwrap(),
            TypeData {
                parent_id: Some(vec![0xff, 0x0, 0x7f, 0x80]),
                symbol: Some("oh!".to_string()),
                name: Some("qwerty".to_string()),
            }
        );
    }
}
