# Based on the template in: https://docs.digitalocean.com/reference/api/spaces-api/
$ErrorActionPreference = "Stop"
. "$PSScriptRoot\lib\blob-store.ps1"

$allowedTargets = @("windows")

function Test-AllowedTarget {
    param (
        [string]$Target
    )
    
    return $allowedTargets -contains $Target
}

# Process arguments
if ($args.Count -gt 0) {
    $target = $args[0]
    if (Test-AllowedTarget $target) {
        # Valid target
    } else {
        Write-Error "Error: Target '$target' is not allowed.`nUsage: $($MyInvocation.MyCommand.Name) [$($allowedTargets -join ', ')]"
        exit 1
    }
} else {
    Write-Error "Error: Target is not specified.`nUsage: $($MyInvocation.MyCommand.Name) [$($allowedTargets -join ', ')]"
    exit 1
}

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

switch ($target) {
    "windows" {
        UploadToBlobStore -BucketName $bucketName -FileToUpload "target/ZedEditorInstaller.exe" -BlobStoreKey "nightly/ZedEditorInstaller.exe"
        UploadToBlobStore -BucketName $bucketName -FileToUpload "target/latest-sha" -BlobStoreKey "nightly/latest-sha"
        
        Remove-Item -Path "target/ZedEditorInstaller.exe" -ErrorAction SilentlyContinue
        Remove-Item -Path "target/latest-sha" -ErrorAction SilentlyContinue
    }
    
    default {
        Write-Error "Error: Unknown target '$target'"
        exit 1
    }
}
