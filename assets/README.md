# Game Assets

This directory contains all media assets for the RTS game, organized by type.

## Directory Structure

- **music/**: Background music files (`.ogg` or `.wav` format)
- **sounds/**: Sound effect files (`.ogg` or `.wav` format)
- **textures/**: Game textures and sprites (when implemented)
- **fonts/**: Game fonts (when implemented)

## Loading Assets

Assets are loaded through the `ResourceManager` class located in `src/resources/mod.rs`. 
All paths are relative to this assets directory.

## Asset Guidelines

1. Keep file sizes optimized
2. Use consistent naming conventions
3. Ensure all assets are properly licensed
4. Document usage in the appropriate README files

## Implementation

To add new assets:
1. Place the file in the appropriate directory
2. Update the corresponding README
3. Add loading code to the ResourceManager if needed
4. Implement usage in the game code
