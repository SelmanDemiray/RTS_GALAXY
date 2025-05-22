use crate::game::types::ResourceType;

pub struct ResourceNode {
    pub x: f32,
    pub y: f32,
    pub resources: i32,
    pub resource_type: ResourceType,
    pub radius: f32,
}
