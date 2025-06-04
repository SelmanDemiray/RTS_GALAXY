use macroquad::prelude::*;
use macroquad::audio::load_sound;

mod game;
mod entity;
mod ui;
mod resources;
mod audio;
mod network;
mod ai;

use game::{GameState, GameScreen};
use resources::ResourceManager;
use audio::AudioManager;
use ui::menu::MenuSystem;
use ai::AIController;
use network::NetworkClient;

// Game configuration constants
const TARGET_FPS: f32 = 60.0;
const FRAME_TIME: f32 = 1.0 / TARGET_FPS;

#[derive(Debug)]
enum GameError {
    ResourceLoading(String),
    NetworkError(String),
    AudioError(String),
}

impl std::fmt::Display for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameError::ResourceLoading(msg) => write!(f, "Resource loading error: {}", msg),
            GameError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            GameError::AudioError(msg) => write!(f, "Audio error: {}", msg),
        }
    }
}

impl std::error::Error for GameError {}

struct GameApplication {
    game_state: GameState,
    resource_manager: ResourceManager,
    audio_manager: AudioManager,
    menu_system: MenuSystem,
    ai_controller: AIController,
    network_client: NetworkClient,
    last_frame_time: f64,
    frame_accumulator: f32,
    is_initialized: bool,
    loading_progress: f32,
}

impl GameApplication {
    fn new() -> Self {
        Self {
            game_state: GameState::new(),
            resource_manager: ResourceManager::new(),
            audio_manager: AudioManager::new(),
            menu_system: MenuSystem::new(),
            ai_controller: AIController::new(),
            network_client: NetworkClient::new(),
            last_frame_time: get_time(),
            frame_accumulator: 0.0,
            is_initialized: false,
            loading_progress: 0.0,
        }
    }

    async fn initialize(&mut self) -> Result<(), GameError> {
        println!("Initializing RTS Galaxy...");
        
        // Load resources asynchronously
        self.resource_manager.load_resources().await;
        
        // Initialize audio system
        self.audio_manager = AudioManager::new();
        
        // Set initial game state
        self.game_state.current_screen = GameScreen::MainMenu;
        
        self.is_initialized = true;
        println!("Initialization complete!");
        Ok(())
    }

    fn update(&mut self, dt: f32) -> Result<(), GameError> {
        // Update loading progress if still loading
        if !self.resource_manager.is_loading_complete() {
            self.loading_progress = self.resource_manager.get_loading_progress();
            return Ok(());
        }

        // Update audio volumes
        self.audio_manager.update_volumes(&self.resource_manager, &self.game_state);

        // Handle different game screens
        match self.game_state.current_screen {
            GameScreen::MainMenu => {
                self.update_main_menu(dt)?;
            }
            GameScreen::Playing => {
                self.update_gameplay(dt)?;
            }
            GameScreen::Settings => {
                self.update_settings(dt)?;
            }
            GameScreen::Credits => {
                self.update_credits(dt)?;
            }
        }

        // Handle input for screen transitions and global controls
        self.handle_global_input();

        Ok(())
    }

