#!/usr/bin/env pwsh
# Provision the default WSL distro for the Windows sandbox behavior tests and
# run them. The Windows analog of running `cargo xtask sandbox-tests` on Linux.
#
# What it does:
#   1. Checks that WSL is installed.
#   2. (Unless -NoProvision) installs `bubblewrap` into the default distro and
#      enables unprivileged user namespaces, so the sandbox can actually be
#      enforced. Without both, the helper can only SKIP the enforcement checks —
#      it can't verify a sandbox that was never set up. On Ubuntu 24.04 (the
#      current default WSL distro) user namespaces are restricted by AppArmor by
#      default, which is exactly why this step is needed.
#   3. Runs `cargo xtask wsl-sandbox-tests`, which builds and runs
#      `wsl_sandbox_test_helper` against the real WSL/Bubblewrap sandbox.
#
# By default it requires the sandbox to actually be enforced (so a broken setup
# fails loudly instead of silently skipping). Pass -AllowSkip to keep the
# helper's default skip-when-unenforceable behavior.
#
# Notes:
#   * Provisioning runs as root via `wsl -u root`, so it needs no sudo password.
#   * Auto-provisioning only supports apt-based distros (Ubuntu/Debian). For
#     others, install `bubblewrap` yourself and re-run with -NoProvision.
#   * The user-namespace sysctl is set for the running WSL VM only; it resets
#     after `wsl --shutdown`.
#   * If the network checks are skipped, Windows Firewall may be blocking the
#     test's local listener; that only affects the network assertions.
#
# Usage:
#   ./script/test-wsl-sandbox.ps1
#   ./script/test-wsl-sandbox.ps1 -NoProvision
#   ./script/test-wsl-sandbox.ps1 -Release
#   ./script/test-wsl-sandbox.ps1 -AllowSkip

[CmdletBinding()]
param(
    [switch]$NoProvision,
    [switch]$Release,
    [switch]$AllowSkip
)

$ErrorActionPreference = "Stop"

if (-not (Get-Command wsl.exe -ErrorAction SilentlyContinue)) {
    Write-Error "wsl.exe was not found. Install WSL (https://learn.microsoft.com/windows/wsl/install) and try again."
}

# Run a command in the default WSL distro as root, throwing on failure.
function Invoke-WslRoot {
    param([Parameter(Mandatory = $true)][string]$Script)
    & wsl.exe -u root -- sh -c $Script
    if ($LASTEXITCODE -ne 0) {
        throw "WSL command failed (exit $LASTEXITCODE): $Script"
    }
}

# Run a command in the default WSL distro as the default user, returning its
# exit code instead of throwing.
function Test-Wsl {
    param([Parameter(Mandatory = $true)][string]$Script)
    & wsl.exe -- sh -lc $Script *> $null
    return $LASTEXITCODE
}

if (-not $NoProvision) {
    Write-Host "==> Provisioning the default WSL distro for sandbox testing"

    if ((Test-Wsl "command -v apt-get >/dev/null 2>&1") -eq 0) {
        Write-Host "    Installing bubblewrap (apt-get)..."
        Invoke-WslRoot "apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y bubblewrap"
    }
    else {
        Write-Warning "Auto-provisioning only supports apt-based distros. Install 'bubblewrap' in your default WSL distro manually, then re-run with -NoProvision."
    }

    Write-Host "    Enabling unprivileged user namespaces (for this WSL VM session)..."
    # Older kernels lack this key (the AppArmor restriction simply isn't
    # present), so don't treat a missing key as an error.
    Invoke-WslRoot "sysctl -w kernel.apparmor_restrict_unprivileged_userns=0 2>/dev/null || true"
}

if ((Test-Wsl "command -v bwrap >/dev/null 2>&1") -ne 0) {
    Write-Warning "bwrap is not installed in the default WSL distro; the sandbox cannot be enforced. Install 'bubblewrap' (or run without -NoProvision)."
}

$repoRoot = Split-Path -Parent $PSScriptRoot
Push-Location $repoRoot
try {
    $xtaskArgs = @("xtask", "wsl-sandbox-tests")
    if (-not $AllowSkip) { $xtaskArgs += "--require-enforced" }
    if ($Release) { $xtaskArgs += "--release" }

    Write-Host "==> Running: cargo $($xtaskArgs -join ' ')"
    & cargo @xtaskArgs
    $exitCode = $LASTEXITCODE
}
finally {
    Pop-Location
}

exit $exitCode
