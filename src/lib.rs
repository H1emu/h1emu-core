#![feature(cursor_remaining)]
#[cfg(feature = "crc")]
pub mod crc;
#[cfg(feature = "crc")]
mod crc_table;
#[cfg(feature = "gatewayprotocol")]
pub mod gatewayprotocol;
#[cfg(feature = "gatewayprotocol")]
pub mod gatewayprotocol_packets_structs;
#[cfg(feature = "jenkins")]
pub mod jenkins;
pub mod lib_utils;
#[cfg(feature = "protocols")]
pub mod protocol_errors;
#[cfg(feature = "rc4")]
pub mod rc4;
#[cfg(feature = "soeprotocol")]
pub mod soeprotocol;
#[cfg(feature = "soeprotocol")]
pub mod soeprotocol_functions;
#[cfg(feature = "soeprotocol")]
pub mod soeprotocol_packets_structs;
#[cfg(feature = "game-utils")]
pub mod utils;
