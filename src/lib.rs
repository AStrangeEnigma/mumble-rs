#![feature(plugin)]

#![plugin(clippy)]

extern crate byteorder;
extern crate openssl;
extern crate protobuf; // depend on rust-protobuf runtime

mod proto;
pub mod client;

pub const DEFAULT_PORT: u16 = 64738;
