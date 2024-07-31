#!/usr/bin/env nix
#!nix shell nixpkgs#toml-cli nixpkgs#jq nixpkgs#url-parser --command bash

set -euo pipefail

script_dir=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

lockfile=$(toml get "$script_dir/../../Cargo.lock" .)

result='{}'

locks=$(jq -c '.package[] | select(.source != null) | select(.source | startswith("git+")) | { name: .name, version: .version, source: .source }' <<< "$lockfile")

declare -A seen
while IFS= read -r lock; do

  name=$(jq -r '.name' <<< "$lock")
  version=$(jq -r '.version' <<< "$lock")
  source=$(jq -r '.source' <<< "$lock")

  case "$source" in
    git+* )
      source="${source#'git+'}"
      scheme=$(url-parser --url "$source" scheme)
      host=$(url-parser --url "$source" host)
      port=$(url-parser --url "$source" port)
      path=$(url-parser --url "$source" path)
      rev=$(url-parser --url "$source" fragment)

      [ -n "${port:-}" ] && port=":${port}"

      url="${scheme}://${host}${port}${path}"

      key="${url}#${rev}"
      [ -n "${seen["$key"]:-}" ] && continue
      seen["$key"]=1

      echo "Fetching hash for $key..." 1>&2
      hash=$(nix-prefetch-git --url "${url}" --rev "${rev}" --quiet | jq '.hash')
      echo "Got $hash" 1>&2
      result=$(jq --arg key "${name}-${version}" --argjson value "${hash}" '.[$key] = $value' <<< "$result")
      echo ""
    ;;
    *)
      echo "unsupported source: $source" >&2
      exit 1
    ;;
  esac

done <<< "$locks"

jq <<< "$result" > "$script_dir/pins.json"
