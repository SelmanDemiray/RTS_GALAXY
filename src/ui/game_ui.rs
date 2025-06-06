use macroquad::prelude::*;
use crate::game::state::{GameState, GameScreen};
use crate::entity::building::BuildingType;
use crate::entity::UnitType;

pub struct GameUI {
    build_menu_open: bool,
    selected_building_type: Option<BuildingType>,
    notification_messages: Vec<(String, f32)>, // (message, timer)
    resource_bar_rect: Rect,
    build_menu_rect: Rect,
}

impl GameUI {
    pub fn new() -> Self {
        Self {
            build_menu_open: false,
            selected_building_type: None,
            notification_messages: Vec::new(),
            resource_bar_rect: Rect::new(10.0, 10.0, 400.0, 50.0),
            build_menu_rect: Rect::new(10.0, 70.0, 200.0, 300.0),
        }
    }
    
    pub fn update(&mut self, dt: f32) {
        // Update notification timers
        self.notification_messages.retain_mut(|(_, timer)| {
            *timer -= dt;
            *timer > 0.0
        });
    }
    
    pub fn handle_input(&mut self, game_state: &mut GameState) {
        // Toggle build menu
        if is_key_pressed(KeyCode::B) {
            self.build_menu_open = !self.build_menu_open;
        }
        
        // Close build menu on escape
        if is_key_pressed(KeyCode::Escape) && self.build_menu_open {
            self.build_menu_open = false;
            self.selected_building_type = None;
        }
        
        // Handle build menu clicks
        if self.build_menu_open && is_mouse_button_pressed(MouseButton::Left) {
            let mouse_pos = mouse_position();
            if self.build_menu_rect.contains(Vec2::new(mouse_pos.0, mouse_pos.1)) {
                self.handle_build_menu_click(mouse_pos, game_state);
            }
        }
        
        // Handle building placement
        if let Some(building_type) = &self.selected_building_type {
            if is_mouse_button_pressed(MouseButton::Right) {
                // Cancel building placement
                self.selected_building_type = None;
            } else if is_mouse_button_pressed(MouseButton::Left) {
                self.place_building(building_type.clone(), game_state);
            }
        }
    }
    
    fn handle_build_menu_click(&mut self, mouse_pos: (f32, f32), game_state: &GameState) {
        let menu_y = self.build_menu_rect.y + 30.0; // Skip header
        let button_height = 40.0;
        let button_spacing = 45.0;
        
        let relative_y = mouse_pos.1 - menu_y;
        let button_index = (relative_y / button_spacing) as usize;
        
        let buildings = vec![
            BuildingType::Barracks,
            BuildingType::Factory,
            BuildingType::ResourceDepot,
            BuildingType::DefenseTurret,
            BuildingType::EnergyPlant,
        ];
        
        if button_index < buildings.len() {
            let building_type = &buildings[button_index];
            let cost = self.get_building_cost(building_type);
            
            let player = &game_state.players[game_state.current_player_id];
            if player.minerals >= cost {
                self.selected_building_type = Some(building_type.clone());
                self.build_menu_open = false;
            } else {
                self.add_notification("Insufficient minerals!".to_string());
            }
        }
    }
    
    fn place_building(&mut self, building_type: BuildingType, game_state: &mut GameState) {
        // Get world coordinates for building placement
        let world_x = 0.0; // This should be calculated from mouse position
        let world_y = 0.0;
        let valid_location = true; // This should check for valid placement
        
        if valid_location {
            let cost = self.get_building_cost(&building_type);
            let building_id = game_state.spawn_unit(UnitType::Building, world_x, world_y, game_state.current_player);
            
            // Set building type and construction progress
            for unit in &mut game_state.units {
                if unit.id == building_id {
                    unit.building_type = Some(building_type.clone());
                    unit.construction_progress = Some(0.0);
                    break;
                }
            }
            
            // Deduct cost
            game_state.players[game_state.current_player].minerals -= cost;
            
            self.add_notification(format!("Building {:?} placed", building_type));
        } else {
            self.add_notification("Cannot build here - too close to other structures".to_string());
        }
        
        self.selected_building_type = None;
    }

