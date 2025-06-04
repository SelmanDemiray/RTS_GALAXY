use macroquad::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoomSystem {
    pub current_level: i32,
    pub target_level: i32,
    pub interpolation_progress: f32,
    pub interpolation_speed: f32,
    #[serde(with = "vec2_serde")]
    pub home_position: Vec2,
    pub base_scale: f64,
    pub zoom_factor: f64,
    pub max_level: i32,
}

mod vec2_serde {
    use super::*;
    use serde::{Deserializer, Serializer};

    pub fn serialize<S>(vec: &Vec2, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (vec.x, vec.y).serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec2, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (x, y) = <(f32, f32)>::deserialize(deserializer)?;
        Ok(Vec2::new(x, y))
    }
}

impl ZoomSystem {
    pub fn new() -> Self {
        // Constants based on the mathematical model
        const UNIV_DIAMETER_M: f64 = 8.8e26; // ≈93 billion light-years in meters
        const BASE_SCALE: f64 = 1.0; // 1 meter at level 1
        const MAX_LEVEL: i32 = 50;
        
        // Calculate zoom factor: F = (D_univ / L_1)^(1/49) ≈ 3.55
        let zoom_factor = (UNIV_DIAMETER_M / BASE_SCALE).powf(1.0 / (MAX_LEVEL - 1) as f64);
        
        Self {
            current_level: 15, // Start at a reasonable "region" level
            target_level: 15,
            interpolation_progress: 1.0,
            interpolation_speed: 3.0,
            home_position: Vec2::new(0.0, 0.0), // Will be set to HQ position
            base_scale: BASE_SCALE,
            zoom_factor,
            max_level: MAX_LEVEL,
        }
    }
    
    pub fn get_scale_for_level(&self, level: i32) -> f64 {
        self.base_scale * self.zoom_factor.powi(level - 1)
    }
    
    pub fn get_current_scale(&self) -> f64 {
        if self.interpolation_progress >= 1.0 {
            self.get_scale_for_level(self.current_level)
        } else {
            // Interpolate between current and target level scales
            let current_scale = self.get_scale_for_level(self.current_level);
            let target_scale = self.get_scale_for_level(self.target_level);
            
            // Use logarithmic interpolation for smooth zoom
            let log_current = current_scale.ln();
            let log_target = target_scale.ln();
            let log_interpolated = log_current + (log_target - log_current) * self.interpolation_progress as f64;
            log_interpolated.exp()
        }
    }
    
    pub fn zoom_in(&mut self) {
        if self.current_level > 1 {
            self.target_level = self.current_level - 1;
            self.interpolation_progress = 0.0;
        }
    }
    
    pub fn zoom_out(&mut self) {
        if self.current_level < self.max_level {
            self.target_level = self.current_level + 1;
            self.interpolation_progress = 0.0;
        }
    }
    
    pub fn set_zoom_level(&mut self, level: i32) {
        let clamped_level = level.clamp(1, self.max_level);
        if clamped_level != self.current_level {
            self.target_level = clamped_level;
            self.interpolation_progress = 0.0;
        }
    }
    
    pub fn go_home(&mut self) -> Vec2 {
        // Return to a reasonable zoom level for viewing HQ (level 8 = small city scale)
        self.set_zoom_level(8);
        self.home_position
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
    
    pub fn set_home_position(&mut self, position: Vec2) {
        self.home_position = position;
    }
    
    pub fn get_zoom_label(&self) -> String {
        match self.current_level {
            1..=3 => format!("Level {}: Unit Detail", self.current_level),
            4..=6 => format!("Level {}: Small Village", self.current_level),
            7..=9 => format!("Level {}: Town/City", self.current_level),
            10..=12 => format!("Level {}: Region", self.current_level),
            13..=15 => format!("Level {}: Continent", self.current_level),
            16..=18 => format!("Level {}: Planet Surface", self.current_level),
            19..=21 => format!("Level {}: Planetary System", self.current_level),
            22..=24 => format!("Level {}: Solar Neighborhood", self.current_level),
            25..=27 => format!("Level {}: Nearby Stars", self.current_level),
            28..=30 => format!("Level {}: Local Cluster", self.current_level),
            31..=33 => format!("Level {}: Spiral Arm", self.current_level),
            34..=36 => format!("Level {}: Galaxy", self.current_level),
            37..=39 => format!("Level {}: Local Group", self.current_level),
            40..=42 => format!("Level {}: Supercluster", self.current_level),
            43..=49 => format!("Level {}: Cosmic Web", self.current_level),
            50 => "Level 50: Observable Universe".to_string(),
            _ => format!("Level {}: Unknown", self.current_level),
        }
    }
    
    pub fn get_zoom_description(&self) -> String {
        let scale = self.get_current_scale();
        match self.current_level {
            1..=3 => format!("~{:.1} meters - Individual units and buildings", scale),
            4..=6 => format!("~{:.0} meters - Village scale", scale),
            7..=9 => format!("~{:.1} km - City scale", scale / 1000.0),
            10..=12 => format!("~{:.0} km - Regional scale", scale / 1000.0),
            13..=15 => format!("~{:.0} km - Continental scale", scale / 1000.0),
            16..=18 => format!("~{:.0} km - Planetary scale", scale / 1000.0),
            19..=24 => {
                let au = scale / 1.496e11; // Convert to AU
                format!("~{:.1} AU - Solar system scale", au)
            },
            25..=30 => {
                let ly = scale / 9.461e15; // Convert to light-years
                format!("~{:.2} ly - Interstellar scale", ly)
            },
            31..=36 => {
                let ly = scale / 9.461e15;
                format!("~{:.0} ly - Galactic scale", ly)
            },
            37..=42 => {
                let ly = scale / 9.461e15;
                format!("~{:.0} Mly - Intergalactic scale", ly / 1e6)
            },
            43..=50 => {
                let ly = scale / 9.461e15;
                format!("~{:.0} Gly - Cosmic scale", ly / 1e9)
            },
            _ => format!("~{:.2e} meters", scale),
        }
    }
}
