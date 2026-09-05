#!/usr/bin/env pwsh
# Exercise GPUI's real Windows vsync and renderer-recovery path without changing
# the installed display driver. Each scenario launches an existing GPUI example,
# waits for its structured recovery result, verifies that the process is still
# alive, and stops only that exact child process.
#
# This test needs an interactive Windows desktop and therefore is not part of
# the parallel nextest suite.
#
# Usage:
#   ./script/test-gpui-windows-device-loss.ps1
#   ./script/test-gpui-windows-device-loss.ps1 -NoBuild
#   ./script/test-gpui-windows-device-loss.ps1 -EvidenceDirectory D:/tmp/gpui-device-loss

[CmdletBinding()]
param(
    [switch]$NoBuild,
    [ValidateRange(10, 120)]
    [int]$TimeoutSeconds = 35,
    [string]$EvidenceDirectory
)

$ErrorActionPreference = "Stop"

if (-not $IsWindows) {
    throw "This integration test requires Windows."
}

$repoRoot = Split-Path -Parent $PSScriptRoot
$cargo = if ($env:CARGO) { $env:CARGO } else { "cargo" }
$environmentNames = @(
    "GPUI_TEST_DEVICE_LOSS_AT_VSYNCS",
    "GPUI_TEST_DEVICE_RECOVERY_FAILURE",
    "GPUI_DISABLE_DIRECT_COMPOSITION"
)

function Set-ScenarioEnvironment {
    param(
        [Parameter(Mandatory = $true)]$Scenario
    )

    $env:GPUI_TEST_DEVICE_LOSS_AT_VSYNCS = $Scenario.DeviceLossVsyncs
    $env:GPUI_TEST_DEVICE_RECOVERY_FAILURE = $Scenario.RecoveryFailure
    if ($Scenario.DisableDirectComposition) {
        $env:GPUI_DISABLE_DIRECT_COMPOSITION = "true"
    }
    else {
        Remove-Item Env:GPUI_DISABLE_DIRECT_COMPOSITION -ErrorAction SilentlyContinue
    }
}

function Restore-Environment {
    param(
        [Parameter(Mandatory = $true)][hashtable]$OriginalEnvironment
    )

    foreach ($name in $environmentNames) {
        $value = $OriginalEnvironment[$name]
        if ($null -eq $value) {
            Remove-Item "Env:$name" -ErrorAction SilentlyContinue
        }
        else {
            Set-Item "Env:$name" $value
        }
    }
}

function Test-ExpectedPatterns {
    param(
        [Parameter(Mandatory = $true)][AllowEmptyString()][string]$Log,
        [Parameter(Mandatory = $true)][string[]]$ExpectedPatterns
    )

    foreach ($pattern in $ExpectedPatterns) {
        if (-not [regex]::IsMatch($Log, $pattern)) {
            return $false
        }
    }
    return $true
}

function Get-InjectedFailurePatterns {
    param(
        [Parameter(Mandatory = $true)][string]$Stage,
        [Parameter(Mandatory = $true)][int]$Count,
        [int[]]$Generations = @(1)
    )

    $escapedStage = [regex]::Escape($Stage)
    foreach ($generation in $Generations) {
        foreach ($attempt in 1..$Count) {
            "generation=$generation stage=$escapedStage attempt=$attempt result=injected_failure"
        }
    }
}

function Test-ExpectedActiveWindows {
    param(
        [Parameter(Mandatory = $true)][AllowEmptyString()][string]$Log,
        [Parameter(Mandatory = $true)][int]$Generation,
        [Parameter(Mandatory = $true)][int]$ExpectedCount,
        [Parameter(Mandatory = $true)][int[]]$ExpectedAttempts
    )

    $windowResults = [regex]::Matches(
        $Log,
        "generation=$Generation window=(?<window>HWND\([^)]+\)) attempt=(?<attempt>\d+) result=active"
    )
    $windows = @(
        $windowResults |
            ForEach-Object { $_.Groups["window"].Value } |
            Sort-Object -Unique
    )
    $attempts = @(
        $windowResults |
            ForEach-Object { [int]$_.Groups["attempt"].Value } |
            Sort-Object
    )
    $expectedAttemptsSorted = @($ExpectedAttempts | Sort-Object)

    return $windowResults.Count -eq $ExpectedCount `
        -and $windows.Count -eq $ExpectedCount `
        -and ($attempts -join ",") -eq ($expectedAttemptsSorted -join ",")
}

