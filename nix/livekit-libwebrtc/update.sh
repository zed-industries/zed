#!/usr/bin/env nix-shell
#!nix-shell -i bash -p gitMinimal curl gojq gclient2nix

set -eou pipefail
package="livekit-libwebrtc"
pkg_dir="$(dirname "$0")"
nixpkgs="$(git rev-parse --show-toplevel)"

gh-curl () {
  curl --silent ${GITHUB_TOKEN:+-u ":$GITHUB_TOKEN"} "$1"
}

# Get the current version part before the "-unstable-" for the branch name.
# To manually update to a new major version, you can also invoke the script
# with the new major version, e.g., UPDATE_MAJOR_VERSION=137.
old_version="${UPDATE_NIX_OLD_VERSION:-$(nix-instantiate --eval -E "(import \"$nixpkgs\" { }).$package.version" | tr -d '"')}"
major_version="${UPDATE_MAJOR_VERSION:-${old_version%%-unstable-*}}"
branch="m${major_version}_release"

# Fetch the current HEAD commit of the release branch
head="$(gh-curl "https://api.github.com/repos/webrtc-sdk/webrtc/git/refs/heads/$branch" | gojq '.object.sha' --raw-output)"
if gojq -e ".src.args.rev == \"$head\"" "$pkg_dir/sources.json"; then
  echo "$package is already up-to-date: $head"
  exit 0
fi

# Get the commit's date for the version field
date="$(gh-curl "https://api.github.com/repos/webrtc-sdk/webrtc/git/commits/$head" | gojq '.committer.date| split("T") | .[0]' --raw-output)"

echo "Updating sources.json to $head"
gclient2nix generate --root src "https://github.com/webrtc-sdk/webrtc@$head" > "$pkg_dir/sources.json"

sed -i "s|$old_version|$major_version-unstable-$date|g" "$pkg_dir/package.nix"
