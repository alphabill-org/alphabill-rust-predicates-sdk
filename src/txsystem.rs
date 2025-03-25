/*!
Generic Alphabill transaction system support.
*/

use alloc::vec::Vec;
extern crate alloc;

use crate::{
    api, decoder,
    error::{Error, error_code},
    evaluation_ctx::{self, ABHandle},
    memory,
};

pub mod token;

pub mod money;

#[derive(Default)]
pub struct TxOrder {
    // the partitionID, unitID and txType must always be there, the
    // tx order doesn't make sense without them?
    // IOW they are so fundamental that we do not expect them
    // to go away so do not wrap them into an Option? Zero value
    // of the field is enough to detect missing value in this case...?
    pub partition: u32,
    pub typ: u32,
    pub unit_id: Vec<u8>,
    // ClientMetadata fields
    /// reference number from transaction client metadata
    pub ref_number: Option<Vec<u8>>,
    /// for internal use, mapping the obj to the host variable
    pub(crate) handle: ABHandle,
}

impl TxOrder {
    pub const TAG_VER: u8 = 1;

    pub fn from_handle(handle: ABHandle) -> Result<TxOrder, Error> {
        let addr = evaluation_ctx::serialize_obj(handle, Self::TAG_VER);
        let input = memory::byte_array(addr);
        let mut txo = Self::from(input).or_else(error_code(12))?;
        txo.handle = handle;
        Ok(txo)
    }

    pub fn from(input: &mut [u8]) -> Result<TxOrder, Error> {
        let mut txo = TxOrder::default();
        for fld in decoder::TagValueIter::new(&input) {
            match fld {
                (1, v) => txo.partition = v.try_into().or_else(error_code(fld.0))?,
                (2, v) => txo.unit_id = v.try_into().or_else(error_code(fld.0))?,
                (3, v) => txo.typ = v.try_into().or_else(error_code(fld.0))?,
                (4, v) => txo.ref_number = v.try_into().or_else(error_code(fld.0))?,
                _ => (), // unknown field to us, ignore
            }
        }
        Ok(txo)
    }

    /**
    Attempts to verify the transaction's bearer predicate input (aka OwnerProof)
    as input to P2PKH predicate with given PubKey hash. Returns
    - "True" on success (the transaction is signed by the given PKH);
    - "False" when OwnerProof appears to be valid argument for the P2PKH predicate
      but the predicate evaluated to "false";
    - Error when the input for the host API is invalid (PubKey hash or tx) or the
      OwnerProof is not valid argument for a P2PKH predicate;
    */
    pub fn signed_by(&self, pkh: ABHandle) -> api::SignedByResult {
        api::signed_by_pkh(self.handle, pkh, evaluation_ctx::HANDLE_ARGUMENT)
    }
}