    fn get_building_cost(&self, building_type: &BuildingType) -> u32 {
        building_type.get_cost()
    }
    
    pub fn add_notification(&mut self, message: String) {
        self.notification_messages.push((message, 3.0)); // Show for 3 seconds
    }
    
    pub fn draw(&self, game_state: &GameState, _resource_manager: &ResourceManager) {
        // Draw resource bar
        self.draw_resource_bar(game_state);
        
        // Draw build menu if open
        if self.build_menu_open {
            self.draw_build_menu(game_state);
        }
        
        // Draw building placement preview
        if let Some(building_type) = &self.selected_building_type {
            self.draw_building_preview(building_type);
        }
        
        // Draw notifications
        self.draw_notifications();
        
        // Draw selection info
        if !game_state.selected_units.is_empty() {
            self.draw_selection_info(game_state);
        }
        
        // Draw zoom level indicator
        self.draw_zoom_indicator(game_state);
    }
    
    fn draw_resource_bar(&self, game_state: &GameState) {
        let player = &game_state.players[game_state.current_player_id];
        
        // Background
        draw_rectangle(
            self.resource_bar_rect.x,
            self.resource_bar_rect.y,
            self.resource_bar_rect.w,
            self.resource_bar_rect.h,
            Color::new(0.0, 0.0, 0.0, 0.7)
        );
        
        // Border
        draw_rectangle_lines(
            self.resource_bar_rect.x,
            self.resource_bar_rect.y,
            self.resource_bar_rect.w,
            self.resource_bar_rect.h,
            2.0,
            WHITE
        );
        
        // Resource text
        let minerals_text = format!("Minerals: {}", player.minerals);
        let energy_text = format!("Energy: {}", player.energy);
        
        draw_text(&minerals_text, self.resource_bar_rect.x + 10.0, self.resource_bar_rect.y + 25.0, 20.0, YELLOW);
        draw_text(&energy_text, self.resource_bar_rect.x + 200.0, self.resource_bar_rect.y + 25.0, 20.0, LIGHTBLUE);
    }
    
    fn draw_build_menu(&self, game_state: &GameState) {
        // Background
        draw_rectangle(
            self.build_menu_rect.x,
            self.build_menu_rect.y,
            self.build_menu_rect.w,
            self.build_menu_rect.h,
            Color::new(0.0, 0.0, 0.0, 0.8)
        );
        
        // Border
        draw_rectangle_lines(
            self.build_menu_rect.x,
            self.build_menu_rect.y,
            self.build_menu_rect.w,
            self.build_menu_rect.h,
            2.0,
            WHITE
        );
        
        // Title
        draw_text("BUILD MENU", self.build_menu_rect.x + 10.0, self.build_menu_rect.y + 25.0, 20.0, WHITE);
        
        // Building options
        let buildings = vec![
            ("Barracks", BuildingType::Barracks, 150),
            ("Factory", BuildingType::Factory, 200),
            ("Resource Depot", BuildingType::ResourceDepot, 100),
            ("Defense Turret", BuildingType::DefenseTurret, 120),
            ("Energy Plant", BuildingType::EnergyPlant, 180),
        ];
        
        let button_height = 35.0;
        let button_spacing = 40.0;
        let start_y = self.build_menu_rect.y + 40.0;
        
        let player = &game_state.players[game_state.current_player_id];
        
        for (i, (name, _building_type, cost)) in buildings.iter().enumerate() {
            let y = start_y + i as f32 * button_spacing;
            let can_afford = player.minerals >= *cost;
            
            let color = if can_afford { WHITE } else { GRAY };
            let bg_color = if can_afford { 
                Color::new(0.0, 0.3, 0.0, 0.5) 
            } else { 
                Color::new(0.3, 0.0, 0.0, 0.5) 
            };
            
            // Button background
            draw_rectangle(
                self.build_menu_rect.x + 5.0,
                y,
                self.build_menu_rect.w - 10.0,
                button_height,
                bg_color
            );
            
            // Text
            draw_text(name, self.build_menu_rect.x + 10.0, y + 20.0, 16.0, color);
            draw_text(&format!("{}M", cost), self.build_menu_rect.x + 150.0, y + 20.0, 14.0, YELLOW);
        }
        
        // Instructions
        draw_text("ESC to cancel", self.build_menu_rect.x + 10.0, self.build_menu_rect.y + self.build_menu_rect.h - 10.0, 12.0, LIGHTGRAY);
    }
    
