use connection::{Connection, SendError};
use error;
use options::ClientOptions;

use std::net::IpAddr;
use std::sync::Arc;
use std::{thread, time};

// Version Exchange
const VERSION_RELEASE_PREFIX: &'static str = "mumble-rs";
const VERSION_RELEASE: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
// These sizes are important, and correspond to the number of bytes sent in the Version message
const VERSION_MAJOR: u16 = 1;
const VERSION_MINOR: u8 = 3;
const VERSION_PATCH: u8 = 0;

// Ping thread
const PING_INTERVAL: u64 = 5; // (in seconds)

pub struct Client {
    connection: Connection,
    options: ClientOptions
}

// TODO: use force TCP option
impl Client {
    pub fn new(options: ClientOptions) -> Result<Client, error::Error> {
        let mut connection = try!(Client::establish_connection(options.host, options.port, options.username.clone(), options.password.clone(), options.tcp_nodelay));
        let mut client = Client { connection: connection, options: options };
        Ok(client)
    }

    fn establish_connection(host: IpAddr, port: u16, username: String, password: String, tcp_nodelay: bool) -> Result<Connection, error::Error> {
        let mut connection = try!(Connection::connect(host, port, tcp_nodelay));
        //try!(client.auto_reconnect(&Client::version_exchange));
        try!(Client::version_exchange(&mut connection));
        try!(connection.authenticate(username, password));
        Ok(connection)
    }

    // Version exchange with our client version (matching mumble client latest)
    // as well as mumble-rs release info
    fn version_exchange(connection: &mut Connection) -> Result<(), SendError> {
        let major = (VERSION_MAJOR as u32) << 16;
        let minor = (VERSION_MINOR as u32) << 8;
        let patch = VERSION_PATCH as u32;
        // TODO: os and os version (some sort of cross platform uname needed)
        connection.version_exchange(major | minor | patch,
                              format!("{} {}", VERSION_RELEASE_PREFIX, VERSION_RELEASE.unwrap_or("Unknown")),
                              String::from("DenialAdams OS"),
                              String::from("1.3.3.7"))
    }

    pub fn break_pls(&mut self) {
        self.connection.break_pls();
    }

    // TODO WIP
    /*
    fn auto_reconnect(&mut self, method: &Fn(&Client) -> Result<(), SendError>) -> Result<(), SendError> {
        let mut retry_attempts: u8 = 0;
        while retry_attempts <  3 {
            match method(self) {
                Err(e) => match e {
                    SendError::Ssl(ssl_error) => match ssl_error {
                        openssl::ssl::Error::ZeroReturn => {
                            // We've been disconnected
                            // call our thing again (oh god)
                        },
                        _ => return Err(SendError::Ssl(ssl_error))
                    },
                    _ => return Err(e)
                },
                Ok(val) => return Ok(val)
            }
            retry_attempts += 1;
            // Wait a few seconds before retrying
            thread::sleep(time::Duration::from_secs(PING_INTERVAL));
        }
        // out of attempts, can't reconnect
        // TODO TEMP
        Err(SendError::MessageTooLarge("aaah"))
    } */
}
