/*!
Alphabill specific APIs, ie to verify units etc.
*/

extern crate alloc;
use alloc::vec::Vec;

use crate::memory;

pub fn digest_sha256(msg: Vec<u8>) -> Vec<u8> {
    let p = msg.as_ptr();
    let mut addr = crate::memory::pack_pointer((p as usize) as u32, msg.len());
    addr = unsafe { _digest_sha256(addr) };
    crate::memory::load_bytes(addr)
}

pub fn verify_tx_proof(h_proof: u64, h_tx_rec: u64) -> bool {
    unsafe { _verify_tx_proof(h_proof, h_tx_rec) == 0 }
}

pub enum SignedByResult {
    True,
    False,
    Error,
}

pub fn signed_by_pkh(tx_h: u64, pkh: &[u8]) -> SignedByResult {
    let p = pkh.as_ptr();
    let addr_pkh = memory::pack_pointer((p as usize) as u32, pkh.len());
    let code = unsafe { _tx_signed_by_pkh(tx_h, addr_pkh) };
    match code {
        0 => SignedByResult::True,
        1 => SignedByResult::False,
        _ => SignedByResult::Error,
    }
}

/**
Verify that money transfer has taken place.
# Parameters
- tx_h: handle to raw CBOR containing array of tx proofs, see wallet save proof flag;
- receiver_pkh: PubKey hash to which the money has been transferred to;
- ref_no: if given the transfer(s) must have the given reference number;
*/
pub fn amount_transferred(tx_proof_h: u64, receiver_pkh: &[u8], ref_no: Option<&[u8]>) -> u64 {
    let p = receiver_pkh.as_ptr();
    let addr_pk = memory::pack_pointer((p as usize) as u32, receiver_pkh.len());
    let addr_rn = match ref_no {
        None => 0,
        Some(d) => {
            let p = d.as_ptr();
            memory::pack_pointer((p as usize) as u32, d.len())
        }
    };
    unsafe { _amount_transferred(tx_proof_h, addr_pk, addr_rn) }
}

#[link(wasm_import_module = "ab")]
extern "C" {
    #[link_name = "amount_transferred"]
    fn _amount_transferred(handle: u64, pk: u64, ref_no: u64) -> u64;

    #[link_name = "verify_tx_proof"]
    fn _verify_tx_proof(h_proof: u64, h_tx_rec: u64) -> u32;

    #[link_name = "tx_signed_by_pkh"]
    fn _tx_signed_by_pkh(tx_h: u64, pkh: u64) -> u32;

    #[link_name = "digest_sha256"]
    fn _digest_sha256(addr: u64) -> u64;
}
