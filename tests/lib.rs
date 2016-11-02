extern crate mumble;

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[test]
fn test_connection() {
    // Connect to a local murmur server
    mumble::connect(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), mumble::DEFAULT_PORT).unwrap();
    // Connect to the same server ipv6
    mumble::connect(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)), mumble::DEFAULT_PORT).unwrap();
}
