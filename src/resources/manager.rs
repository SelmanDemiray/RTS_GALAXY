use macroquad::prelude::*;
use macroquad::audio::{Sound, load_sound};
use std::collections::HashMap;
use std::path::Path;
use serde::{Deserialize, Serialize};
use std::fs;

// Asset manifest structures
#[derive(Serialize, Deserialize)]
struct TextureInfo {
    name: String,
    file: String,
    dimensions: [u32; 2],
    #[serde(default)]
    frames: u32,
}

#[derive(Serialize, Deserialize)]
struct SoundInfo {
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
struct MusicInfo {
    name: String,
    file: String,
}

#[derive(Serialize, Deserialize)]
struct FontInfo {
    name: String,
    file: String,
}

#[derive(Serialize, Deserialize)]
struct TextureCategory {
    units: Vec<TextureInfo>,
    buildings: Vec<TextureInfo>,
    resources: Vec<TextureInfo>,
    terrain: Vec<TextureInfo>,
    ui: Vec<TextureInfo>,
    effects: Vec<TextureInfo>,
}

#[derive(Serialize, Deserialize)]
struct SoundCategory {
    units: Vec<SoundInfo>,
    buildings: Vec<SoundInfo>,
    ui: Vec<SoundInfo>,
    game: Vec<SoundInfo>,
}

#[derive(Serialize, Deserialize)]
struct AssetManifest {
    textures: TextureCategory,
    sounds: SoundCategory,
    music: Vec<MusicInfo>,
    fonts: Vec<FontInfo>,
}

pub struct ResourceManager {
    textures: HashMap<String, Texture2D>,
    sounds: HashMap<String, Sound>,
    music: HashMap<String, Sound>,
    fonts: HashMap<String, Font>,
    // Track if we've loaded all assets
    assets_loaded: bool,
    // Track the total number of assets and how many are loaded
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
            assets_loaded: false,
            total_assets: 0,
            loaded_assets: 0,
        }
    }
    
    pub async fn load_resources(&mut self) {
        // First try to load the asset manifest
        let manifest_result = self.load_asset_manifest().await;
        
        match manifest_result {
            Ok(manifest) => {
                // Count total assets to track loading progress
                self.total_assets = 
                    manifest.textures.units.len() +
                    manifest.textures.buildings.len() +
                    manifest.textures.resources.len() +
                    manifest.textures.terrain.len() +
                    manifest.textures.ui.len() +
                    manifest.textures.effects.len() +
                    manifest.sounds.units.len() +
                    manifest.sounds.buildings.len() +
                    manifest.sounds.ui.len() +
                    manifest.sounds.game.len() +
                    manifest.music.len() +
                    manifest.fonts.len();
                
                // Load textures
                self.load_texture_category(&manifest.textures.units, "units").await;
                self.load_texture_category(&manifest.textures.buildings, "buildings").await;
                self.load_texture_category(&manifest.textures.resources, "resources").await;
                self.load_texture_category(&manifest.textures.terrain, "terrain").await;
                self.load_texture_category(&manifest.textures.ui, "ui").await;
                self.load_texture_category(&manifest.textures.effects, "effects").await;
                
                // Load sounds
                self.load_sound_category(&manifest.sounds.units).await;
                self.load_sound_category(&manifest.sounds.buildings).await;
                self.load_sound_category(&manifest.sounds.ui).await;
                self.load_sound_category(&manifest.sounds.game).await;
                
                // Load music
                for music_info in &manifest.music {
                    if let Some(sound) = self.load_music(&music_info.file).await {
                        self.music.insert(music_info.name.clone(), sound);
                        self.loaded_assets += 1;
                    }
                }
                
                // Load fonts
                for font_info in &manifest.fonts {
                    if let Some(font) = self.load_font(&font_info.file).await {
                        self.fonts.insert(font_info.name.clone(), font);
                        self.loaded_assets += 1;
                    }
                }
            },
            Err(e) => {
                println!("Failed to load asset manifest: {}", e);
                // Fallback to built-in font if manifest loading fails
                self.fonts.insert("default".to_string(), Font::default());
            }
        }
        
        // If we couldn't load any assets, ensure we at least have a default font
        if self.fonts.is_empty() {
            self.fonts.insert("default".to_string(), Font::default());
        }
        
        // Mark assets as loaded
        self.assets_loaded = true;
    }
    
    async fn load_asset_manifest(&self) -> Result<AssetManifest, String> {
        let manifest_path = "assets/asset_manifest.json";
        
        if !Path::new(manifest_path).exists() {
            return Err(format!("Asset manifest not found at {}", manifest_path));
        }
        
        match fs::read_to_string(manifest_path) {
            Ok(content) => {
                match serde_json::from_str::<AssetManifest>(&content) {
                    Ok(manifest) => Ok(manifest),
                    Err(e) => Err(format!("Failed to parse asset manifest: {}", e))
                }
            },
            Err(e) => Err(format!("Failed to read asset manifest: {}", e))
        }
    }
    
    async fn load_texture_category(&mut self, textures: &[TextureInfo], category: &str) {
        for texture_info in textures {
            if let Some(texture) = self.load_texture(&texture_info.file).await {
                let key = if category.is_empty() {
                    texture_info.name.clone()
                } else {
                    format!("{}_{}", category, texture_info.name)
                };
                
                self.textures.insert(key, texture);
                self.loaded_assets += 1;
            }
        }
    }
    
    async fn load_sound_category(&mut self, sounds: &[SoundInfo]) {
        for sound_info in sounds {
            if let Some(sound) = self.load_sound(&sound_info.file).await {
                self.sounds.insert(sound_info.name.clone(), sound);
                self.loaded_assets += 1;
            }
        }
    }
    
    async fn load_texture(&self, path: &str) -> Option<Texture2D> {
        match load_texture(path).await {
            Ok(texture) => Some(texture),
            Err(e) => {
                println!("Failed to load texture {}: {:?}", path, e);
                None
            }
        }
    }
    
    async fn load_sound(&self, path: &str) -> Option<Sound> {
        match load_sound(path).await {
            Ok(sound) => Some(sound),
            Err(e) => {
                println!("Failed to load sound {}: {:?}", path, e);
                None
            }
        }
    }
    
    async fn load_music(&self, path: &str) -> Option<Sound> {
        // Music is loaded the same way as sound in macroquad
        self.load_sound(path).await
    }
    
    async fn load_font(&self, path: &str) -> Option<Font> {
        match load_ttf_font(path).await {
            Ok(font) => Some(font),
            Err(e) => {
                println!("Failed to load font {}: {:?}", path, e);
                None
            }
        }
    }
    
    #[allow(dead_code)]
    pub fn get_texture(&self, name: &str) -> Option<&Texture2D> {
        self.textures.get(name)
    }
    
    #[allow(dead_code)]
    pub fn get_sound(&self, name: &str) -> Option<&Sound> {
        self.sounds.get(name)
    }
    
    pub fn get_music(&self, name: &str) -> Option<&Sound> {
        self.music.get(name)
    }
    
    #[allow(dead_code)]
    pub fn get_font(&self, name: &str) -> Option<&Font> {
        self.fonts.get(name)
    }
    
    pub fn is_loading_complete(&self) -> bool {
        self.assets_loaded
    }
    
    pub fn get_loading_progress(&self) -> f32 {
        if self.total_assets == 0 {
            return 1.0;
        }
        self.loaded_assets as f32 / self.total_assets as f32
    }
}
