extern crate mumble;

use mumble::client::MumbleClient;

use std::{thread, time};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[test]
fn test_all() {
    // Connect to a local murmur server
    let mut connection = MumbleClient::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)), mumble::DEFAULT_PORT, "mumble-rs", "nil").unwrap();
    loop {
        thread::sleep(time::Duration::from_secs(1));
    }
}
