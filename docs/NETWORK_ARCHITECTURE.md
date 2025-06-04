# Network Architecture Documentation

## Overview

Galaxy RTS features a scalable network architecture designed to support massive multiplayer gameplay across galactic scales. The system uses hierarchical spatial partitioning, delta compression, and distributed server architecture.

## Core Architecture

### Sector-Based Partitioning
The universe is divided into hierarchical sectors:

```rust
#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct SectorId {
    pub x: i64,
    pub y: i64,
    pub scale: u8, // 0 = smallest sectors, higher = larger regions
}
```

### Sector Hierarchy
- **Scale 0**: Individual unit areas (1km × 1km)
- **Scale 1**: Small regions (2km × 2km)  
- **Scale 2**: Large regions (4km × 4km)
- **Scale N**: Exponentially larger areas

### Benefits
- **Scalability**: Only relevant sectors are synchronized
- **Load Distribution**: Sectors can migrate between servers
- **Bandwidth Optimization**: Players only receive nearby updates
- **Fault Tolerance**: Sector failures don't affect entire game

## Message System

### Core Message Types

#### Unit Updates
```rust
UnitUpdate { 
    sector_id: SectorId,
    updates: Vec<UnitDelta>,
    timestamp: u64
}
```

#### Delta Compression
```rust
pub struct UnitDelta {
    pub unit_id: u64,
    pub changes: u32, // Bitfield indicating what changed
    pub position_delta: Option<(i16, i16)>,
    pub health_delta: Option<i16>,
    pub state_changes: Option<u8>,
}
```

#### Batch Operations
```rust
BatchCommand {
    sector_id: SectorId,
    commands: Vec<CompressedCommand>,
    player_id: u64
}
```

### Compression Techniques

#### Bitfield Change Tracking
```rust
const POSITION_CHANGED: u32 = 0b00000001;
const HEALTH_CHANGED: u32   = 0b00000010;
const STATE_CHANGED: u32    = 0b00000100;
const TARGET_CHANGED: u32   = 0b00001000;

// Example usage
let mut changes = 0u32;
if position_changed { changes |= POSITION_CHANGED; }
if health_changed { changes |= HEALTH_CHANGED; }
```

#### Position Delta Compression
```rust
// Store position changes as 16-bit integers relative to last known position
let delta_x = ((new_x - old_x) * 100.0) as i16; // Centimeter precision
let delta_y = ((new_y - old_y) * 100.0) as i16;
```

## Server Architecture

### Distributed Servers
```rust
ServerSync {
    server_id: u32,
    sector_states: HashMap<SectorId, SectorSnapshot>,
    timestamp: u64
}
```

### Load Balancing
```rust
LoadBalance {
    target_server: u32,
    sectors_to_migrate: Vec<SectorId>
}
```

### Inter-Server Communication
- **State Synchronization**: Periodic sync of sector states
- **Player Migration**: Seamless movement between server regions
- **Load Monitoring**: Real-time server performance tracking
- **Fault Recovery**: Automatic failover for failed servers

## Client Architecture

### NetworkClient Implementation
```rust
pub struct NetworkClient {
    stream: Option<TcpStream>,
    buffer: Vec<u8>,
    runtime: Runtime,
    pub status: ConnectionStatus,
    is_connected: bool,
}
```

### Connection Management
```rust
// Asynchronous connection
client.connect("127.0.0.1:8080")?;

// Send messages
client.send(&NetworkMessage::UnitUpdate { ... })?;

// Receive updates
if let Some(message) = client.receive() {
    handle_network_message(message);
}
```

### Connection States
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
    Failed(String),
}
```

## Scalability Features

### Sector Migration
Players can seamlessly move between sectors managed by different servers:

```rust
SectorTransfer { 
    player_id: u64, 
    from_sector: SectorId, 
    to_sector: SectorId,
    units: Vec<CompressedUnit>
}
```

### Real-Time Events
Critical events are prioritized for immediate delivery:

```rust
CombatEvent {
    sector_id: SectorId,
    event_type: CombatEventType,
    participants: Vec<u64>,
    timestamp: u64
}
```

### Event Types
- **UnitDestroyed**: Individual unit elimination
- **MassiveBattle**: Large-scale combat events
- **ResourceCapture**: Strategic resource control
- **BaseDestroyed**: Major base destruction

## Performance Optimization

### Bandwidth Reduction
1. **Delta Compression**: Only send changes, not full state
2. **Sector Filtering**: Only relevant sectors per player
3. **Priority Queues**: Critical updates first
4. **Batch Processing**: Multiple updates per message

### Latency Optimization
1. **Predictive Movement**: Client-side prediction
2. **Interpolation**: Smooth movement between updates
3. **Priority Routing**: Fast path for critical messages
4. **Local Caching**: Reduce server round-trips

### Server Performance
1. **Sector Threading**: Parallel processing of sectors
2. **Update Batching**: Group multiple changes
3. **Memory Pooling**: Reuse message objects
4. **Efficient Serialization**: Optimized data formats

## Security Features

### Authentication
```rust
Connect { 
    player_id: u64, 
    auth_token: String 
}
```

### Anti-Cheat Measures
- **Server Authority**: All game state validated server-side
- **Movement Validation**: Impossible movements rejected
- **Rate Limiting**: Prevent command flooding
- **State Verification**: Periodic client state validation

### Data Integrity
- **Message Checksums**: Detect corrupted data
- **Replay Protection**: Prevent message replay attacks
- **Encrypted Tokens**: Secure authentication
- **Session Management**: Timeout inactive connections

## Implementation Examples

### Basic Client Setup
```rust
// Initialize network client
let mut client = NetworkClient::new();

