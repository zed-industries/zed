# Like 'set -e' in bash
$ErrorActionPreference = "Stop"

# Check if rustup is already installed
if (Get-Command rustup -ErrorAction SilentlyContinue)
{
    Write-Output "Rustup found: $((rustup --version 2>&1 | Select-Object -First 1))"
} else
{
    # Install rust-up
    Invoke-WebRequest `
        -OutFile rustup-init.exe `
        -Uri https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe
    .\rustup-init.exe -y --default-toolchain none
    Remove-Item -Force rustup-init.exe
}
