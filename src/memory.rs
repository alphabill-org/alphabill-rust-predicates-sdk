/*!
Memory management support.
*/

use alloc::vec::Vec;
extern crate alloc;

/// return (address, size) pair for the "packed pointer"
pub fn unpack_pointer(ptr: u64) -> (u32, usize) {
    ((ptr & 0xFFFFFFFF) as u32, (ptr >> 32) as usize)
}

/// pack address and data size (in bytes) as u64 "packed pointer"
pub fn pack_pointer(addr: u32, size: usize) -> u64 {
    ((size as u64) << 32) | (addr as u64)
}

/// given the "packed pointer" in shared memory returns "Rust pointer"
/// to the data as u8 slice.
pub fn byte_array<'a>(ptr: u64) -> &'a mut [u8] {
    let (addr, size) = unpack_pointer(ptr);
    let data: *mut u8 = addr as *mut u8;
    let buf: &mut [u8] = unsafe { core::slice::from_raw_parts_mut(data, size) };
    buf
}

pub fn load_bytes(ptr: u64) -> Vec<u8> {
    byte_array(ptr).to_vec()
}

#[link(wasm_import_module = "host")]
unsafe extern "C" {
    /// returns address of allocated block.
    #[link_name = "ext_malloc"]
    fn _ext_malloc(size: u32) -> u32;

    /// frees memory allocated on given address.
    #[link_name = "ext_free"]
    fn _ext_free(addr: u32);
}

#[cfg(not(test))]
#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOCATOR: WasmAllocator = WasmAllocator;

#[cfg(target_arch = "wasm32")]
struct WasmAllocator;

#[cfg(target_arch = "wasm32")]
mod allocator_impl {
    use super::*;
    use core::alloc::{GlobalAlloc, Layout};

    unsafe impl GlobalAlloc for WasmAllocator {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            unsafe { _ext_malloc(layout.size() as u32) as *mut u8 }
        }

        unsafe fn dealloc(&self, ptr: *mut u8, _: Layout) {
            unsafe { _ext_free(ptr as u32) }
        }
    }
}
