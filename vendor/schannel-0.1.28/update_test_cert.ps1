# ServicePointManager is removed in newer .NET versions: https://github.com/dotnet/runtime/issues/71836
if ($PSVersionTable.PSVersion.Major -ne 5) {
    throw 'This script requires Windows PowerShell, newer versions will not work'
}

# Setup a callback to always trust the certificate.
[System.Net.ServicePointManager]::ServerCertificateValidationCallback = { $true }

$webRequest = [Net.WebRequest]::Create("https://self-signed.badssl.com")
$webRequest.GetResponse() | Out-Null

$cert = $webRequest.ServicePoint.Certificate
[byte[]] $certBytes = $cert.Export([Security.Cryptography.X509Certificates.X509ContentType]::Cert)
[System.IO.File]::WriteAllBytes('.\test\self-signed.badssl.com.cer', $certBytes)

$thumbprint = (Get-PfxCertificate .\test\self-signed.badssl.com.cer).Thumbprint
[byte[]] $thumbprintBytes = -split ($thumbprint -replace '..', '0x$& ')
[System.IO.File]::WriteAllBytes('.\test\self-signed.badssl.com.cer.sha1', $thumbprintBytes)
