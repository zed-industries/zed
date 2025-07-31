$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

$env:POWERSHELL = $true

if (!(Get-Command sqlx -ErrorAction SilentlyContinue) -or (sqlx --version) -notlike "sqlx-cli 0.7.2") {
    Write-Output "sqlx-cli not found or not the required version, installing version 0.7.2..."
    cargo install sqlx-cli --version 0.7.2
}

Set-Location .\crates\collab

# Export contents of .env.toml
$env = (cargo run --bin dotenv) -join "`n";
Invoke-Expression $env

Set-Location ../..

Write-Output "creating databases..."
sqlx database create --database-url "$env:DATABASE_URL"
sqlx database create --database-url "$env:LLM_DATABASE_URL"
