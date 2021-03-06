#![feature(plugin)]

#![plugin(clippy)]

extern crate byteorder;
extern crate openssl;
extern crate protobuf; // depend on rust-protobuf runtime

pub mod client;
pub mod options;
mod connection;
mod error;
mod proto;

pub const DEFAULT_PORT: u16 = 64738;

pub use client::Client;
pub use error::Error;
pub use options::ClientOptions;
