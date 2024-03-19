#!/usr/bin/env bash

bash -euo pipefail
source script/lib/blob-store.sh

commit=$1
if [ "$#" -ne 1 ] || ! [[ $commit =~ ^[0-9a-f]{40}$ ]]; then
    echo "Usage: $0 <git-sha>"
    exit 1
fi

bucket_name="zed-extension-cli"
target_triple=$(rustc -vV | sed -n 's|host: ||p')

upload_to_blob_store_public $bucket_name "target/release/zed-extension" "${commit}/${target_triple}/zed-extension"