    fn update_main_menu(&mut self, _dt: f32) -> Result<(), GameError> {
        // Handle menu input and transitions
        if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
            self.game_state.current_screen = GameScreen::Playing;
            self.audio_manager.play_ui_click(&self.resource_manager, &self.game_state);
        }
        Ok(())
    }

    fn update_gameplay(&mut self, dt: f32) -> Result<(), GameError> {
        // Update game state
        self.game_state.update();
        
        // Update AI
        self.ai_controller.update(&mut self.game_state);
        
        // Handle input
        self.handle_gameplay_input();
        
        // Update network if connected
        if self.network_client.is_connected() {
            if let Some(message) = self.network_client.receive() {
                self.handle_network_message(message);
            }
        }

        Ok(())
    }

    fn update_settings(&mut self, _dt: f32) -> Result<(), GameError> {
        // Handle settings input
        if is_key_pressed(KeyCode::Escape) {
            self.game_state.current_screen = GameScreen::MainMenu;
            self.audio_manager.play_ui_click(&self.resource_manager, &self.game_state);
        }
        Ok(())
    }

    fn update_credits(&mut self, _dt: f32) -> Result<(), GameError> {
        // Handle credits input
        if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Enter) {
            self.game_state.current_screen = GameScreen::MainMenu;
            self.audio_manager.play_ui_click(&self.resource_manager, &self.game_state);
        }
        Ok(())
    }

    fn handle_global_input(&mut self) {
        // Global hotkeys
        if is_key_pressed(KeyCode::F11) {
            // Toggle fullscreen (macroquad doesn't support this directly)
            println!("Fullscreen toggle requested");
        }
        
        if is_key_pressed(KeyCode::F1) {
            // Toggle debug info
            println!("Debug info toggle");
        }

        // Volume controls
        if is_key_down(KeyCode::LeftControl) {
            if is_key_pressed(KeyCode::Up) {
                self.game_state.sound_volume = (self.game_state.sound_volume + 0.1).min(1.0);
            }
            if is_key_pressed(KeyCode::Down) {
                self.game_state.sound_volume = (self.game_state.sound_volume - 0.1).max(0.0);
            }
        }
    }

    fn handle_gameplay_input(&mut self) {
        let (mouse_x, mouse_y) = mouse_position();
        let world_x = mouse_x + self.game_state.camera_x - screen_width() / 2.0;
        let world_y = mouse_y + self.game_state.camera_y - screen_height() / 2.0;

        // Left click - selection
        if is_mouse_button_pressed(MouseButton::Left) {
            // Check if clicking on minimap
            if !self.game_state.minimap_rect.contains(Vec2::new(mouse_x, mouse_y)) {
                self.handle_unit_selection(world_x, world_y);
            } else {
                self.handle_minimap_click(mouse_x, mouse_y);
            }
        }

        // Right click - move/attack commands
        if is_mouse_button_pressed(MouseButton::Right) {
            self.handle_unit_command(world_x, world_y);
        }

        // Box selection
        self.handle_box_selection(world_x, world_y);

        // Building hotkeys
        if is_key_pressed(KeyCode::B) {
            println!("Build mode activated");
        }

        // Unit training hotkeys
        if is_key_pressed(KeyCode::Key1) {
            self.try_train_unit(entity::UnitType::Worker);
        }
        if is_key_pressed(KeyCode::Key2) {
            self.try_train_unit(entity::UnitType::Fighter);
        }
        if is_key_pressed(KeyCode::Key3) {
            self.try_train_unit(entity::UnitType::Ranger);
        }
        if is_key_pressed(KeyCode::Key4) {
            self.try_train_unit(entity::UnitType::Tank);
        }
    }

    fn handle_unit_selection(&mut self, world_x: f32, world_y: f32) {
        // Find unit at position
        let mut selected_unit = None;
        for unit in &self.game_state.units {
            if unit.player_id == self.game_state.current_player_id {
                let distance = ((unit.x - world_x).powi(2) + (unit.y - world_y).powi(2)).sqrt();
                let selection_radius = match unit.unit_type {
                    entity::UnitType::Worker => 20.0,
                    entity::UnitType::Fighter => 25.0,
                    entity::UnitType::Ranger => 22.0,
                    entity::UnitType::Tank => 35.0,
                    entity::UnitType::Building => 50.0,
                    entity::UnitType::Headquarters => 60.0,
                };
                
                if distance <= selection_radius {
                    selected_unit = Some(unit.id);
                    break;
                }
            }
        }

        // Update selection
        if let Some(unit_id) = selected_unit {
            if is_key_down(KeyCode::LeftControl) {
                // Add to selection
                if !self.game_state.selected_units.contains(&unit_id) {
                    self.game_state.selected_units.push(unit_id);
                }
            } else {
                // Replace selection
                self.game_state.selected_units.clear();
                self.game_state.selected_units.push(unit_id);
            }
            self.audio_manager.play_selection_sound(&self.resource_manager, &self.game_state);
        } else if !is_key_down(KeyCode::LeftControl) {
            // Clear selection if not holding ctrl
            self.game_state.selected_units.clear();
        }
    }

    fn handle_unit_command(&mut self, world_x: f32, world_y: f32) {
        if !self.game_state.selected_units.is_empty() {
            // Move selected units to target location
            for &unit_id in &self.game_state.selected_units {
                for unit in &mut self.game_state.units {
                    if unit.id == unit_id {
                        unit.target_x = Some(world_x);
                        unit.target_y = Some(world_y);
                        break;
                    }
                }
            }
            // Play movement sound (implement in audio manager)
        }
    }

    fn handle_box_selection(&mut self, world_x: f32, world_y: f32) {
        if is_mouse_button_pressed(MouseButton::Left) {
            self.game_state.selection_start = Some((world_x, world_y));
        }
        
        if is_mouse_button_down(MouseButton::Left) && self.game_state.selection_start.is_some() {
            self.game_state.selection_end = Some((world_x, world_y));
        }
        
        if is_mouse_button_released(MouseButton::Left) {
            if let (Some(start), Some(end)) = (self.game_state.selection_start, self.game_state.selection_end) {
                // Perform box selection
                let min_x = start.0.min(end.0);
                let max_x = start.0.max(end.0);
                let min_y = start.1.min(end.1);
                let max_y = start.1.max(end.1);
                
                if !is_key_down(KeyCode::LeftControl) {
                    self.game_state.selected_units.clear();
                }
                
                for unit in &self.game_state.units {
                    if unit.player_id == self.game_state.current_player_id &&
                       unit.x >= min_x && unit.x <= max_x &&
                       unit.y >= min_y && unit.y <= max_y {
                        if !self.game_state.selected_units.contains(&unit.id) {
                            self.game_state.selected_units.push(unit.id);
                        }
                    }
                }
                
                if !self.game_state.selected_units.is_empty() {
                    self.audio_manager.play_selection_sound(&self.resource_manager, &self.game_state);
                }
            }
            
            self.game_state.selection_start = None;
            self.game_state.selection_end = None;
        }
    }

    fn handle_minimap_click(&mut self, mouse_x: f32, mouse_y: f32) {
        let minimap = &self.game_state.minimap_rect;
        let relative_x = (mouse_x - minimap.x) / minimap.w;
        let relative_y = (mouse_y - minimap.y) / minimap.h;
        
        self.game_state.camera_x = relative_x * self.game_state.map_width;
        self.game_state.camera_y = relative_y * self.game_state.map_height;
    }

    fn try_train_unit(&mut self, unit_type: entity::UnitType) {
        let player_id = self.game_state.current_player_id;
        
        if self.game_state.can_afford(player_id, &unit_type) {
            // Find appropriate building to train from
            let training_building = self.game_state.units.iter().find(|unit| {
                unit.player_id == player_id && 
                match (&unit_type, &unit.unit_type) {
                    (entity::UnitType::Worker, entity::UnitType::Headquarters) => true,
                    (entity::UnitType::Fighter | entity::UnitType::Ranger, entity::UnitType::Building) => {
                        unit.building_type == Some(entity::BuildingType::Barracks)
                    },
                    (entity::UnitType::Tank, entity::UnitType::Building) => {
                        unit.building_type == Some(entity::BuildingType::Factory)
                    },
                    _ => false,
                }
            });

            if let Some(building) = training_building {
                let spawn_x = building.x + 40.0;
                let spawn_y = building.y + 40.0;
                
                self.game_state.spawn_unit(unit_type, spawn_x, spawn_y, player_id);
                self.game_state.deduct_cost(player_id, &unit_type);
                
                // Play training sound (implement in audio manager)
                println!("Training {:?}", unit_type);
            } else {
                println!("No suitable building found for training {:?}", unit_type);
            }
        } else {
            println!("Insufficient resources for {:?}", unit_type);
        }
    }

    fn handle_network_message(&mut self, _message: network::NetworkMessage) {
        // Handle incoming network messages
        // This would update game state based on server updates
        println!("Received network message");
    }

    fn render(&self) -> Result<(), GameError> {
        clear_background(BLACK);

        if !self.is_initialized || !self.resource_manager.is_loading_complete() {
            self.render_loading_screen();
            return Ok(());
        }

        match self.game_state.current_screen {
            GameScreen::MainMenu => {
                self.render_main_menu();
            }
            GameScreen::Playing => {
                game::rendering::draw_game(&self.game_state, &self.resource_manager);
                self.render_ui_overlay();
            }
            GameScreen::Settings => {
                self.render_settings();
            }
            GameScreen::Credits => {
                ui::menu::credits::draw_credits(
                    &mut self.menu_system.clone(),
                    &mut self.game_state.clone(),
                    &self.audio_manager,
                    &self.resource_manager
                );
            }
        }

        // Render debug info if enabled
        self.render_debug_info();

        Ok(())
    }

    fn render_loading_screen(&self) {
        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;
        
        draw_text("Loading RTS Galaxy...", center_x - 100.0, center_y - 50.0, 32.0, WHITE);
        
        // Progress bar
        let bar_width = 300.0;
        let bar_height = 20.0;
        let progress_width = bar_width * self.loading_progress;
        
        draw_rectangle(center_x - bar_width/2.0, center_y, bar_width, bar_height, DARKGRAY);
        draw_rectangle(center_x - bar_width/2.0, center_y, progress_width, bar_height, GREEN);
        
        let progress_text = format!("{}%", (self.loading_progress * 100.0) as i32);
        draw_text(&progress_text, center_x - 20.0, center_y + 40.0, 20.0, WHITE);
    }

    fn render_main_menu(&self) {
        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;
        
        // Title
        draw_text("RTS GALAXY", center_x - 120.0, center_y - 150.0, 48.0, WHITE);
        
        // Subtitle
        draw_text("Advanced Real-Time Strategy", center_x - 140.0, center_y - 100.0, 24.0, LIGHTGRAY);
        
        // Menu options
        draw_text("Press ENTER to Start", center_x - 100.0, center_y, 24.0, WHITE);
        draw_text("Press ESC for Settings", center_x - 105.0, center_y + 40.0, 20.0, LIGHTGRAY);
        draw_text("Press C for Credits", center_x - 85.0, center_y + 80.0, 20.0, LIGHTGRAY);
        
        // Version info
        draw_text("v0.1.0", 10.0, screen_height() - 20.0, 16.0, GRAY);
    }

    fn render_settings(&self) {
        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;
        
        draw_text("SETTINGS", center_x - 60.0, center_y - 100.0, 32.0, WHITE);
        
        // Sound volume
        let sound_text = format!("Sound Volume: {}%", (self.game_state.sound_volume * 100.0) as i32);
        draw_text(&sound_text, center_x - 100.0, center_y - 40.0, 20.0, WHITE);
        
        // Music volume
        let music_text = format!("Music Volume: {}%", (self.game_state.music_volume * 100.0) as i32);
        draw_text(&music_text, center_x - 100.0, center_y, 20.0, WHITE);
        
        // Controls
        draw_text("Use Ctrl+Up/Down to adjust volume", center_x - 140.0, center_y + 60.0, 16.0, LIGHTGRAY);
        draw_text("Press ESC to return", center_x - 80.0, center_y + 100.0, 16.0, LIGHTGRAY);
    }

    fn render_ui_overlay(&self) {
        // Zoom level indicator
        let zoom_text = self.game_state.zoom_system.get_zoom_label();
        draw_text(&zoom_text, 10.0, screen_height() - 80.0, 16.0, WHITE);
        
        let zoom_desc = self.game_state.zoom_system.get_zoom_description();
        draw_text(&zoom_desc, 10.0, screen_height() - 60.0, 14.0, LIGHTGRAY);
        
        // Controls help
        draw_text("Controls: WASD-Move, Mouse-Select, +/- Zoom, H-Home", 10.0, screen_height() - 40.0, 12.0, GRAY);
        draw_text("Build: 1-Worker, 2-Fighter, 3-Ranger, 4-Tank", 10.0, screen_height() - 20.0, 12.0, GRAY);
    }

    fn render_debug_info(&self) {
        // FPS and performance info
        let fps = get_fps();
        draw_text(&format!("FPS: {}", fps), screen_width() - 80.0, 30.0, 16.0, if fps < 30 { RED } else { GREEN });
        
        // Unit count
        let unit_count = self.game_state.units.len();
        draw_text(&format!("Units: {}", unit_count), screen_width() - 80.0, 50.0, 16.0, WHITE);
        
        // Selected units
        if !self.game_state.selected_units.is_empty() {
            draw_text(&format!("Selected: {}", self.game_state.selected_units.len()), 
                     screen_width() - 100.0, 70.0, 16.0, YELLOW);
        }
    }

    fn should_quit(&self) -> bool {
        self.game_state.should_quit || is_key_pressed(KeyCode::Escape) && 
        self.game_state.current_screen == GameScreen::MainMenu
    }
}

#[macroquad::main("RTS Galaxy")]
async fn main() -> Result<(), GameError> {
    // Initialize logging
    env_logger::init();
    
    println!("Starting RTS Galaxy...");
    
    // Create and initialize game application
    let mut app = GameApplication::new();
    app.initialize().await?;
    
    // Main game loop
    loop {
        let current_time = get_time();
        let dt = (current_time - app.last_frame_time) as f32;
        app.last_frame_time = current_time;
        
        // Cap frame time to prevent large jumps
        let clamped_dt = dt.min(0.05);
        
        // Update game
        if let Err(e) = app.update(clamped_dt) {
            eprintln!("Update error: {}", e);
            break;
        }
        
        // Render game
        if let Err(e) = app.render() {
            eprintln!("Render error: {}", e);
            break;
        }
        
        // Check for quit condition
        if app.should_quit() {
            break;
        }
        
        next_frame().await;
    }
    
    println!("RTS Galaxy shutting down...");
    Ok(())
}
