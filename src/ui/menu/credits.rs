use macroquad::prelude::*;
use crate::game::{GameState, GameScreen};
use crate::audio::AudioManager;
use crate::resources::ResourceManager;
use super::system::MenuSystem;

pub struct CreditsScreen {
    scroll_offset: f32,
    scroll_speed: f32,
    credits_data: Vec<CreditSection>,
}

#[derive(Clone)]
pub struct CreditSection {
    pub title: String,
    pub entries: Vec<String>,
    pub spacing: f32,
}

impl CreditsScreen {
    pub fn new() -> Self {
        let credits_data = vec![
            CreditSection {
                title: "RTS GALAXY".to_string(),
                entries: vec![
                    "Advanced Real-Time Strategy Game".to_string(),
                    "Built with Rust and macroquad".to_string(),
                ],
                spacing: 80.0,
            },
            CreditSection {
                title: "DEVELOPMENT TEAM".to_string(),
                entries: vec![
                    "Lead Developer: Galaxy Studios".to_string(),
                    "Game Design: Strategy Team".to_string(),
                    "Programming: Rust Developers".to_string(),
                    "3D Art: Asset Creation Team".to_string(),
                    "Audio Design: Sound Engineers".to_string(),
                ],
                spacing: 60.0,
            },
            CreditSection {
                title: "SPECIAL FEATURES".to_string(),
                entries: vec![
                    "50-Level Zoom System".to_string(),
                    "From 1 meter to 93 billion light-years".to_string(),
                    "Mathematical cosmic scale modeling".to_string(),
                    "Logarithmic zoom factor F ≈ 3.55".to_string(),
                    "Observable universe representation".to_string(),
                ],
                spacing: 50.0,
            },
            CreditSection {
                title: "TECHNOLOGY STACK".to_string(),
                entries: vec![
                    "Rust Programming Language".to_string(),
                    "macroquad Game Engine".to_string(),
                    "Serde Serialization".to_string(),
                    "Tokio Async Runtime".to_string(),
                    "nalgebra Mathematics".to_string(),
                    "glTF 3D Model Format".to_string(),
                ],
                spacing: 50.0,
            },
            CreditSection {
                title: "GAME FEATURES".to_string(),
                entries: vec![
                    "Real-Time Strategy Gameplay".to_string(),
                    "Multi-scale Unit Management".to_string(),
                    "Advanced AI Opponents".to_string(),
                    "Networked Multiplayer".to_string(),
                    "3D Animation System".to_string(),
                    "Dynamic Audio Management".to_string(),
                    "Spatial Partitioning Network".to_string(),
                ],
                spacing: 50.0,
            },
            CreditSection {
                title: "COSMIC INSPIRATION".to_string(),
                entries: vec![
                    "Observable Universe: ~93 billion light-years".to_string(),
                    "Galaxy clusters and superclusters".to_string(),
                    "Cosmic web structure".to_string(),
                    "Powers of Ten documentary".to_string(),
                    "Scale of the Universe visualizations".to_string(),
                ],
                spacing: 50.0,
            },
            CreditSection {
                title: "ACKNOWLEDGMENTS".to_string(),
                entries: vec![
                    "Rust gamedev community".to_string(),
                    "macroquad contributors".to_string(),
                    "Classic RTS game inspiration:".to_string(),
                    "  • StarCraft series".to_string(),
                    "  • Command & Conquer".to_string(),
                    "  • Age of Empires".to_string(),
                    "  • Total Annihilation".to_string(),
                ],
                spacing: 50.0,
            },
            CreditSection {
                title: "OPEN SOURCE".to_string(),
                entries: vec![
                    "This project is open source".to_string(),
                    "Licensed under MIT License".to_string(),
                    "Contributions welcome".to_string(),
                    "github.com/rts-galaxy".to_string(),
                ],
                spacing: 60.0,
            },
            CreditSection {
                title: "THANK YOU".to_string(),
                entries: vec![
                    "Thank you for playing RTS Galaxy!".to_string(),
                    "".to_string(),
                    "Explore the universe,".to_string(),
                    "command your forces,".to_string(),
                    "and conquer the galaxy!".to_string(),
                ],
                spacing: 80.0,
            },
        ];
        
        Self {
            scroll_offset: screen_height(),
            scroll_speed: 50.0,
            credits_data,
        }
    }
    
    pub fn update(&mut self, dt: f32) {
        self.scroll_offset -= self.scroll_speed * dt;
        
        // Reset scroll when credits finish
        let total_height = self.calculate_total_height();
        if self.scroll_offset < -total_height {
            self.scroll_offset = screen_height();
        }
    }
    
