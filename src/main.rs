use crate::server::Server;

pub mod server;

const SERVER_ADDRESS: &str = "127.0.0.1:8080";

fn main() {
    let server = Server::new(SERVER_ADDRESS);
    let _ = server.run();
}
