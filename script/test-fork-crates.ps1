# Runs the unit tests of the crates this fork adds on top of upstream Zed.
#
# Upstream's own CI (`.github/workflows/run_tests.yml`) only runs for the
# zed-industries organisation, so it never tests these crates.
#
# GitHub runs this for every push to main and for pull requests
# (`.github/workflows/fork_tests.yml`). There is also an optional pre-push
# hook in `script/git-hooks`, off by default, that runs it locally and stops
# the push on a failure.
#
# It can also be run by hand:
#
#   script/test-fork-crates.ps1                    # the fork-authored crates
#   script/test-fork-crates.ps1 -IncludeVendored   # also gpui_component & co.
#   script/test-fork-crates.ps1 -Jobs 4            # fewer parallel build jobs
#   script/test-fork-crates.ps1 -Only npm_backend,helm_panel

[CmdletBinding()]
param(
    # Parallel build jobs passed to cargo.
    [int]$Jobs = 8,

    # Also test the crates vendored from longbridge/gpui-component and
    # pacifio/gpui-flow. Off by default: they are large, and not all of
    # their examples build against this tree's forked `lsp_types`.
    [switch]$IncludeVendored,

    # Limit the run to these packages (cargo package names).
    [string[]]$Only
)

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $false

# Cargo package names. Written by the fork, on top of the crates below.
$forkCrates = @(
    'cargo_backend',
    'cargo_manager_panel',
    'cockpit_panel',
    'dashboard_panel',
    'database_backend',
    'database_panel',
    'designer_panel',
    'dotnet_backend',
    'dotnet_panel',
    'flows_panel',
    'helm_backend',
    'helm_ui',
    'helm_workspace',
    'helm_panel',
    'node_backend',
    'node_panel',
    'npm_backend',
    'npm_bootstrap',
    'npm_manager_panel',
    'nuget_manager_panel',
    'processes_panel',
    'python_backend',
    'python_manager_panel',
    'python_panel',
    'rust_panel',
    'runtime_processes',
    'script_runner_panel',
    'workflow_engine'
)

# Vendored from upstream projects. Package names use hyphens where the
# directories under crates/ use underscores.
$vendoredCrates = @(
    'gpui-base',
    'gpui-component',
    'gpui-component-assets',
    'gpui-component-macros',
    'gpui-fps',
    'gpui_flow'
)

$crates = $forkCrates
if ($IncludeVendored) {
    $crates += $vendoredCrates
} else {
    # Small, and the fork adds icons to it, so it is always worth checking.
    $crates += 'gpui-component-assets'
}

if ($Only) {
    # Started with `pwsh -File`, `-Only a,b` arrives as the single string
    # "a,b" rather than two values, so split it here.
    $Only = @($Only | ForEach-Object { $_ -split ',' } | ForEach-Object { $_.Trim() } | Where-Object { $_ })
    $unknown = $Only | Where-Object { ($forkCrates + $vendoredCrates) -notcontains $_ }
    if ($unknown) {
        Write-Error "Not a fork crate: $($unknown -join ', ')"
        exit 2
    }
    $crates = $Only
}

$repoRoot = Split-Path -Parent $PSScriptRoot
Push-Location $repoRoot
try {
    $packageArgs = $crates | ForEach-Object { '-p'; $_ }
    Write-Host "Testing $($crates.Count) crate(s) with -j $Jobs" -ForegroundColor Cyan

    # `--lib` keeps this to unit tests: the vendored crates ship examples and
    # benches that are not part of what the fork relies on.
    & cargo test -j $Jobs --lib @packageArgs
    $exitCode = $LASTEXITCODE
} finally {
    Pop-Location
}

if ($exitCode -eq 0) {
    Write-Host "All fork crate tests passed." -ForegroundColor Green
} else {
    Write-Host "Fork crate tests failed (cargo exit code $exitCode)." -ForegroundColor Red
}
exit $exitCode
