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

# $files = $ENV:files
# if (-Not [string]::IsNullOrWhiteSpace($files)) {
$params["Files"] = $FILE_PATH
# }

$filesFolder = "${$ENV:files-folder}"
if (-Not [string]::IsNullOrWhiteSpace($filesFolder)) {
    $params["FilesFolder"] = $filesFolder
}

$filesFolderFilter = "${$ENV:files-folder-filter}"
if (-Not [string]::IsNullOrWhiteSpace($filesFolderFilter)) {
    $params["FilesFolderFilter"] = $filesFolderFilter
}

$filesFolderRecurse = "${$ENV:files-folder-recurse}"
if (-Not [string]::IsNullOrWhiteSpace($filesFolderRecurse)) {
    $params["FilesFolderRecurse"] = [System.Convert]::ToBoolean($filesFolderRecurse)
}

$filesFolderDepth = "${$ENV:files-folder-depth}"
if (-Not [string]::IsNullOrWhiteSpace($filesFolderDepth)) {
    $params["FilesFolderDepth"] = [System.Convert]::ToInt32($filesFolderDepth)
}

$filesCatalog = "${$ENV:files-catalog}"
if (-Not [string]::IsNullOrWhiteSpace($filesCatalog)) {
    $params["FilesCatalog"] = $filesCatalog
}

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

$appendSignature = "${$ENV:append-signature}"
if (-Not [string]::IsNullOrWhiteSpace($appendSignature)) {
    $params["AppendSignature"] = [System.Convert]::ToBoolean($appendSignature)
}

$description = "${$ENV:description}"
if (-Not [string]::IsNullOrWhiteSpace($description)) {
    $params["Description"] = $description
}

$descriptionUrl = "${$ENV:description-url}"
if (-Not [string]::IsNullOrWhiteSpace($descriptionUrl)) {
    $params["DescriptionUrl"] = $descriptionUrl
}

$generateDigestPath = "${$ENV:generate-digest-path}"
if (-Not [string]::IsNullOrWhiteSpace($generateDigestPath)) {
    $params["GenerateDigestPath"] = $generateDigestPath
}

$generateDigestXml = "${$ENV:generateDigestXml}"
if (-Not [string]::IsNullOrWhiteSpace($generateDigestXml)) {
    $params["GenerateDigestXml"] = [System.Convert]::ToBoolean($generateDigestXml)
}

$ingestDigestPath = "${$ENV:ingest-digest-path}"
if (-Not [string]::IsNullOrWhiteSpace($ingestDigestPath)) {
    $params["IngestDigestPath"] = $ingestDigestPath
}

$signDigest = "${$ENV:sign-digest}"
if (-Not [string]::IsNullOrWhiteSpace($signDigest)) {
    $params["SignDigest"] = [System.Convert]::ToBoolean($signDigest)
}

$generatePageHashes = "${$ENV:generate-page-hashes}"
if (-Not [string]::IsNullOrWhiteSpace($generatePageHashes)) {
    $params["GeneratePageHashes"] = [System.Convert]::ToBoolean($generatePageHashes)
}

$suppressPageHashes = "${$ENV:suppress-page-hashes}"
if (-Not [string]::IsNullOrWhiteSpace($suppressPageHashes)) {
    $params["SuppressPageHashes"] = [System.Convert]::ToBoolean($suppressPageHashes)
}

$generatePkcs7 = "${$ENV:generate-pkcs7}"
if (-Not [string]::IsNullOrWhiteSpace($generatePkcs7)) {
    $params["GeneratePkcs7"] = [System.Convert]::ToBoolean($generatePkcs7)
}

$pkcs7Options = "${$ENV:pkcs7-options}"
if (-Not [string]::IsNullOrWhiteSpace($pkcs7Options)) {
    $params["Pkcs7Options"] = $pkcs7Options
}

$pkcs7Oid = "${$ENV:pkcs7-oid}"
if (-Not [string]::IsNullOrWhiteSpace($pkcs7Oid)) {
    $params["Pkcs7Oid"] = $pkcs7Oid
}

$enhancedKeyUsage = "${$ENV:enhanced-key-usage}"
if (-Not [string]::IsNullOrWhiteSpace($enhancedKeyUsage)) {
    $params["EnhancedKeyUsage"] = $enhancedKeyUsage
}

