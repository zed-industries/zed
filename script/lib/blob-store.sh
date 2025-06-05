function upload_to_blob_store_with_acl
{
    bucket_name="$1"
    file_to_upload="$2"
    blob_store_key="$3"
    acl="$4"
    custom_headers="${5:-}"

    date=$(date +"%a, %d %b %Y %T %z")
    content_type="application/octet-stream"
    storage_type="x-amz-storage-class:STANDARD"

    # Build the canonical amz headers string for signature calculation
    amz_headers=""
    if [ -n "$custom_headers" ]; then
        # Extract x-amz-* headers and format them for signature
        # Convert to lowercase and sort (required for AWS signature)
        amz_header=$(echo "$custom_headers" | tr '[:upper:]' '[:lower:]' | sed 's/: */:/g')
        if [[ "$amz_header" == x-amz-* ]]; then
            amz_headers="${amz_header}\n"
        fi
    fi

    string="PUT\n\n${content_type}\n${date}\n${amz_headers}${acl}\n${storage_type}\n/${bucket_name}/${blob_store_key}"

    string="PUT\n\n${content_type}\n${date}\n${acl}\n${storage_type}\n/${bucket_name}/${blob_store_key}"
    signature=$(echo -en "${string}" | openssl sha1 -hmac "${DIGITALOCEAN_SPACES_SECRET_KEY}" -binary | base64)

    curl_cmd="curl --fail -vv -s -X PUT -T \"$file_to_upload\" \
        -H \"Host: ${bucket_name}.nyc3.digitaloceanspaces.com\" \
        -H \"Date: $date\" \
        -H \"Content-Type: $content_type\" \
        -H \"$storage_type\" \
        -H \"$acl\" \
        -H \"Authorization: AWS ${DIGITALOCEAN_SPACES_ACCESS_KEY}:$signature\""

    if [ -n "$custom_headers" ]; then
        curl_cmd="$curl_cmd -H \"$custom_headers\""
    fi

    curl_cmd="$curl_cmd \"https://${bucket_name}.nyc3.digitaloceanspaces.com/${blob_store_key}\""

    eval $curl_cmd
}

function upload_to_blob_store_public
{
    upload_to_blob_store_with_acl "$1" "$2" "$3" "x-amz-acl:public-read" "${4:-}"
}

function upload_to_blob_store
{
    upload_to_blob_store_with_acl "$1" "$2" "$3" "x-amz-acl:private" "${4:-}"
}
