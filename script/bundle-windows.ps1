param()

$ErrorActionPreference = "Stop"
$ProgressPreference = 'SilentlyContinue'

$ExecutableName = "zed.exe"
$BuildConfiguration = "release"
$SourceDir = "target/$BuildConfiguration"
$FullExePath = Join-Path -Path $SourceDir -ChildPath $ExecutableName

$OutputZipName = "zed-windows-x86_64.zip"
$OutputZipPath = Join-Path -Path "target" -ChildPath $OutputZipName

Write-Host "Starting Zed Windows build (Configuration: $BuildConfiguration)..."
try {
    cargo build --release --locked --verbose
    Write-Host "Cargo build completed successfully."
}
catch {
    Write-Error "Cargo build failed!"
    Write-Error $_.Exception.Message
    exit 1
}

Write-Host "Packaging the application to '$OutputZipPath'..."

if (-not (Test-Path -Path $FullExePath -PathType Leaf)) {
    Write-Error "Build succeeded but executable '$FullExePath' not found!"
    exit 1
}

$FilesToPackage = @($FullExePath)

if (Test-Path -Path $OutputZipPath) {
    Write-Host "Removing existing zip file: $OutputZipPath"
    Remove-Item -Path $OutputZipPath -Force
}

try {
    Compress-Archive -Path $FilesToPackage -DestinationPath $OutputZipPath -CompressionLevel Optimal
    Write-Host "Successfully created package: $OutputZipPath"
}
catch {
    Write-Error "Failed to create zip archive!"
    Write-Error $_.Exception.Message
    exit 1
}

Write-Host "Windows bundle script finished successfully."
exit 0
