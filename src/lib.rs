#![feature(plugin)]

#![plugin(clippy)]

extern crate openssl;
extern crate protobuf; // depend on rust-protobuf runtime

use std::net::{IpAddr, TcpStream};

use openssl::ssl::{SslContext, SslMethod, SslStream};
use openssl::ssl::HandshakeError;

pub const DEFAULT_PORT: u16 = 64738;
const SSL_HANDSHAKE_RETRIES: u8 = 3;

#[derive(Debug)]
pub enum ConnectionError {
    ExceededHandshakeRetries(&'static str),
    Ssl(openssl::ssl::Error),
    TcpStream(std::io::Error)
} // TODO: this should impl error, display

pub fn connect(host: IpAddr, port: u16) -> Result<SslStream<TcpStream>, ConnectionError> {
    let mut context: SslContext;
    match SslContext::new(SslMethod::Tlsv1) {
        Ok(val) => context = val,
        Err(err) => return Err(ConnectionError::Ssl(openssl::ssl::Error::from(err)))
    }
    // TODO: Investigate this - Since we're given arbitrary hosts, up to user to verify?
    context.set_verify(openssl::ssl::SSL_VERIFY_NONE);
    let stream: TcpStream;
    match TcpStream::connect((host, port)) {
        Ok(val) => stream = val,
        Err(err) => return Err(ConnectionError::TcpStream(err))
    }
    match SslStream::connect(&context, stream) {
        Ok(val) => Ok(val),
        Err(err) => match err {
            HandshakeError::Failure(handshake_err) => Err(ConnectionError::Ssl(handshake_err)),
            HandshakeError::Interrupted(interrupted_stream) => {
                let mut x = interrupted_stream;
                let mut tries: u8 = 1;
                while tries < SSL_HANDSHAKE_RETRIES {
                    match x.handshake() {
                        Ok(val) => return Ok(val),
                        Err(err) => match err {
                            HandshakeError::Failure(handshake_err) => return Err(ConnectionError::Ssl(handshake_err)),
                            HandshakeError::Interrupted(new_interrupted_stream) => {
                                x = new_interrupted_stream;
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