    fn calculate_total_height(&self) -> f32 {
        let mut height = 0.0;
        for section in &self.credits_data {
            height += section.spacing;
            height += 30.0; // Title height
            height += section.entries.len() as f32 * 25.0; // Entry heights
        }
        height
    }
    
    pub fn handle_input(&self, game_state: &mut GameState) {
        if is_key_pressed(KeyCode::Escape) || 
           is_key_pressed(KeyCode::Enter) || 
           is_key_pressed(KeyCode::Space) ||
           is_mouse_button_pressed(MouseButton::Left) {
            game_state.current_screen = GameScreen::MainMenu;
        }
    }
    
    pub fn draw(&self) {
        // Background with stars
        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), BLACK);
        
        // Draw animated stars
        let time = get_time() as f32;
        for i in 0..100 {
            let star_seed = i as f32 * 1234.567;
            let x = ((time * 5.0 + star_seed).sin() * 0.5 + 0.5) * screen_width();
            let y = ((time * 3.0 + star_seed * 2.0).cos() * 0.5 + 0.5) * screen_height();
            let alpha = ((time * 2.0 + star_seed).sin() * 0.5 + 0.5) * 0.6;
            let size = ((star_seed.sin() * 0.5 + 0.5) * 2.0 + 0.5);
            
            draw_circle(x, y, size, Color::new(1.0, 1.0, 1.0, alpha));
        }
        
        // Draw credits text
        let center_x = screen_width() / 2.0;
        let mut current_y = self.scroll_offset;
        
        for section in &self.credits_data {
            current_y += section.spacing;
            
            // Only draw if on screen
            if current_y > -100.0 && current_y < screen_height() + 100.0 {
                // Draw section title
                let title_size = 32.0;
                let title_width = measure_text(&section.title, None, title_size as u16, 1.0).width;
                
                // Title glow effect
                draw_text(&section.title, center_x - title_width / 2.0 + 2.0, current_y + 2.0, 
                         title_size, Color::new(0.0, 0.5, 1.0, 0.3));
                draw_text(&section.title, center_x - title_width / 2.0, current_y, 
                         title_size, GOLD);
                
                current_y += 40.0;
                
                // Draw section entries
                for entry in &section.entries {
                    let entry_size = 20.0;
                    let entry_width = measure_text(entry, None, entry_size as u16, 1.0).width;
                    
                    // Calculate fade effect based on position
                    let fade_alpha = if current_y < 100.0 {
                        (current_y / 100.0).max(0.0)
                    } else if current_y > screen_height() - 100.0 {
                        ((screen_height() - current_y) / 100.0).max(0.0)
                    } else {
                        1.0
                    };
                    
                    let color = Color::new(1.0, 1.0, 1.0, fade_alpha);
                    draw_text(entry, center_x - entry_width / 2.0, current_y, entry_size, color);
                    
                    current_y += 25.0;
                }
            } else {
                // Skip drawing but still update position
                current_y += 40.0 + section.entries.len() as f32 * 25.0;
            }
        }
        
        // Draw instructions at bottom
        let instruction_text = "Press any key to return to main menu";
        let instruction_size = 18.0;
        let instruction_width = measure_text(instruction_text, None, instruction_size as u16, 1.0).width;
        
        // Pulsing effect
        let pulse = (time * 3.0).sin() * 0.3 + 0.7;
        let instruction_color = Color::new(1.0, 1.0, 1.0, pulse);
        
        draw_text(instruction_text, 
                 center_x - instruction_width / 2.0, 
                 screen_height() - 30.0, 
                 instruction_size, 
                 instruction_color);
        
        // Draw galaxy logo overlay (simple representation)
        let logo_center_x = screen_width() - 100.0;
        let logo_center_y = 100.0;
        let logo_time = time * 0.5;
        
        // Draw spiral galaxy effect
        for i in 0..50 {
            let angle = i as f32 * 0.3 + logo_time;
            let radius = (i as f32 * 1.5) + (logo_time * 10.0).sin() * 5.0;
            let x = logo_center_x + angle.cos() * radius;
            let y = logo_center_y + angle.sin() * radius;
            let alpha = (1.0 - i as f32 / 50.0) * 0.4;
            
            draw_circle(x, y, 2.0, Color::new(0.0, 0.8, 1.0, alpha));
        }
    }
}

pub fn draw_credits(
    _menu_system: &mut MenuSystem,
    game_state: &mut GameState,
    _audio_manager: &AudioManager,
    _resource_manager: &ResourceManager
) {
    let mut credits_screen = CreditsScreen::new();
    let dt = get_frame_time();
    
    credits_screen.update(dt);
    credits_screen.handle_input(game_state);
    credits_screen.draw();
}
