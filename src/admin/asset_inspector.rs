use crate::game::state::GameState;
use crate::resources::manager::ResourceManager;
use crate::entity::{UnitType, BuildingType};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetStatus {
    pub name: String,
    pub asset_type: AssetType,
    pub exists: bool,
    pub file_path: Option<String>,
    pub error_message: Option<String>,
    pub animations: Vec<AnimationStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssetType {
    Unit(UnitType),
    Building(BuildingType),
    Resource,
    Effect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationStatus {
    pub name: String,
    pub exists: bool,
    pub file_path: Option<String>,
    pub duration: Option<f32>,
    pub loops: Option<bool>,
}

#[cfg(feature = "admin")]
pub struct AssetInspector {
    asset_statuses: HashMap<String, AssetStatus>,
    selected_asset: Option<String>,
    preview_zoom: f32,
    preview_rotation: f32,
    show_missing_only: bool,
    asset_filter: String,
}

#[cfg(feature = "admin")]
impl AssetInspector {
    pub fn new() -> Self {
        Self {
            asset_statuses: HashMap::new(),
            selected_asset: None,
            preview_zoom: 1.0,
            preview_rotation: 0.0,
            show_missing_only: false,
            asset_filter: String::new(),
        }
    }
    
    pub fn update(&mut self, game_state: &GameState, resource_manager: &ResourceManager) {
        self.scan_assets(resource_manager);
    }
    
    fn scan_assets(&mut self, resource_manager: &ResourceManager) {
        self.asset_statuses.clear();
        
        // Scan unit assets
        for unit_type in [UnitType::Worker, UnitType::Fighter, UnitType::Ranger, UnitType::Tank] {
            let asset_name = format!("{:?}", unit_type).to_lowercase();
            let status = self.check_unit_asset(&asset_name, unit_type.clone(), resource_manager);
            self.asset_statuses.insert(asset_name, status);
        }
        
        // Scan building assets
        for building_type in [
            BuildingType::Headquarters,
            BuildingType::Barracks,
            BuildingType::Factory,
            BuildingType::ResourceDepot,
            BuildingType::DefenseTurret,
            BuildingType::EnergyPlant,
        ] {
            let asset_name = format!("{:?}", building_type).to_lowercase();
            let status = self.check_building_asset(&asset_name, building_type.clone(), resource_manager);
            self.asset_statuses.insert(asset_name, status);
        }
        
        // Scan resource assets
        for resource_name in ["minerals", "energy"] {
            let status = self.check_resource_asset(resource_name, resource_manager);
            self.asset_statuses.insert(resource_name.to_string(), status);
        }
    }
    
    fn check_unit_asset(&self, name: &str, unit_type: UnitType, resource_manager: &ResourceManager) -> AssetStatus {
        let base_path = format!("models/units/{}/{}.glb", name, name);
        let model_exists = resource_manager.model_exists(&base_path);
        
        let expected_animations = self.get_expected_unit_animations(&unit_type);
        let animations = expected_animations.into_iter().map(|anim_name| {
            let anim_path = format!("models/units/{}/animations/{}.glb", name, anim_name);
            AnimationStatus {
                name: anim_name.clone(),
                exists: resource_manager.animation_exists(&anim_path),
                file_path: Some(anim_path),
                duration: Some(2.0), // Default duration
                loops: Some(true),
            }
        }).collect();
        
        AssetStatus {
            name: name.to_string(),
            asset_type: AssetType::Unit(unit_type),
            exists: model_exists,
            file_path: Some(base_path),
            error_message: if !model_exists { 
                Some("Base model file not found".to_string()) 
            } else { 
                None 
            },
            animations,
        }
    }
    
    fn check_building_asset(&self, name: &str, building_type: BuildingType, resource_manager: &ResourceManager) -> AssetStatus {
        let base_path = format!("models/buildings/{}/{}.glb", name, name);
        let model_exists = resource_manager.model_exists(&base_path);
        
        let expected_animations = self.get_expected_building_animations(&building_type);
        let animations = expected_animations.into_iter().map(|anim_name| {
            let anim_path = format!("models/buildings/{}/animations/{}.glb", name, anim_name);
            AnimationStatus {
                name: anim_name.clone(),
                exists: resource_manager.animation_exists(&anim_path),
                file_path: Some(anim_path),
                duration: Some(3.0),
                loops: Some(true),
            }
        }).collect();
        
        AssetStatus {
            name: name.to_string(),
            asset_type: AssetType::Building(building_type),
            exists: model_exists,
            file_path: Some(base_path),
            error_message: if !model_exists { 
                Some("Base model file not found".to_string()) 
            } else { 
                None 
            },
            animations,
        }
    }
    
    fn check_resource_asset(&self, name: &str, resource_manager: &ResourceManager) -> AssetStatus {
        let base_path = format!("models/resources/{}/{}.glb", name, name);
        let model_exists = resource_manager.model_exists(&base_path);
        
        let animations = vec![
            AnimationStatus {
                name: "idle".to_string(),
                exists: resource_manager.animation_exists(&format!("models/resources/{}/animations/idle.glb", name)),
                file_path: Some(format!("models/resources/{}/animations/idle.glb", name)),
                duration: Some(4.0),
                loops: Some(true),
            }
        ];
        
        AssetStatus {
            name: name.to_string(),
            asset_type: AssetType::Resource,
            exists: model_exists,
            file_path: Some(base_path),
            error_message: if !model_exists { 
                Some("Resource model file not found".to_string()) 
            } else { 
                None 
            },
            animations,
        }
    }
    
    fn get_expected_unit_animations(&self, unit_type: &UnitType) -> Vec<String> {
        let mut base_animations = vec![
            "idle".to_string(),
            "walking".to_string(),
            "dying".to_string(),
        ];
        
        match unit_type {
            UnitType::Worker => {
                base_animations.extend(vec![
                    "gathering_minerals".to_string(),
                    "gathering_energy".to_string(),
                    "building".to_string(),
                    "carrying_resources".to_string(),
                ]);
            }
            UnitType::Fighter => {
                base_animations.extend(vec![
                    "running".to_string(),
                    "melee_attack".to_string(),
                    "blocking".to_string(),
                    "victory_pose".to_string(),
                ]);
            }
            UnitType::Ranger => {
                base_animations.extend(vec![
                    "running".to_string(),
                    "aiming".to_string(),
                    "shooting".to_string(),
                    "reloading".to_string(),
                ]);
            }
            UnitType::Tank => {
                base_animations.extend(vec![
                    "turret_rotate".to_string(),
                    "firing_cannon".to_string(),
                    "damaged_idle".to_string(),
                ]);
            }
            _ => {}
        }
        
        base_animations
    }
    
    fn get_expected_building_animations(&self, building_type: &BuildingType) -> Vec<String> {
        let mut base_animations = vec![
            "idle".to_string(),
            "construction".to_string(),
            "damaged".to_string(),
            "destroyed".to_string(),
        ];
        
        match building_type {
            BuildingType::Headquarters => {
                base_animations.extend(vec![
                    "command_active".to_string(),
                    "communication".to_string(),
                    "shield_up".to_string(),
                ]);
            }
            BuildingType::Barracks => {
                base_animations.extend(vec![
                    "training".to_string(),
                    "deployment".to_string(),
                ]);
            }
            BuildingType::Factory => {
                base_animations.extend(vec![
                    "manufacturing".to_string(),
                    "assembly_line".to_string(),
                    "vehicle_rollout".to_string(),
                ]);
            }
            BuildingType::DefenseTurret => {
                base_animations.extend(vec![
                    "scanning".to_string(),
                    "targeting".to_string(),
                    "firing".to_string(),
                    "turret_rotate".to_string(),
                ]);
            }
            BuildingType::EnergyPlant => {
                base_animations.extend(vec![
                    "power_generation".to_string(),
                    "energy_surge".to_string(),
                    "cooling_cycle".to_string(),
                ]);
            }
            _ => {}
        }
        
        base_animations
    }
    
    pub fn get_asset_statuses(&self) -> &HashMap<String, AssetStatus> {
        &self.asset_statuses
    }
    
    pub fn get_selected_asset(&self) -> &Option<String> {
        &self.selected_asset
    }
    
    pub fn set_selected_asset(&mut self, asset_name: Option<String>) {
        self.selected_asset = asset_name;
    }
    
    pub fn get_preview_settings(&self) -> (f32, f32) {
        (self.preview_zoom, self.preview_rotation)
    }
    
    pub fn set_preview_settings(&mut self, zoom: f32, rotation: f32) {
        self.preview_zoom = zoom;
        self.preview_rotation = rotation;
    }
    
    pub fn get_filter_settings(&self) -> (&str, bool) {
        (&self.asset_filter, self.show_missing_only)
    }
    
    pub fn set_filter_settings(&mut self, filter: String, missing_only: bool) {
        self.asset_filter = filter;
        self.show_missing_only = missing_only;
    }
}
