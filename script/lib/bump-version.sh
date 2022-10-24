#!/bin/bash

set -eu

if [[ $# < 3 ]]; then
  echo "Missing version increment (major, minor, or patch)" >&2
  exit 1
fi

package=$1
tag_prefix=$2
version_increment=$3

if [[ -n $(git status --short --untracked-files=no) ]]; then
  echo "Can't push a new version with uncommitted changes"
  exit 1
fi

which cargo-set-version > /dev/null || cargo install cargo-edit
cargo set-version --package $package --bump $version_increment
cargo check --quiet

new_version=$(cargo metadata --no-deps --format-version=1 | jq --raw-output ".packages[] | select(.name == \"${package}\") | .version")
branch_name=$(git rev-parse --abbrev-ref HEAD)
old_sha=$(git rev-parse HEAD)
tag_name=${tag_prefix}${new_version}

git commit --quiet --all --message "${package} ${new_version}"
git tag ${tag_name}

cat <<MESSAGE
Committed and tagged ${package} version ${new_version}

To push this:
    git push origin ${tag_name} ${branch_name}

To undo this:
    git tag -d ${tag_name} && git reset --hard $old_sha
MESSAGE
