param (
    [Parameter(Mandatory = $true)]
    [string]$FILE_PATH
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

$params["Files"] = $FILE_PATH
$params["FileDigest"] = "SHA256"
$params["TimestampRfc3161"] = "http://timestamp.acs.microsoft.com"
$params["TimestampDigest"] = "SHA256"

$trace = $ENV:TRACE
if (-Not [string]::IsNullOrWhiteSpace($trace)) {
    if ([System.Convert]::ToBoolean($trace)) {
        Set-PSDebug -Trace 2
    }
}

Invoke-TrustedSigning @params
