# 3D Game Assets

This directory contains all 3D models and media assets for the RTS game, organized by type.

## Directory Structure

- **models/**
  - **units/**: 3D models for units (.glb format)
  - **buildings/**: 3D models for buildings (.glb format)
  - **resources/**: 3D models for resource nodes (.glb format)
  - **terrain/**: 3D terrain models and elements (.glb format)
  - **props/**: 3D models for props and decorations (.glb format)
  - **effects/**: 3D models for visual effects (.glb format)
- **textures/**
  - **ui/**: User interface elements and icons (2D)
- **sounds/**: Sound effect files (.ogg or .wav format)
- **music/**: Background music files (.ogg format)
- **fonts/**: Game fonts (.ttf format)

## 3D Models List

### Units
| Filename | Format | Description | Recommended Scale |
|----------|--------|-------------|------------------|
| worker.glb | glTF Binary | Worker unit model | 1.0 |
| fighter.glb | glTF Binary | Fighter unit model | 1.0 |
| ranger.glb | glTF Binary | Ranger unit model | 1.0 |
| tank.glb | glTF Binary | Tank unit model | 1.2 |

### Buildings
| Filename | Format | Description | Recommended Scale |
|----------|--------|-------------|------------------|
| headquarters.glb | glTF Binary | Main base building | 2.0 |
| barracks.glb | glTF Binary | Unit training building | 1.8 |
| factory.glb | glTF Binary | Advanced unit building | 1.8 |
| resource_depot.glb | glTF Binary | Resource storage | 1.8 |
| defense_turret.glb | glTF Binary | Defensive structure | 1.2 |
| building_construction.glb | glTF Binary | Building under construction | 1.8 |

### Resources
| Filename | Format | Description | Recommended Scale |
|----------|--------|-------------|------------------|
| minerals.glb | glTF Binary | Mineral resource node | 1.0 |
| minerals_depleted.glb | glTF Binary | Depleted mineral node | 1.0 |
| energy.glb | glTF Binary | Energy resource node | 1.0 |
| energy_depleted.glb | glTF Binary | Depleted energy node | 1.0 |

### Terrain
| Filename | Format | Description | Recommended Scale |
|----------|--------|-------------|------------------|
| grass_tile.glb | glTF Binary | Basic grass terrain | 1.0 |
| dirt_tile.glb | glTF Binary | Dirt terrain | 1.0 |
| sand_tile.glb | glTF Binary | Sand terrain | 1.0 |
| rock_tile.glb | glTF Binary | Rocky terrain | 1.0 |
| water_tile.glb | glTF Binary | Water terrain | 1.0 |

### Props
| Filename | Format | Description | Recommended Scale |
|----------|--------|-------------|------------------|
| obstacle_rock.glb | glTF Binary | Rock obstacle | 1.0 |
| obstacle_tree.glb | glTF Binary | Tree obstacle | 1.0 |

### Effects
| Filename | Format | Description | Recommended Scale |
|----------|--------|-------------|------------------|
| explosion.glb | glTF Binary | Explosion animation | 1.0 |
| laser_beam.glb | glTF Binary | Ranger attack effect | 1.0 |
| impact_hit.glb | glTF Binary | Attack impact effect | 0.8 |
| dust_cloud.glb | glTF Binary | Movement dust effect | 0.8 |
| construction_effect.glb | glTF Binary | Building construction | 1.5 |

### UI Elements (2D)
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

## Creating Models in Blender

When creating models in Blender for the game:

1. Use appropriate scale (1 Blender unit = 1 meter in game)
2. Apply all transformations before export
3. Export as glTF Binary (.glb) format
4. Set the correct orientation (Y-up for Blender exports)
5. Include materials and textures in the export
6. For animated models, include armatures and animations

## Export Settings for Blender

When exporting from Blender to glTF:

- Format: glTF Binary (.glb)
- Include: Selected Objects
- Transform: Y Up
- Geometry:
  - Apply Modifiers: On
  - UVs: On
  - Normals: On
  - Tangents: On
  - Vertex Colors: On
  - Materials: On
  - Textures: On
  - Animations: On (for animated assets)

## Implementation

To add new 3D assets:
1. Create the model in Blender
2. Export to .glb format
3. Place the file in the appropriate models directory
4. Update the asset manifest (see below)

## Resource Manifest

The `asset_manifest.json` file in this directory maintains a registry of all 
game assets and their properties. When adding new assets, please update this
file to ensure the ResourceManager properly loads them.
