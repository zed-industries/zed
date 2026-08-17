#!/usr/bin/env bash

set -ex

rm -rf target/redoxer
mkdir -p target/redoxer

redoxer install \
    --no-track \
    --path examples/editor-orbclient \
    --root "target/redoxer"

args=(env RUST_LOG=cosmic_text=debug,editor_orbclient=debug /root/bin/editor-orbclient)
if [ -f "$1" ]
then
    filename="$(basename "$1")"
    cp "$1" "target/redoxer/${filename}"
    args+=("${filename}")
fi

cd target/redoxer

# TODO: remove need for linking fonts
redoxer exec \
    --gui \
    --folder . \
    "${args[@]}"
