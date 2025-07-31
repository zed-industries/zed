param (
    [Parameter(Mandatory = $true)]
    [int]$MAX_SIZE_IN_GB
)

$ErrorActionPreference = "Stop"
$PSNativeCommandUseErrorActionPreference = $true
$ProgressPreference = "SilentlyContinue"

if (-Not (Test-Path -Path "target")) {
    Write-Host "target directory does not exist yet"
    exit 0
}

$current_size_gb = (Get-ChildItem -Recurse -Force -File -Path "target" | Measure-Object -Property Length -Sum).Sum / 1GB

Write-Host "target directory size: ${current_size_gb}GB. max size: ${MAX_SIZE_IN_GB}GB"

if ($current_size_gb -gt $MAX_SIZE_IN_GB) {
    Write-Host "clearing target directory"
    Remove-Item -Recurse -Force -Path "target\*" -ErrorAction SilentlyContinue
}
