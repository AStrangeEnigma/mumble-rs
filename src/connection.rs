use proto;

use byteorder::{BigEndian, WriteBytesExt};

use std;
use std::io::Write;
use std::net::{IpAddr, TcpStream};
use std::sync::{Arc, Mutex};

use protobuf;

use rustls;
use rustls::Session;
use webpki_roots;

// Connect
const SSL_HANDSHAKE_RETRIES: u8 = 3;

#[derive(Debug)]
pub enum ConnectionError {
    ExceededHandshakeRetries(&'static str),
    TcpStream(std::io::Error)
} // TODO: this should impl error, display

impl From<std::io::Error> for ConnectionError {
    fn from(e: std::io::Error) -> Self {
        ConnectionError::TcpStream(e)
    }
}

#[derive(Debug)]
pub enum SendError {
    MessageTooLarge(&'static str),
} // TODO: this should impl error, display

pub struct Connection {
    control_channel: TcpStream,
    tls_session: rustls::ClientSession
}

impl Connection {
    /*
    pub fn new(host: IpAddr, port: u16, nodelay: bool) -> Result<Connection, ConnectionError> {
        let stream = try!(Connection::connect(host, port, verify, nodelay));
        Ok(Connection { control_channel: Mutex::new(stream) })
    }*/

    pub fn connect(host: IpAddr, port: u16, nodelay: bool) -> Result<Connection, ConnectionError> {
        let stream = try!(TcpStream::connect((host, port)));
        // I don't know how this can fail, so just unwrapping for now...
        // TODO: figure this out
        stream.set_nodelay(nodelay).unwrap();
        let mut config = rustls::ClientConfig::new();
        config.root_store.add_trust_anchors(&webpki_roots::ROOTS);
        let rc_config = Arc::new(config);
        // TODO constant
        let mut client = rustls::ClientSession::new(&rc_config, "brick.codes");
        Ok(Connection { control_channel: stream, tls_session: client })
    }

    pub fn version_exchange(&mut self, version: u32, release: String, os: String, os_version: String) -> Result<(), SendError> {
        let mut version_message = proto::Version::new();
        version_message.set_version(version);
        version_message.set_release(release);
        version_message.set_os(os);
        version_message.set_os_version(os_version);
        self.send_message(0, version_message)
    }

    // TODO: authentication with tokens
    pub fn authenticate(&mut self, username: String, password: String) -> Result<(), SendError> {
        let mut auth_message = proto::Authenticate::new();
        auth_message.set_username(username);
        auth_message.set_password(password);
        // TODO: register 0 celt versions
        auth_message.set_opus(true);
        self.send_message(2, auth_message)
    }

    pub fn ping(&mut self) -> Result<(), SendError> {
        let ping_message = proto::Ping::new();
        // TODO: fill the ping with info
        self.send_message(3, ping_message)
    }

    fn send_message<M: protobuf::core::Message>(&mut self, id: u16, message: M) -> Result<(), SendError> {
        let mut packet = vec![];
        // ID - what type of message are we sending
        packet.write_u16::<BigEndian>(id).unwrap();
        let payload = message.write_to_bytes().unwrap();
        if payload.len() as u64 > u32::max_value() as u64  {
            return Err(SendError::MessageTooLarge("Payload too large to fit in one packet!"))
        }
        // The length of the payload
        packet.write_u32::<BigEndian>(payload.len() as u32).unwrap();
        // The payload itself
        packet.extend(payload);
        // Panic on poisoned mutex - this is desired (because could only be poisoned from panic)
        // https://doc.rust-lang.org/std/sync/struct.Mutex.html#poisoning
        self.tls_session.write(&*packet).unwrap();
        self.tls_session.write_tls(&mut self.control_channel).unwrap();
        Ok(())
    }

    pub fn break_pls(&mut self) {
        self.tls_session.process_new_packets().unwrap();
    }

    //fn read_message(&self) -> Result<protobuf::core::Message, ReadError> {
    //}
}


