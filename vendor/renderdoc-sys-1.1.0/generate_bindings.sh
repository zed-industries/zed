#!/usr/bin/env bash

# This script generates Rust bindings to the in-application Renderdoc API.
#
# Dependencies:
# * bindgen (>=0.63.0)
# * curl

set -euo pipefail

readonly VERSION=v1.x
readonly TEMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/renderdoc-rs.XXXXXXXXX")"

trap -- "rm -rf '${TEMP_DIR}'" EXIT

curl -o "${TEMP_DIR}/renderdoc_app.h" -L "https://raw.githubusercontent.com/baldurk/renderdoc/${VERSION}/renderdoc/api/app/renderdoc_app.h"

bindgen \
  --blocklist-type '__uint64_t|__uint32_t' \
  --allowlist-type 'RENDERDOC.*|pRENDERDOC.*' \
  --generate-inline-functions \
  --no-prepend-enum-name \
  --impl-debug \
  "${TEMP_DIR}/renderdoc_app.h" > ./src/bindings.rs
