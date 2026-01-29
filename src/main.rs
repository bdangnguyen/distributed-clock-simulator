use std::{thread, time::Duration};

use crate::{client::Client, server::Server};

pub mod client;
pub mod lamport_clock;
pub mod server;

const SERVER_ADDRESS: &str = "127.0.0.1:8080";
const NODE_IDS: [&str; 3] = ["A", "B", "C"];

fn main() {
    thread::spawn(|| {
        let server = Server::new(SERVER_ADDRESS);
        let _ = server.run();
    });

    // Wait for server to start
    thread::sleep(Duration::from_millis(100));

    let handles = NODE_IDS.map(|node_id| {
        thread::spawn(move || {
            let client = Client::new(node_id);
            client.run(SERVER_ADDRESS);
        })
    });

    for handle in handles {
        handle.join().unwrap();
    }
}
