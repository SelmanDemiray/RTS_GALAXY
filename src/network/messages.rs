use serde::{Deserialize, Serialize};
use crate::entity::Unit;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum NetworkMessage {
    GameState {
        units: Vec<Unit>,
        timestamp: f64,
    },
    UnitUpdate {
        unit_id: u32,
        x: f32,
        y: f32,
    },
    PlayerJoined {
        player_id: u8,
        name: String,
    },
    PlayerLeft {
        player_id: u8,
    },
    ChatMessage {
        player_id: u8,
        message: String,
    },
    // Connection and authentication
    Connect { player_id: u64, auth_token: String },
    Disconnect { player_id: u64 },
    
    // Spatial partitioning
    SectorTransfer { 
        player_id: u64, 
        from_sector: SectorId, 
        to_sector: SectorId,
        units: Vec<CompressedUnit>
    },
    
    // Unit operations with delta compression
    UnitUpdate { 
        sector_id: SectorId,
        updates: Vec<UnitDelta>,
        timestamp: u64
    },
    
    // Batch operations for efficiency
    BatchCommand {
        sector_id: SectorId,
        commands: Vec<CompressedCommand>,
        player_id: u64
    },
    
    // Inter-server communication
    ServerSync {
        server_id: u32,
        sector_states: HashMap<SectorId, SectorSnapshot>,
        timestamp: u64
    },
    
    // Load balancing
    LoadBalance {
        target_server: u32,
        sectors_to_migrate: Vec<SectorId>
    },
    
    // Real-time events
    CombatEvent {
        sector_id: SectorId,
        event_type: CombatEventType,
        participants: Vec<u64>, // Unit IDs
        timestamp: u64
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct SectorId {
    pub x: i64,
    pub y: i64,
    pub scale: u8, // 0 = smallest sectors, higher = larger regions
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressedUnit {
    pub id: u64,
    pub type_id: u16,
    pub position: CompressedPosition,
    pub health: u16,
    pub state: u8, // Bitpacked state flags
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressedPosition {
    pub x: u32, // Relative to sector, high precision
    pub y: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitDelta {
    pub unit_id: u64,
    pub changes: u32, // Bitfield indicating what changed
    pub position_delta: Option<(i16, i16)>,
    pub health_delta: Option<i16>,
    pub state_changes: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressedCommand {
    pub command_type: u8,
    pub unit_ids: Vec<u64>,
    pub target: Option<CompressedPosition>,
    pub parameters: Vec<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectorSnapshot {
    pub unit_count: u32,
    pub resource_density: u16,
    pub activity_level: u8,
    pub last_update: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CombatEventType {
    UnitDestroyed,
    MassiveBattle { participants: u32 },
    ResourceCapture,
    BaseDestroyed,
}

impl SectorId {
    pub fn new(x: i64, y: i64, scale: u8) -> Self {
        Self { x, y, scale }
    }
    
    pub fn parent(&self) -> Option<SectorId> {
        if self.scale < 255 {
            Some(SectorId {
                x: self.x >> 1,
                y: self.y >> 1,
                scale: self.scale + 1,
            })
        } else {
            None
        }
    }
    
    pub fn children(&self) -> Vec<SectorId> {
        if self.scale > 0 {
            let base_x = self.x << 1;
            let base_y = self.y << 1;
            vec![
                SectorId::new(base_x, base_y, self.scale - 1),
                SectorId::new(base_x + 1, base_y, self.scale - 1),
                SectorId::new(base_x, base_y + 1, self.scale - 1),
                SectorId::new(base_x + 1, base_y + 1, self.scale - 1),
            ]
        } else {
            vec![]
        }
    }
}
