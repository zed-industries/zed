# Checks if cargo is in the user's path or in default install path
# If not, download with rustup-installer (which respects CARGO_HOME / RUSTUP_HOME)

# Like 'set -e' in bash
$ErrorActionPreference = "Stop"

$cargoHome = if ($env:CARGO_HOME) { $env:CARGO_HOME } else { "$env:USERPROFILE\.cargo" }
$rustupPath = "$cargoHome\bin\rustup.exe"
$cargoPath = "$cargoHome\bin\cargo.exe"

# Check if cargo is already available in path
if (Get-Command cargo -ErrorAction SilentlyContinue)
{
    cargo --version
    exit
}
# Check if rustup and cargo are available in CARGO_HOME
elseif (-not ((Test-Path $rustupPath) -and (Test-Path $cargoPath))) {
    Write-Output "Rustup or Cargo not found in $cargoHome, installing..."

    $tempDir = [System.IO.Path]::GetTempPath()

    # Download and install rustup
    $RustupInitPath = "$tempDir\rustup-init.exe"
    Write-Output "Downloading rustup installer..."
    Invoke-WebRequest `
        -OutFile $RustupInitPath `
        -Uri https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe

    Write-Output "Installing rustup..."
    & $RustupInitPath -y --default-toolchain none
    Remove-Item -Force $RustupInitPath

    Write-Output "Rust installation complete."
    # This is necessary
}

& $rustupPath --version
& $cargoPath --version
