use crate::http::Request;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::io::Read;
use std::net::TcpListener;
pub struct Server {
    addr: String,
}
impl Server {
    // Associated function
    pub fn new(addr: String) -> Self {
        // Create & return a struct:
        Server { addr }
    }
    // Method, with 'self', the method takes ownership of the struct:
    pub fn run(self) {
        println!("Listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        // followng is the same as 'while true':
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024]; // create and allocate a buffer array with 1024 elements
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            match Request::try_from(&buffer as &[u8]) {
                                Ok(request) => {}
                                Err(e) => println!("Failed to parse a request {}", e),
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}
