use macroquad::prelude::*;
use macroquad::audio::Sound;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::resources::model3d::{Model3D, ModelInfo};
use serde_json;

// Asset manifest structures
#[derive(Serialize, Deserialize)]
pub struct TextureInfo {
    name: String,
    file: String,
    dimensions: [u32; 2],
    #[serde(default)]
    frames: u32,
}

#[derive(Serialize, Deserialize)]
pub struct SoundInfo {
    name: String,
    file: String,
    #[serde(default = "default_volume")]
    volume: f32,
}

// Default volume function for serde
fn default_volume() -> f32 {
    1.0
}

#[derive(Serialize, Deserialize)]
pub struct MusicInfo {
    name: String,
    file: String,
}

#[derive(Serialize, Deserialize)]
pub struct FontInfo {
    name: String,
    file: String,
}

#[derive(Serialize, Deserialize)]
pub struct AssetManifest {
    models: ModelCategories,
    textures: TextureCategories,
    sounds: SoundCategories,
    music: Vec<MusicInfo>,
    fonts: Vec<FontInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct ModelCategories {
    units: Vec<ModelInfo>,
    buildings: Vec<ModelInfo>,
    resources: Vec<ModelInfo>,
    terrain: Vec<ModelInfo>,
    props: Vec<ModelInfo>,
    effects: Vec<ModelInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct TextureCategories {
    ui: Vec<TextureInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct SoundCategories {
    units: Vec<SoundInfo>,
    buildings: Vec<SoundInfo>,
    ui: Vec<SoundInfo>,
    game: Vec<SoundInfo>,
}

pub struct ResourceManager {
    textures: HashMap<String, Texture2D>,
    sounds: HashMap<String, Sound>,
    music: HashMap<String, Sound>,
    fonts: HashMap<String, Font>,
    models: HashMap<String, Model3D>,
    loading_progress: f32,
    total_assets: usize,
    loaded_assets: usize,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            sounds: HashMap::new(),
            music: HashMap::new(),
            fonts: HashMap::new(),
            models: HashMap::new(),
            loading_progress: 0.0,
            total_assets: 0,
            loaded_assets: 0,
        }
    }

    pub async fn load_resources(&mut self) {
        // Load basic resources for the demo
        self.load_basic_resources().await;
    }

    async fn load_basic_resources(&mut self) {
        // Basic fallback resources - only essential sounds for now
        let basic_assets = vec![
            ("button_click", "assets/sounds/ui/button_click.wav", "sound"),
            ("unit_select", "assets/sounds/units/unit_select.wav", "sound"),
            ("building_place", "assets/sounds/buildings/building_place.wav", "sound"),
        ];

        self.total_assets = basic_assets.len();

        for (name, path, asset_type) in basic_assets {
            match asset_type {
                "sound" => {
                    // For now, we'll just mark as loaded without actually loading files
                    // In a real implementation, you would load the actual audio files
                    println!("Loading sound: {} from {}", name, path);
                },
                _ => {}
            }
            self.loaded_assets += 1;
            self.update_progress();
        }
    }

    fn update_progress(&mut self) {
        if self.total_assets > 0 {
            self.loading_progress = self.loaded_assets as f32 / self.total_assets as f32;
        } else {
            self.loading_progress = 1.0;
        }
    }

    pub fn is_loading_complete(&self) -> bool {
        self.loading_progress >= 1.0
    }

    pub fn get_loading_progress(&self) -> f32 {
        self.loading_progress
    }

    pub fn get_texture(&self, name: &str) -> Option<&Texture2D> {
        self.textures.get(name)
    }

    pub fn get_sound(&self, name: &str) -> Option<&Sound> {
        self.sounds.get(name)
    }

    pub fn get_font(&self, name: &str) -> Option<&Font> {
        self.fonts.get(name)
    }

    pub fn get_music(&self, name: &str) -> Option<&Sound> {
        self.sounds.get(name)
    }

    pub fn get_model(&self, name: &str) -> Option<&Model3D> {
        self.models.get(name)
    }

    pub fn model_exists(&self, path: &str) -> bool {
        // Check if model file exists in the loaded resources
        // For now, we'll simulate this by checking against known assets
        let known_models = [
            "models/units/worker/worker.glb",
            "models/units/fighter/fighter.glb", 
            "models/units/ranger/ranger.glb",
            "models/units/tank/tank.glb",
            "models/buildings/headquarters/headquarters.glb",
            "models/buildings/barracks/barracks.glb",
            "models/buildings/war_factory/war_factory.glb",
            "models/buildings/energy_plant/energy_plant.glb",
            "models/buildings/defense_turret/defense_turret.glb",
            "models/resources/minerals/minerals.glb",
            "models/resources/energy/energy.glb",
        ];
        
        known_models.contains(&path)
    }
    
    pub fn animation_exists(&self, path: &str) -> bool {
        // Check if animation file exists
        // For demonstration, we'll mark some animations as existing
        let known_animations = [
            "models/units/worker/animations/idle.glb",
            "models/units/worker/animations/walking.glb",
            "models/units/worker/animations/gathering_minerals.glb",
            "models/units/fighter/animations/idle.glb",
            "models/units/fighter/animations/walking.glb",
            "models/units/fighter/animations/melee_attack.glb",
            "models/buildings/headquarters/animations/idle.glb",
            "models/buildings/headquarters/animations/construction.glb",
            "models/resources/minerals/animations/idle.glb",
            "models/resources/energy/animations/idle.glb",
        ];
        
        known_animations.contains(&path)
    }
    
    pub async fn load_initial_assets(&mut self) {
        self.load_resources().await;
    }
}

impl Default for ResourceManager {
    fn default() -> Self {
        Self::new()
    }
}
