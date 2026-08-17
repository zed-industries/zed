#!/bin/bash
set -euo pipefail

rustup target list | sed 's/ (.*//' > list.txt
rustc +nightly --print target-list >> list.txt
cat list.txt | sort | uniq |sed 's/\(.*\)/            "\1",/' > sorted.txt
rm list.txt
