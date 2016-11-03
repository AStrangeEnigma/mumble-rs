extern crate mumble;

use mumble::{Client, ClientOptions};

use std::{thread, time};

#[test]
fn test_connection() {
    // Connect to a local murmur server
    let client = Client::new(ClientOptions::default()).unwrap();
    loop {
        thread::sleep(time::Duration::from_secs(1));
    }
}
