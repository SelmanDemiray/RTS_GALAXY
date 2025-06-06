use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum UnitType {
    Worker,
    Fighter,
    Ranger,
    Tank,
    Building,
    Headquarters,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ResourceType {
    Minerals,
    Energy,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UnitAnimationState {
    Idle,
    Walking,
    Running,
    Attacking,
    Gathering,
    Building,
    Dying,
    Special,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum BuildingType {
    Headquarters,
    Barracks,
    Factory,
    ResearchCenter,
    TurretDefense,
    ResourceCollector,
    ResourceDepot, // Added missing variant referenced in AI behaviors
    DefenseTurret, // Added missing variant referenced in AI behaviors
}
