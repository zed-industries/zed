$ErrorActionPreference = "Stop"

$HAKARI_VERSION = "0.9"

$scriptPath = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location (Split-Path -Parent $scriptPath)

$hakariInstalled = $false
try {
    $versionOutput = cargo hakari --version 2>&1
    if ($versionOutput -match "cargo-hakari $HAKARI_VERSION") {
        $hakariInstalled = $true
    }
}
catch {
    $hakariInstalled = $false
}

if (-not $hakariInstalled) {
    Write-Host "Installing cargo-hakari@^$HAKARI_VERSION..."
    cargo install "cargo-hakari@^$HAKARI_VERSION"
    if ($LASTEXITCODE -ne 0) {
        throw "Failed to install cargo-hakari@^$HAKARI_VERSION"
    }
}
else {
    Write-Host "cargo-hakari@^$HAKARI_VERSION is already installed."
}

# update the workspace-hack crate
cargo hakari generate
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

# make sure workspace-hack is added as a dep for all crates in the workspace
cargo hakari manage-deps
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
