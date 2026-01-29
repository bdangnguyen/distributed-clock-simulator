use std::net::TcpStream;

use crate::lamport_clock::LamportClock;

#[derive(Default)]
pub struct Client {
    user_id: String,
    lamport_clock: LamportClock,
}

impl Client {
    pub fn new(user_id: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
            lamport_clock: LamportClock::default(),
        }
    }

    pub fn run(&self, address: &str) {
        let _stream = TcpStream::connect(address).expect("Failed to connect to server");

        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}
