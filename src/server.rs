use std::net::TcpListener;

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub fn new(address: &str) -> Self {
        let listener = TcpListener::bind(address).unwrap();

        println!("Server started on {}", address);

        Self { listener }
    }

    pub fn run(&self) -> std::io::Result<()> {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New connection: {}", stream.peer_addr().unwrap());
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }

        Ok(())
    }
}
