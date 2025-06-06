use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BuildingType {
    // Command & Control
    Headquarters,
    CommandCenter,
    ControlTower,
    
    // Military Production
    Barracks,
    TrainingCenter,
    WarFactory,
    Starport,
    NavalYard,
    Factory,
    
    // Research & Technology
    ResearchLab,
    TechCenter,
    Observatory,
    DataCenter,
    
    // Resource Management
    ResourceDepot,
    MineralProcessor,
    EnergyPlant,
    PowerStation,
    RefineryComplex,
    StorageSilo,
    
    // Defense Structures
    DefenseTurret,
    MissileLauncher,
    Shield,
    Bunker,
    WallSegment,
    Gate,
    
    // Support Buildings
    RepairBay,
    SupplyDepot,
    Communications,
    Scanner,
    
    // Advanced Structures
    Foundry,
    Assembly,
    LaunchPad,
    Portal,
    
    // Special Buildings
    Monument,
    Shrine,
    Academy,
    Archives,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BuildingState {
    Construction,
    Operational,
    Damaged,
    Destroyed,
    Upgrading,
    PoweredDown,
    Overcharged,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingStats {
    pub max_health: f32,
    pub armor: f32,
    pub power_requirement: i32,
    pub build_time: f32,
    pub mineral_cost: i32,
    pub energy_cost: i32,
    pub population_provided: i32,
    pub research_points: i32,
}

impl BuildingType {
    pub fn get_stats(&self) -> BuildingStats {
        match self {
            // Command & Control
            BuildingType::Headquarters => BuildingStats {
                max_health: 2000.0, armor: 15.0, power_requirement: 0,
                build_time: 120.0, mineral_cost: 1000, energy_cost: 0,
                population_provided: 20, research_points: 10,
            },
            BuildingType::CommandCenter => BuildingStats {
                max_health: 1500.0, armor: 12.0, power_requirement: -50,
                build_time: 90.0, mineral_cost: 800, energy_cost: 200,
                population_provided: 15, research_points: 8,
            },
            BuildingType::ControlTower => BuildingStats {
                max_health: 800.0, armor: 8.0, power_requirement: -30,
                build_time: 60.0, mineral_cost: 400, energy_cost: 150,
                population_provided: 0, research_points: 5,
            },
            
            // Military Production
            BuildingType::Barracks => BuildingStats {
                max_health: 1200.0, armor: 10.0, power_requirement: 20,
                build_time: 60.0, mineral_cost: 300, energy_cost: 100,
                population_provided: 0, research_points: 2,
            },
            BuildingType::TrainingCenter => BuildingStats {
                max_health: 1400.0, armor: 10.0, power_requirement: 30,
                build_time: 75.0, mineral_cost: 350, energy_cost: 150,
                population_provided: 0, research_points: 3,
            },
            BuildingType::WarFactory => BuildingStats {
                max_health: 1400.0, armor: 12.0, power_requirement: 40,
                build_time: 90.0, mineral_cost: 600, energy_cost: 200,
                population_provided: 0, research_points: 4,
            },
            BuildingType::Starport => BuildingStats {
                max_health: 1600.0, armor: 8.0, power_requirement: 60,
                build_time: 120.0, mineral_cost: 800, energy_cost: 400,
                population_provided: 0, research_points: 6,
            },
            
            // Resource Management
            BuildingType::ResourceDepot => BuildingStats {
                max_health: 1000.0, armor: 5.0, power_requirement: 10,
                build_time: 45.0, mineral_cost: 200, energy_cost: 50,
                population_provided: 0, research_points: 1,
            },
            BuildingType::EnergyPlant => BuildingStats {
                max_health: 800.0, armor: 3.0, power_requirement: -100,
                build_time: 50.0, mineral_cost: 250, energy_cost: 0,
                population_provided: 0, research_points: 2,
            },
            BuildingType::PowerStation => BuildingStats {
                max_health: 900.0, armor: 4.0, power_requirement: -80,
                build_time: 55.0, mineral_cost: 300, energy_cost: 0,
                population_provided: 0, research_points: 2,
            },
            
            // Defense Structures
            BuildingType::DefenseTurret => BuildingStats {
                max_health: 600.0, armor: 8.0, power_requirement: 15,
                build_time: 30.0, mineral_cost: 150, energy_cost: 75,
                population_provided: 0, research_points: 1,
            },
            BuildingType::MissileLauncher => BuildingStats {
                max_health: 700.0, armor: 7.0, power_requirement: 25,
                build_time: 40.0, mineral_cost: 250, energy_cost: 100,
                population_provided: 0, research_points: 3,
            },
            BuildingType::Shield => BuildingStats {
                max_health: 500.0, armor: 5.0, power_requirement: 80,
                build_time: 75.0, mineral_cost: 400, energy_cost: 300,
                population_provided: 0, research_points: 5,
            },
            
            // Default for other buildings
            _ => BuildingStats {
                max_health: 800.0, armor: 5.0, power_requirement: 20,
                build_time: 60.0, mineral_cost: 300, energy_cost: 100,
                population_provided: 0, research_points: 2,
            },
        }
    }
    
    pub fn get_required_animations(&self) -> Vec<String> {
        let mut base_animations = vec![
            "idle".to_string(),
            "construction".to_string(),
            "damaged".to_string(),
            "destroyed".to_string(),
        ];
        
        match self {
            // Command buildings
            BuildingType::Headquarters | BuildingType::CommandCenter => {
                base_animations.extend(vec![
                    "command_active".to_string(),
                    "communication".to_string(),
                    "emergency_mode".to_string(),
                    "shield_up".to_string(),
                ]);
            },
            
            // Production buildings
            BuildingType::Barracks => {
                base_animations.extend(vec![
                    "training".to_string(),
                    "deployment".to_string(),
                    "drill_mode".to_string(),
                ]);
            },
            BuildingType::TrainingCenter => {
                base_animations.extend(vec![
                    "advanced_training".to_string(),
                    "tactics_discussion".to_string(),
                    "unit_parade".to_string(),
                ]);
            },
            BuildingType::WarFactory => {
                base_animations.extend(vec![
                    "manufacturing".to_string(),
                    "assembly_line".to_string(),
                    "heavy_production".to_string(),
                    "vehicle_rollout".to_string(),
                ]);
            },
            BuildingType::Starport => {
                base_animations.extend(vec![
                    "landing_pad_active".to_string(),
                    "launch_sequence".to_string(),
                    "refueling".to_string(),
                    "hangar_doors".to_string(),
                ]);
            },
            
            // Resource buildings
            BuildingType::EnergyPlant => {
                base_animations.extend(vec![
                    "power_generation".to_string(),
                    "energy_surge".to_string(),
                    "cooling_cycle".to_string(),
                    "overload".to_string(),
                ]);
            },
            BuildingType::MineralProcessor => {
                base_animations.extend(vec![
                    "processing".to_string(),
                    "conveyor_active".to_string(),
                    "refining".to_string(),
                    "storage_full".to_string(),
                ]);
            },
            
            // Defense buildings
            BuildingType::DefenseTurret | BuildingType::MissileLauncher => {
                base_animations.extend(vec![
                    "scanning".to_string(),
                    "targeting".to_string(),
                    "firing".to_string(),
                    "reloading".to_string(),
                    "turret_rotate".to_string(),
                ]);
            },
            BuildingType::Shield => {
                base_animations.extend(vec![
                    "shield_charging".to_string(),
                    "shield_active".to_string(),
                    "shield_overload".to_string(),
                    "energy_transfer".to_string(),
                ]);
            },
            
            // Research buildings
            BuildingType::ResearchLab | BuildingType::TechCenter => {
                base_animations.extend(vec![
                    "researching".to_string(),
                    "experiment".to_string(),
                    "data_processing".to_string(),
                    "breakthrough".to_string(),
                ]);
            },
            
            _ => {
                base_animations.extend(vec![
                    "working".to_string(),
                    "maintenance".to_string(),
                ]);
            }
        }
        
        base_animations
    }
    
    pub fn get_cost(&self) -> u32 {
        match self {
            BuildingType::Headquarters => 0,
            BuildingType::CommandCenter => 400,
            BuildingType::ControlTower => 200,
            BuildingType::Barracks => 150,
            BuildingType::TrainingCenter => 175,
            BuildingType::Starport => 300,
            BuildingType::NavalYard => 250,
            BuildingType::Factory => 200,
            BuildingType::WarFactory => 300,
            BuildingType::ResearchLab => 200,
            BuildingType::TechCenter => 350,
            BuildingType::ResourceDepot => 175,
            BuildingType::EnergyPlant => 150,
            BuildingType::PowerStation => 200,
            BuildingType::DefenseTurret => 75,
            // ...existing code...
            _ => 100,
        }
    }

    pub fn get_health(&self) -> f32 {
        match self {
            BuildingType::Headquarters => 2000.0,
            BuildingType::CommandCenter => 1800.0,
            BuildingType::ControlTower => 800.0,
            BuildingType::Barracks => 1000.0,
            BuildingType::TrainingCenter => 1200.0,
            BuildingType::Starport => 1500.0,
            BuildingType::NavalYard => 1300.0,
            BuildingType::Factory => 1200.0,
            BuildingType::WarFactory => 1600.0,
            BuildingType::ResearchLab => 800.0,
            BuildingType::TechCenter => 1000.0,
            BuildingType::ResourceDepot => 900.0,
            BuildingType::EnergyPlant => 600.0,
            BuildingType::PowerStation => 800.0,
            BuildingType::DefenseTurret => 400.0,
            // ...existing code...
            _ => 500.0,
        }
    }
}
