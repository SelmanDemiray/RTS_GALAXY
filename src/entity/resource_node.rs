use crate::game::types::ResourceType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ResourceType {
    Minerals,
    Energy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceNode {
    pub id: u64,
    pub x: f32,
    pub y: f32,
    pub resource_type: ResourceType,
    pub amount: i32,
    pub max_amount: i32,
    pub resources: i32,
    pub radius: f32,
}

impl ResourceNode {
    pub fn new(id: u64, x: f32, y: f32, resource_type: ResourceType, amount: i32) -> Self {
        Self {
            id,
            x,
            y,
            resource_type,
            amount,
            max_amount: amount,
            resources: amount,
            radius: 30.0,
        }
    }

    pub fn gather(&mut self, amount: i32) -> i32 {
        let gathered = amount.min(self.resources);
        self.resources -= gathered;
        self.amount = self.resources;
        gathered
    }

    pub fn is_depleted(&self) -> bool {
        self.resources <= 0
    }
}
