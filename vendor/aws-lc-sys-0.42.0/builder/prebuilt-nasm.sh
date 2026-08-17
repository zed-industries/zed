#!/usr/bin/env bash
# Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0 OR ISC

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

while [[ $# -gt 0 ]]; do
    case "$1" in
        -o)
            shift
            path="$1"
            filename="$(basename "$path")"
            filename="$(echo "$filename" | cut -f 1 -d '.')"
            cp "$SCRIPT_DIR/prebuilt-nasm/${filename}".obj "$path"
            exit 0
            ;;
        *)
            shift
            ;;
    esac
done

# If we reach here, it means we didn't find the -o option
echo "PATH: $path" >&2
echo "FILENAME: $filename" >&2
echo "SCRIPT_DIR: $SCRIPT_DIR" >&2
exit 1
