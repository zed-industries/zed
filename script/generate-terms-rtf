#!/bin/bash

set -e

if ! command -v pandoc &> /dev/null
then
    brew install pandoc   # Install pandoc using Homebrew
fi

pandoc ./legal/terms.md -f markdown-smart -t html -o ./script/terms/terms.html
textutil -convert rtf ./script/terms/terms.html -output ./script/terms/terms.rtf
rm ./script/terms/terms.html
