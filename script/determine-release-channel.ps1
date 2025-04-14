$ErrorActionPreference = "Stop"

if (-not $env:GITHUB_ACTIONS) {
    Write-Error "Error: This script must be run in a GitHub Actions environment"
    exit 1
}

if (-not $env:GITHUB_REF) {
    Write-Error "Error: GITHUB_REF is not set"
    exit 1
}

$version = & "script/get-crate-version.ps1" "zed"
$channel = Get-Content "crates/zed/RELEASE_CHANNEL"

Write-Host "Publishing version: $version on release channel $channel"
Write-Output "RELEASE_CHANNEL=$channel" >> $env:GITHUB_ENV
Write-Output "RELEASE_VERSION=$version" >> $env:GITHUB_ENV

$expectedTagName = ""
switch ($channel) {
    "stable" {
        $expectedTagName = "v$version"
    }
    "preview" {
        $expectedTagName = "v$version-pre"
    }
    default {
        Write-Error "can't publish a release on channel $channel"
        exit 1
    }
}

if ($env:GITHUB_REF_NAME -ne $expectedTagName) {
    Write-Error "invalid release tag $($env:GITHUB_REF_NAME). expected $expectedTagName"
    exit 1
}
