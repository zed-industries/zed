#!/bin/bash

# Script which automates publishing a crates.io release of the prost crates.

set -ex

if [ "$#" -ne 0 ]
then
  echo "Usage: $0"
  exit 1
fi

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

CRATES=( \
  # "prost-derive" \
  "." \
  "prost-types" \
  "prost-build" \
)

for CRATE in "${CRATES[@]}"; do
  pushd "$DIR/$CRATE"
  cargo publish --allow-dirty
  popd

  echo "sleeping for 5s"
  sleep 5
done
