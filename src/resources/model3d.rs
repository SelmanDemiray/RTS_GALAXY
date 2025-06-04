use macroquad::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AnimationClip {
    pub name: String,
    pub duration: f32,
    pub loop_: bool,
    pub keyframes: Vec<Keyframe>,
}

#[derive(Debug, Clone)]
pub struct Keyframe {
    pub time: f32,
    pub transform: Mat4,
}

#[derive(Debug, Clone)]
pub struct Model3D {
    pub name: String,
    pub vertices: Vec<Vec3>,
    pub indices: Vec<u32>,
    pub textures: Vec<Texture2D>,
    pub animations: HashMap<String, AnimationClip>,
    pub default_animation: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ModelInfo {
    pub name: String,
    pub file: String,
    pub scale: f32,
    #[serde(default = "default_rotation")]
    pub rotation: [f32; 3],
    #[serde(default)]
    pub animations: Vec<AnimationInfo>,
    #[serde(default)]
    pub default_animation: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AnimationInfo {
    pub name: String,
    pub duration: f32,
    #[serde(default = "default_true")]
    pub loop_: bool,
    #[serde(default = "default_animation_speed")]
    pub speed: f32,
}

fn default_rotation() -> [f32; 3] {
    [0.0, 0.0, 0.0]
}

fn default_true() -> bool {
    true
}

fn default_animation_speed() -> f32 {
    1.0
}

impl Model3D {
    pub async fn load(info: &ModelInfo, base_path: &str) -> Option<Self> {
        let path = format!("{}/{}", base_path, info.file);
        
        // Create animations based on model info
        let mut animations = HashMap::new();
        for anim_info in &info.animations {
            let clip = AnimationClip {
                name: anim_info.name.clone(),
                duration: anim_info.duration,
                loop_: anim_info.loop_,
                keyframes: Self::generate_placeholder_keyframes(anim_info.duration),
            };
            animations.insert(anim_info.name.clone(), clip);
        }
        
        Some(Self {
            name: info.name.clone(),
            vertices: vec![],
            indices: vec![],
            textures: vec![],
            animations,
            default_animation: info.default_animation.clone(),
        })
    }

    pub fn from_glb(data: &[u8], name: String) -> Option<Self> {
        // Simplified implementation for now
        Some(Self {
            name,
            vertices: vec![],
            indices: vec![],
            textures: vec![],
            animations: HashMap::new(),
            default_animation: None,
        })
    }
    
    pub fn draw(&self, position: Vec3, rotation: Vec3, scale: f32) {
        // Draw a simple cube as placeholder
        draw_cube(position, Vec3::splat(scale), None, WHITE);
    }
    
    pub fn draw_with_animation(&self, position: Vec3, rotation: Vec3, scale: f32, animation_name: &str, animation_time: f32) {
        if let Some(_animation) = self.animations.get(animation_name) {
            // Apply animation transformations (placeholder implementation)
            let animated_rotation = Vec3::new(
                rotation.x + (animation_time * 2.0).sin() * 0.1,
                rotation.y + animation_time * 0.5,
                rotation.z
            );
            
            // Draw the animated model
            draw_cube(position, Vec3::splat(scale), None, WHITE);
            
            // Visual indicator of animation state
            let color = match animation_name {
                "walking" | "running" => GREEN,
                "attacking" | "shooting" | "melee_attack" => RED,
                "gathering_minerals" | "gathering_energy" => BLUE,
                "building" => ORANGE,
                "dying" => DARKGRAY,
                _ => WHITE,
            };
            
            // Draw a small indicator above the unit
            draw_circle(position.x, position.y - scale - 10.0, 3.0, color);
        } else {
            // Fallback to basic draw
            self.draw(position, rotation, scale);
        }
    }
    
    pub fn has_animation(&self, animation_name: &str) -> bool {
        self.animations.contains_key(animation_name)
    }
    
    pub fn get_animation_duration(&self, animation_name: &str) -> Option<f32> {
        self.animations.get(animation_name).map(|anim| anim.duration)
    }
    
    fn generate_placeholder_keyframes(duration: f32) -> Vec<Keyframe> {
        let frame_count = (duration * 30.0) as usize; // 30 FPS
        let mut keyframes = Vec::new();
        
        for i in 0..frame_count {
            let time = (i as f32) / 30.0;
            keyframes.push(Keyframe {
                time,
                transform: Mat4::IDENTITY,
            });
        }
        
        keyframes
    }
}
