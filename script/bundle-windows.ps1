[CmdletBinding()]
Param(
    [Parameter()][Alias('i')][switch]$Install,
    [Parameter()][Alias('h')][switch]$Help,
    [Parameter()][string]$Name
)

. "$PSScriptRoot/lib/blob-store.ps1"
. "$PSScriptRoot/lib/workspace.ps1"

# https://stackoverflow.com/questions/57949031/powershell-script-stops-if-program-fails-like-bash-set-o-errexit
$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

$buildSuccess = $false

if ($Help) {
    Write-Output "Usage: test.ps1 [-Install] [-Help]"
    Write-Output "Build the installer for Windows.\n"
    Write-Output "Options:"
    Write-Output "  -Install, -i  Run the installer after building."
    Write-Output "  -Help, -h     Show this help message."
    exit 0
}

Push-Location -Path crates/zed
$channel = Get-Content "RELEASE_CHANNEL"
$env:ZED_RELEASE_CHANNEL = $channel
$env:RELEASE_CHANNEL = $channel
Pop-Location

function CheckEnvironmentVariables {
    $requiredVars = @(
        'ZED_WORKSPACE', 'RELEASE_VERSION', 'ZED_RELEASE_CHANNEL',
        'AZURE_TENANT_ID', 'AZURE_CLIENT_ID', 'AZURE_CLIENT_SECRET',
        'ACCOUNT_NAME', 'CERT_PROFILE_NAME', 'ENDPOINT',
        'FILE_DIGEST', 'TIMESTAMP_DIGEST', 'TIMESTAMP_SERVER'
    )

    foreach ($var in $requiredVars) {
        if (-not (Test-Path "env:$var")) {
            Write-Error "$var is not set"
            exit 1
        }
    }
}

function PrepareForBundle {
    if (Test-Path "$innoDir") {
        Remove-Item -Path "$innoDir" -Recurse -Force
    }
    New-Item -Path "$innoDir" -ItemType Directory -Force
    Copy-Item -Path "$env:ZED_WORKSPACE\crates\zed\resources\windows\*" -Destination "$innoDir" -Recurse -Force
    New-Item -Path "$innoDir\make_appx" -ItemType Directory -Force
    New-Item -Path "$innoDir\appx" -ItemType Directory -Force
    New-Item -Path "$innoDir\bin" -ItemType Directory -Force
    New-Item -Path "$innoDir\tools" -ItemType Directory -Force
}

function GenerateLicenses {
    $oldErrorActionPreference = $ErrorActionPreference
    $ErrorActionPreference = 'Continue'
    . $PSScriptRoot/generate-licenses.ps1
    $ErrorActionPreference = $oldErrorActionPreference
}

function BuildZedAndItsFriends {
    Write-Output "Building Zed and its friends, for channel: $channel"
    # Build zed.exe, cli.exe and auto_update_helper.exe
    cargo build --release --package zed --package cli --package auto_update_helper
    Copy-Item -Path ".\target\release\zed.exe" -Destination "$innoDir\Zed.exe" -Force
    Copy-Item -Path ".\target\release\cli.exe" -Destination "$innoDir\cli.exe" -Force
    Copy-Item -Path ".\target\release\auto_update_helper.exe" -Destination "$innoDir\auto_update_helper.exe" -Force
    # Build explorer_command_injector.dll
    switch ($channel) {
        "stable" {
            cargo build --release --features stable --no-default-features --package explorer_command_injector
        }
        "preview" {
            cargo build --release --features preview --no-default-features --package explorer_command_injector
        }
        default {
            cargo build --release --package explorer_command_injector
        }
    }
    Copy-Item -Path ".\target\release\explorer_command_injector.dll" -Destination "$innoDir\zed_explorer_command_injector.dll" -Force
}

function ZipZedAndItsFriendsDebug {
    $items = @(
        ".\target\release\zed.pdb",
        ".\target\release\cli.pdb",
        ".\target\release\auto_update_helper.pdb",
        ".\target\release\explorer_command_injector.pdb"
    )

    Compress-Archive -Path $items -DestinationPath ".\target\release\zed-$env:RELEASE_VERSION-$env:ZED_RELEASE_CHANNEL.dbg.zip" -Force
}


function UploadToSentry {
    if (-not (Get-Command "sentry-cli" -ErrorAction SilentlyContinue)) {
        Write-Output "sentry-cli not found. skipping sentry upload."
        Write-Output "install with: 'winget install -e --id=Sentry.sentry-cli'"
        return
    }
    if (-not (Test-Path "env:SENTRY_AUTH_TOKEN")) {
        Write-Output "missing SENTRY_AUTH_TOKEN. skipping sentry upload."
        return
    }
    Write-Output "Uploading zed debug symbols to sentry..."
    sentry-cli debug-files upload --include-sources --wait -p zed -o zed-dev .\target\release\
}

