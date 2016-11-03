use std::net::{IpAddr, Ipv4Addr};

pub struct ClientOptions {
    pub host: IpAddr,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub validate_host_cert: bool,
    pub tcp_nodelay: bool,
    pub force_tcp_audio: bool
}

impl Default for ClientOptions {
    fn default () -> ClientOptions {
        ClientOptions { host: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port: ::DEFAULT_PORT, username: String::from("mumble-rs"), password: String::from(""), validate_host_cert: false, tcp_nodelay: false, force_tcp_audio: false }
    }
}
