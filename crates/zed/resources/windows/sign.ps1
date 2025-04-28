param (
    [Parameter(Mandatory = $true)]
    [string]$filePath
)

$params = @{}

$endpoint = $ENV:ENDPOINT
if ([string]::IsNullOrWhiteSpace($endpoint)) {
    throw "The 'ENDPOINT' env is required."
}
$params["Endpoint"] = $endpoint

$trustedSigningAccountName = $ENV:ACCOUNT_NAME
if ([string]::IsNullOrWhiteSpace($trustedSigningAccountName)) {
    throw "The 'ACCOUNT_NAME' env is required."
}
$params["CodeSigningAccountName"] = $trustedSigningAccountName

$certificateProfileName = $ENV:CERT_PROFILE_NAME
if ([string]::IsNullOrWhiteSpace($certificateProfileName)) {
    throw "The 'CERT_PROFILE_NAME' env is required."
}
$params["CertificateProfileName"] = $certificateProfileName

$fileDigest = $ENV:FILE_DIGEST
if ([string]::IsNullOrWhiteSpace($fileDigest)) {
    throw "The 'FILE_DIGEST' env is required."
}
$params["FileDigest"] = $fileDigest

$timeStampDigest = $ENV:TIMESTAMP_DIGEST
if ([string]::IsNullOrWhiteSpace($timeStampDigest)) {
    throw "The 'TIMESTAMP_DIGEST' env is required."
}
$params["TimestampDigest"] = $timeStampDigest

$timeStampServer = $ENV:TIMESTAMP_SERVER
if ([string]::IsNullOrWhiteSpace($timeStampServer)) {
    throw "The 'TIMESTAMP_SERVER' env is required."
}
$params["TimestampRfc3161"] = $timeStampServer

$params["Files"] = $filePath

$trace = $ENV:TRACE
if (-Not [string]::IsNullOrWhiteSpace($trace)) {
    if ([System.Convert]::ToBoolean($trace)) {
        Set-PSDebug -Trace 2
    }
}

Invoke-TrustedSigning @params
