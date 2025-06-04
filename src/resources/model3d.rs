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
    pub fn new(name: String) -> Self {
        Self {
            name,
            vertices: Vec::new(),
            indices: Vec::new(),
            textures: Vec::new(),
            animations: HashMap::new(),
            default_animation: None,
        }
    }
    
    pub fn draw_with_animation(
        &self,
        position: Vec3,
        rotation: Vec3,
        scale: f32,
        animation_name: &str,
        animation_time: f32,
    ) {
        // Since we don't have actual 3D models loaded, draw a simple representation
        let screen_x = position.x;
        let screen_y = position.y;
        
        // Animation affects the visual representation
        let animation_scale = if let Some(animation) = self.animations.get(animation_name) {
            // Simple pulse effect based on animation time
            let pulse = (animation_time * 3.14159 * 2.0 / animation.duration).sin() * 0.1 + 1.0;
            scale * pulse
        } else {
            scale
        };
        
        // Draw based on model name with animation scaling
        match self.name.as_str() {
            "worker" => {
                draw_circle(screen_x, screen_y, animation_scale * 0.8, BLUE);
                // Draw a small tool if gathering animation
                if animation_name.contains("gathering") {
                    let tool_offset = (animation_time * 2.0).sin() * 5.0;
                    draw_circle(screen_x + tool_offset, screen_y - 10.0, 3.0, GRAY);
                }
            },
            "fighter" => {
                draw_circle(screen_x, screen_y, animation_scale, RED);
                // Draw weapon effect if attacking
                if animation_name.contains("attack") {
                    let flash = ((animation_time * 10.0).sin() * 0.5 + 0.5) * 255.0;
                    draw_circle(screen_x + 15.0, screen_y, 5.0, Color::new(1.0, flash / 255.0, 0.0, 1.0));
                }
            },
            "ranger" => {
                draw_circle(screen_x, screen_y, animation_scale, GREEN);
                // Draw aiming line if aiming/shooting
                if animation_name.contains("aiming") || animation_name.contains("shooting") {
                    draw_line(screen_x, screen_y, screen_x + 50.0, screen_y, 2.0, WHITE);
                }
            },
            "tank" => {
                draw_rectangle(screen_x - animation_scale, screen_y - animation_scale * 0.7, 
                             animation_scale * 2.0, animation_scale * 1.4, DARKGRAY);
                // Draw turret rotation
                if animation_name.contains("turret") {
                    let turret_angle = animation_time * 0.5;
                    let turret_x = screen_x + (turret_angle.cos() * 20.0);
                    let turret_y = screen_y + (turret_angle.sin() * 20.0);
                    draw_line(screen_x, screen_y, turret_x, turret_y, 4.0, BLACK);
                }
            },
            "headquarters" => {
                draw_rectangle(screen_x - animation_scale, screen_y - animation_scale, 
                             animation_scale * 2.0, animation_scale * 2.0, DARKBLUE);
                // Command pulses
                if animation_name.contains("command") {
                    let pulse_radius = animation_scale + (animation_time * 2.0).sin().abs() * 20.0;
                    draw_circle_lines(screen_x, screen_y, pulse_radius, 2.0, LIGHTBLUE);
                }
            },
            // Buildings
            name if name.contains("barracks") => {
                draw_rectangle(screen_x - animation_scale * 0.8, screen_y - animation_scale * 0.6, 
                             animation_scale * 1.6, animation_scale * 1.2, BROWN);
                if animation_name.contains("training") {
                    // Training activity indicator
                    let activity = (animation_time * 4.0).sin().abs();
                    draw_circle(screen_x, screen_y - 20.0, 5.0 * activity, YELLOW);
                }
            },
            name if name.contains("energy_plant") => {
                draw_rectangle(screen_x - animation_scale * 0.7, screen_y - animation_scale * 0.9, 
                             animation_scale * 1.4, animation_scale * 1.8, YELLOW);
                if animation_name.contains("power") {
                    // Energy effects
                    for i in 0..3 {
                        let spark_time = animation_time + i as f32 * 0.5;
                        let spark_x = screen_x + (spark_time * 3.0).sin() * 15.0;
                        let spark_y = screen_y + (spark_time * 2.0).cos() * 10.0;
                        draw_circle(spark_x, spark_y, 2.0, GOLD);
                    }
                }
            },
            name if name.contains("defense_turret") => {
                draw_circle(screen_x, screen_y, animation_scale * 0.8, GRAY);
                // Turret barrel
                let barrel_angle = rotation.y + if animation_name.contains("rotate") {
                    animation_time * 0.5
                } else {
                    0.0
                };
                let barrel_end_x = screen_x + barrel_angle.cos() * animation_scale * 1.5;
                let barrel_end_y = screen_y + barrel_angle.sin() * animation_scale * 1.5;
                draw_line(screen_x, screen_y, barrel_end_x, barrel_end_y, 4.0, DARKGRAY);
                
                if animation_name.contains("firing") {
                    // Muzzle flash
                    draw_circle(barrel_end_x, barrel_end_y, 8.0, WHITE);
                }
            },
            _ => {
                // Default representation
                draw_circle(screen_x, screen_y, animation_scale, PURPLE);
            }
        }
        
        // Draw construction effect for buildings under construction
        if animation_name.contains("construction") {
            let construction_progress = (animation_time % 2.0) / 2.0; // 0 to 1
            let sparks_count = (construction_progress * 10.0) as i32;
            
            for i in 0..sparks_count {
                let spark_angle = (i as f32 / sparks_count as f32) * 6.28318 + animation_time;
                let spark_x = screen_x + spark_angle.cos() * (animation_scale + 10.0);
                let spark_y = screen_y + spark_angle.sin() * (animation_scale + 10.0);
                draw_circle(spark_x, spark_y, 2.0, ORANGE);
            }
        }
    }
    
    pub fn get_animation(&self, name: &str) -> Option<&AnimationClip> {
        self.animations.get(name)
    }
    
    pub fn has_animation(&self, name: &str) -> bool {
        self.animations.contains_key(name)
    }
    
    pub fn add_animation(&mut self, animation: AnimationClip) {
        self.animations.insert(animation.name.clone(), animation);
    }
}
