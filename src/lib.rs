#![cfg_attr(not(test), no_std)]

extern crate alloc;

pub const OCTET: usize = 8;

pub mod packet;
pub mod pri_header;
pub mod data;
