use macroquad::audio::{play_sound, PlaySoundParams};
use crate::resources::manager::ResourceManager;
use crate::game::state::GameState;

pub struct AudioManager {
    current_music: Option<String>,
    music_volume: f32,
    sound_volume: f32,
}

impl AudioManager {
    pub fn new() -> Self {
        Self {
            current_music: None,
            music_volume: 0.7,
            sound_volume: 0.8,
        }
    }
    
    pub fn play_ui_click(&self, resource_manager: &ResourceManager, game_state: &GameState) {
        if let Some(sound) = resource_manager.get_sound("button_click") {
            let volume = if game_state.sound_muted { 0.0 } else { game_state.sound_volume };
            play_sound(
                *sound,
                PlaySoundParams {
                    looped: false,
                    volume,
                }
            );
        }
    }
    
    pub fn play_selection_sound(&self, resource_manager: &ResourceManager, game_state: &GameState) {
        if let Some(sound) = resource_manager.get_sound("unit_select") {
            let volume = if game_state.sound_muted { 0.0 } else { game_state.sound_volume };
            play_sound(
                *sound,
                PlaySoundParams {
                    looped: false,
                    volume,
                }
            );
        }
    }
    
    pub fn play_build_sound(&self, resource_manager: &ResourceManager, game_state: &GameState) {
        if let Some(sound) = resource_manager.get_sound("building_place") {
            let volume = if game_state.sound_muted { 0.0 } else { game_state.sound_volume };
            play_sound(
                *sound,
                PlaySoundParams {
                    looped: false,
                    volume,
                }
            );
        }
    }
    
    pub fn play_music(&mut self, music_name: &str, resource_manager: &ResourceManager, game_state: &GameState) {
        // Stop current music if different
        if let Some(current) = &self.current_music {
            if current != music_name {
                // In macroquad, we can't easily stop specific sounds, so we just track what's playing
                self.current_music = None;
            }
        }
        
        if let Some(music) = resource_manager.get_music(music_name) {
            let volume = if game_state.music_muted { 0.0 } else { game_state.music_volume };
            play_sound(
                *music,
                PlaySoundParams {
                    looped: true,
                    volume,
                }
            );
            self.current_music = Some(music_name.to_string());
        }
    }
    
    pub fn update_volumes(&mut self, _resource_manager: &ResourceManager, game_state: &GameState) {
        // Update volumes if they've changed
        if self.music_volume != game_state.music_volume || self.sound_volume != game_state.sound_volume {
            self.music_volume = game_state.music_volume;
            self.sound_volume = game_state.sound_volume;
            
            // Note: macroquad doesn't provide easy access to change volume of already playing sounds
            // In a real implementation, you'd want to use a more sophisticated audio system
        }
    }
    
    pub fn get_current_music(&self) -> Option<&str> {
        self.current_music.as_deref()
    }
}
