/*!
Generic Alphabill transaction system support.
*/

use alloc::vec::Vec;
extern crate alloc;

use crate::{
    api, decoder,
    error::{error_code, Error},
    evaluation_ctx, memory,
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
    pub(crate) handle: u64,
}

impl TxOrder {
    pub const TAG_VER: u8 = 1;

    pub fn from_handle(handle: u64) -> Result<TxOrder, Error> {
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
    pub fn signed_by(&self, pkh: &[u8]) -> api::SignedByResult {
        api::signed_by_pkh(self.handle, pkh)
    }
}

pub struct TxRecord {
    tx: Result<TxOrder, u64>, // tx order or it's handle
    //md: Result<ServerMetadata, u64>,
    handle: u64,
}

impl TxRecord {
    pub const TAG_TYPE: u16 = 8;
    pub const TAG_VER: u8 = 1;

    pub fn from<'b>(input: &'b mut [u8]) -> Result<TxRecord, Error> {
        let mut r = Self {
            tx: Err(0),
            handle: 0,
        };
        for fld in decoder::TagValueIter::new(&input) {
            match fld {
                (1, v) => r.tx = Err(v.try_into().or_else(error_code(fld.0))?),
                _ => (), // unknown field to us, ignore
            }
        }
        Ok(r)
    }

    pub fn from_handle(handle: u64) -> Result<TxRecord, Error> {
        let obj_handle = evaluation_ctx::create_obj_from_handle(Self::TAG_TYPE, handle);
        let addr = evaluation_ctx::serialize_obj(obj_handle, Self::TAG_VER);
        let input = memory::byte_array(addr);
        let mut txr = Self::from(input).or_else(error_code(1))?;
        txr.handle = obj_handle;
        Ok(txr)
    }

    pub fn from_cbor(data: Vec<u8>) -> Result<TxRecord, Error> {
        // call back to host with addr and let it return the addr of the wasm encoded data
        let handle = evaluation_ctx::create_obj(Self::TAG_TYPE, data);
        let addr = evaluation_ctx::serialize_obj(handle, Self::TAG_VER);
        let input = memory::byte_array(addr);
        let mut txr = Self::from(input)?;
        txr.handle = handle;
        Ok(txr)
    }

    pub fn get_tx_order(&mut self) -> &TxOrder {
        self.init_tx_order();
        match &self.tx {
            Ok(txo) => &txo,
            Err(_) => {
                panic!("failed to load tx rec")
            }
        }
    }

    fn init_tx_order(&mut self) {
        let handle: u64;
        match self.tx {
            Ok(_) => return,
            Err(h) => {
                handle = h;
            }
        }
        let txo = match TxOrder::from_handle(handle) {
            Ok(txo) => txo,
            Err(err) => panic!("failed to load txo: {:?}", err),
        };
        self.tx = Ok(txo);
    }
}

pub struct TxProof {
    handle: u64,
}

impl TxProof {
    pub const TAG_TYPE: u16 = 9;

    pub fn from_handle(handle: u64) -> Result<TxProof, Error> {
        Ok(Self {
            handle: evaluation_ctx::create_obj_from_handle(Self::TAG_TYPE, handle),
        })
    }

    pub fn verify(&self, tx_rec: &TxRecord) -> bool {
        crate::api::verify_tx_proof(self.handle, tx_rec.handle)
    }
}

#[cfg(test)]
mod tests {

    use alloc::string::String;

    use super::*;

    #[test]
    fn tx_record_from() -> Result<(), String> {
        // load ver 1 of txRecord struct, input generated by Go backend
        let data: &mut [u8] = &mut [0x1, 0x2, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];
        let txr = TxRecord::from(data).unwrap();
        match txr.tx {
            Ok(..) => Err(String::from("expected to have handle, got txo")),
            Err(h) => {
                if h != 1 {
                    return Err(String::from("unexpected handle value"));
                }
                Ok(())
            }
        }
    }

    #[test]
    fn txorder_from() {
        // load ver 1 of txOrder struct, input generated by Go backend:
        // types.Payload{_:struct {}{}, NetworkID:0x0, PartitionID:0x7, UnitID:types.UnitID{0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xa}, Type:0x16, Attributes:types.RawCBOR(nil), StateLock:(*types.StateLock)(nil), ClientMetadata:(*types.ClientMetadata)(0x14000114c40)}
        // &types.ClientMetadata{_:struct {}{}, Timeout:0x0, MaxTransactionFee:0x0, FeeCreditRecordID:[]uint8(nil), ReferenceNumber:[]uint8{0x72, 0x65, 0x66, 0x2d, 0x6e, 0x6f}}
        let data: &mut [u8] = &mut [
            0x1, 0x3, 0x7, 0x0, 0x0, 0x0, 0x2, 0x1, 0xa, 0x0, 0x0, 0x0, 0x1, 0x2, 0x3, 0x4, 0x5,
            0x6, 0x7, 0x8, 0x9, 0xa, 0x3, 0x3, 0x16, 0x0, 0x0, 0x0, 0x4, 0x1, 0x6, 0x0, 0x0, 0x0,
            0x72, 0x65, 0x66, 0x2d, 0x6e, 0x6f,
        ];
        let txo = TxOrder::from(data).unwrap();
        assert_eq!(txo.partition, 7);
        assert_eq!(txo.typ, 22);
        assert_eq!(
            txo.unit_id,
            &[0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xa]
        );
        assert_eq!(
            txo.ref_number.unwrap(),
            &[0x72, 0x65, 0x66, 0x2d, 0x6e, 0x6f]
        );
    }
}
