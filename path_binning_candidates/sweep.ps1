# Sweep the binner's cost-model parameters on CPU (measure.rs) and GPU (PresentMon
# on the painting example). Applies hooks.patch, builds once, and reverts it.
# Each config is "visits_per_listing,min_tile_size,max_grid_cells[,forced_cell_size[,visits_per_cell]]";
# omitted fields keep the constants' values.
#
# Usage: sweep.ps1 -Configs '2000','3000,8,4096' [-Cpu] [-Gpu] [-Rounds 2]
#        [-Scenes default,scribble,...] [-Fixtures substring] [-Seconds 4]
# Turn on stable power state first for absolute GPU numbers.
param(
  [Parameter(Mandatory)][string[]]$Configs,
  [switch]$Cpu,
  [switch]$Gpu,
  [int]$Rounds = 2,
  [string[]]$Scenes = @('default', 'scribble', 'dashed_scribble', 'grid', 'dashed_grid'),
  [string]$Fixtures = '',
  [int]$Seconds = 4
)
$ErrorActionPreference = 'Stop'
if (-not ($Cpu -or $Gpu)) { $Cpu = $true; $Gpu = $true }
$patch = Join-Path $PSScriptRoot 'hooks.patch'
$out = New-Item -ItemType Directory -Force 'target/path_sweep'

git apply $patch
try {
  if ($Cpu) {
    $built = cargo test --locked --release -p gpui --lib path_binning_measure --no-run 2>&1
    $exe = ($built | Select-String 'Executable unittests src.gpui\.rs \((.*)\)').Matches[0].Groups[1].Value
  }
  if ($Gpu) {
    cargo build --locked --release -p gpui --example painting 2>&1 | Select-String '^error' -Context 0,5
  }
} finally {
  git apply -R $patch
}

if ($Cpu) {
  $rows = foreach ($config in $Configs) {
    $env:PATH_SWEEP = $config; $env:FIXTURES = $Fixtures
    $log = Join-Path $out "cpu-$($config -replace ',', '_').tsv"
    $process = Start-Process -FilePath $exe -NoNewWindow -PassThru -RedirectStandardOutput $log `
      -ArgumentList 'path_binning_measure::measure', '--exact', '--ignored', '--nocapture'
    $process.ProcessorAffinity = [IntPtr]4
    $process.WaitForExit()
    Get-Content $log | Where-Object { $_ -match "`t" } | ConvertFrom-Csv -Delimiter "`t" |
      ForEach-Object { $_ | Add-Member config $config -PassThru }
  }
  Remove-Item Env:PATH_SWEEP, Env:FIXTURES
  $rows | Export-Csv (Join-Path $out 'cpu.csv') -NoTypeInformation
  foreach ($metric in 'bin_us', 'visits') {
    "== CPU: $metric =="
    $rows | Group-Object fixture | ForEach-Object {
      $line = [ordered]@{ fixture = $_.Name }
      foreach ($row in $_.Group) { $line[$row.config] = $row.$metric }
      [pscustomobject]$line
    } | Format-Table -AutoSize | Out-String -Width 250
  }
}

if ($Gpu) {
  $painting = (Resolve-Path 'target/release/examples/painting.exe').Path
  $samples = foreach ($round in 1..$Rounds) {
    foreach ($scene in $Scenes) {
      foreach ($config in $Configs) {
        $env:PATH_SWEEP = $config; $env:PAINTING_SCENE = $scene
        $csv = Join-Path $out "gpu-$scene-$($config -replace ',', '_')-$round.csv"
        $app = Start-Process -FilePath $painting -PassThru
        try {
          Start-Sleep -Seconds 2
          presentmon.exe --process_id $app.Id --session_name path_sweep --stop_existing_session `
            --timed $Seconds --terminate_after_timed --no_console_stats --output_file $csv 2>&1 | Out-Null
        } finally {
          Stop-Process -Id $app.Id -ErrorAction SilentlyContinue
        }
        $frames = @(Import-Csv $csv | ForEach-Object { [double]$_.MsGPUBusy } | Sort-Object)
        [pscustomobject]@{
          round = $round; scene = $scene; config = $config; frames = $frames.Count
          gpu_ms = if ($frames.Count) { $frames[[int]($frames.Count / 2)] } else { [double]::NaN }
        }
      }
    }
  }
  Remove-Item Env:PATH_SWEEP, Env:PAINTING_SCENE
  $samples | Export-Csv (Join-Path $out 'gpu.csv') -NoTypeInformation
  "== GPU: median MsGPUBusy per round (ms) =="
  $samples | Group-Object scene | ForEach-Object {
    $line = [ordered]@{ scene = $_.Name }
    foreach ($config in $Configs) {
      $line[$config] = ($_.Group | Where-Object config -eq $config |
        ForEach-Object { '{0:N3}' -f $_.gpu_ms }) -join ' / '
    }
    [pscustomobject]$line
  } | Format-Table -AutoSize | Out-String -Width 250
  $short = $samples | Where-Object { $_.frames -lt 50 }
  if ($short) { "Captures with under 50 frames:"; $short | Format-Table -AutoSize | Out-String }
}
