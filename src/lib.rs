extern crate wee_alloc;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod crc;
mod jenkins;
mod soeprotocol;
mod utils;
mod rc4;
pub use crc::*;
pub use jenkins::*;
pub use soeprotocol::*;
pub use utils::*;
pub use rc4::RC4;
