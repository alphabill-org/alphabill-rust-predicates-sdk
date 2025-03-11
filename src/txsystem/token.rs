/*!
Token transaction system APIs and data structures.
*/

/// Fungible- and Non-Fungible tokens share transaction system ID.
pub const SYSTEM_ID: u32 = 2;

/// Fungible Tokens API
#[path = "ft.rs"]
pub mod ft;

/// Non-Fungible Token data structures and APIs.
#[path = "nft.rs"]
pub mod nft;