function MakeAppx {
    switch ($channel) {
        "stable" {
            $manifestFile = "$env:ZED_WORKSPACE\crates\explorer_command_injector\AppxManifest.xml"
        }
        "preview" {
            $manifestFile = "$env:ZED_WORKSPACE\crates\explorer_command_injector\AppxManifest-Preview.xml"
        }
        default {
            $manifestFile = "$env:ZED_WORKSPACE\crates\explorer_command_injector\AppxManifest-Nightly.xml"
        }
    }
    Copy-Item -Path "$manifestFile" -Destination "$innoDir\make_appx\AppxManifest.xml"
    # Add makeAppx.exe to Path
    $sdk = "C:\Program Files (x86)\Windows Kits\10\bin\10.0.26100.0\x64"
    $env:Path += ';' + $sdk
    makeAppx.exe pack /d "$innoDir\make_appx" /p "$innoDir\zed_explorer_command_injector.appx" /nv
}

function SignZedAndItsFriends {
    $files = "$innoDir\Zed.exe,$innoDir\cli.exe,$innoDir\auto_update_helper.exe,$innoDir\zed_explorer_command_injector.dll,$innoDir\zed_explorer_command_injector.appx"
    & "$innoDir\sign.ps1" $files
}

function DownloadAMDGpuServices {
    # If you update the AGS SDK version, please also update the version in `crates/gpui/src/platform/windows/directx_renderer.rs`
    $url = "https://codeload.github.com/GPUOpen-LibrariesAndSDKs/AGS_SDK/zip/refs/tags/v6.3.0"
    $zipPath = ".\AGS_SDK_v6.3.0.zip"
    # Download the AGS SDK zip file
    Invoke-WebRequest -Uri $url -OutFile $zipPath
    # Extract the AGS SDK zip file
    Expand-Archive -Path $zipPath -DestinationPath "." -Force
}

function DownloadConpty {
    $url = "https://www.nuget.org/api/v2/package/CI.Microsoft.Windows.Console.ConPTY/1.22.250314001"
    $zipPath = ".\conpty.zip"
    Invoke-WebRequest -Uri $url -OutFile $zipPath
    Expand-Archive -Path $zipPath -DestinationPath ".\conpty" -Force
}

function CollectFiles {
    Move-Item -Path "$innoDir\zed_explorer_command_injector.appx" -Destination "$innoDir\appx\zed_explorer_command_injector.appx" -Force
    Move-Item -Path "$innoDir\zed_explorer_command_injector.dll" -Destination "$innoDir\appx\zed_explorer_command_injector.dll" -Force
    Move-Item -Path "$innoDir\cli.exe" -Destination "$innoDir\bin\zed.exe" -Force
    Move-Item -Path "$innoDir\zed.sh" -Destination "$innoDir\bin\zed" -Force
    Move-Item -Path "$innoDir\auto_update_helper.exe" -Destination "$innoDir\tools\auto_update_helper.exe" -Force
    Move-Item -Path ".\AGS_SDK-6.3.0\ags_lib\lib\amd_ags_x64.dll" -Destination "$innoDir\amd_ags_x64.dll" -Force
    Move-Item -Path ".\conpty\build\native\runtimes\x64\OpenConsole.exe" -Destination "$innoDir\OpenConsole.exe" -Force
    Move-Item -Path ".\conpty\runtimes\win10-x64\native\conpty.dll" -Destination "$innoDir\conpty.dll" -Force
}

