use macroquad::prelude::*;
use std::collections::HashMap;

pub struct ResourceManager {
    #[allow(dead_code)]
    textures: HashMap<String, Texture2D>,  // Keeping for future texture implementation
    fonts: HashMap<String, Font>,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            fonts: HashMap::new(),
        }
    }
    
    pub async fn load_resources(&mut self) {
        // Use macroquad's built-in font instead of trying to load from file
        let font = Font::default();
        self.fonts.insert("default".to_string(), font);
        
        // Future texture loading code will go here
        // self.textures.insert("example".to_string(), load_texture("path/to/texture.png").await.unwrap());
    }
    
    #[allow(dead_code)]
    pub fn get_texture(&self, name: &str) -> Option<&Texture2D> {
        self.textures.get(name)
    }
    
    pub fn get_font(&self, name: &str) -> Option<&Font> {
        self.fonts.get(name)
    }
}
