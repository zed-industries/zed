# Download and install Zed (ChxisB fork) on Windows.
# Run: irm https://raw.githubusercontent.com/ChxisB/zed/main/script/install.ps1 | iex

$Repo = "ChxisB/zed"
$Version = "latest"
$Target = "windows-x86_64"

$Url = "https://github.com/$Repo/releases/$Version/download/zed-$Target.tar.gz"
$InstallDir = "$env:LOCALAPPDATA\Zed"

Write-Host "Downloading Zed..." -ForegroundColor Green

# Create install directory
New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null

# Download
$Archive = "$env:TEMP\zed.tar.gz"
Invoke-WebRequest -Uri $Url -OutFile $Archive

# Extract (requires tar which is built into Windows 10 1803+)
tar xzf $Archive -C $InstallDir

# Add to PATH if not already there
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable("Path", "$UserPath;$InstallDir", "User")
    $env:Path += ";$InstallDir"
}

Write-Host "Installed to $InstallDir\zed.exe" -ForegroundColor Green
Write-Host "Run: zed" -ForegroundColor Green
