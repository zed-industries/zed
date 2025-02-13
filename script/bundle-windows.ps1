$ErrorActionPreference = "Stop"

$issFilePath = "$env:ZED_WORKSPACE/crates/zed/resources/windows/installer/zed.iss"
$channel = $env:RELEASE_CHANNEL

switch ($channel) {
    "stable" {
        $appId = "{{2DB0DA96-CA55-49BB-AF4F-64AF36A86712}"
        $appName = "Zed Editor"
        $appDisplayName = "Zed Editor (User)"
        $appSetupName = "ZedEditorUserSetup-x64-$env:RELEASE_VERSION"
        $appMutex = "ZedSetupMutex" # TODO:
        $appExeName = "zed"
        $regValueName = "ZedEditor"
        $appUserId = "ZedIndustry.Zed"
        $appShellNameShort = "Z&ed Editor"
    }
    "preview" {
        $appId = "{{F70E4811-D0E2-4D88-AC99-D63752799F95}"
        $appName = "Zed Editor Preview"
        $appDisplayName = "Zed Editor Preview (User)"
        $appSetupName = "ZedEditorUserSetup-x64-$env:RELEASE_VERSION-preview"
        $appMutex = "ZedSetupMutex" # TODO:
        $appExeName = "zed"
        $regValueName = "ZedEditorPreview"
        $appUserId = "ZedIndustry.Zed.Preview"
        $appShellNameShort = "Z&ed Editor Preview"
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
