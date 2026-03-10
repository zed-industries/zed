# Uploads debug files to Sentry with retry logic.
#
# Usage: Upload-ToSentry -Paths @("file1", "file2")
#
# Requires sentry-cli and SENTRY_AUTH_TOKEN to be available.
# Throws if all attempts fail.
function Upload-ToSentry {
    param(
        [Parameter(Mandatory=$true)]
        [string[]]$Paths
    )

    for ($attempt = 1; $attempt -le 3; $attempt++) {
        try {
            Write-Output "Sentry upload attempt $attempt/3..."
            sentry-cli debug-files upload --include-sources --wait -p zed -o zed-dev @Paths
            Write-Output "Sentry upload successful on attempt $attempt"
            return
        }
        catch {
            Write-Output "Sentry upload failed on attempt ${attempt}: $_"
            if ($attempt -eq 3) {
                throw "All sentry upload attempts failed"
            }
            Start-Sleep -Seconds 5
        }
    }
}
