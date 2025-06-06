use macroquad::prelude::*;

pub struct ZoomSystem {
    pub current_level: i32,
    pub target_level: i32,
    pub interpolation_progress: f32,
    pub interpolation_speed: f32,
    pub home_position: Vec2,
    pub base_scale: f64,
    pub zoom_factor: f64,
    pub max_level: i32,
}

impl ZoomSystem {
    pub fn new() -> Self {
        const UNIVERSE_DIAMETER_M: f64 = 8.8e26;
        const BASE_SCALE: f64 = 1.0;
        const MAX_LEVEL: i32 = 50;
        
        let zoom_factor = (UNIVERSE_DIAMETER_M / BASE_SCALE).powf(1.0 / (MAX_LEVEL - 1) as f64);
        
        Self {
            current_level: 8,
            target_level: 8,
            interpolation_progress: 1.0,
            interpolation_speed: 3.0,
            home_position: Vec2::new(400.0, 300.0),
            base_scale: BASE_SCALE,
            zoom_factor,
            max_level: MAX_LEVEL,
        }
    }

    pub fn update(&mut self) {
        if self.interpolation_progress < 1.0 {
            self.interpolation_progress += self.interpolation_speed * 0.016;
            if self.interpolation_progress >= 1.0 {
                self.interpolation_progress = 1.0;
                self.current_level = self.target_level;
            }
        }

        // Handle zoom input
        let mouse_wheel = mouse_wheel().1;
        if mouse_wheel > 0.0 && self.current_level > 1 {
            self.zoom_to_level(self.current_level - 1);
        } else if mouse_wheel < 0.0 && self.current_level < self.max_level {
            self.zoom_to_level(self.current_level + 1);
        }

        // Handle keyboard zoom
        if is_key_pressed(KeyCode::Equal) || is_key_pressed(KeyCode::KpAdd) {
            if self.current_level > 1 {
                self.zoom_to_level(self.current_level - 1);
            }
        }
        if is_key_pressed(KeyCode::Minus) || is_key_pressed(KeyCode::KpSubtract) {
            if self.current_level < self.max_level {
                self.zoom_to_level(self.current_level + 1);
            }
        }

        // Home key
        if is_key_pressed(KeyCode::Home) {
            self.zoom_to_level(8);
        }
    }

    pub fn zoom_to_level(&mut self, level: i32) {
        if level >= 1 && level <= self.max_level && level != self.target_level {
            self.target_level = level;
            self.interpolation_progress = 0.0;
        }
    }

    pub fn get_current_scale(&self) -> f64 {
        if self.interpolation_progress >= 1.0 {
            self.get_scale_for_level(self.current_level)
        } else {
            let current_scale = self.get_scale_for_level(self.current_level);
            let target_scale = self.get_scale_for_level(self.target_level);
            
            let log_current = current_scale.ln();
            let log_target = target_scale.ln();
            let log_interpolated = log_current + (log_target - log_current) * self.interpolation_progress as f64;
            
            log_interpolated.exp()
        }
    }

    pub fn get_scale_for_level(&self, level: i32) -> f64 {
        self.base_scale * self.zoom_factor.powi(level - 1)
    }

    pub fn get_lod_level(&self) -> i32 {
        match self.current_level {
            1..=5 => 3,
            6..=15 => 2,
            16..=30 => 1,
            _ => 0,
        }
    }
    
    pub fn get_zoom_label(&self) -> String {
        format!("Zoom Level: {}", self.current_level)
    }
    
    pub fn get_zoom_description(&self) -> String {
        match self.current_level {
            1..=5 => "Tactical View".to_string(),
            6..=15 => "Regional View".to_string(),
            16..=30 => "Strategic View".to_string(),
            31..=50 => "Cosmic View".to_string(),
            _ => "Unknown Scale".to_string(),
        }
    }
}
