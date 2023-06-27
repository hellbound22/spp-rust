#![cfg_attr(not(test), no_std)]

extern crate alloc;

pub const OCTET: usize = 8;
pub const MAX_DATA_SIZE: usize = 65536;
pub const PRIMARY_HEADER: usize = 6 * OCTET;

pub mod packet;
pub mod pri_header;
pub mod data;
pub mod errors;
