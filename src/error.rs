use connection::{ConnectionError, SendError};

#[derive(Debug)]
pub enum Error {
    Connection(ConnectionError),
    Send(SendError)
} // TODO: this should impl error, display

impl From<ConnectionError> for Error {
    fn from(e: ConnectionError) -> Self {
        Error::Connection(e)
    }
}

impl From<SendError> for Error {
    fn from(e: SendError) -> Self {
        Error::Send(e)
    }
}
