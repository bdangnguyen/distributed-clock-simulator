use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Server {
    listener: TcpListener,
    clients: Arc<Mutex<Vec<TcpStream>>>,
}

impl Server {
    pub fn new(address: &str) -> Self {
        let listener = TcpListener::bind(address).unwrap();

        println!("Server started on {}", address);

        Self {
            listener,
            clients: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Accepts new connections and spawns a thread to handle each client.
    pub fn run(&self) -> std::io::Result<()> {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New connection: {}", stream.peer_addr().unwrap());

                    // Cache client for future reference
                    let stream_clone = stream.try_clone().unwrap();
                    self.clients.lock().unwrap().push(stream_clone);

                    // Handle each client in a separate thread
                    let clients_clone = self.clients.clone();
                    thread::spawn(move || {
                        read_message(&stream, clients_clone);
                    });
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }

        Ok(())
    }
}

/// Reads messages from a client and broadcasts them to all connected clients.
fn read_message(stream: &TcpStream, clients: Arc<Mutex<Vec<TcpStream>>>) {
    let reader = BufReader::new(stream);
    for line in reader.lines() {
        match line {
            Ok(message) => {
                if !message.trim().is_empty() {
                    println!("Received: {}", message);
                    let sender = stream.peer_addr().unwrap().to_string();
                    broadcast(&clients, &message, &sender);
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
        }
    }
}

/// Broadcasts a message to all connected clients except the sender.
fn broadcast<T: ClientStream>(clients: &Arc<Mutex<Vec<T>>>, message: &str, sender: &str) {
    for client in clients.lock().unwrap().iter_mut() {
        if client.peer_addr() != sender {
            writeln!(client, "{}", message)
                .and_then(|_| client.flush())
                .unwrap_or_else(|e| eprintln!("Failed to send: {}", e));
        }
    }
}

trait ClientStream: Write + Send + Sync {
    fn peer_addr(&self) -> String;
}

impl ClientStream for TcpStream {
    fn peer_addr(&self) -> String {
        self.peer_addr().unwrap().to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct MockStream {
        addr: String,
        // Buffer to store messages written to the stream by the server
        buffer: Arc<Mutex<Vec<u8>>>,
    }

    impl ClientStream for MockStream {
        fn peer_addr(&self) -> String {
            self.addr.clone()
        }
    }

    impl Write for MockStream {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.buffer.lock().unwrap().extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn test_broadcast() {
        const MESSAGE: &str = "Hello World!";
        const SENDER_ADDRESS: &str = "127.0.0.1:8080";
        const CLIENT_ADDRESS: &str = "127.0.0.1:8081";

        let clients = Arc::new(Mutex::new(Vec::new()));
        let output_buffer = Arc::new(Mutex::new(Vec::new()));
        let mock_client = MockStream {
            addr: CLIENT_ADDRESS.to_string(),
            buffer: output_buffer.clone(),
        };

        clients.lock().unwrap().push(mock_client);

        broadcast(&clients, MESSAGE, SENDER_ADDRESS);

        let data = String::from_utf8(output_buffer.lock().unwrap().clone()).unwrap();
        assert_eq!(data.trim(), MESSAGE);
    }
}
