#!/usr/bin/env bash

set -euo pipefail

scratch_dir=$(mktemp -d)
project_dir="${scratch_dir}/scratch"

cargo_toml="${project_dir}/Cargo.toml"
rustfmt_toml="${project_dir}/rustfmt.toml"
lib_rs="${project_dir}/src/lib.rs"

cargo new --lib "${project_dir}"

cat <<-EOF > "${rustfmt_toml}"
format_code_in_doc_comments = true
EOF

for markdown_file in $(git ls-files | rg '.md$'); do
    awk '{ print "/// " $0 } END { print "fn dummy() {}"}' "${markdown_file}" > "${lib_rs}"
    cargo +nightly fmt --manifest-path="${cargo_toml}"
    sed -E -e '$ d' -e 's@/// ?@@' "${lib_rs}" > "${markdown_file}"
done
