#!/bin/bash

set -ex

if [ "$#" -ne 1 ]
then
  echo "Usage: $0 <protobuf-version>"
  exit 1
fi

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
VERSION="$1"
TEMPDIR=$(mktemp -d "protobuf-$VERSION-XXX")
ARCHS=( \
  "linux-aarch_64" \
  "linux-x86_32" \
  "linux-x86_64" \
  "osx-x86_64" \
  "win32" \
)

for ARCH in "${ARCHS[@]}"; do
  mkdir "$TEMPDIR/$ARCH"
  curl --proto '=https' --tlsv1.2 -sSfL \
    "https://github.com/protocolbuffers/protobuf/releases/download/v$VERSION/protoc-$VERSION-$ARCH.zip" \
    -o "$TEMPDIR/$ARCH/protoc.zip"

  EXTENSION=""
  if [[ "$ARCH" == *"win"* ]]; then
    EXTENSION=".exe"
  fi

  unzip "$TEMPDIR/$ARCH/protoc.zip" -d "$TEMPDIR/$ARCH"
  mv "$TEMPDIR/$ARCH/bin/protoc$EXTENSION" "$DIR/protobuf/protoc-$ARCH$EXTENSION"
done


# Update the include directory
rm -rf "$DIR/protobuf/include"
mv "$TEMPDIR/linux-x86_64/include" "$DIR/protobuf/"

# Update the Protocol Buffers license.
mkdir "$TEMPDIR/src"
curl --proto '=https' --tlsv1.2 -sSfL \
  "https://github.com/protocolbuffers/protobuf/archive/v$VERSION.zip" \
  -o "$TEMPDIR/src/protobuf.zip"
unzip "$TEMPDIR/src/protobuf.zip" -d "$TEMPDIR/src"
mv "$TEMPDIR/src/protobuf-$VERSION/LICENSE" "$DIR/protobuf/LICENSE"

rm -rf $TEMPDIR
