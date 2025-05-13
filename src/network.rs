use std::error::Error;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::runtime::{Runtime, Handle};
use crate::game::NetworkMessage;

pub struct NetworkClient {
    stream: Option<TcpStream>,
    buffer: Vec<u8>,
    runtime: Runtime,
}

impl NetworkClient {
    pub fn new() -> Self {
        // Create a single-threaded runtime
        let runtime = Runtime::new().expect("Failed to create Tokio runtime");
        
        Self {
            stream: None,
            buffer: vec![0; 1024],
            runtime,
        }
    }
    
    pub fn connect(&mut self, addr: &str) -> Result<(), Box<dyn Error>> {
        self.runtime.block_on(async {
            match TcpStream::connect(addr).await {
                Ok(stream) => {
                    self.stream = Some(stream);
                    Ok(())
                }
                Err(e) => {
                    eprintln!("Failed to connect: {}", e);
                    // For this simple example, we'll continue without connection
                    Ok(())
                }
            }
        })
    }
    
    pub fn send(&mut self, message: &NetworkMessage) -> Result<(), Box<dyn Error>> {
        self.runtime.block_on(async {
            if let Some(stream) = &mut self.stream {
                let data = serde_json::to_string(message)?;
                stream.write_all(data.as_bytes()).await?;
                Ok(())
            } else {
                println!("Not connected, can't send message");
                Ok(())
            }
        })
    }
    
    pub fn receive(&mut self) -> Option<NetworkMessage> {
        self.runtime.block_on(async {
            if let Some(stream) = &mut self.stream {
                match stream.read(&mut self.buffer).await {
                    Ok(n) if n > 0 => {
                        if let Ok(message) = serde_json::from_slice::<NetworkMessage>(&self.buffer[..n]) {
                            return Some(message);
                        }
                    }
                    _ => {}
                }
            }
            None
        })
    }
    
    pub fn is_connected(&self) -> bool {
        self.stream.is_some()
    }
}
