#!/bin/sh
set -e

cargo run -p peg-macros -- peg-macros/grammar.rustpeg > peg-macros/grammar_new.rs

mv peg-macros/grammar.rs peg-macros/grammar_old.rs
cp peg-macros/grammar_new.rs peg-macros/grammar.rs

if cargo run -p peg-macros -- peg-macros/grammar.rustpeg > peg-macros/grammar_new.rs
then
    diff -qs peg-macros/grammar.rs peg-macros/grammar_new.rs
    rustfmt peg-macros/grammar.rs
else
    echo "Failed"
fi