function BuildInstaller {
    $issFilePath = "$innoDir\zed.iss"
    switch ($channel) {
        "stable" {
            $appId = "{{2DB0DA96-CA55-49BB-AF4F-64AF36A86712}"
            $appIconName = "app-icon"
            $appName = "Zed"
            $appDisplayName = "Zed"
            $appSetupName = "ZedEditorUserSetup-x64-$env:RELEASE_VERSION"
            # The mutex name here should match the mutex name in crates\zed\src\zed\windows_only_instance.rs
            $appMutex = "Zed-Stable-Instance-Mutex"
            $appExeName = "Zed"
            $regValueName = "Zed"
            $appUserId = "ZedIndustries.Zed"
            $appShellNameShort = "Z&ed"
            $appAppxFullName = "ZedIndustries.Zed_1.0.0.0_neutral__japxn1gcva8rg"
        }
        "preview" {
            $appId = "{{F70E4811-D0E2-4D88-AC99-D63752799F95}"
            $appIconName = "app-icon-preview"
            $appName = "Zed Preview"
            $appDisplayName = "Zed Preview"
            $appSetupName = "ZedEditorUserSetup-x64-$env:RELEASE_VERSION-preview"
            # The mutex name here should match the mutex name in crates\zed\src\zed\windows_only_instance.rs
            $appMutex = "Zed-Preview-Instance-Mutex"
            $appExeName = "Zed"
            $regValueName = "ZedPreview"
            $appUserId = "ZedIndustries.Zed.Preview"
            $appShellNameShort = "Z&ed Preview"
            $appAppxFullName = "ZedIndustries.Zed.Preview_1.0.0.0_neutral__japxn1gcva8rg"
        }
        "nightly" {
            $appId = "{{1BDB21D3-14E7-433C-843C-9C97382B2FE0}"
            $appIconName = "app-icon-nightly"
            $appName = "Zed Nightly"
            $appDisplayName = "Zed Nightly"
            $appSetupName = "ZedEditorUserSetup-x64-$env:RELEASE_VERSION-nightly"
            # The mutex name here should match the mutex name in crates\zed\src\zed\windows_only_instance.rs
            $appMutex = "Zed-Nightly-Instance-Mutex"
            $appExeName = "Zed"
            $regValueName = "ZedNightly"
            $appUserId = "ZedIndustries.Zed.Nightly"
            $appShellNameShort = "Z&ed Editor Nightly"
            $appAppxFullName = "ZedIndustries.Zed.Nightly_1.0.0.0_neutral__japxn1gcva8rg"
        }
        "dev" {
            $appId = "{{8357632E-24A4-4F32-BA97-E575B4D1FE5D}"
            $appIconName = "app-icon-dev"
            $appName = "Zed Dev"
            $appDisplayName = "Zed Dev"
            $appSetupName = "ZedEditorUserSetup-x64-$env:RELEASE_VERSION-dev"
            # The mutex name here should match the mutex name in crates\zed\src\zed\windows_only_instance.rs
            $appMutex = "Zed-Dev-Instance-Mutex"
            $appExeName = "Zed"
            $regValueName = "ZedDev"
            $appUserId = "ZedIndustries.Zed.Dev"
            $appShellNameShort = "Z&ed Dev"
            $appAppxFullName = "ZedIndustries.Zed.Dev_1.0.0.0_neutral__japxn1gcva8rg"
        }
        default {
            Write-Error "can't bundle installer for $channel."
            exit 1
        }
    }

    # Windows runner 2022 default has iscc in PATH, https://github.com/actions/runner-images/blob/main/images/windows/Windows2022-Readme.md
    # Currently, we are using Windows 2022 runner.
    # Windows runner 2025 doesn't have iscc in PATH for now, https://github.com/actions/runner-images/issues/11228
    $innoSetupPath = "C:\Program Files (x86)\Inno Setup 6\ISCC.exe"

    $definitions = @{
        "AppId"          = $appId
        "AppIconName"    = $appIconName
        "OutputDir"      = "$env:ZED_WORKSPACE\target"
        "AppSetupName"   = $appSetupName
        "AppName"        = $appName
        "AppDisplayName" = $appDisplayName
        "RegValueName"   = $regValueName
        "AppMutex"       = $appMutex
        "AppExeName"     = $appExeName
        "ResourcesDir"   = "$innoDir"
        "ShellNameShort" = $appShellNameShort
        "AppUserId"      = $appUserId
        "Version"        = "$env:RELEASE_VERSION"
        "SourceDir"      = "$env:ZED_WORKSPACE"
        "AppxFullName"   = $appAppxFullName
    }

    $signTool = "powershell.exe -ExecutionPolicy Bypass -File $innoDir\sign.ps1 `$f"

    $defs = @()
    foreach ($key in $definitions.Keys) {
        $defs += "/d$key=`"$($definitions[$key])`""
    }

    $innoArgs = @($issFilePath) + $defs + "/sDefaultsign=`"$signTool`""

    # Execute Inno Setup
    Write-Host "🚀 Running Inno Setup: $innoSetupPath $innoArgs"
    $process = Start-Process -FilePath $innoSetupPath -ArgumentList $innoArgs -NoNewWindow -Wait -PassThru

    if ($process.ExitCode -eq 0) {
        Write-Host "✅ Inno Setup successfully compiled the installer"
        Write-Output "SETUP_PATH=target/$appSetupName.exe" >> $env:GITHUB_ENV
        $script:buildSuccess = $true
    }
    else {
        Write-Host "❌ Inno Setup failed: $($process.ExitCode)"
        $script:buildSuccess = $false
    }
}

ParseZedWorkspace
$innoDir = "$env:ZED_WORKSPACE\inno"
$debugArchive = ".\target\release\zed-$env:RELEASE_VERSION-$env:ZED_RELEASE_CHANNEL.dbg.zip"
$debugStoreKey = "$env:ZED_RELEASE_CHANNEL/zed-$env:RELEASE_VERSION-$env:ZED_RELEASE_CHANNEL.dbg.zip"

CheckEnvironmentVariables
PrepareForBundle
GenerateLicenses
BuildZedAndItsFriends
MakeAppx
SignZedAndItsFriends
ZipZedAndItsFriendsDebug
DownloadAMDGpuServices
DownloadConpty
CollectFiles
BuildInstaller

UploadToBlobStorePublic -BucketName "zed-debug-symbols" -FileToUpload $debugArchive -BlobStoreKey $debugStoreKey
UploadToSentry

if ($buildSuccess) {
    Write-Output "Build successful"
    if ($Install) {
        Write-Output "Installing Zed..."
        Start-Process -FilePath "$env:ZED_WORKSPACE/target/ZedEditorUserSetup-x64-$env:RELEASE_VERSION.exe"
    }
    exit 0
}
else {
    Write-Output "Build failed"
    exit 1
}
