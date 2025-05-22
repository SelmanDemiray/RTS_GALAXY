use crate::entity::BuildingType;
use crate::entity::UnitType;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ResourceType {
    Minerals,
    Energy,
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    Build(BuildingType),
    Train(UnitType),
    Attack,
    Move,
    Gather,
}
