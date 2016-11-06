use std::net::{IpAddr, Ipv4Addr};

pub struct ClientOptions {
    pub host: IpAddr,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub tcp_nodelay: bool,
    pub force_tcp_audio: bool,
    pub auto_reconnect: bool
}

impl Default for ClientOptions {
    fn default () -> ClientOptions {
        ClientOptions { host: IpAddr::V4(Ipv4Addr::new(96, 252, 105, 205)), port: ::DEFAULT_PORT, username: String::from("mumble-rs"), password: String::from(""), tcp_nodelay: false, force_tcp_audio: false, auto_reconnect: true }
    }
}
