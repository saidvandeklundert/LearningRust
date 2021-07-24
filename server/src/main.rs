use http::Method;
use http::Request;
use server::Server;
mod http;
mod server;

fn main() {
    // .to_string() takes a string literal and
    //   turns it into a String that is allocated on the heap.
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}
