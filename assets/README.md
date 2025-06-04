# Enhanced 3D Game Assets with Organized Animation System

This directory contains all 3D models and media assets for the RTS game, organized with individual folders for each unit and building to properly manage their animations.

## Directory Structure

- **models/**
  - **units/**: Unit models with individual animation folders
    - **worker/**: Worker model and animations
      - worker.glb (main model)
      - **animations/**: All worker animations (idle.glb, walking.glb, etc.)
    - **fighter/**: Fighter model and animations
    - **ranger/**: Ranger model and animations  
    - **tank/**: Tank model and animations
  - **buildings/**: Building models with individual animation folders
    - **headquarters/**: HQ model and animations
    - **barracks/**: Barracks model and animations
    - **war_factory/**: Factory model and animations
    - **energy_plant/**: Energy plant model and animations
    - **defense_turret/**: Turret model and animations
  - **resources/**: Resource node models with animations
    - **minerals/**: Mineral node model and animations
    - **energy/**: Energy node model and animations
  - **effects/**: Visual effect models
    - **explosion/**: Explosion effect and animation
- **textures/**: Organized UI textures
  - **ui/**: User interface elements organized by category
    - **buttons/**: Button textures (normal, hover, pressed)
    - **panels/**: Panel backgrounds and frames
    - **minimap/**: Minimap-related textures
    - **selection/**: Selection indicators
    - **health/**: Health bar components
    - **resources/**: Resource icons
- **sounds/**: Sound effect files organized by category
  - **units/**: Unit-related sounds
  - **buildings/**: Building-related sounds
  - **ui/**: Interface sounds
  - **game/**: General game sounds
- **music/**: Background music files
- **fonts/**: Game fonts

## Animation System

### Individual Animation Files
Each unit and building type has its own folder containing:
- Main model file (e.g., worker.glb)
- **animations/** subfolder with individual animation files
- Each animation is a separate .glb file for modularity

### Animation Categories

#### Unit Animations
- **Movement**: idle, walking, running
- **Combat**: attacking, blocking, dying
- **Work**: gathering, building, carrying
- **Special**: victory poses, special abilities

#### Building Animations
- **Construction**: Building process animations
- **Operational**: Normal working state animations
- **Production**: Manufacturing and training animations
- **Combat**: Defensive actions (turrets)
- **Damage**: Damage states and destruction

### Benefits of This Structure
1. **Modularity**: Each animation can be updated independently
2. **Memory Efficiency**: Load only needed animations
3. **Version Control**: Track changes to individual animations
4. **Asset Management**: Easy to organize and maintain
5. **Development**: Artists can work on animations separately

## Asset Loading
The ResourceManager loads assets based on the manifest structure, supporting both embedded and separate animation files.
