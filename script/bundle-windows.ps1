$ErrorActionPreference = "Stop"

$issFilePath = "$env:ZED_WORKSPACE/crates/zed/resources/windows/installer/zed.iss"
$channel = $env:RELEASE_CHANNEL

switch ($channel) {
    "stable" {
        $appId = "{{2DB0DA96-CA55-49BB-AF4F-64AF36A86712}"
        $appName = "Zed Editor"
        $appDisplayName = "Zed Editor (User)"
        $appSetupName = "ZedEditorUserSetup-x64-$env:RELEASE_VERSION"
        # The mutex name here should match the mutex name in crates\zed\src\zed\windows_only_instance.rs
        $appMutex = "Zed-Editor-Stable-Instance-Mutex"
        $appExeName = "Zed"
        $regValueName = "ZedEditor"
        $appUserId = "ZedIndustry.Zed"
        $appShellNameShort = "Z&ed Editor"
        # TODO: Update this value
        $appAppxFullName = "ZedIndusty.Zed_1.0.0.0_neutral__jr6ek54py7bac"
    }
    "preview" {
        $appId = "{{F70E4811-D0E2-4D88-AC99-D63752799F95}"
        $appName = "Zed Editor Preview"
        $appDisplayName = "Zed Editor Preview (User)"
        $appSetupName = "ZedEditorUserSetup-x64-$env:RELEASE_VERSION-preview"
        # The mutex name here should match the mutex name in crates\zed\src\zed\windows_only_instance.rs
        $appMutex = "Zed-Editor-Preview-Instance-Mutex"
        $appExeName = "Zed"
        $regValueName = "ZedEditorPreview"
        $appUserId = "ZedIndustry.Zed.Preview"
        $appShellNameShort = "Z&ed Editor Preview"
        # TODO: Update this value
        $appAppxFullName = "ZedIndusty.Zed.Preview_1.0.0.0_neutral__jr6ek54py7bac"
    }
    "nightly" {
        $appId = "{{1BDB21D3-14E7-433C-843C-9C97382B2FE0}"
        $appName = "Zed Editor Nightly"
        $appDisplayName = "Zed Editor Nightly (User)"
        $appSetupName = "ZedEditorUserSetup-x64-$env:RELEASE_VERSION-nightly"
        # The mutex name here should match the mutex name in crates\zed\src\zed\windows_only_instance.rs
        $appMutex = "Zed-Editor-Nightly-Instance-Mutex"
        $appExeName = "Zed"
        $regValueName = "ZedEditorNightly"
        $appUserId = "ZedIndustry.Zed.Nightly"
        $appShellNameShort = "Z&ed Editor Nightly"
        # TODO: Update this value
        $appAppxFullName = "ZedIndusty.Zed.Nightly_1.0.0.0_neutral__jr6ek54py7bac"
    }
    default {
        Write-Error "can't bundle installer for $channel"
        exit 1
    }
}

# Windows runner 2022 default has iscc in PATH, https://github.com/actions/runner-images/blob/main/images/windows/Windows2022-Readme.md
# Currently, we are using Windows 2022 runner.
# Windows runner 2025 doesn't have iscc in PATH for now, https://github.com/actions/runner-images/issues/11228
$innoSetupPath = "iscc.exe"

$definitions = @{
    "AppId"          = $appId
    "OutputDir"      = "$env:ZED_WORKSPACE/target"
    "AppSetupName"   = $appSetupName
    "AppName"        = $appName
    "AppDisplayName" = $appDisplayName
    "RegValueName"   = $regValueName
    "AppMutex"       = $appMutex
    "AppExeName"     = $appExeName
    "ResourcesDir"   = "$env:ZED_WORKSPACE/crates/zed/resources/windows"
    "ShellNameShort" = $appShellNameShort
    "AppUserId"      = $appUserId
    "Version"        = "$env:RELEASE_VERSION"
    "SourceDir"      = "$env:ZED_WORKSPACE"
    "AppxFullName"   = $appAppxFullName
}

$signTool = "pwsh.exe -ExecutionPolicy Bypass -File $env:ZED_WORKSPACE/crates/zed/resources/windows/installer/sign.ps1 `$f"

$defs = @()
foreach ($key in $definitions.Keys) {
    $defs += "/d$key=`"$($definitions[$key])`""
}

$innoArgs = @($issFilePath) + $innoFilePath + $defs + "/sDefaultsign=`"$signTool`""

# Execute Inno Setup
Write-Host "🚀 Running Inno Setup: $innoSetupPath $innoArgs"
$process = Start-Process -FilePath $innoSetupPath -ArgumentList $innoArgs -NoNewWindow -Wait -PassThru

if ($process.ExitCode -eq 0) {
    Write-Host "✅ Inno Setup successfully compiled the installer"
    Write-Output "SETUP_PATH=target/$appSetupName.exe" >> $env:GITHUB_ENV
    exit 0
}
else {
    Write-Host "❌ Inno Setup failed: $($process.ExitCode)"
    exit $process.ExitCode
}
