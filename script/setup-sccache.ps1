#Requires -Version 5.1
$ErrorActionPreference = "Stop"

$SCCACHE_VERSION = "v0.10.0"
$SCCACHE_DIR = "./target/sccache"

function Install-Sccache {
    New-Item -ItemType Directory -Path $SCCACHE_DIR -Force | Out-Null

    $sccachePath = Join-Path $SCCACHE_DIR "sccache.exe"

    if (Test-Path $sccachePath) {
        Write-Host "sccache already cached: $(& $sccachePath --version)"
    }
    else {
        Write-Host "Installing sccache ${SCCACHE_VERSION} from GitHub releases..."

        $arch = if ([Environment]::Is64BitOperatingSystem) { "x86_64" } else { "i686" }
        $archive = "sccache-${SCCACHE_VERSION}-${arch}-pc-windows-msvc.zip"
        $basename = "sccache-${SCCACHE_VERSION}-${arch}-pc-windows-msvc"
        $url = "https://github.com/mozilla/sccache/releases/download/${SCCACHE_VERSION}/${archive}"

        $tempDir = Join-Path $env:TEMP "sccache-install"
        New-Item -ItemType Directory -Path $tempDir -Force | Out-Null

        try {
            $archivePath = Join-Path $tempDir $archive
            Invoke-WebRequest -Uri $url -OutFile $archivePath
            Expand-Archive -Path $archivePath -DestinationPath $tempDir

            $extractedPath = Join-Path $tempDir $basename "sccache.exe"
            Move-Item -Path $extractedPath -Destination $sccachePath -Force

            Write-Host "Installed sccache: $(& $sccachePath --version)"
        }
        finally {
            Remove-Item -Recurse -Force $tempDir -ErrorAction SilentlyContinue
        }
    }

    $absolutePath = (Resolve-Path $SCCACHE_DIR).Path
    if ($env:GITHUB_PATH) {
        $absolutePath | Out-File -FilePath $env:GITHUB_PATH -Append -Encoding utf8
    }
    $env:PATH = "$absolutePath;$env:PATH"

    # Verify sccache is available in PATH - fail fast if not
    $sccacheCmd = Get-Command sccache -ErrorAction SilentlyContinue
    if (-not $sccacheCmd) {
        Write-Host "::error::sccache was installed but is not found in PATH"
        Write-Host "PATH: $env:PATH"
        Write-Host "Expected location: $absolutePath"
        if (Test-Path (Join-Path $absolutePath "sccache.exe")) {
            Write-Host "sccache.exe exists at expected location but is not in PATH"
            Write-Host "Directory contents:"
            Get-ChildItem $absolutePath | ForEach-Object { Write-Host "  $_" }
        } else {
            Write-Host "sccache.exe NOT found at expected location"
        }
        exit 1
    }
}

function Configure-Sccache {
    if (-not $env:R2_ACCOUNT_ID) {
        Write-Host "R2_ACCOUNT_ID not set, skipping sccache configuration"
        return
    }

    # Verify sccache is available before configuring
    $sccacheCmd = Get-Command sccache -ErrorAction SilentlyContinue
    if (-not $sccacheCmd) {
        Write-Host "::error::sccache not found in PATH, cannot configure RUSTC_WRAPPER"
        Write-Host "PATH: $env:PATH"
        exit 1
    }

    Write-Host "Configuring sccache with Cloudflare R2..."

    $bucket = if ($env:SCCACHE_BUCKET) { $env:SCCACHE_BUCKET } else { "sccache-zed" }
    $keyPrefix = if ($env:SCCACHE_KEY_PREFIX) { $env:SCCACHE_KEY_PREFIX } else { "sccache/" }
    $baseDir = if ($env:GITHUB_WORKSPACE) { $env:GITHUB_WORKSPACE } else { (Get-Location).Path }

    # Use the absolute path to sccache binary for RUSTC_WRAPPER to avoid
    # any PATH race conditions between GITHUB_PATH and GITHUB_ENV
    $sccacheBin = (Get-Command sccache).Source

    # Set in current process
    $env:SCCACHE_ENDPOINT = "https://$($env:R2_ACCOUNT_ID).r2.cloudflarestorage.com"
    $env:SCCACHE_BUCKET = $bucket
    $env:SCCACHE_REGION = "auto"
    $env:SCCACHE_S3_KEY_PREFIX = $keyPrefix
    $env:SCCACHE_BASEDIR = $baseDir
    $env:AWS_ACCESS_KEY_ID = $env:R2_ACCESS_KEY_ID
    $env:AWS_SECRET_ACCESS_KEY = $env:R2_SECRET_ACCESS_KEY
    $env:RUSTC_WRAPPER = $sccacheBin

    # Also write to GITHUB_ENV for subsequent steps
    if ($env:GITHUB_ENV) {
        @(
            "SCCACHE_ENDPOINT=$($env:SCCACHE_ENDPOINT)"
            "SCCACHE_BUCKET=$($env:SCCACHE_BUCKET)"
            "SCCACHE_REGION=$($env:SCCACHE_REGION)"
            "SCCACHE_S3_KEY_PREFIX=$($env:SCCACHE_S3_KEY_PREFIX)"
            "SCCACHE_BASEDIR=$($env:SCCACHE_BASEDIR)"
            "AWS_ACCESS_KEY_ID=$($env:AWS_ACCESS_KEY_ID)"
            "AWS_SECRET_ACCESS_KEY=$($env:AWS_SECRET_ACCESS_KEY)"
            "RUSTC_WRAPPER=$($env:RUSTC_WRAPPER)"
        ) | Out-File -FilePath $env:GITHUB_ENV -Append -Encoding utf8
    }

    Write-Host "✓ sccache configured with Cloudflare R2 (bucket: $bucket)"
}

function Show-Config {
    Write-Host "=== sccache configuration ==="
    Write-Host "sccache version: $(sccache --version)"
    Write-Host "sccache path: $((Get-Command sccache).Source)"
    Write-Host "RUSTC_WRAPPER: $($env:RUSTC_WRAPPER ?? '<not set>')"
    Write-Host "SCCACHE_BUCKET: $($env:SCCACHE_BUCKET ?? '<not set>')"
    Write-Host "SCCACHE_ENDPOINT: $($env:SCCACHE_ENDPOINT ?? '<not set>')"
    Write-Host "SCCACHE_REGION: $($env:SCCACHE_REGION ?? '<not set>')"
    Write-Host "SCCACHE_S3_KEY_PREFIX: $($env:SCCACHE_S3_KEY_PREFIX ?? '<not set>')"
    Write-Host "SCCACHE_BASEDIR: $($env:SCCACHE_BASEDIR ?? '<not set>')"

    if ($env:AWS_ACCESS_KEY_ID) {
        Write-Host "AWS_ACCESS_KEY_ID: <set>"
    }
    else {
        Write-Host "AWS_ACCESS_KEY_ID: <not set>"
    }

    if ($env:AWS_SECRET_ACCESS_KEY) {
        Write-Host "AWS_SECRET_ACCESS_KEY: <set>"
    }
    else {
        Write-Host "AWS_SECRET_ACCESS_KEY: <not set>"
    }

    Write-Host "=== sccache stats ==="
    sccache --show-stats
}

Install-Sccache
Configure-Sccache
Show-Config
