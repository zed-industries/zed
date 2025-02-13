$ErrorActionPreference = "Stop"

$channel = $env:RELEASE_CHANNEL

$appId = ""
$appName = "Zed Editor"
$appDisplayName = "Zed Editor"
$appSetupName = ""
$appMutex = "ZedSetupMutex" # TODO:
$appExeName = "zed"
$regValueName = "ZedEditor"
$appUserId = "ZedIndustry.Zed"

switch ($channel) {
    "stable" {
        $appId = "{{2DB0DA96-CA55-49BB-AF4F-64AF36A86712}"
        $appSetupName = "ZedEditorUserSetup-x64-$env:RELEASE_VERSION.exe"
    }
    "preview" {
        $appId = "{{F70E4811-D0E2-4D88-AC99-D63752799F95}"
        $appName = $appName + " Preview"
        $appDisplayName = $appDisplayName + " Preview"
        $appSetupName = "ZedEditorUserSetup-x64-$env:RELEASE_VERSION-preview.exe"
        $regValueName = $regValueName + "Preview"
        $appUserId = $appUserId + ".Preview"
    }
    default {
        Write-Error "can't bundle installer for $channel"
        exit 1
    }
}

$appDisplayName = $appDisplayName + " (User)"

Write-Output "APP_ID=$appId" >> $env:GITHUB_ENV
Write-Output "APP_NAME=$appName" >> $env:GITHUB_ENV
Write-Output "APP_DISPLAY_NAME=$appDisplayName" >> $env:GITHUB_ENV
Write-Output "APP_SETUP_NAME=$appSetupName" >> $env:GITHUB_ENV
Write-Output "APP_MUTEX=$appMutex" >> $env:GITHUB_ENV
Write-Output "APP_EXE_NAME=$appExeName" >> $env:GITHUB_ENV
Write-Output "REG_VALUE_NAME=$regValueName" >> $env:GITHUB_ENV
Write-Output "APP_USER_ID=$appUserId" >> $env:GITHUB_ENV
