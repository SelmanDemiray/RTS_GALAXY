# Game Sound Effects

This folder contains all sound effect files for the RTS game. All sound files should be in `.ogg` or `.wav` format for best compatibility.

## Required Sound Files

| Filename              | Description                   | Used in Code                           | In-game Usage                          |
|-----------------------|-------------------------------|----------------------------------------|----------------------------------------|
| ui_click.ogg          | UI button click sound         | Used in MenuSystem.draw_button()       | Plays when clicking any menu button    |
| ui_hover.ogg          | UI hover sound                | Used in MenuSystem                     | Plays when hovering over menu items    |
| unit_select.ogg       | Unit selection sound          | Called when GameState.select_unit_at() | Plays when selecting a unit            |
| unit_move.ogg         | Unit movement order sound     | Called in GameState.move_selected_unit() | Plays when ordering units to move    |
| unit_attack.ogg       | Unit attack sound             | Played during combat in update_units() | Plays when a unit attacks              |
| unit_death.ogg        | Unit death sound              | Played when unit health reaches 0      | Plays when a unit is destroyed         |
| resource_gather.ogg   | Resource gathering sound      | Played during resource collection      | Plays when worker collects resources   |
| building_construct.ogg| Building construction sound   | Played when constructing a building    | Plays when a building is placed        |
| alert.ogg             | Alert/notification sound      | Used for game notifications            | Plays for important alerts to player   |
| error.ogg             | Error sound                   | Used for invalid actions               | Plays when player attempts invalid action |

## Sound Categories

- **UI Sounds**: Used for menu interactions and game interface
- **Unit Sounds**: Used for unit actions and feedback
- **Building Sounds**: Used for construction and building activities
- **Notification Sounds**: Used for game state notifications

## Implementation Notes

- Sound volume is controlled via the `game_state.sound_volume` setting
- Each sound file should be short and optimized for quick response
- All sound files should be properly licensed or created specifically for this game
