$ErrorActionPreference = "Stop"

Write-Host "Your PATH entries:"
$env:Path -split ";" | ForEach-Object { Write-Host "  $_" }

$needAddWorkspace = $false
if ($args -notcontains "-p" -and $args -notcontains "--package")
{
    $needAddWorkspace = $true
}

# https://stackoverflow.com/questions/41324882/how-to-run-a-powershell-script-with-verbose-output/70020655#70020655
Set-PSDebug -Trace 2

if ($env:CARGO)
{
    $Cargo = $env:CARGO
} elseif (Get-Command "cargo" -ErrorAction SilentlyContinue)
{
    $Cargo = "cargo"
} else
{
    Write-Error "Could not find cargo in path." -ErrorAction Stop
}

if ($needAddWorkspace)
{
    & $Cargo clippy @args --workspace --release --all-targets --all-features -- --deny warnings
} else
{
    & $Cargo clippy @args --release --all-targets --all-features -- --deny warnings
}
