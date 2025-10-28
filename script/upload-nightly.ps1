[CmdletBinding()]
Param(
    [Parameter()][string]$Architecture
)

# Based on the template in: https://docs.digitalocean.com/reference/api/spaces-api/
$ErrorActionPreference = "Stop"
. "$PSScriptRoot\lib\blob-store.ps1"
. "$PSScriptRoot\lib\workspace.ps1"

ParseZedWorkspace
Write-Host "Uploading nightly for target: $target"

$bucketName = "zed-nightly-host"

# Get current git SHA
$sha = git rev-parse HEAD
$sha | Out-File -FilePath "target/latest-sha" -NoNewline

# TODO:
# Upload remote server files
# $remoteServerFiles = Get-ChildItem -Path "target" -Filter "zed-remote-server-*.gz" -Recurse -File
# foreach ($file in $remoteServerFiles) {
#     Upload-ToBlobStore -BucketName $bucketName -FileToUpload $file.FullName -BlobStoreKey "nightly/$($file.Name)"
#     Remove-Item -Path $file.FullName
# }

UploadToBlobStore -BucketName $bucketName -FileToUpload "target/Zed-$Architecture.exe" -BlobStoreKey "nightly/Zed-$Architecture.exe"
UploadToBlobStore -BucketName $bucketName -FileToUpload "target/latest-sha" -BlobStoreKey "nightly/latest-sha-windows"

Remove-Item -Path "target/Zed-$Architecture.exe" -ErrorAction SilentlyContinue
Remove-Item -Path "target/latest-sha" -ErrorAction SilentlyContinue
