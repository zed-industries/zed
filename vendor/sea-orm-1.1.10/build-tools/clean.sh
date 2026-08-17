#!/bin/bash
set -x

for dir in */; do
    cd "$dir";
    cargo clean;
    cd ..;
done