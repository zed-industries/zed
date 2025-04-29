function UploadToBlobStoreWithACL {
    param (
        [string]$BucketName,
        [string]$FileToUpload,
        [string]$BlobStoreKey,
        [string]$ACL
    )

    # Format date to match AWS requirements
    $Date = (Get-Date).ToUniversalTime().ToString("r")
    # Note: Original script had a bug where it overrode the ACL parameter
    # I'm keeping the same behavior for compatibility
    $ACL = "x-amz-acl:public-read"
    $ContentType = "application/octet-stream"
    $StorageType = "x-amz-storage-class:STANDARD"
    
    # Create string to sign
    $StringToSign = "PUT`n`n${ContentType}`n${Date}`n${ACL}`n${StorageType}`n/${BucketName}/${BlobStoreKey}"
    
    # Generate HMAC-SHA1 signature
    $HMACSHA1 = New-Object System.Security.Cryptography.HMACSHA1
    $HMACSHA1.Key = [System.Text.Encoding]::UTF8.GetBytes($env:DIGITALOCEAN_SPACES_SECRET_KEY)
    $Signature = [System.Convert]::ToBase64String($HMACSHA1.ComputeHash([System.Text.Encoding]::UTF8.GetBytes($StringToSign)))
    
    # Upload file using Invoke-WebRequest (equivalent to curl)
    $Headers = @{
        "Host" = "${BucketName}.nyc3.digitaloceanspaces.com"
        "Date" = $Date
        "Content-Type" = $ContentType
        $StorageType = ""
        $ACL = ""
        "Authorization" = "AWS ${env:DIGITALOCEAN_SPACES_ACCESS_KEY}:$Signature"
    }
    
    $Uri = "https://${BucketName}.nyc3.digitaloceanspaces.com/${BlobStoreKey}"
    
    # Read file content
    $FileContent = Get-Content $FileToUpload -Raw -Encoding Byte
    
    try {
        Invoke-WebRequest -Uri $Uri -Method PUT -Headers $Headers -Body $FileContent -ContentType $ContentType -Verbose
        Write-Host "Successfully uploaded $FileToUpload to $Uri" -ForegroundColor Green
    }
    catch {
        Write-Error "Failed to upload file: $_"
        throw $_
    }
}

function UploadToBlobStorePublic {
    param (
        [string]$BucketName,
        [string]$FileToUpload,
        [string]$BlobStoreKey
    )
    
    UploadToBlobStoreWithACL -BucketName $BucketName -FileToUpload $FileToUpload -BlobStoreKey $BlobStoreKey -ACL "x-amz-acl:public-read"
}

function UploadToBlobStore {
    param (
        [string]$BucketName,
        [string]$FileToUpload,
        [string]$BlobStoreKey
    )
    
    UploadToBlobStoreWithACL -BucketName $BucketName -FileToUpload $FileToUpload -BlobStoreKey $BlobStoreKey -ACL "x-amz-acl:private"
}
