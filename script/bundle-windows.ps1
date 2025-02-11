# Inno Setup executable path
$innoSetupPath = "C:\zjk\apps\Inno Setup 6\ISCC.exe"
$innoFilePath = ".\crates\zed\resources\windows\installer\zed.iss"
$signToolPath = "C:\zjk\projects\zed\crates\zed\resources\windows\installer\sign.cmd"

$product = @{
    "nameLong"        = "zed"
    "nameShort"       = "zed"
    "DirName"         = "zed"
    "RegValueName"    = "zed"
    "ShellNameShort"  = "&Zed Editor"
    "MutexName"       = "ZedSetupMutex" # TODO:
    "applicationName" = "Zed"
    "AppUserModelId"  = "ZedIndustry.Zed"
    # "RepoDir"         = ".\crates\zed\resources\windows\installer"
    "RepoDir"         = "C:\zjk\projects\zed\crates\zed\resources\windows\installer"
    ”AppId"           = "{{2DB0DA96-CA55-49BB-AF4F-64AF36A86712}"
}

$sourcePath = ".\crates\zed\resources\windows\installer\"
$outputPath = "C:\zjk\projects\zed\target\windows"
New-Item -ItemType Directory -Force -Path $outputPath | Out-Null

$definitions = @{
    "NameLong"        = $product.nameLong
    "NameShort"       = $product.nameShort
    "DirName"         = $product.DirName
    "Version"         = "1.0.0"
    "RawVersion"      = "1.0.0"
    "ExeBasename"     = $product.nameShort
    "RegValueName"    = $product.RegValueName
    "ShellNameShort"  = $product.ShellNameShort
    "AppMutex"        = $product.MutexName
    "ApplicationName" = $product.applicationName
    "SourceDir"       = $sourcePath
    "OutputDir"       = $outputPath
    "RepoDir"         = $product.RepoDir
    "AppId"           = $product.AppId
    "AppUserId"       = $product.AppUserModelId
    "InstallTarget"   = "user"
    "signToolPath"    = $signToolPath
}

$defs = @()
foreach ($key in $definitions.Keys) {
    $defs += "/d$key=`"$($definitions[$key])`""
}

$innoArgs = @($issPath) + $innoFilePath + $defs + "/sDefaultsign=`"$signToolPath `$f`""

# Execute Inno Setup
Write-Host "🚀 Running Inno Setup: $innoSetupPath $innoArgs"
$process = Start-Process -FilePath $innoSetupPath -ArgumentList $innoArgs -NoNewWindow -Wait -PassThru

if ($process.ExitCode -eq 0) {
    Write-Host "✅ Inno Setup successfully compiled the installer"
    exit 0
}
else {
    Write-Host "❌ Inno Setup failed: $($process.ExitCode)"
    exit $process.ExitCode
}
