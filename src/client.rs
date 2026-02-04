use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;

use serde::{Deserialize, Serialize};

use crate::lamport_clock::LamportClock;

#[derive(Serialize, Deserialize)]
struct SendMessageRequest {
    content: String,
    node_id: String,
    time: u64,
}

#[derive(Default, Clone)]
pub struct Client {
    node_id: String,
    lamport_clock: Arc<Mutex<LamportClock>>,
}

impl Client {
    pub fn new(node_id: &str) -> Self {
        Self {
            node_id: node_id.to_string(),
            lamport_clock: Arc::new(Mutex::new(LamportClock::default())),
        }
    }

    pub fn run(&self, address: &str) {
        let stream = TcpStream::connect(address).expect("Failed to connect to server");

        println!("Node {} connected to server: {}", self.node_id, address);

        // Handle reading from server in a different thread.
        let client_clone = self.clone();
        let reader = BufReader::new(stream);
        let receiver_handler = thread::spawn(move || {
            client_clone.receive_message(reader);
        });

        let _ = receiver_handler.join();
    }

    fn receive_message(&self, reader: impl BufRead) {
        for line in reader.lines() {
            match line {
                Ok(message) => {
                    if let Ok(request) = serde_json::from_str::<SendMessageRequest>(&message) {
                        self.lamport_clock.lock().unwrap().update(request.time);
                        println!(
                            "Node {} received message: {} at {}",
                            self.node_id,
                            request.content,
                            self.lamport_clock.lock().unwrap().time
                        );
                    }
                }
                Err(e) => {
                    eprintln!("Node {} failed to receive message: {}", self.node_id, e);
                    break;
                }
            }
        }
    }

    fn send_message(&self, mut stream: impl Write, message: String) {
        self.lamport_clock.lock().unwrap().tick();
        let request = SendMessageRequest {
            content: message,
            node_id: self.node_id.clone(),
            time: self.lamport_clock.lock().unwrap().time,
        };
        let message = serde_json::to_string(&request).unwrap();

        writeln!(stream, "{}", message)
            .and_then(|_| stream.flush())
            .unwrap_or_else(|e| {
                eprintln!("Failed to send message: {}", e);
            });
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const NODE_ID1: &str = "A";
    const NODE_ID2: &str = "B";
    const MESSAGE: &str = "Hello World!";

    #[test]
    fn test_receive_message() {
        let client = Client::new(NODE_ID1);
        let request = SendMessageRequest {
            content: MESSAGE.to_string(),
            node_id: NODE_ID2.to_string(),
            time: 10,
        };
        let mut message = serde_json::to_string(&request).unwrap();
        message.push('\n');
        let reader = BufReader::new(message.as_bytes());

        client.receive_message(reader);

        assert_eq!(client.lamport_clock.lock().unwrap().time, 11);
    }

    #[test]
    fn test_send_message() {
        let client = Client::new(NODE_ID1);
        let mut stream = Vec::new();

        client.send_message(&mut stream, MESSAGE.to_string());

        let output = String::from_utf8(stream).unwrap();
        let request: SendMessageRequest = serde_json::from_str(&output).unwrap();

        assert_eq!(request.content, MESSAGE.to_string());
        assert_eq!(request.node_id, NODE_ID1.to_string());
        assert_eq!(request.time, 1);
    }
}
