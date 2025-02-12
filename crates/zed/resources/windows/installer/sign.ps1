param (
    [Parameter(Mandatory = $true)]
    [string]$FILE_PATH
)

$params = @{}

$endpoint = $ENV:ENDPOINT
if (-Not [string]::IsNullOrWhiteSpace($endpoint)) {
    $params["Endpoint"] = $endpoint
}

$trustedSigningAccountName = $ENV:ACCOUNT_NAME
if ([string]::IsNullOrWhiteSpace($trustedSigningAccountName)) {
    throw "The 'trusted-signing-account-name' input is required."
}
$params["CodeSigningAccountName"] = $trustedSigningAccountName

$certificateProfileName = $ENV:CERT_PROFILE_NAME
if (-Not [string]::IsNullOrWhiteSpace($certificateProfileName)) {
    $params["CertificateProfileName"] = $certificateProfileName
}

$params["Files"] = $FILE_PATH

$fileDigest = $ENV:FILE_DIGEST
if (-Not [string]::IsNullOrWhiteSpace($fileDigest)) {
    $params["FileDigest"] = $fileDigest
}

$timestampRfc3161 = $ENV:TIMESTAMP_SERVER
if (-Not [string]::IsNullOrWhiteSpace($timestampRfc3161)) {
    $params["TimestampRfc3161"] = $timestampRfc3161
}

$timestampDigest = $ENV:TIMESTAMP_DIGEST
if (-Not [string]::IsNullOrWhiteSpace($timestampDigest)) {
    $params["TimestampDigest"] = $timestampDigest
}

$trace = $ENV:TRACE
if (-Not [string]::IsNullOrWhiteSpace($trace)) {
    if ([System.Convert]::ToBoolean($trace)) {
        Set-PSDebug -Trace 2
    }
}

Invoke-TrustedSigning @params