    fn draw_building_preview(&self, building_type: &BuildingType) {
        let mouse_pos = mouse_position();
        
        // Draw preview circle
        let preview_color = Color::new(0.0, 1.0, 0.0, 0.3);
        draw_circle(mouse_pos.0, mouse_pos.1, 30.0, preview_color);
        
        // Draw building name
        let name = format!("{:?}", building_type);
        draw_text(&name, mouse_pos.0 - 40.0, mouse_pos.1 - 40.0, 16.0, WHITE);
        
        // Instructions
        draw_text("Left click to place, Right click to cancel", 
                 mouse_pos.0 - 100.0, mouse_pos.1 + 50.0, 12.0, LIGHTGRAY);
    }
    
    fn draw_notifications(&self) {
        let start_y = screen_height() / 2.0 - 100.0;
        
        for (i, (message, timer)) in self.notification_messages.iter().enumerate() {
            let y = start_y + i as f32 * 30.0;
            let alpha = (*timer / 3.0).min(1.0);
            let color = Color::new(1.0, 1.0, 0.0, alpha);
            
            // Background
            let text_width = measure_text(message, None, 18, 1.0).width;
            draw_rectangle(
                screen_width() / 2.0 - text_width / 2.0 - 10.0,
                y - 15.0,
                text_width + 20.0,
                25.0,
                Color::new(0.0, 0.0, 0.0, alpha * 0.7)
            );
            
            // Text
            draw_text(message, screen_width() / 2.0 - text_width / 2.0, y, 18.0, color);
        }
    }
    
    fn draw_selection_info(&self, game_state: &GameState) {
        let selected_count = game_state.selected_units.len();
        let info_text = format!("Selected: {} units", selected_count);
        
        // Find first selected unit for detailed info
        if let Some(&first_id) = game_state.selected_units.first() {
            if let Some(unit) = game_state.units.iter().find(|u| u.id == first_id) {
                let details = format!("{:?} - Health: {:.0}/{:.0}", 
                                    unit.unit_type, unit.health, unit.max_health);
                
                draw_text(&info_text, 10.0, 120.0, 18.0, WHITE);
                draw_text(&details, 10.0, 140.0, 16.0, LIGHTGRAY);
                
                // Show resource carrying for workers
                if unit.unit_type == UnitType::Worker {
                    if let Some(resources) = unit.current_resources {
                        let resource_text = format!("Carrying: {} resources", resources);
                        draw_text(&resource_text, 10.0, 160.0, 14.0, GOLD);
                    }
                }
            }
        }
    }
    
    fn draw_zoom_indicator(&self, game_state: &GameState) {
        let zoom_text = game_state.zoom_system.get_zoom_label();
        let zoom_desc = game_state.zoom_system.get_zoom_description();
        
        let indicator_x = screen_width() - 300.0;
        let indicator_y = screen_height() - 80.0;
        
        // Background
        draw_rectangle(indicator_x - 10.0, indicator_y - 25.0, 290.0, 60.0, 
                      Color::new(0.0, 0.0, 0.0, 0.6));
        
        // Text
        draw_text(&zoom_text, indicator_x, indicator_y, 16.0, WHITE);
        draw_text(&zoom_desc, indicator_x, indicator_y + 20.0, 14.0, LIGHTGRAY);
        
        // Zoom controls hint
        draw_text("Mouse wheel or +/- to zoom", indicator_x, indicator_y + 40.0, 12.0, GRAY);
    }
}