// Connect to server
match client.connect("game.server.com:8080") {
    Ok(_) => println!("Connected successfully"),
    Err(e) => println!("Connection failed: {}", e),
}

// Main game loop
loop {
    // Send player commands
    if let Some(command) = get_player_command() {
        let message = NetworkMessage::BatchCommand {
            sector_id: current_sector,
            commands: vec![command],
            player_id: player_id,
        };
        client.send(&message)?;
    }
    
    // Receive updates
    while let Some(message) = client.receive() {
        match message {
            NetworkMessage::UnitUpdate { updates, .. } => {
                apply_unit_updates(updates);
            },
            NetworkMessage::CombatEvent { event_type, .. } => {
                display_combat_event(event_type);
            },
            _ => {}
        }
    }
}
```

### Server Message Processing
```rust
fn handle_client_message(message: NetworkMessage, client_id: u64) {
    match message {
        NetworkMessage::BatchCommand { sector_id, commands, player_id } => {
            // Validate player owns these units
            if validate_player_authority(player_id, &commands) {
                // Process commands
                for command in commands {
                    execute_command(command, sector_id);
                }
                
                // Broadcast updates to relevant clients
                let updates = generate_updates(sector_id);
                broadcast_to_sector(sector_id, updates);
            }
        },
        NetworkMessage::SectorTransfer { player_id, from_sector, to_sector, units } => {
            // Handle player moving between sectors
            transfer_player_units(player_id, from_sector, to_sector, units);
        },
        _ => {}
    }
}
```

## Configuration

### Network Settings
```rust
pub struct NetworkConfig {
    pub server_address: String,
    pub port: u16,
    pub max_connections: usize,
    pub sector_size: f32,
    pub update_rate: f32,
    pub compression_enabled: bool,
}
```

### Performance Tuning
```rust
// Adjust update rates based on activity
let update_rate = match activity_level {
    ActivityLevel::Low => 10.0,     // 10 Hz
    ActivityLevel::Medium => 20.0,  // 20 Hz
    ActivityLevel::High => 60.0,    // 60 Hz
};
```

## Monitoring and Debugging

### Network Statistics
```rust
pub struct NetworkStats {
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub messages_sent: u64,
    pub messages_received: u64,
    pub average_latency: f32,
    pub packet_loss: f32,
}
```

### Debug Features
```rust
#[cfg(debug_assertions)]
fn log_network_message(message: &NetworkMessage) {
    println!("Network: {:?}", message);
}
```

### Performance Monitoring
- **Latency Tracking**: Monitor round-trip times
- **Bandwidth Usage**: Track data consumption
- **Server Load**: Monitor CPU and memory usage
- **Connection Quality**: Detect unstable connections

## Error Handling

### Connection Recovery
```rust
fn handle_connection_error(&mut self, error: &str) {
    self.status = ConnectionStatus::Failed(error.to_string());
    self.disconnect();
    
    // Attempt reconnection after delay
    if self.auto_reconnect {
        self.schedule_reconnect();
    }
}
```

### Graceful Degradation
- **Reduced Update Rate**: Lower bandwidth when connection is poor
- **Local Prediction**: Continue gameplay during short disconnections
- **State Recovery**: Resynchronize when connection restored
- **Fallback Servers**: Connect to backup servers if primary fails

## Future Enhancements

### Planned Features
- **UDP Protocol**: Lower latency for real-time updates
- **WebSocket Support**: Browser-based clients
- **Mobile Optimization**: Reduced bandwidth for mobile clients
- **Cloud Integration**: Auto-scaling server infrastructure

### Advanced Features
- **AI Takeover**: AI controls during disconnections
- **Spectator Mode**: Watch games without affecting gameplay
- **Replay System**: Record and playback network games
- **Cross-Platform**: Support for different client platforms

The network architecture provides a robust foundation for massive multiplayer gameplay while maintaining performance and reliability across galactic scales.
