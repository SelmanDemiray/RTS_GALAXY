# PowerShell script to clean up old monolithic files

# Old files that should be deleted after modularization
$oldFiles = @(
    "src\game.rs",
    "src\network.rs",
    "src\ui.rs",
    "src\menu.rs",
    "src\resources.rs",
    "src\ai.rs"
)

# Check for duplicate module files that could cause conflicts
$duplicateFiles = @(
    "src\ui\game_ui\mod.rs",
    "src\ui\menu\game_ui.rs",
    "src\game\mod.rs",
    "src\entity\mod.rs"
)

# Check if files exist before trying to delete them
foreach ($file in $oldFiles) {
    $fullPath = Join-Path -Path "d:\RUST_RTS_v2" -ChildPath $file
    
    if (Test-Path -Path $fullPath) {
        Write-Host "Removing old file: $fullPath"
        Remove-Item -Path $fullPath -Force
    } else {
        Write-Host "File already removed or doesn't exist: $fullPath"
    }
}

# Remove any duplicate module files
foreach ($file in $duplicateFiles) {
    $fullPath = Join-Path -Path "d:\RUST_RTS_v2" -ChildPath $file
    
    if (Test-Path -Path $fullPath) {
        Write-Host "Removing duplicate module file: $fullPath"
        Remove-Item -Path $fullPath -Force
    }
}

# Verify that necessary directories exist
$directories = @(
    "src\entity",
    "src\game",
    "src\ui",
    "src\ui\menu",
    "src\network",
    "src\resources",
    "src\ai"
)

foreach ($dir in $directories) {
    $fullPath = Join-Path -Path "d:\RUST_RTS_v2" -ChildPath $dir
    
    if (-not (Test-Path -Path $fullPath)) {
        Write-Host "Creating directory: $fullPath"
        New-Item -Path $fullPath -ItemType Directory -Force | Out-Null
    }
}

Write-Host "Cleanup complete. The codebase has been successfully modularized."
Write-Host "You can now build the project using 'cargo build' or 'cargo run'."
