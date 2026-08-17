#!/bin/bash

# This is a script that runs the code generation and export a copy of it under gen/$1.
# This is NOT the main code generation script, but a tool to help looking at the generated code
# (which is otherwise lost somewhere under target/) and performing diffs with previous versions.

if [ -z "$1" ]
then
    echo -e "Error: a subfolder in gen/ must be supplied as argument"
    exit 1
fi

# trigger rebuild
touch build/cg/mod.rs
# rebuild and export code
RXCB_EXPORT=gen/$1 RUST_BACKTRACE=1 cargo build --all-features
# format exported code
for file in gen/$1/*.rs
do
    rustfmt $file --edition=2018
done
