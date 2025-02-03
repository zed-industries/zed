#!/bin/bash

channel=$(cat crates/zed/RELEASE_CHANNEL)

tag_suffix=""
case $channel in
  stable)
    ;;
  preview)
    tag_suffix="-pre"
    ;;
  *)
    echo "this must be run on either of stable|preview release branches" >&2
    exit 1
    ;;
esac

exec script/lib/bump-version.sh zed v "$tag_suffix" patch
