param (
    [Parameter(Mandatory = $true)]
    [int]$MAX_SIZE_IN_GB,
    [Parameter(Mandatory = $false)]
    [int]$SMALL_CLEAN_SIZE_IN_GB = -1
)

$ErrorActionPreference = "Stop"
$PSNativeCommandUseErrorActionPreference = $true
$ProgressPreference = "SilentlyContinue"

if (-Not (Test-Path -Path "target")) {
    Write-Host "target directory does not exist yet"
    exit 0
}

if ($SMALL_CLEAN_SIZE_IN_GB -ge 0 -and $SMALL_CLEAN_SIZE_IN_GB -ge $MAX_SIZE_IN_GB) {
    Write-Host "error: small clean threshold (${SMALL_CLEAN_SIZE_IN_GB}GB) must be smaller than max size (${MAX_SIZE_IN_GB}GB)"
    exit 1
}

$current_size_gb = (Get-ChildItem -Recurse -Force -File -Path "target" | Measure-Object -Property Length -Sum).Sum / 1GB

Write-Host "target directory size: ${current_size_gb}GB. max size: ${MAX_SIZE_IN_GB}GB"

if ($current_size_gb -gt $MAX_SIZE_IN_GB) {
    Write-Host "clearing target directory"
    Remove-Item -Recurse -Force -Path "target\*" -ErrorAction SilentlyContinue
} elseif ($SMALL_CLEAN_SIZE_IN_GB -ge 0 -and $current_size_gb -gt $SMALL_CLEAN_SIZE_IN_GB) {
    Write-Host "running cargo clean --workspace (size above small clean threshold of ${SMALL_CLEAN_SIZE_IN_GB}GB)"
    cargo clean --workspace
}
