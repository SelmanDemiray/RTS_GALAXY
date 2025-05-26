use macroquad::prelude::*;
use macroquad::audio::Sound;
use std::collections::HashMap;
use std::path::Path;
use serde::{Deserialize, Serialize};

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
    textures: TextureCategory,
    sounds: SoundCategory,
    music: Vec<MusicInfo>,
    fonts: Vec<FontInfo>,
}

pub struct ResourceManager {
    textures: HashMap<String, Texture2D>,
    sounds: HashMap<String, Sound>,
    fonts: HashMap<String, Font>,
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
            loading_progress: 0.0,
            total_assets: 0,
            loaded_assets: 0,
        }
    }

    pub async fn load_resources(&mut self) {
        // Load from manifest if it exists, otherwise load basic assets
        if let Ok(manifest_data) = macroquad::prelude::load_string("assets/asset_manifest.json").await {
            if let Ok(manifest) = serde_json::from_str::<AssetManifest>(&manifest_data) {
                self.load_from_manifest(manifest).await;
                return;
            }
        }

        // Fallback to basic resource loading
        self.load_basic_resources().await;
    }

    async fn load_from_manifest(&mut self, manifest: AssetManifest) {
        // Count total assets
        self.total_assets = manifest.textures.units.len() +
                           manifest.textures.buildings.len() +
                           manifest.textures.resources.len() +
                           manifest.textures.terrain.len() +
                           manifest.textures.ui.len() +
                           manifest.sounds.units.len() +
                           manifest.sounds.buildings.len() +
                           manifest.sounds.ui.len() +
                           manifest.sounds.game.len() +
                           manifest.music.len() +
                           manifest.fonts.len();

        // Load textures
        for texture_info in manifest.textures.units.iter()
            .chain(manifest.textures.buildings.iter())
            .chain(manifest.textures.resources.iter())
            .chain(manifest.textures.terrain.iter())
            .chain(manifest.textures.ui.iter()) {
            self.load_texture_from_info(texture_info).await;
        }

        // Load sounds
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
        // Basic fallback resources
        let basic_assets = vec![
            ("worker", "assets/textures/units/worker.png"),
            ("fighter", "assets/textures/units/fighter.png"),
            ("headquarters", "assets/textures/buildings/headquarters.png"),
            ("button_click", "assets/sounds/ui/button_click.wav"),
            ("main_theme", "assets/music/main_theme.ogg"),
        ];

        self.total_assets = basic_assets.len();

        for (name, path) in basic_assets {
            if path.ends_with(".png") {
                if let Ok(texture) = macroquad::prelude::load_texture(path).await {
                    self.textures.insert(name.to_string(), texture);
                }
            } else if path.ends_with(".wav") || path.ends_with(".ogg") {
                if let Ok(sound) = macroquad::audio::load_sound(path).await {
                    self.sounds.insert(name.to_string(), sound);
                }
            }
            self.loaded_assets += 1;
            self.update_progress();
        }
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

    #[allow(dead_code)] // Will be used when texture rendering is implemented
    pub fn get_texture(&self, name: &str) -> Option<&Texture2D> {
        self.textures.get(name)
    }

    pub fn get_sound(&self, name: &str) -> Option<&Sound> {
        self.sounds.get(name)
    }

    #[allow(dead_code)] // Will be used when custom fonts are implemented
    pub fn get_font(&self, name: &str) -> Option<&Font> {
        self.fonts.get(name)
    }

    pub fn get_music(&self, name: &str) -> Option<&Sound> {
        self.sounds.get(name)
    }

    #[allow(dead_code)]
    pub async fn load_sound_from_bytes(&mut self, name: &str, data: &[u8], _path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let sound = macroquad::audio::load_sound_from_bytes(data).await?;
        self.sounds.insert(name.to_string(), sound);
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn load_texture(&mut self, path: &str) -> Result<(), macroquad::prelude::FileError> {
        let texture = macroquad::prelude::load_texture(path).await?;
        let name = Path::new(path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        self.textures.insert(name, texture);
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn load_sound(&mut self, path: &str) -> Result<(), macroquad::prelude::FileError> {
        let sound = macroquad::audio::load_sound(path).await?;
        let name = Path::new(path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        self.sounds.insert(name, sound);
        Ok(())
    }
}
