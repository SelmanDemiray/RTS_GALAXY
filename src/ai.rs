use crate::game::{UnitType, GameState};
use macroquad::rand::gen_range;

pub struct AIController {
    think_timer: f32,
}

impl AIController {
    pub fn new() -> Self {
        Self {
            think_timer: 0.0,
        }
    }
    
    pub fn update(&mut self, game_state: &mut GameState) {
        // Update AI timer
        self.think_timer += 0.016; // Assuming ~60 FPS
        
        // Only think every second
        if self.think_timer >= 1.0 {
            self.think_timer = 0.0;
            self.make_decisions(game_state);
        }
    }
    
    fn make_decisions(&mut self, game_state: &mut GameState) {
        // Example: Each soldier unit tries to find an enemy
        for unit_index in 0..game_state.units.len() {
            // Skip if not a soldier
            if game_state.units[unit_index].unit_type != UnitType::Soldier {
                continue;
            }
            
            // Simple AI: Move toward random position or another unit
            if gen_range(0, 2) == 0 {
                // Move to random position
                let target_x = gen_range(50.0, 750.0);
                let target_y = gen_range(50.0, 550.0);
                
                if let Some(unit) = game_state.units.get_mut(unit_index) {
                    self.move_unit_toward(unit, target_x, target_y);
                }
            } else {
                // Find another unit to move toward (e.g., a worker)
                // First find the target's position without holding a reference
                let mut target_x = 400.0;
                let mut target_y = 300.0;
                
                let found_target = game_state.units.iter().enumerate()
                    .find(|(i, u)| *i != unit_index && u.unit_type == UnitType::Worker);
                
                if let Some((_, target)) = found_target {
                    // Store just the coordinates rather than keeping a reference
                    target_x = target.x;
                    target_y = target.y;
                }
                
                // Now we can safely modify the unit without borrowing conflicts
                if let Some(unit) = game_state.units.get_mut(unit_index) {
                    self.move_unit_toward(unit, target_x, target_y);
                }
            }
        }
    }
    
    fn move_unit_toward(&self, unit: &mut crate::game::Unit, target_x: f32, target_y: f32) {
        // Move toward target slowly
        let dx = target_x - unit.x;
        let dy = target_y - unit.y;
        let distance = (dx * dx + dy * dy).sqrt();
        
        if distance > 5.0 {
            let speed = 2.0;
            unit.x += dx / distance * speed;
            unit.y += dy / distance * speed;
        }
    }
}
