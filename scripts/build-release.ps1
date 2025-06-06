# Build script for Windows
param (
    [string]$Target = "all"
)

# Set error action preference
$ErrorActionPreference = "Stop"

# Define build targets
$buildTargets = @()

switch ($Target) {
    "all" { 
        $buildTargets = @(
            "x86_64-pc-windows-msvc", 
            "x86_64-unknown-linux-gnu",
            "x86_64-apple-darwin",
            "aarch64-apple-darwin"
        )
    }
    "windows" { $buildTargets = @("x86_64-pc-windows-msvc") }
    "linux" { $buildTargets = @("x86_64-unknown-linux-gnu") }
    "macos" { $buildTargets = @("x86_64-apple-darwin", "aarch64-apple-darwin") }
    default { $buildTargets = @($Target) }
}

# Install required targets if not already installed
foreach ($target in $buildTargets) {
    rustup target add $target
}

# Create release directory
$version = git describe --tags --always
$releaseDir = "release\$version"
New-Item -ItemType Directory -Force -Path $releaseDir | Out-Null

# Build for each target
foreach ($target in $buildTargets) {
    Write-Host "Building for $target..." -ForegroundColor Green
    
    # Determine platform-specific settings
    $extension = ""
    $packageSuffix = ""
    
    if ($target -like "*windows*") {
        $extension = ".exe"
        $packageSuffix = "windows"
    } elseif ($target -like "*linux*") {
        $packageSuffix = "linux-x64"
    } elseif ($target -like "*aarch64*") {
        $packageSuffix = "macos-arm64"
    } else {
        $packageSuffix = "macos-x64"
    }
    
    # Build the binary
    cargo build --release --target $target
    
    if ($LASTEXITCODE -ne 0) {
        Write-Error "Failed to build for $target"
        continue
    }
    
    # Package the release
    $packageDir = "$releaseDir\zed-$packageSuffix"
    New-Item -ItemType Directory -Force -Path $packageDir | Out-Null
    
    # Copy binary
    $binaryPath = "target\$target\release\zed$extension"
    if (Test-Path $binaryPath) {
        Copy-Item -Path $binaryPath -Destination $packageDir
    } else {
        Write-Warning "Binary not found at $binaryPath"
    }
    
    # Copy assets and documentation
    if (Test-Path "assets") {
        Copy-Item -Path "assets" -Destination $packageDir -Recurse -Force
    }
    if (Test-Path "README.md") {
        Copy-Item -Path "README.md" -Destination $packageDir
    }
    if (Test-Path "LICENSE") {
        Copy-Item -Path "LICENSE" -Destination $packageDir
    }
    
    # Create archive
    Push-Location $releaseDir
    $archiveName = "zed-$packageSuffix"
    if ($packageSuffix -eq "windows") {
        Compress-Archive -Path $archiveName -DestinationPath "$archiveName.zip" -Force
    } else {
        # On Windows, we'll create a .tar.gz using 7-Zip if available
        if (Get-Command 7z -ErrorAction SilentlyContinue) {
            7z a -ttar -so "$archiveName.tar" $archiveName | 7z a -si "$archiveName.tar.gz"
        } else {
            Write-Warning "7-Zip not found. Skipping .tar.gz creation for $archiveName"
        }
    }
    Pop-Location
    
    Write-Host "Built package: $releaseDir\$archiveName.*" -ForegroundColor Green
}

# Generate checksums if we have OpenSSL or certutil available
Push-Location $releaseDir
$checksumFile = "SHA256SUMS"
if (Test-Path $checksumFile) { Remove-Item $checksumFile }

Get-ChildItem -File | ForEach-Object {
    $file = $_.FullName
    $hash = (Get-FileHash -Path $file -Algorithm SHA256).Hash.ToLower()
    "$hash  $($_.Name)" | Add-Content -Path $checksumFile
}

Write-Host "\nBuild complete! Release artifacts are in $releaseDir" -ForegroundColor Green
Write-Host "Checksums saved to $releaseDir\$checksumFile" -ForegroundColor Green
Pop-Location
