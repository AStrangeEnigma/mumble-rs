extern crate mumble;

use std::net::IpAddr;
use std::net::Ipv4Addr;

#[test]
fn test_connection() {
    // Connect to a local murmur server
    mumble::connect(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), mumble::DEFAULT_PORT).unwrap();
}