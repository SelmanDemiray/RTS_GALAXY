use crate::game::{GameState, UnitType, BuildingType};
use super::behaviors;

pub struct AIController {
    think_timer: f32,
    resource_timer: f32,
    attack_timer: f32,
    build_timer: f32,
    last_unit_built: Option<UnitType>,
    has_barracks: bool,
    has_factory: bool,
}

impl AIController {
    pub fn new() -> Self {
        Self {
            think_timer: 0.0,
            resource_timer: 0.0,
            attack_timer: 0.0,
            build_timer: 10.0,  // Initial delay before building
            last_unit_built: None,
            has_barracks: false,
            has_factory: false,
        }
    }
    
    pub fn update(&mut self, game_state: &mut GameState) {
        // Update AI timers
        let dt = 0.016; // Assuming ~60 FPS
        self.think_timer += dt;
        self.resource_timer += dt;
        self.attack_timer += dt;
        self.build_timer += dt;
        
        // Process resource gathering (more frequently)
        if self.resource_timer >= 0.5 {
            self.resource_timer = 0.0;
            behaviors::manage_resources(game_state);
        }
        
        // Process general AI decision making
        if self.think_timer >= 1.0 {
            self.think_timer = 0.0;
            behaviors::make_decisions(game_state);
        }
        
        // Decide on attacking
        if self.attack_timer >= 30.0 {
            self.attack_timer = 0.0;
            behaviors::plan_attack(game_state);
        }
        
        // Check buildings
        self.check_buildings(game_state);
        
        // Handle building and training
        if self.build_timer >= 15.0 {
            self.build_timer = 0.0;
            self.build_or_train(game_state);
        }
    }
    
    fn check_buildings(&mut self, game_state: &GameState) {
        self.has_barracks = false;
        self.has_factory = false;
        
        for unit in &game_state.units {
            if unit.player_id == 1 && unit.unit_type == UnitType::Building {
                if let Some(building_type) = &unit.building_type {
                    if *building_type == BuildingType::Barracks {
                        self.has_barracks = true;
                    }
                    if *building_type == BuildingType::Factory {
                        self.has_factory = true;
                    }
                }
            }
        }
    }
    
    fn build_or_train(&mut self, game_state: &mut GameState) {
        let ai_player = &game_state.players[1];
        
        // Train units if we have enough resources
        if !self.has_barracks && ai_player.minerals >= 150 {
            // Need to build a barracks
            behaviors::build_structure(game_state, BuildingType::Barracks);
        } else if !self.has_factory && ai_player.minerals >= 200 && self.has_barracks {
            // Build a factory next
            behaviors::build_structure(game_state, BuildingType::Factory);
        } else {
            // Train units
            let ai_headquarters = game_state.units.iter().find(|u| 
                u.player_id == 1 && u.unit_type == UnitType::Headquarters
            );
            
            if let Some(headquarters) = ai_headquarters {
                // Count current units
                let mut worker_count = 0;
                let mut fighter_count = 0;
                let mut ranger_count = 0;
                let mut tank_count = 0;
                
                for unit in &game_state.units {
                    if unit.player_id == 1 {
                        match unit.unit_type {
                            UnitType::Worker => worker_count += 1,
                            UnitType::Fighter => fighter_count += 1,
                            UnitType::Ranger => ranger_count += 1,
                            UnitType::Tank => tank_count += 1,
                            _ => {}
                        }
                    }
                }
                
                // Decide what to train
                let hq_x = headquarters.x;
                let hq_y = headquarters.y;
                
                if worker_count < 5 && game_state.can_afford(1, &UnitType::Worker) {
                    // Train worker
                    let _id = game_state.spawn_unit(UnitType::Worker, hq_x + 30.0, hq_y, 1);
                    game_state.deduct_cost(1, &UnitType::Worker);
                    self.last_unit_built = Some(UnitType::Worker);
                } else if self.has_barracks && fighter_count < 5 && game_state.can_afford(1, &UnitType::Fighter) {
                    // Train fighter
                    let barracks = game_state.units.iter().find(|u| 
                        u.player_id == 1 && u.unit_type == UnitType::Building && 
                        u.building_type == Some(BuildingType::Barracks)
                    );
                    
                    if let Some(barracks) = barracks {
                        let _id = game_state.spawn_unit(UnitType::Fighter, barracks.x + 30.0, barracks.y, 1);
                        game_state.deduct_cost(1, &UnitType::Fighter);
                        self.last_unit_built = Some(UnitType::Fighter);
                    }
                } else if self.has_barracks && ranger_count < 4 && game_state.can_afford(1, &UnitType::Ranger) {
                    // Train ranger
                    let barracks = game_state.units.iter().find(|u| 
                        u.player_id == 1 && u.unit_type == UnitType::Building && 
                        u.building_type == Some(BuildingType::Barracks)
                    );
                    
                    if let Some(barracks) = barracks {
                        let _id = game_state.spawn_unit(UnitType::Ranger, barracks.x + 30.0, barracks.y, 1);
                        game_state.deduct_cost(1, &UnitType::Ranger);
                        self.last_unit_built = Some(UnitType::Ranger);
                    }
                } else if self.has_factory && tank_count < 2 && game_state.can_afford(1, &UnitType::Tank) {
                    // Train tank
                    let factory = game_state.units.iter().find(|u| 
                        u.player_id == 1 && u.unit_type == UnitType::Building && 
                        u.building_type == Some(BuildingType::Factory)
                    );
                    
                    if let Some(factory) = factory {
                        let _id = game_state.spawn_unit(UnitType::Tank, factory.x + 40.0, factory.y, 1);
                        game_state.deduct_cost(1, &UnitType::Tank);
                        self.last_unit_built = Some(UnitType::Tank);
                    }
                }
            }
        }
    }
}
