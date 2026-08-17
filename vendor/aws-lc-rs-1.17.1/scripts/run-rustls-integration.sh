#!/bin/bash -exu
# Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0 OR ISC

# This script tests aws-lc-rs integration with the rustls ecosystem (rcgen, webpki, rustls).
# It uses Cargo's [patch.crates-io] feature to override dependencies, which is more robust
# than modifying individual dependency declarations.

function usage() {
  cat << EOF
Usage: $(basename "$0") [OPTIONS]

Tests aws-lc-rs integration with the rustls ecosystem.

Options:
  --latest-release  Test against latest stable releases (instead of main branch)
  --cleanup         Automatically delete cloned repositories on exit
  --help            Show this help message

Dependencies: jq, cargo-show, cargo-download
EOF
  exit 0
}

[[ " $* " =~ " --help " ]] && usage

ROOT="${GITHUB_WORKSPACE:-$(git rev-parse --show-toplevel)}"

latest_release=0
cleanup=0
for arg in "$@"; do
  if [ "$arg" = "--latest-release" ]; then
    latest_release=1
  fi
  if [ "$arg" = "--cleanup" ]; then
    cleanup=1
  fi
done

function check_dependencies() {
  local missing=()
  command -v jq >/dev/null 2>&1 || missing+=("jq")
  command -v cargo-show >/dev/null 2>&1 >/dev/null 2>&1 || missing+=("cargo-show (cargo install cargo-show)")
  command -v cargo-download >/dev/null 2>&1 || missing+=("cargo-download (cargo install cargo-download)")

  if [ ${#missing[@]} -gt 0 ]; then
    echo "Missing dependencies: ${missing[*]}" >&2
    exit 1
  fi
}
check_dependencies

CLEANUP_ON_EXIT=()

function cleanup() {
  if [ ${#CLEANUP_ON_EXIT[@]} -eq 0 ]; then
    return
  fi
  if [ "$cleanup" -eq 0 ]; then
    echo "You can delete the following directories:"
    echo "${CLEANUP_ON_EXIT[@]}"
  else
    for x in "${CLEANUP_ON_EXIT[@]}"; do
      echo "Deleting: ${x}"
      rm -rf "${x}"
    done
  fi
}

trap cleanup EXIT

# Get the latest stable (non-prerelease) version of a crate from crates.io
function get_latest_stable_version() {
  local crate="$1"
  cargo show --json "$crate" | jq -r '
    [.versions[] |
     select(.yanked == false and (.num | test("alpha|beta|rc") | not))
    ][0].num
  '
}

# Get the git commit SHA for a specific crate version from crates.io
function get_crate_commit() {
  local crate="$1"
  local version="$2"
  local tmp_dir
  tmp_dir="$(mktemp -d)"

  cargo download -o "$tmp_dir/crate.tar.gz" "${crate}=${version}"
  tar xzf "$tmp_dir/crate.tar.gz" -C "$tmp_dir" --strip-components=1
  local sha
  sha="$(jq -r '.git.sha1' "$tmp_dir/.cargo_vcs_info.json")"
  rm -rf "$tmp_dir"
  echo "$sha"
}

# Add [patch.crates-io] section to a Cargo.toml to override aws-lc-rs and aws-lc-sys
# Usage: add_aws_lc_patch <cargo_toml_path> <aws_lc_rs_workspace_root>
function add_aws_lc_patch() {
  local cargo_toml="$1"
  local aws_lc_workspace="$2"

  # Skip if already patched
  if grep -q "aws-lc-rs = { path = \"${aws_lc_workspace}" "$cargo_toml"; then
    echo "Patch already present in $cargo_toml"
    return
  fi

  local aws_lc_rs_patch="aws-lc-rs = { path = \"${aws_lc_workspace}/aws-lc-rs\" }"
  local aws_lc_sys_patch="aws-lc-sys = { path = \"${aws_lc_workspace}/aws-lc-sys\" }"

  if grep -q '^\[patch\.crates-io\]' "$cargo_toml"; then
    # [patch.crates-io] section exists - insert our patches after the header
    local tmp_file
    tmp_file="$(mktemp)"
    trap "rm -f '$tmp_file'" RETURN
    while IFS= read -r line || [[ -n "$line" ]]; do
      echo "$line"
      if [[ "$line" == "[patch.crates-io]" ]]; then
        echo "$aws_lc_rs_patch"
        echo "$aws_lc_sys_patch"
      fi
    done < "$cargo_toml" > "$tmp_file"
    mv "$tmp_file" "$cargo_toml"
  else
    # No existing [patch.crates-io] section - append to end of file
    cat >> "$cargo_toml" << EOF

[patch.crates-io]
${aws_lc_rs_patch}
${aws_lc_sys_patch}
EOF
  fi
}

# Clone a repository and optionally checkout a specific commit
# Usage: clone_repo <url> <destination> [commit]
function clone_repo() {
  local url="$1"
  local dest="$2"
  local commit="${3:-}"

  git clone --recurse-submodules "$url" "$dest"
  if [ -n "$commit" ]; then
    pushd "$dest" > /dev/null
    git checkout "$commit"
    popd > /dev/null
  fi
}

echo "=== Testing rcgen with aws-lc-rs ==="

RCGEN_DIR="$(mktemp -d)"
CLEANUP_ON_EXIT+=("$RCGEN_DIR")

if [[ $latest_release == "1" ]]; then
  RCGEN_VERSION="$(get_latest_stable_version rcgen)"
  RCGEN_COMMIT="$(get_crate_commit rcgen "$RCGEN_VERSION")"
  echo "Using rcgen version ${RCGEN_VERSION} (commit: ${RCGEN_COMMIT})"
  clone_repo "https://github.com/rustls/rcgen" "$RCGEN_DIR" "$RCGEN_COMMIT"
else
  clone_repo "https://github.com/rustls/rcgen" "$RCGEN_DIR"
fi

pushd "$RCGEN_DIR"
add_aws_lc_patch "Cargo.toml" "$ROOT"
if [[ $latest_release != "1" ]]; then
  rm -f Cargo.lock
  cargo update
else
  cargo update -p aws-lc-rs -p aws-lc-sys
fi
cargo tree -i aws-lc-rs --features aws_lc_rs
cargo test --features aws_lc_rs
popd > /dev/null

echo "=== Testing rustls-webpki with aws-lc-rs ==="

WEBPKI_DIR="$(mktemp -d)"
CLEANUP_ON_EXIT+=("$WEBPKI_DIR")

if [[ $latest_release == "1" ]]; then
  WEBPKI_VERSION="$(get_latest_stable_version rustls-webpki)"
  WEBPKI_COMMIT="$(get_crate_commit rustls-webpki "$WEBPKI_VERSION")"
  echo "Using rustls-webpki version ${WEBPKI_VERSION} (commit: ${WEBPKI_COMMIT})"
  clone_repo "https://github.com/rustls/webpki.git" "$WEBPKI_DIR" "$WEBPKI_COMMIT"
else
  clone_repo "https://github.com/rustls/webpki.git" "$WEBPKI_DIR"
fi

pushd "$WEBPKI_DIR"
add_aws_lc_patch "Cargo.toml" "$ROOT"
if [[ $latest_release != "1" ]]; then
  rm -f Cargo.lock
  cargo update
else
  cargo update -p aws-lc-rs -p aws-lc-sys
fi
# Extract just the [features] section and check for aws-lc-rs feature there.
FEATURES_SECTION=$(sed -n '/^\[features\]/,/^\[/p' Cargo.toml)
if echo "$FEATURES_SECTION" | grep -qE '^aws(-|_)lc(-|_)rs\s*='; then
  WEBPKI_FEATURE="aws-lc-rs"
  cargo tree -i aws-lc-rs --features "$WEBPKI_FEATURE"
  cargo test --features "$WEBPKI_FEATURE"
else
  # No aws-lc-rs feature - newer structure uses rustls-aws-lc-rs dev-dependency
  echo "No aws-lc-rs feature found, running tests with default configuration"
  cargo tree -i aws-lc-rs
  cargo test
fi
popd > /dev/null

echo "=== Testing rustls with aws-lc-rs ==="

RUSTLS_DIR="$(mktemp -d)"
CLEANUP_ON_EXIT+=("$RUSTLS_DIR")

if [[ $latest_release == "1" ]]; then
  RUSTLS_VERSION="$(get_latest_stable_version rustls)"
  RUSTLS_COMMIT="$(get_crate_commit rustls "$RUSTLS_VERSION")"
  echo "Using rustls version ${RUSTLS_VERSION} (commit: ${RUSTLS_COMMIT})"
  clone_repo "https://github.com/rustls/rustls.git" "$RUSTLS_DIR" "$RUSTLS_COMMIT"
else
  clone_repo "https://github.com/rustls/rustls.git" "$RUSTLS_DIR"
fi

pushd "$RUSTLS_DIR"
add_aws_lc_patch "Cargo.toml" "$ROOT"
if [[ $latest_release != "1" ]]; then
  rm -f Cargo.lock
  cargo update
else
  cargo update -p aws-lc-rs -p aws-lc-sys
fi

# Detect which package has the aws-lc-rs feature by checking [features] section.
# Old structure (<=0.23.x): aws-lc-rs feature is in rustls/Cargo.toml
# New structure (>=0.24.x): aws-lc-rs feature is in rustls-test/Cargo.toml
if grep -q '^aws-lc-rs\s*=' ./rustls/Cargo.toml; then
  # Old structure: aws-lc-rs feature is in the main rustls crate
  pushd ./rustls
  cargo tree -i aws-lc-rs --features aws-lc-rs
  cargo test --features aws-lc-rs
  popd > /dev/null # ./rustls
else
  # New structure: aws-lc-rs feature is in rustls-test
  pushd ./rustls-test
  cargo tree -i aws-lc-rs --features aws-lc-rs
  cargo test --features aws-lc-rs
  popd > /dev/null # ./rustls-test
fi
popd > /dev/null # "$RUSTLS_DIR"

echo "=== All rustls integration tests passed ==="
