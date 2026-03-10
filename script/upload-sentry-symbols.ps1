# Builds Zed and remote_server binaries and uploads debug symbols to Sentry.
# Intended for re-uploading symbols for releases where the original upload
# failed or the files were corrupted.
#
# Usage: script/upload-sentry-symbols.ps1 [-Verify]
#
# Environment:
#   SENTRY_AUTH_TOKEN  (required) Sentry authentication token

[CmdletBinding()]
Param(
    [Parameter()][switch]$Verify
)

. "$PSScriptRoot/lib/sentry-upload.ps1"

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

if (-not (Test-Path "env:SENTRY_AUTH_TOKEN")) {
    Write-Error "SENTRY_AUTH_TOKEN is required"
}

if (-not (Get-Command "sentry-cli" -ErrorAction SilentlyContinue)) {
    Write-Error "sentry-cli is not installed. Install with: winget install -e --id=Sentry.sentry-cli"
}

$target_dir = if ($env:CARGO_TARGET_DIR) { $env:CARGO_TARGET_DIR } else { "target" }

Write-Output "==> Building zed and cli (release)..."
cargo build --release --package zed --package cli

Write-Output "==> Building remote_server (release)..."
cargo build --release --package remote_server

Write-Output "==> Uploading debug symbols to Sentry..."
Upload-ToSentry -Paths @($target_dir)

Write-Output "==> Done."
