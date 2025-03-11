/*!
Send messages to host logger.
*/

use crate::memory;

/// log the message at error level on host logger
pub fn error(msg: &str) {
    log(0, msg)
}

/// log the message at warn level on host logger
pub fn warn(msg: &str) {
    log(1, msg)
}

/// log the message at info level on host logger
pub fn info(msg: &str) {
    log(2, msg)
}

/// log the message at debug level on host logger
pub fn debug(msg: &str) {
    log(3, msg)
}

fn log(level: u32, msg: &str) {
    let p = msg.as_ptr();
    let addr = memory::pack_pointer((p as usize) as u32, msg.len());
    unsafe { _log_host(level, addr) };
}

#[link(wasm_import_module = "host")]
extern "C" {
    /// logs the data (pointed by data_ptr) on level "lvl" using host logger
    #[link_name = "log_msg"]
    fn _log_host(lvl: u32, data_ptr: u64);
}
