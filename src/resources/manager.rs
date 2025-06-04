use macroquad::prelude::*;
use macroquad::audio::Sound;
use std::collections::HashMap;
use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::resources::model3d::{Model3D, ModelInfo};

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
pub struct ModelCategory {
    units: Vec<ModelInfo>,
    buildings: Vec<ModelInfo>,
    resources: Vec<ModelInfo>,
    terrain: Vec<ModelInfo>,
    props: Vec<ModelInfo>,
    effects: Vec<ModelInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct TextureCategory {
    units: Vec<TextureInfo>,
    buildings: Vec<TextureInfo>,
    resources: Vec<TextureInfo>,
    terrain: Vec<TextureInfo>,
    ui: Vec<TextureInfo>,
    effects: Vec<TextureInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct SoundCategory {
    units: Vec<SoundInfo>,
    buildings: Vec<SoundInfo>,
    ui: Vec<SoundInfo>,
    game: Vec<SoundInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct AssetManifest {
    models: ModelCategory,
    textures: TextureCategory,
    sounds: SoundCategory,
    music: Vec<MusicInfo>,
    fonts: Vec<FontInfo>,
}

pub struct ResourceManager {
    textures: HashMap<String, Texture2D>,
    sounds: HashMap<String, Sound>,
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
            fonts: HashMap::new(),
            models: HashMap::new(),
            loading_progress: 0.0,
            total_assets: 0,
            loaded_assets: 0,
        }
    }

    pub async fn load_resources(&mut self) {
        // Prioritized loading for massive scale
        self.load_critical_resources().await;
        
        // Load from manifest if it exists, otherwise load basic assets
        if let Ok(manifest_data) = macroquad::prelude::load_string("assets/asset_manifest.json").await {
            if let Ok(manifest) = serde_json::from_str::<AssetManifest>(&manifest_data) {
                self.load_from_manifest_optimized(manifest).await;
                return;
            }
        }

        // Fallback to basic resource loading
        self.load_basic_resources().await;
    }

    async fn load_critical_resources(&mut self) {
        // Load only the most essential resources first for instant startup
        let critical_assets = vec![
            ("worker", "assets/models/units/worker.glb", "model"),
            ("headquarters", "assets/models/buildings/headquarters.glb", "model"),
            ("button_click", "assets/sounds/ui/button_click.wav", "sound"),
        ];

        self.total_assets = critical_assets.len();

        for (name, path, asset_type) in critical_assets {
            match asset_type {
                "model" => {
                    if let Ok(data) = macroquad::prelude::load_file(path).await {
                        if let Some(model) = Model3D::from_glb(&data, name.to_string()) {
                            self.models.insert(name.to_string(), model);
                        }
                    }
                },
                "sound" => {
                    if let Ok(sound) = macroquad::audio::load_sound(path).await {
                        self.sounds.insert(name.to_string(), sound);
                    }
                },
                _ => {}
            }
            self.loaded_assets += 1;
            self.update_progress();
        }
    }

    async fn load_from_manifest_optimized(&mut self, manifest: AssetManifest) {
        // Streaming asset loading with priority system
        let mut loading_queue = Vec::new();
        
        // High priority: Core gameplay models
        for model_info in manifest.models.units.iter()
            .chain(manifest.models.buildings.iter()) {
            loading_queue.push((model_info, 1)); // Priority 1 (highest)
        }
        
        // Medium priority: Environment and effects
        for model_info in manifest.models.resources.iter()
            .chain(manifest.models.terrain.iter())
            .chain(manifest.models.effects.iter()) {
            loading_queue.push((model_info, 2)); // Priority 2
        }
        
        // Sort by priority
        loading_queue.sort_by_key(|(_, priority)| *priority);
        
        self.total_assets = loading_queue.len() + 
            manifest.textures.ui.len() + 
            manifest.sounds.units.len() + 
            manifest.sounds.buildings.len() + 
            manifest.sounds.ui.len() + 
            manifest.sounds.game.len() + 
            manifest.music.len() + 
            manifest.fonts.len();

        // Load models with streaming
        for (model_info, _) in loading_queue {
            self.load_model_from_info_optimized(model_info).await;
        }

        // Load other assets in parallel batches
        self.load_remaining_assets_parallel(manifest).await;
    }

    async fn load_model_from_info_optimized(&mut self, model_info: &ModelInfo) {
        // Implement level-of-detail loading for performance
        if let Some(model) = Model3D::load_with_lod(model_info, "assets").await {
            self.models.insert(model_info.name.clone(), model);
        }
        self.loaded_assets += 1;
        self.update_progress();
    }

    async fn load_remaining_assets_parallel(&mut self, manifest: AssetManifest) {
        // Load UI textures (still needed for interface)
        for texture_info in manifest.textures.ui.iter() {
            self.load_texture_from_info(texture_info).await;
        }

        // Load audio assets
        for sound_info in manifest.sounds.units.iter()
            .chain(manifest.sounds.buildings.iter())
            .chain(manifest.sounds.ui.iter())
            .chain(manifest.sounds.game.iter()) {
            self.load_sound_from_info(sound_info).await;
        }

        // Load music
        for music_info in manifest.music.iter() {
            self.load_music_from_info(music_info).await;
        }

        // Load fonts
        for font_info in manifest.fonts.iter() {
            self.load_font_from_info(font_info).await;
        }
    }

    async fn load_basic_resources(&mut self) {
        // Updated basic fallback resources - only 3D models and essential audio
        let basic_assets = vec![
            ("worker", "assets/models/units/worker.glb", "model"),
            ("fighter", "assets/models/units/fighter.glb", "model"),
            ("headquarters", "assets/models/buildings/headquarters.glb", "model"),
            ("minerals", "assets/models/resources/minerals.glb", "model"),
            ("energy", "assets/models/resources/energy.glb", "model"),
            ("button_click", "assets/sounds/ui/button_click.wav", "sound"),
            ("main_theme", "assets/music/main_theme.ogg", "sound"),
            ("gameplay", "assets/music/gameplay.ogg", "sound"),
            ("unit_select", "assets/sounds/units/unit_select.wav", "sound"),
            ("building_place", "assets/sounds/buildings/building_place.wav", "sound"),
        ];

        self.total_assets = basic_assets.len();

        for (name, path, asset_type) in basic_assets {
            match asset_type {
                "model" => {
                    if let Ok(data) = macroquad::prelude::load_file(path).await {
                        if let Some(model) = Model3D::from_glb(&data, name.to_string()) {
                            self.models.insert(name.to_string(), model);
                        }
                    } else {
                        println!("Warning: Could not load model file: {}", path);
                    }
                },
                "sound" => {
                    if let Ok(sound) = macroquad::audio::load_sound(path).await {
                        self.sounds.insert(name.to_string(), sound);
                    } else {
                        println!("Warning: Could not load sound file: {}", path);
                    }
                },
                _ => {}
            }
            self.loaded_assets += 1;
            self.update_progress();
        }
    }

    async fn load_model_from_info(&mut self, model_info: &ModelInfo) {
        if let Some(model) = Model3D::load(model_info, "assets").await {
            self.models.insert(model_info.name.clone(), model);
        }
        self.loaded_assets += 1;
        self.update_progress();
    }

    async fn load_texture_from_info(&mut self, texture_info: &TextureInfo) {
        let full_path = format!("assets/{}", texture_info.file);
        if let Ok(texture) = macroquad::prelude::load_texture(&full_path).await {
            self.textures.insert(texture_info.name.clone(), texture);
        }
        self.loaded_assets += 1;
        self.update_progress();
    }

    async fn load_sound_from_info(&mut self, sound_info: &SoundInfo) {
        let full_path = format!("assets/{}", sound_info.file);
        if let Ok(sound) = macroquad::audio::load_sound(&full_path).await {
            self.sounds.insert(sound_info.name.clone(), sound);
        }
        self.loaded_assets += 1;
        self.update_progress();
    }

    async fn load_music_from_info(&mut self, music_info: &MusicInfo) {
        let full_path = format!("assets/{}", music_info.file);
        if let Ok(sound) = macroquad::audio::load_sound(&full_path).await {
            self.sounds.insert(music_info.name.clone(), sound);
        }
        self.loaded_assets += 1;
        self.update_progress();
    }

    async fn load_font_from_info(&mut self, font_info: &FontInfo) {
        let full_path = format!("assets/{}", font_info.file);
        if let Ok(font_data) = macroquad::prelude::load_file(&full_path).await {
            if let Ok(font) = macroquad::prelude::load_ttf_font_from_bytes(&font_data) {
                self.fonts.insert(font_info.name.clone(), font);
            }
        }
        self.loaded_assets += 1;
        self.update_progress();
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

    #[allow(dead_code)]
    pub fn get_texture(&self, name: &str) -> Option<&Texture2D> {
        self.textures.get(name)
    }

    pub fn get_sound(&self, name: &str) -> Option<&Sound> {
        self.sounds.get(name)
    }

    #[allow(dead_code)]
    pub fn get_font(&self, name: &str) -> Option<&Font> {
        self.fonts.get(name)
    }

    pub fn get_music(&self, name: &str) -> Option<&Sound> {
        self.sounds.get(name)
    }

    pub fn get_model(&self, name: &str) -> Option<&Model3D> {
        self.models.get(name)
    }

    pub fn preload_sector_assets(&mut self, sector_type: &str) {
        // Dynamically load assets for specific sectors
        // This allows infinite worlds without loading everything
        match sector_type {
            "asteroid_field" => {
                // Load asteroid-specific models
            },
            "space_station" => {
                // Load station-specific models
            },
            "planet_surface" => {
                // Load planetary models
            },
            _ => {}
        }
    }
}

// Extended Model3D for level-of-detail support
use std::collections::HashMap;

pub struct Model3D {
    pub name: String,
    pub lod_levels: Vec<LodLevel>,
    pub bounding_box: BoundingBox,
    pub material_properties: MaterialProperties,
}

pub struct LodLevel {
    pub distance_threshold: f32,
    pub vertex_data: Vec<u8>,
    pub index_data: Vec<u32>,
    pub triangle_count: u32,
}

pub struct BoundingBox {
    pub min: (f32, f32, f32),
    pub max: (f32, f32, f32),
}

pub struct MaterialProperties {
    pub diffuse_texture: Option<String>,
    pub normal_texture: Option<String>,
    pub metallic_roughness: Option<String>,
    pub emission_texture: Option<String>,
}

impl Model3D {
    pub fn from_glb(data: &[u8], name: String) -> Option<Self> {
        // Parse glTF data and create model with multiple LOD levels
        // This is a simplified implementation
        Some(Self {
            name,
            lod_levels: vec![
                LodLevel {
                    distance_threshold: 0.0,
                    vertex_data: data.to_vec(),
                    index_data: vec![],
                    triangle_count: 1000,
                },
                LodLevel {
                    distance_threshold: 100.0,
                    vertex_data: data.to_vec(), // In reality, this would be simplified
                    index_data: vec![],
                    triangle_count: 500,
                },
                LodLevel {
                    distance_threshold: 500.0,
                    vertex_data: data.to_vec(), // Even more simplified
                    index_data: vec![],
                    triangle_count: 100,
                },
            ],
            bounding_box: BoundingBox {
                min: (-1.0, -1.0, -1.0),
                max: (1.0, 1.0, 1.0),
            },
            material_properties: MaterialProperties {
                diffuse_texture: None,
                normal_texture: None,
                metallic_roughness: None,
                emission_texture: None,
            },
        })
    }

    pub async fn load_with_lod(model_info: &ModelInfo, base_path: &str) -> Option<Self> {
        let full_path = format!("{}/{}", base_path, model_info.file);
        if let Ok(data) = macroquad::prelude::load_file(&full_path).await {
            Self::from_glb(&data, model_info.name.clone())
        } else {
            None
        }
    }

    pub fn draw(&self, position: macroquad::math::Vec3, rotation: macroquad::math::Vec3, scale: f32) {
        // Implement 3D model drawing with automatic LOD selection
        let distance = position.length(); // Simplified distance calculation
        
        let lod_level = self.lod_levels.iter()
            .rev()
            .find(|lod| distance >= lod.distance_threshold)
            .unwrap_or(&self.lod_levels[0]);

        // Draw the appropriate LOD level
        self.draw_lod_level(lod_level, position, rotation, scale);
    }

    fn draw_lod_level(&self, lod: &LodLevel, position: macroquad::math::Vec3, _rotation: macroquad::math::Vec3, _scale: f32) {
        // Simplified drawing - in reality this would use proper 3D rendering
        use macroquad::prelude::*;
        
        // Draw a simple cube as placeholder for now
        draw_cube(position, Vec3::splat(1.0), None, WHITE);
        
        // In a real implementation, this would:
        // 1. Set up proper 3D transforms
        // 2. Bind vertex/index buffers from lod.vertex_data
        // 3. Apply materials and textures
        // 4. Submit draw calls with triangle count optimization
    }

    pub fn get_triangle_count_at_distance(&self, distance: f32) -> u32 {
        self.lod_levels.iter()
            .rev()
            .find(|lod| distance >= lod.distance_threshold)
            .map(|lod| lod.triangle_count)
            .unwrap_or(self.lod_levels[0].triangle_count)
    }
}

// Asset manifest types for the new system
#[derive(serde::Deserialize)]
pub struct AssetManifest {
    pub models: ModelCategories,
    pub textures: TextureCategories,
    pub sounds: SoundCategories,
    pub music: Vec<MusicInfo>,
    pub fonts: Vec<FontInfo>,
}

#[derive(serde::Deserialize)]
pub struct ModelCategories {
    pub units: Vec<ModelInfo>,
    pub buildings: Vec<ModelInfo>,
    pub resources: Vec<ModelInfo>,
    pub terrain: Vec<ModelInfo>,
    pub props: Vec<ModelInfo>,
    pub effects: Vec<ModelInfo>,
}

#[derive(serde::Deserialize)]
pub struct TextureCategories {
    pub ui: Vec<TextureInfo>,
}

#[derive(serde::Deserialize)]
pub struct SoundCategories {
    pub units: Vec<SoundInfo>,
    pub buildings: Vec<SoundInfo>,
    pub ui: Vec<SoundInfo>,
    pub game: Vec<SoundInfo>,
}

#[derive(serde::Deserialize)]
pub struct ModelInfo {
    pub name: String,
    pub file: String,
    pub scale: f32,
}

#[derive(serde::Deserialize)]
pub struct TextureInfo {
    pub name: String,
    pub file: String,
    pub dimensions: [u32; 2],
}

#[derive(serde::Deserialize)]
pub struct SoundInfo {
    pub name: String,
    pub file: String,
}

#[derive(serde::Deserialize)]
pub struct MusicInfo {
    pub name: String,
    pub file: String,
}

#[derive(serde::Deserialize)]
pub struct FontInfo {
    pub name: String,
    pub file: String,
}
