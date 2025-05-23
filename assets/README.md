# Game Assets

This directory contains all media assets for the RTS game, organized by type.

## Directory Structure

- **textures/**
  - **units/**: Unit sprites and animations
  - **buildings/**: Building sprites
  - **resources/**: Resource node graphics
  - **terrain/**: Background textures and terrain elements
  - **ui/**: Interface elements and icons
  - **effects/**: Visual effects (explosions, magic, etc.)
- **sounds/**: Sound effect files (.ogg or .wav format)
- **music/**: Background music files (.ogg format)
- **fonts/**: Game fonts (.ttf format)
- **animations/**: Special animation sequences

## Asset List

### Units
| Filename | Type | Description | Size |
|----------|------|-------------|------|
| worker.png | PNG | Worker unit sprite | 128×128 |
| worker_selected.png | PNG | Selected worker highlight | 128×128 |
| fighter.png | PNG | Fighter unit sprite | 128×128 |
| ranger.png | PNG | Ranger unit sprite | 128×128 |
| tank.png | PNG | Tank unit sprite | 128×128 |
| unit_shadow.png | PNG | Shadow for all units | 64×64 |

### Buildings
| Filename | Type | Description | Size |
|----------|------|-------------|------|
| headquarters.png | PNG | Main base building | 256×256 |
| barracks.png | PNG | Unit training building | 192×192 |
| factory.png | PNG | Advanced unit building | 192×192 |
| resource_depot.png | PNG | Resource storage | 192×192 |
| defense_turret.png | PNG | Defensive structure | 128×128 |
| building_construction.png | PNG | Building under construction | 192×192 |

### Resources
| Filename | Type | Description | Size |
|----------|------|-------------|------|
| minerals.png | PNG | Mineral resource node | 128×128 |
| minerals_depleted.png | PNG | Depleted mineral node | 128×128 |
| energy.png | PNG | Energy resource node | 128×128 |
| energy_depleted.png | PNG | Depleted energy node | 128×128 |

### Terrain
| Filename | Type | Description | Size |
|----------|------|-------------|------|
| grass_tile.png | PNG | Basic grass terrain | 128×128 |
| dirt_tile.png | PNG | Dirt terrain | 128×128 |
| sand_tile.png | PNG | Sand terrain | 128×128 |
| rock_tile.png | PNG | Rocky terrain | 128×128 |
| water_tile.png | PNG | Water terrain | 128×128 |
| obstacle_rock.png | PNG | Rock obstacle | 128×128 |
| obstacle_tree.png | PNG | Tree obstacle | 128×128 |

### UI Elements
| Filename | Type | Description | Size |
|----------|------|-------------|------|
| button_normal.png | PNG | Standard button | 196×64 |
| button_hover.png | PNG | Button hover state | 196×64 |
| button_pressed.png | PNG | Button pressed state | 196×64 |
| panel_background.png | PNG | UI panel background | 512×512 |
| minimap_frame.png | PNG | Minimap border | 256×256 |
| selection_circle.png | PNG | Unit selection indicator | 128×128 |
| health_bar_frame.png | PNG | Health bar container | 64×16 |
| health_bar_fill.png | PNG | Health bar fill | 60×12 |
| resource_icon_minerals.png | PNG | Minerals UI icon | 32×32 |
| resource_icon_energy.png | PNG | Energy UI icon | 32×32 |

### Effects
| Filename | Type | Description | Size |
|----------|------|-------------|------|
| explosion.gif | GIF | Explosion animation | 256×256 |
| laser_beam.png | PNG | Ranger attack effect | 64×512 |
| impact_hit.png | PNG | Attack impact effect | 64×64 |
| dust_cloud.png | PNG | Movement dust effect | 64×64 |
| construction_effect.gif | GIF | Building construction | 192×192 |

### Sounds
| Filename | Type | Description | Duration |
|----------|------|-------------|----------|
| unit_select.wav | WAV | Unit selection sound | 0.5s |
| unit_acknowledge.wav | WAV | Unit acknowledgment | 1.0s |
| unit_move.wav | WAV | Unit movement sound | 0.8s |
| unit_attack.wav | WAV | Unit attack sound | 0.7s |
| building_place.wav | WAV | Building placement | 1.2s |
| resource_gather.wav | WAV | Resource collection | 0.6s |
| button_click.wav | WAV | UI button click | 0.3s |
| game_start.wav | WAV | Game start sound | 2.5s |
| victory.wav | WAV | Victory fanfare | 5.0s |
| defeat.wav | WAV | Defeat sound | 3.5s |

### Music
| Filename | Type | Description | Duration |
|----------|------|-------------|----------|
| main_theme.ogg | OGG | Main menu theme | 3:25 |
| gameplay_1.ogg | OGG | In-game music track 1 | 3:10 |
| gameplay_2.ogg | OGG | In-game music track 2 | 2:55 |
| gameplay_3.ogg | OGG | In-game music track 3 | 3:15 |
| battle_intense.ogg | OGG | Combat music | 2:45 |
| victory_theme.ogg | OGG | Victory sequence music | 1:30 |

### Fonts
| Filename | Type | Description | Styles |
|----------|------|-------------|--------|
| main_font.ttf | TTF | Primary game font | Regular, Bold, Italic |
| title_font.ttf | TTF | Font for titles and headers | Regular, Bold |
| ui_font.ttf | TTF | Font for UI elements | Regular |

## Asset Guidelines

1. Keep file sizes optimized - use compression when possible
2. All textures should use power-of-two dimensions when possible
3. All non-transparent backgrounds should use the same palette
4. Sound effects should be normalized to avoid volume inconsistencies
5. Unit sprites should face right in their default state

## Implementation

To add new assets:
1. Place the file in the appropriate directory
2. Update the resource manifest (see below)
3. Implement loading code in the ResourceManager
4. Use the asset in the game code

## Resource Manifest

The `asset_manifest.json` file in this directory maintains a registry of all 
game assets and their properties. When adding new assets, please update this
file to ensure the ResourceManager properly loads them.
