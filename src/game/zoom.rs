use macroquad::prelude::*;

pub struct ZoomSystem {
    pub current_level: i32,
    pub target_level: i32,
    pub interpolation_progress: f32,
    pub interpolation_speed: f32,
    pub home_position: Vec2,
    
    // Mathematical constants
    pub base_scale: f64,
    pub zoom_factor: f64,
    pub max_level: i32,
    pub min_level: i32,
}

impl ZoomSystem {
    pub fn new() -> Self {
        // Calculate zoom factor to cover observable universe
        let universe_diameter = 8.8e26; // meters (93 billion light-years)
        let base_scale = 1.0; // 1 meter at level 1
        let max_level = 50;
        
        let zoom_factor = (universe_diameter / base_scale).powf(1.0 / (max_level - 1) as f64);
        
        Self {
            current_level: 8, // Start at tactical level
            target_level: 8,
            interpolation_progress: 1.0,
            interpolation_speed: 3.0,
            home_position: Vec2::new(100.0, 100.0),
            
            base_scale,
            zoom_factor,
            max_level,
            min_level: 1,
        }
    }
    
    pub fn zoom_in(&mut self) {
        if self.current_level > self.min_level {
            self.target_level = (self.current_level - 1).max(self.min_level);
            self.interpolation_progress = 0.0;
        }
    }
    
    pub fn zoom_out(&mut self) {
        if self.current_level < self.max_level {
            self.target_level = (self.current_level + 1).min(self.max_level);
            self.interpolation_progress = 0.0;
        }
    }
    
    pub fn set_zoom_level(&mut self, level: i32) {
        let clamped_level = level.clamp(self.min_level, self.max_level);
        if clamped_level != self.current_level {
            self.target_level = clamped_level;
            self.interpolation_progress = 0.0;
        }
    }
    
    pub fn update(&mut self, dt: f32) {
        if self.interpolation_progress < 1.0 {
            self.interpolation_progress += self.interpolation_speed * dt;
            if self.interpolation_progress >= 1.0 {
                self.interpolation_progress = 1.0;
                self.current_level = self.target_level;
            }
        }
    }
    
    pub fn get_current_scale(&self) -> f64 {
        if self.interpolation_progress >= 1.0 {
            self.get_scale_for_level(self.current_level)
        } else {
            // Logarithmic interpolation between current and target scales
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
    
    pub fn get_zoom_label(&self) -> String {
        let level = if self.interpolation_progress >= 1.0 {
            self.current_level
        } else {
            self.current_level // Show current during transition
        };
        
        format!("Zoom Level {}: {}", level, self.get_level_name(level))
    }
    
    pub fn get_zoom_description(&self) -> String {
        let scale = self.get_current_scale();
        self.format_scale_description(scale)
    }
    
    fn get_level_name(&self, level: i32) -> &'static str {
        match level {
            1..=3 => "Unit Detail",
            4..=6 => "Local Area",
            7..=9 => "Tactical View",
            10..=12 => "Regional Command",
            13..=15 => "Continental Scale",
            16..=18 => "Planetary Surface",
            19..=21 => "System Overview",
            22..=24 => "Local Space",
            25..=27 => "Stellar Neighborhood",
            28..=30 => "Star Cluster",
            31..=33 => "Galactic Arm",
            34..=36 => "Galaxy View",
            37..=39 => "Local Group",
            40..=42 => "Supercluster",
            43..=49 => "Cosmic Web",
            50 => "Observable Universe",
            _ => "Unknown Scale",
        }
    }
    
    fn format_scale_description(&self, scale: f64) -> String {
        if scale < 1e3 {
            format!("{:.0} meters", scale)
        } else if scale < 1e6 {
            format!("{:.1} kilometers", scale / 1e3)
        } else if scale < 9.461e15 {
            format!("{:.0} kilometers", scale / 1e3)
        } else if scale < 9.461e18 {
            format!("{:.2} light-years", scale / 9.461e15)
        } else if scale < 9.461e21 {
            format!("{:.0} light-years", scale / 9.461e15)
        } else if scale < 9.461e24 {
            format!("{:.0} thousand light-years", scale / 9.461e18)
        } else if scale < 9.461e27 {
            format!("{:.1} million light-years", scale / 9.461e21)
        } else {
            format!("{:.1} billion light-years", scale / 9.461e24)
        }
    }
    
    pub fn set_home_position(&mut self, position: Vec2) {
        self.home_position = position;
    }
    
    pub fn go_home(&mut self) -> Vec2 {
        // Return to optimal zoom level for base management
        self.set_zoom_level(8);
        self.home_position
    }
    
    pub fn get_lod_level(&self) -> i32 {
        // Return level of detail based on zoom
        // Higher zoom levels = lower detail needed
        match self.current_level {
            1..=5 => 3,    // High detail
            6..=15 => 2,   // Medium detail
            16..=30 => 1,  // Low detail
            _ => 0,        // Icon/minimal detail
        }
    }
    
    pub fn should_render_at_scale(&self, object_size: f32) -> bool {
        let current_scale = self.get_current_scale() as f32;
        
        // Object should be visible if it's reasonably sized at current scale
        let apparent_size = object_size / current_scale;
        apparent_size >= 0.01 && apparent_size <= 100.0
    }
}