$excludeEnvironmentCredential = $ENV:EXCLUDE_ENV_CREDS
if (-Not [string]::IsNullOrWhiteSpace($excludeEnvironmentCredential)) {
    $params["ExcludeEnvironmentCredential"] = [System.Convert]::ToBoolean($excludeEnvironmentCredential)
}

$excludeWorkloadIdentityCredential = "${$ENV:exclude-workload-identity-credential}"
if (-Not [string]::IsNullOrWhiteSpace($excludeWorkloadIdentityCredential)) {
    $params["ExcludeWorkloadIdentityCredential"] = [System.Convert]::ToBoolean($excludeWorkloadIdentityCredential)
}

$excludeManagedIdentityCredential = "${$ENV:exclude-managed-identity-credential}"
if (-Not [string]::IsNullOrWhiteSpace($excludeManagedIdentityCredential)) {
    $params["ExcludeManagedIdentityCredential"] = [System.Convert]::ToBoolean($excludeManagedIdentityCredential)
}

$excludeSharedTokenCacheCredential = "${$ENV:exclude-shared-token-cache-credential}"
if (-Not [string]::IsNullOrWhiteSpace($excludeSharedTokenCacheCredential)) {
    $params["ExcludeSharedTokenCacheCredential"] = [System.Convert]::ToBoolean($excludeSharedTokenCacheCredential)
}

$excludeVisualStudioCredential = "${$ENV:exclude-visual-studio-credential}"
if (-Not [string]::IsNullOrWhiteSpace($excludeVisualStudioCredential)) {
    $params["ExcludeVisualStudioCredential"] = [System.Convert]::ToBoolean($excludeVisualStudioCredential)
}

$excludeVisualStudioCodeCredential = "${$ENV:exclude-visual-studio-code-credential}"
if (-Not [string]::IsNullOrWhiteSpace($excludeVisualStudioCodeCredential)) {
    $params["ExcludeVisualStudioCodeCredential"] = [System.Convert]::ToBoolean($excludeVisualStudioCodeCredential)
}

$excludeAzureCliCredential = "${$ENV:exclude-azure-cli-credential}"
if (-Not [string]::IsNullOrWhiteSpace($excludeAzureCliCredential)) {
    $params["ExcludeAzureCliCredential"] = [System.Convert]::ToBoolean($excludeAzureCliCredential)
}

$excludeAzurePowerShellCredential = "${$ENV:exclude-azure-powershell-credential}"
if (-Not [string]::IsNullOrWhiteSpace($excludeAzurePowerShellCredential)) {
    $params["ExcludeAzurePowerShellCredential"] = [System.Convert]::ToBoolean($excludeAzurePowerShellCredential)
}

$excludeAzureDeveloperCliCredential = "${$ENV:exclude-azure-developer-cli-credential}"
if (-Not [string]::IsNullOrWhiteSpace($excludeAzureDeveloperCliCredential)) {
    $params["ExcludeAzureDeveloperCliCredential"] = [System.Convert]::ToBoolean($excludeAzureDeveloperCliCredential)
}

$excludeInteractiveBrowserCredential = "${$ENV:exclude-interactive-browser-credential}"
if (-Not [string]::IsNullOrWhiteSpace($excludeInteractiveBrowserCredential)) {
    $params["ExcludeInteractiveBrowserCredential"] = [System.Convert]::ToBoolean($excludeInteractiveBrowserCredential)
}

$timeout = "${$ENV:timeout}"
if (-Not [string]::IsNullOrWhiteSpace($timeout)) {
    $params["Timeout"] = [System.Convert]::ToInt32($timeout)
}

$batchSize = "${$ENV:batch-size}"
if (-Not [string]::IsNullOrWhiteSpace($batchSize)) {
    $params["BatchSize"] = [System.Convert]::ToInt32($batchSize)
}

$trace = "${$ENV:trace}"
if (-Not [string]::IsNullOrWhiteSpace($trace)) {
    if ([System.Convert]::ToBoolean($trace)) {
        Set-PSDebug -Trace 2
    }
}

$clickOnceApplicationName = "${$ENV:clickonce-application-name}"
if (-Not [string]::IsNullOrWhiteSpace($clickOnceApplicationName)) {
    $params["ClickOnceApplicationName"] = $clickOnceApplicationName
}

$clickOncePublisherName = "${$ENV:clickonce-publisher-name}"
if (-Not [string]::IsNullOrWhiteSpace($clickOncePublisherName)) {
    $params["ClickOncePublisherName"] = $clickOncePublisherName
}

Invoke-TrustedSigning @params
