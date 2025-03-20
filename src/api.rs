/*!
Alphabill specific APIs, ie to verify units etc.
*/

extern crate alloc;
use alloc::vec::Vec;

use crate::{evaluation_ctx::ABHandle, memory};

pub fn digest_sha256(msg: Vec<u8>) -> Vec<u8> {
    let p = msg.as_ptr();
    let mut addr = crate::memory::pack_pointer((p as usize) as u32, msg.len());
    addr = unsafe { _digest_sha256(addr) };
    crate::memory::load_bytes(addr)
}

pub fn verify_tx_proof(tx_proof: ABHandle, tx_rec: ABHandle) -> bool {
    unsafe { _verify_tx_proof(tx_proof, tx_rec) == 0 }
}

pub enum SignedByResult {
    True,
    False,
    Error,
}

pub fn signed_by_pkh(txo: ABHandle, pkh: ABHandle, proof: ABHandle) -> SignedByResult {
    let code = unsafe { _tx_signed_by_pkh(txo, pkh, proof) };
    match code {
        0 => SignedByResult::True,
        1 => SignedByResult::False,
        _ => SignedByResult::Error,
    }
}

/**
Verify that money transfer has taken place.
# Parameters
- tx_proof: handle to raw CBOR containing array of tx proofs, see wallet save proof flag;
- receiver_pkh: PubKey hash to which the money has been transferred to;
- ref_no: if given the transfer(s) must have the given reference number;
*/
pub fn amount_transferred(tx_proof: ABHandle, receiver_pkh: ABHandle, ref_no: Option<&[u8]>) -> u64 {
    let addr_rn = match ref_no {
        None => 0,
        Some(d) => {
            let p = d.as_ptr();
            memory::pack_pointer((p as usize) as u32, d.len())
        }
    };
    unsafe { _amount_transferred(tx_proof, receiver_pkh, addr_rn) }
}

#[link(wasm_import_module = "ab")]
unsafe extern "C" {
    #[link_name = "amount_transferred"]
    fn _amount_transferred(tx_proof: ABHandle, pkh: ABHandle, ref_no: u64) -> u64;

    #[link_name = "verify_tx_proof"]
    fn _verify_tx_proof(h_proof: ABHandle, h_tx_rec: ABHandle) -> u32;

    #[link_name = "tx_signed_by_pkh"]
    fn _tx_signed_by_pkh(txo: ABHandle, pkh: ABHandle, proof: ABHandle) -> u32;

    #[link_name = "digest_sha256"]
    fn _digest_sha256(addr: u64) -> u64;
}
