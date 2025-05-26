use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BuildingType {
    Headquarters,
    Barracks,
    Factory,
    ResearchCenter,
    TurretDefense,
    ResourceCollector,
    ResourceDepot,
    DefenseTurret,
}
