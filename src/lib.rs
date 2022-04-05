extern crate wee_alloc;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub mod crc;
pub mod jenkins;
pub mod rc4;
pub mod soeprotocol;
pub mod utils;
