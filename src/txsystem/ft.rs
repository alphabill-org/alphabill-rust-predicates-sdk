extern crate alloc;
#[cfg(feature = "ft-create-type")]
use alloc::string::String;
#[cfg(any(
    feature = "ft-create-type",
    feature = "ft-mint-token",
    feature = "ft-transfer",
))]
use alloc::vec::Vec;

#[cfg(any(
    feature = "ft-create-type",
    feature = "ft-mint-token",
    feature = "ft-transfer",
))]
use crate::{decoder::Decoder, evaluation_ctx};

use crate::{error::Error, txsystem::TxOrder};

/*
    TransactionTypeDefineFT    uint16 = 1
    TransactionTypeMintFT      uint16 = 3
    TransactionTypeTransferFT  uint16 = 5
    TransactionTypeLockToken   uint16 = 7
    TransactionTypeUnlockToken uint16 = 8
    TransactionTypeSplitFT     uint16 = 9
    TransactionTypeBurnFT      uint16 = 10
    TransactionTypeJoinFT      uint16 = 11
*/

#[cfg(feature = "ft-create-type")]
const PAYLOAD_TYPE_FT_CREATE_TYPE: u32 = 1;
#[cfg(feature = "ft-mint-token")]
const PAYLOAD_TYPE_FT_MINT: u32 = 3;
#[cfg(feature = "ft-transfer")]
const PAYLOAD_TYPE_FT_TRANSFER: u32 = 5;

/// different fungible token tx attributes
pub enum TxKind {
    #[cfg(feature = "ft-create-type")]
    CreateType(CreateType),
    #[cfg(feature = "ft-mint-token")]
    Mint(Mint),
    #[cfg(feature = "ft-transfer")]
    Transfer(Transfer),
}

pub fn tx_attributes(txo: &TxOrder) -> Result<TxKind, Error> {
    #[cfg(any(
        feature = "ft-create-type",
        feature = "ft-mint-token",
        feature = "ft-transfer",
    ))]
    let data = evaluation_ctx::tx_attributes(txo.handle, 1);
    match txo.typ {
        #[cfg(feature = "ft-create-type")]
        PAYLOAD_TYPE_FT_CREATE_TYPE => Ok(TxKind::CreateType(CreateType::from(data)?)),
        #[cfg(feature = "ft-mint-token")]
        PAYLOAD_TYPE_FT_MINT => Ok(TxKind::Mint(Mint::from(data)?)),
        #[cfg(feature = "ft-transfer")]
        PAYLOAD_TYPE_FT_TRANSFER => Ok(TxKind::Transfer(Transfer::from(data)?)),
        _ => Err(Error::new(1)),
    }
}

#[cfg(feature = "ft-create-type")]
pub struct CreateType {
    pub symbol: String,
    pub name: String,
    /// parent type of the type, empty for root type
    pub type_id: Vec<u8>,
    pub decimals: u32,
}

#[cfg(feature = "ft-create-type")]
impl CreateType {
    pub fn from(input: Vec<u8>) -> Result<Self, Error> {
        let mut p = Decoder::new(&input);
        Ok(Self {
            symbol: p.string(),
            name: p.string(),
            type_id: p.bytes(),
            decimals: p.uint32(),
        })
    }
}

#[cfg(feature = "ft-mint-token")]
pub struct Mint {
    pub type_id: Vec<u8>,
    pub value: u64,
    pub nonce: Vec<u8>,
}

#[cfg(feature = "ft-mint-token")]
impl Mint {
    pub fn from(input: Vec<u8>) -> Result<Self, Error> {
        let mut p = Decoder::new(&input);
        Ok(Self {
            type_id: p.bytes(),
            value: p.uint64(),
            nonce: p.bytes(),
        })
    }
}

#[cfg(feature = "ft-transfer")]
pub struct Transfer {
    pub value: u64,
    pub type_id: Vec<u8>,
    pub counter: u64,
}

#[cfg(feature = "ft-transfer")]
impl Transfer {
    pub fn from(input: Vec<u8>) -> Result<Self, Error> {
        let mut p = Decoder::new(&input);
        Ok(Self {
            value: p.uint64(),
            type_id: p.bytes(),
            counter: p.uint64(),
        })
    }
}
