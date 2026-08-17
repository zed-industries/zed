#!/bin/bash

# Script which automates modifying source version fields, and creating a release
# commit and tag. The commit and tag are not automatically pushed, nor are the
# crates published (see publish-release.sh).

set -ex

if [ "$#" -ne 1 ]
then
  echo "Usage: $0 <version>"
  exit 1
fi

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
VERSION="$1"
MINOR="$( echo ${VERSION} | cut -d\. -f1-2 )"

VERSION_MATCHER="([a-z0-9\\.-]+)"
PROST_CRATE_MATCHER="(prost|prost-[a-z]+)"

# Update the README.md.
sed -i -E "s/${PROST_CRATE_MATCHER} = \"${VERSION_MATCHER}\"/\1 = \"${MINOR}\"/" "$DIR/README.md"

# Update html_root_url attributes.
sed -i -E "s~html_root_url = \"https://docs\.rs/${PROST_CRATE_MATCHER}/$VERSION_MATCHER\"~html_root_url = \"https://docs.rs/\1/${VERSION}\"~" \
  "$DIR/src/lib.rs" \
  "$DIR/prost-derive/src/lib.rs" \
  "$DIR/prost-build/src/lib.rs" \
  "$DIR/prost-types/src/lib.rs"

# Update Cargo.toml version fields.
sed -i -E "s/^version = \"${VERSION_MATCHER}\"$/version = \"${VERSION}\"/" \
  "$DIR/Cargo.toml" \
  "$DIR/prost-derive/Cargo.toml" \
  "$DIR/prost-build/Cargo.toml" \
  "$DIR/prost-types/Cargo.toml"

# Update Cargo.toml dependency versions.
sed -i -E "s/^${PROST_CRATE_MATCHER} = \{ version = \"${VERSION_MATCHER}\"/\1 = { version = \"${VERSION}\"/" \
  "$DIR/Cargo.toml" \
  "$DIR/prost-derive/Cargo.toml" \
  "$DIR/prost-build/Cargo.toml" \
  "$DIR/prost-types/Cargo.toml"

git commit -a -m "release ${VERSION}"
git tag -a "v${VERSION}" -m "release ${VERSION}"
