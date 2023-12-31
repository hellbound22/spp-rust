#![cfg_attr(not(test), no_std)]

pub const OCTET: usize = 8;
pub const MAX_DATA_SIZE: usize = 65536;
pub const PRIMARY_HEADER_SIZE: usize = 6 * OCTET;
pub const MAX_SP_SIZE: usize = MAX_DATA_SIZE + PRIMARY_HEADER_SIZE;

pub mod packet;
pub mod pri_header;
pub mod data;
pub mod errors;
