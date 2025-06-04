use crate::game::types::ResourceType;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceNode {
    pub x: f32,
    pub y: f32,
    pub resources: i32,
    pub resource_type: ResourceType,
    pub radius: f32,
}

impl ResourceNode {
    pub fn new(x: f32, y: f32, resources: i32, resource_type: ResourceType, radius: f32) -> Self {
        Self {
            x,
            y,
            resources,
            resource_type,
            radius,
        }
    }
}
