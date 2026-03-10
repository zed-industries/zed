# Uploads debug files to Sentry with retry logic.
#
# Usage: upload_to_sentry <file1> [file2 ...]
#
# Requires sentry-cli and SENTRY_AUTH_TOKEN to be available.
# Returns non-zero if all attempts fail.
upload_to_sentry() {
    if [[ $# -eq 0 ]]; then
        echo "Error: no files specified for sentry upload" >&2
        return 1
    fi

    # note: this uploads the unstripped binary which is needed because it contains
    # .eh_frame data for stack unwinding. see https://github.com/getsentry/symbolic/issues/783
    for attempt in 1 2 3; do
        echo "Sentry upload attempt $attempt/3..."
        if sentry-cli debug-files upload --include-sources --wait -p zed -o zed-dev "$@"; then
            echo "Sentry upload successful on attempt $attempt"
            return 0
        else
            echo "Sentry upload failed on attempt $attempt"
            if [ $attempt -eq 3 ]; then
                echo "All sentry upload attempts failed" >&2
                return 1
            fi
            sleep 5
        fi
    done
}