function Test-ExpectedPresentedWindows {
    param(
        [Parameter(Mandatory = $true)][AllowEmptyString()][string]$Log,
        [Parameter(Mandatory = $true)][int]$Generation,
        [Parameter(Mandatory = $true)][int]$ExpectedCount
    )

    $presentResults = [regex]::Matches(
        $Log,
        "generation=$Generation window=(?<window>HWND\([^)]+\)) result=presented"
    )
    $windows = @(
        $presentResults |
            ForEach-Object { $_.Groups["window"].Value } |
            Sort-Object -Unique
    )
    return $presentResults.Count -eq $ExpectedCount -and $windows.Count -eq $ExpectedCount
}

Push-Location $repoRoot
try {
    if (-not $NoBuild) {
        Write-Host "==> Building gpui device-loss examples"
        & $cargo build -p gpui --example hello_world --example on_window_close_quit
        if ($LASTEXITCODE -ne 0) {
            throw "cargo build failed with exit code $LASTEXITCODE"
        }
    }

    $metadata = & $cargo metadata --format-version 1 --no-deps | ConvertFrom-Json
    if ($LASTEXITCODE -ne 0) {
        throw "cargo metadata failed with exit code $LASTEXITCODE"
    }
    $examplePaths = @{
        hello_world = Join-Path $metadata.target_directory "debug/examples/hello_world.exe"
        on_window_close_quit = Join-Path $metadata.target_directory "debug/examples/on_window_close_quit.exe"
    }
    foreach ($examplePath in $examplePaths.Values) {
        if (-not (Test-Path -LiteralPath $examplePath)) {
            throw "GPUI example was not found at $examplePath; run without -NoBuild"
        }
    }

    if ([string]::IsNullOrWhiteSpace($EvidenceDirectory)) {
        $EvidenceDirectory = Join-Path ([System.IO.Path]::GetTempPath()) (
            "zed-gpui-device-loss-" + [guid]::NewGuid().ToString("N")
        )
    }
    $EvidenceDirectory = [System.IO.Path]::GetFullPath($EvidenceDirectory)
    New-Item -ItemType Directory -Path $EvidenceDirectory -Force | Out-Null

    $singleFailureStages = @(
        "renderer-devices",
        "resources",
        "global-elements",
        "pipelines",
        "direct-composition",
        "composition-swapchain",
        "atlas-commit"
    )
    $scenarios = @(
        foreach ($stage in $singleFailureStages) {
            [pscustomobject]@{
                Name = "recover-after-one-$stage-failure"
                Example = "hello_world"
                DeviceLossVsyncs = "30"
                RecoveryFailure = "${stage}:1"
                DisableDirectComposition = $false
                ExpectedPatterns = @(
                    Get-InjectedFailurePatterns -Stage $stage -Count 1
                    "generation=1 window=.* attempt=2 result=active"
                    "generation=1 window=.* result=presented"
                )
            }
        }
        [pscustomobject]@{
            Name = "recover-two-windows"
            Example = "on_window_close_quit"
            DeviceLossVsyncs = "30"
            RecoveryFailure = "resources:1"
            DisableDirectComposition = $false
            ExpectedPatterns = @(
                Get-InjectedFailurePatterns -Stage "resources" -Count 1
            )
            ExpectedActiveWindowCount = 2
            ExpectedActiveAttempts = @(1, 2)
            ExpectedPresentedWindowCount = 2
        },
        [pscustomobject]@{
            Name = "recover-on-final-attempt"
            Example = "hello_world"
            DeviceLossVsyncs = "30"
            RecoveryFailure = "resources:7"
            DisableDirectComposition = $false
            ExpectedPatterns = @(
                Get-InjectedFailurePatterns -Stage "resources" -Count 7
                "generation=1 window=.* attempt=8 result=active"
                "generation=1 window=.* result=presented"
            )
        },
        [pscustomobject]@{
            Name = "exhaust-without-abort"
            Example = "hello_world"
            DeviceLossVsyncs = "30"
            RecoveryFailure = "resources:8"
            DisableDirectComposition = $false
            ExpectedPatterns = @(
                Get-InjectedFailurePatterns -Stage "resources" -Count 8
                "generation=1 window=.* attempt=8 result=exhausted"
            )
        },
        [pscustomobject]@{
            Name = "reset-budget-for-next-generation"
            Example = "hello_world"
            DeviceLossVsyncs = "30,180"
            RecoveryFailure = "resources:1"
            DisableDirectComposition = $false
            ExpectedPatterns = @(
                Get-InjectedFailurePatterns -Stage "resources" -Count 1 -Generations @(1, 2)
                "generation=1 window=.* attempt=2 result=active",
                "generation=2 window=.* attempt=2 result=active",
                "generation=1 window=.* result=presented",
                "generation=2 window=.* result=presented"
            )
        },
        [pscustomobject]@{
            Name = "recover-without-direct-composition"
            Example = "hello_world"
            DeviceLossVsyncs = "30"
            RecoveryFailure = "resources:1"
            DisableDirectComposition = $true
            ExpectedPatterns = @(
                Get-InjectedFailurePatterns -Stage "resources" -Count 1
                "generation=1 window=.* attempt=2 result=active"
                "generation=1 window=.* result=presented"
            )
        }
    )

    $originalEnvironment = @{}
    foreach ($name in $environmentNames) {
        $originalEnvironment[$name] = [Environment]::GetEnvironmentVariable(
            $name,
            [EnvironmentVariableTarget]::Process
        )
    }

    $results = [System.Collections.Generic.List[object]]::new()
    foreach ($scenario in $scenarios) {
        $standardOutputPath = Join-Path $EvidenceDirectory "$($scenario.Name).stdout.log"
        $standardErrorPath = Join-Path $EvidenceDirectory "$($scenario.Name).stderr.log"
        $process = $null
        $matched = $false
        $startedAt = [DateTimeOffset]::UtcNow

        try {
            Set-ScenarioEnvironment $scenario
            Write-Host "==> $($scenario.Name)"
            $process = Start-Process `
                -FilePath $examplePaths[$scenario.Example] `
                -RedirectStandardOutput $standardOutputPath `
                -RedirectStandardError $standardErrorPath `
                -PassThru

            $deadline = [DateTimeOffset]::UtcNow.AddSeconds($TimeoutSeconds)
            while ([DateTimeOffset]::UtcNow -lt $deadline) {
                $log = ""
                if (Test-Path -LiteralPath $standardErrorPath) {
                    # Normalize any pipeline values before binding to
                    # Test-ExpectedPatterns' string parameter.
                    $log = @(
                        Get-Content -Raw -LiteralPath $standardErrorPath -ErrorAction SilentlyContinue
                    ) -join [Environment]::NewLine
                }
                $matched = Test-ExpectedPatterns $log $scenario.ExpectedPatterns
                if (
                    $matched `
                    -and $null -ne $scenario.PSObject.Properties["ExpectedActiveWindowCount"]
                ) {
                    $matched = Test-ExpectedActiveWindows `
                        -Log $log `
                        -Generation 1 `
                        -ExpectedCount $scenario.ExpectedActiveWindowCount `
                        -ExpectedAttempts $scenario.ExpectedActiveAttempts
                }
                if (
                    $matched `
                    -and $null -ne $scenario.PSObject.Properties["ExpectedPresentedWindowCount"]
                ) {
                    $matched = Test-ExpectedPresentedWindows `
                        -Log $log `
                        -Generation 1 `
                        -ExpectedCount $scenario.ExpectedPresentedWindowCount
                }
                if ($matched) {
                    break
                }
                if ($process.HasExited) {
                    break
                }
                Start-Sleep -Milliseconds 100
            }

            $elapsedMilliseconds = [int]([DateTimeOffset]::UtcNow - $startedAt).TotalMilliseconds
            if (-not $matched) {
                $exitDescription = if ($process.HasExited) {
                    "exited with code $($process.ExitCode)"
                }
                else {
                    "did not report the expected result within $TimeoutSeconds seconds"
                }
                throw "$($scenario.Name) $exitDescription; evidence: $standardErrorPath"
            }
            if ($process.HasExited) {
                throw "$($scenario.Name) reported success but then exited with code $($process.ExitCode)"
            }

            $results.Add([pscustomobject]@{
                scenario = $scenario.Name
                example = $scenario.Example
                device_loss_vsyncs = $scenario.DeviceLossVsyncs
                recovery_failure = $scenario.RecoveryFailure
                direct_composition = -not $scenario.DisableDirectComposition
                elapsed_ms = $elapsedMilliseconds
                alive_at_result = $true
                result = "passed"
            })
            Write-Host "    passed in $elapsedMilliseconds ms; process alive at result"
        }
        finally {
            try {
                if ($null -ne $process -and -not $process.HasExited) {
                    try {
                        $process.Kill()
                    }
                    catch {
                        if (-not $process.HasExited) {
                            throw
                        }
                    }
                    if (-not $process.WaitForExit(5000)) {
                        throw "failed to stop owned $($scenario.Example) process $($process.Id)"
                    }
                }
            }
            finally {
                Restore-Environment $originalEnvironment
            }
        }
    }

    $resultPath = Join-Path $EvidenceDirectory "results.json"
    $results | ConvertTo-Json -Depth 4 | Set-Content -LiteralPath $resultPath -Encoding utf8NoBOM
    Write-Host "==> All GPUI Windows device-loss scenarios passed"
    Write-Host "    Evidence: $resultPath"
}
finally {
    Pop-Location
}
