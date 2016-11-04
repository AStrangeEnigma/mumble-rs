use proto;

use byteorder::{BigEndian, WriteBytesExt};

use std;
use std::net::{IpAddr, TcpStream};
use std::sync::Mutex;

use openssl;
use openssl::ssl::{HandshakeError, SslContext, SslMethod, SslStream};

use protobuf;

// Connect
const SSL_HANDSHAKE_RETRIES: u8 = 3;

#[derive(Debug)]
pub enum ConnectionError {
    ExceededHandshakeRetries(&'static str),
    Ssl(openssl::ssl::Error),
    TcpStream(std::io::Error)
} // TODO: this should impl error, display

impl From<openssl::ssl::Error> for ConnectionError {
    fn from(e: openssl::ssl::Error) -> Self {
        ConnectionError::Ssl(e)
    }
}

impl From<openssl::error::ErrorStack> for ConnectionError {
    fn from(e: openssl::error::ErrorStack) -> Self {
        ConnectionError::Ssl(openssl::ssl::Error::from(e))
    }
}

impl From<std::io::Error> for ConnectionError {
    fn from(e: std::io::Error) -> Self {
        ConnectionError::TcpStream(e)
    }
}

#[derive(Debug)]
pub enum SendError {
    MessageTooLarge(&'static str),
    Ssl(openssl::ssl::Error)
} // TODO: this should impl error, display

impl From<openssl::ssl::Error> for SendError {
    fn from(e: openssl::ssl::Error) -> Self {
        SendError::Ssl(e)
    }
}

pub struct Connection {
    control_channel: Mutex<SslStream<TcpStream>>
}

impl Connection {
    pub fn new(host: IpAddr, port: u16, verify: bool, nodelay: bool) -> Result<Connection, ConnectionError> {
        let stream = try!(Connection::connect(host, port, verify, nodelay));
        Ok(Connection { control_channel: Mutex::new(stream) })
    }

    fn connect(host: IpAddr, port: u16, verify: bool, nodelay: bool) -> Result<SslStream<TcpStream>, ConnectionError> {
        let mut context = try!(SslContext::new(SslMethod::Tlsv1));
        if verify {
            context.set_verify(openssl::ssl::SSL_VERIFY_PEER);
        } else {
            context.set_verify(openssl::ssl::SSL_VERIFY_NONE);
        }
        let stream = try!(TcpStream::connect((host, port)));
        // I don't know how this can fail, so just unwrapping for now...
        // TODO: figure this out
        stream.set_nodelay(nodelay).unwrap();
        match SslStream::connect(&context, stream) {
            Ok(val) => Ok(val),
            Err(err) => match err {
                HandshakeError::Failure(handshake_err) => Err(ConnectionError::Ssl(handshake_err)),
                HandshakeError::Interrupted(interrupted_stream) => {
                    let mut ssl_stream = interrupted_stream;
                    let mut tries: u8 = 1;
                    while tries < SSL_HANDSHAKE_RETRIES {
                        match ssl_stream.handshake() {
                            Ok(val) => return Ok(val),
                            Err(err) => match err {
                                HandshakeError::Failure(handshake_err) => return Err(ConnectionError::Ssl(handshake_err)),
                                HandshakeError::Interrupted(new_interrupted_stream) => {
                                    ssl_stream = new_interrupted_stream;
                                    tries += 1;
                                    continue
                                }
                            }
                        }
                    }
                    Err(ConnectionError::ExceededHandshakeRetries("Exceeded number of handshake retries"))
                }
            }
        }
    }

    pub fn version_exchange(&self, version: u32, release: String, os: String, os_version: String) -> Result<usize, SendError> {
        let mut version_message = proto::Version::new();
        version_message.set_version(version);
        version_message.set_release(release);
        version_message.set_os(os);
        version_message.set_os_version(os_version);
        self.send_message(0, version_message)
    }

    // TODO: authentication with tokens
    pub fn authenticate(&self, username: String, password: String) -> Result<usize, SendError> {
        let mut auth_message = proto::Authenticate::new();
        auth_message.set_username(username);
        auth_message.set_password(password);
        // TODO: register 0 celt versions
        auth_message.set_opus(true);
        self.send_message(2, auth_message)
    }

    pub fn ping(&self) -> Result<usize, SendError> {
        let ping_message = proto::Ping::new();
        // TODO: fill the ping with info
        self.send_message(3, ping_message)
    }

    fn send_message<M: protobuf::core::Message>(&self, id: u16, message: M) -> Result<usize, SendError> {
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
        let mut channel = self.control_channel.lock().unwrap();
        match channel.ssl_write(&*packet) {
            Err(err) => Err(SendError::Ssl(err)),
            Ok(val) => Ok(val)
        }
    }

    //fn read_message(&self) -> Result<protobuf::core::Message, ReadError> {
    //}
}


