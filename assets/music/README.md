# Game Music Files

This folder contains background music files for the RTS game. All music files should be in `.ogg` or `.wav` format.

## Required Music Files

| Filename           | Description                  | Used in Code                     | In-game Usage                       |
|--------------------|------------------------------|----------------------------------|-------------------------------------|
| main_theme.ogg     | Main menu theme music        | Loaded in ResourceManager.load_resources() | Plays on main menu screen          |
| battle_theme.ogg   | Battle/gameplay music        | Played during GameScreen::Playing | Background music during gameplay     |
| victory.ogg        | Victory fanfare              | Played when player wins a match   | Plays on victory screen             |
| defeat.ogg         | Defeat theme                 | Played when player loses a match  | Plays on defeat screen              |
| ambient_theme.ogg  | Ambient background music     | Alternates with battle_theme      | Plays during low-activity periods   |

## Implementation Notes

- Music volume is controlled via the `game_state.music_volume` setting 
- Files are loaded during the initialization phase in the ResourceManager
- All music files should be properly licensed or created specifically for this game
