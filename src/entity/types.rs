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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum BuildingType {
    Headquarters,
    Barracks,
    Factory,
    ResearchCenter,
    TurretDefense,
    ResourceCollector,
}
