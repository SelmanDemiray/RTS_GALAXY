use crate::entity::{UnitType, BuildingType};

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    Build(BuildingType),
    Train(UnitType),
    Attack,
    Move,
    Gather,
}
