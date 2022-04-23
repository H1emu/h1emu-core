extern crate wee_alloc;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(feature = "crc")]
pub mod crc;
#[cfg(feature = "jenkins")]
pub mod jenkins;
#[cfg(feature = "rc4")]
pub mod rc4;
#[cfg(feature = "soeprotocol")]
pub mod soeprotocol;
#[cfg(feature = "game-utils")]
pub mod utils;
pub mod lib_utils;
