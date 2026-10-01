# Record an 8 kHz sampled CPU profile of the binning loop and report per-function samples.
# Applies hooks.patch, builds, and reverts it. Fixture names are those in measure.rs.
# Usage: profile.ps1 [-Fixture 'rust logo'] [-Seconds 5] [-Tag name]
param([string]$Fixture = 'scribble', [int]$Seconds = 5, [string]$Tag = '')
$ErrorActionPreference = 'Stop'
$xperf = 'C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit\xperf.exe'
$patch = Join-Path $PSScriptRoot 'hooks.patch'
git apply $patch
try {
  $built = cargo test --locked --release -p gpui --lib path_binning_measure --no-run 2>&1
  $exe = ($built | Select-String 'Executable unittests src.gpui\.rs \((.*)\)').Matches[0].Groups[1].Value
} finally {
  git apply -R $patch
}
$exe = (Resolve-Path $exe).Path
$dir = 'target/profile'
New-Item -ItemType Directory -Force $dir | Out-Null
$dir = (Resolve-Path $dir).Path
if ($Tag -eq '') { $Tag = $Fixture -replace '\W', '_' }
$etl = Join-Path $dir "bin-$Tag.etl"
$dump = Join-Path $dir "bin-$Tag.csv"

# No stack walks (the symbolizer resolves inline chains from the sampled address
# alone), and large buffers: with both, the kernel logger dropped all but the
# first fraction of a second of samples.
sudo $xperf -on PROC_THREAD+LOADER+PROFILE -SetProfInt 1221 -BufferSize 1024 -MinBuffers 256 -MaxBuffers 1024 -f (Join-Path $dir "bin-$Tag.kernel.etl")
try {
  $env:FIXTURE = $Fixture; $env:PROFILE_SECONDS = "$Seconds"
  $log = Join-Path $dir "prof-$Tag.log"
  $p = Start-Process -FilePath $exe -NoNewWindow -PassThru -RedirectStandardOutput $log `
    -ArgumentList 'path_binning_measure::profile', '--exact', '--ignored', '--nocapture'
  $p.ProcessorAffinity = [IntPtr]4
  $p.WaitForExit()
  Get-Content $log | Where-Object { $_ -match 'iterations' }
} finally {
  sudo $xperf -d $etl | Out-Null
}

& $xperf -i $etl -quiet -tle -tti -target machine -o $dump 2>&1 | Where-Object { $_ -notmatch '^\s*$' } | Select-Object -First 5
python (Join-Path $PSScriptRoot 'symbolize.py') $dump $exe
