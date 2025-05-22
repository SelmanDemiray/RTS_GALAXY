use std::error::Error;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::runtime::Runtime;
use crate::network::messages::NetworkMessage;

#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
    Failed(String),
}

pub struct NetworkClient {
    stream: Option<TcpStream>,
    buffer: Vec<u8>,
    runtime: Runtime,
    pub status: ConnectionStatus,
    pub last_error: Option<String>,
    is_connected: bool,
}

impl NetworkClient {
    pub fn new() -> Self {
        // Create a single-threaded runtime
        let runtime = Runtime::new().expect("Failed to create Tokio runtime");
        
        Self {
            stream: None,
            buffer: vec![0; 1024],
            runtime,
            status: ConnectionStatus::Disconnected,
            last_error: None,
            is_connected: false,
        }
    }
    
    pub fn connect(&mut self, addr: &str) -> Result<(), Box<dyn Error>> {
        // Reset connection state
        self.stream = None;
        self.last_error = None;
        self.status = ConnectionStatus::Connecting;
        
        let result = self.runtime.block_on(async {
            match TcpStream::connect(addr).await {
                Ok(stream) => Ok(stream),
                Err(e) => {
                    let error_msg = format!("Failed to connect: {}", e);
                    Err(error_msg)
                }
            }
        });
        
        match result {
            Ok(stream) => {
                self.stream = Some(stream);
                self.status = ConnectionStatus::Connected;
                self.is_connected = true;
                Ok(())
            }
            Err(error_msg) => {
                self.status = ConnectionStatus::Failed(error_msg.clone());
                self.last_error = Some(error_msg.clone());
                self.is_connected = false;
                Err(error_msg.into())
            }
        }
    }
    
    pub fn disconnect(&mut self) {
        self.stream = None;
        self.status = ConnectionStatus::Disconnected;
        self.is_connected = false;
    }
    
    pub fn send(&mut self, message: &NetworkMessage) -> Result<(), Box<dyn Error>> {
        if self.status != ConnectionStatus::Connected {
            return Ok(());  // Silently fail if not connected
        }
        
        let data = serde_json::to_string(message)?;
        let bytes = data.as_bytes().to_vec(); // Clone the data
        
        let _should_disconnect = false;
        let result = self.runtime.block_on(async {
            if let Some(stream) = &mut self.stream {
                match stream.write_all(&bytes).await {
                    Ok(_) => Ok(()),
                    Err(e) => Err(format!("Send error: {}", e))
                }
            } else {
                Ok(()) // No connection, silently ignore
            }
        });
        
        match result {
            Ok(_) => Ok(()),
            Err(error_msg) => {
                self.status = ConnectionStatus::Failed(error_msg.clone());
                self.disconnect();
                Err(error_msg.into())
            }
        }
    }
    
    pub fn receive(&mut self) -> Option<NetworkMessage> {
        if self.status != ConnectionStatus::Connected {
            return None;
        }
        
        let result = self.runtime.block_on(async {
            if let Some(stream) = &mut self.stream {
                match stream.read(&mut self.buffer).await {
                    Ok(n) if n > 0 => {
                        if let Ok(message) = serde_json::from_slice::<NetworkMessage>(&self.buffer[..n]) {
                            return Ok(Some(message));
                        } else {
                            // Handle malformed message
                            return Err("Received malformed message".to_string());
                        }
                    }
                    Ok(_) => {
                        // Connection closed
                        return Ok(None);
                    }
                    Err(e) => {
                        return Err(format!("Read error: {}", e));
                    }
                }
            }
            Ok(None)
        });
        
        match result {
            Ok(message) => message,
            Err(error_msg) => {
                self.last_error = Some(error_msg);
                self.disconnect();
                None
            }
        }
    }
    
    pub fn is_connected(&self) -> bool {
        self.is_connected
    }
}
