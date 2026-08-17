#!/bin/bash -ex

# Usage:
#
#     $ ./update-libfuzzer.sh

set -ex

# The LLVM commit from which we are vendoring libfuzzer. This must be a commit
# hash from https://github.com/llvm/llvm-project
COMMIT=6146a88f60492b520a36f8f8f3231e15f3cc6082

cd "$(dirname $0)"
project_dir="$(pwd)"

tmp_dir="$(mktemp -d)"
cd "$tmp_dir"

git init
git remote add llvm https://github.com/llvm/llvm-project.git
git sparse-checkout set compiler-rt/lib/fuzzer

git fetch --depth 1 llvm "$COMMIT" --filter=blob:none
git checkout "$COMMIT"

rm -rf "$project_dir/libfuzzer/"
mv "$tmp_dir/compiler-rt/lib/fuzzer/" "$project_dir/libfuzzer/"
