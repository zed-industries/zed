$CARGO_ABOUT_VERSION="0.7"
$outputFile=$args[0] ? $args[0] : "$(Get-Location)/assets/licenses.md"
$templateFile="script/licenses/template.md.hbs"

New-Item -Path "$outputFile" -ItemType File -Value "" -Force

@(
    "# ###### THEME LICENSES ######\n"
    Get-Content assets/themes/LICENSES
    "\n# ###### ICON LICENSES ######\n"
    Get-Content assets/icons/LICENSES
    "\n# ###### CODE LICENSES ######\n"
) | Add-Content -Path $outputFile

$versionOutput = cargo about --version
if (-not ($versionOutput -match "cargo-about $CARGO_ABOUT_VERSION")) {
    Write-Host "Installing cargo-about@^$CARGO_ABOUT_VERSION..."
    cargo install "cargo-about@^$CARGO_ABOUT_VERSION"
} else {
    Write-Host "cargo-about@^$CARGO_ABOUT_VERSION" is already installed
}

Write-Host "Generating cargo licenses"

$failFlag = $env:ALLOW_MISSING_LICENSES ? "--fail" : ""
$args = @('about', 'generate', $failFlag, '-c', 'script/licenses/zed-licenses.toml', $templateFile, '-o', $outputFile) | Where-Object { $_ }
cargo @args

Write-Host "Applying replacements"
$replacements = @{
    '&quot;' = '"'
    '&#x27;' = "'"
    '&#x3D;' = '='
    '&#x60;' = '`'
    '&lt;'   = '<'
    '&gt;'   = '>'
}
$content = Get-Content $outputFile
foreach ($find in $replacements.keys) {
    $content = $content -replace $find, $replacements[$find]
}
$content | Set-Content $outputFile

Write-Host "generate-licenses completed. See $outputFile"
