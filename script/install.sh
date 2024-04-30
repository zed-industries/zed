#!/usr/bin/env bash
set -euo pipefail

main() {
    platform="$(uname -s)"
    arch="$(uname -m)"
    channel="stable"
    temp="$(mktemp -d "/tmp/zed-XXXXX")"

    if [[ $platform == "Darwin" ]]; then
        platform="macos"
    elif [[ $platform == "Linux" ]]; then
        platform="linux"
        channel="nightly"
    else
        echo "Unsupported platform $platform"
        exit 1
    fi

    if [[ $platform == "macos" ]] && [[ $arch == arm64* ]]; then
        arch="aarch64"
    elif [[ $arch = x86* || $arch == i686* ]]; then
        arch="x86_64"
    else
        echo "Unsupported architecture $arch"
        exit 1
    fi

    if which curl >/dev/null 2>&1; then
        curl () {
            command curl -fL "$@"
        }
    elif which wget >/dev/null 2>&1; then
        curl () {
    	    wget -O- "$@"
         }
    else
    	echo "Could not find 'curl' or 'wget' in your path"
    	exit 1
    fi

    "$platform" "$@"
}

linux() {
    echo "Downloading zed.tar.gz"
    # curl "https://zed.dev/api/download/zed.tar.gz?platform=$platform&arch=$arch&channel=$channel" > "$temp/zed.tar.gz"

    curl "https://zed.dev/api/releases/$channel/latest/zed-linux-$arch.tar.gz" > "$temp/zed-linux-$arch.tar.gz"

    mkdir -p "$HOME/.local/zed.app"
    tar -xzf "$temp/zed-linux-$arch.tar.gz" -C "$HOME/.local/"

    # Set up xdg links so that app shows in the dock
    mkdir -p "$HOME/.local/bin" "$HOME/.local/share/applications"
    ln -sf ~/.local/zed.app/bin/zed ~/.local/bin/
    cp ~/.local/zed.app/share/applications/zed.desktop ~/.local/share/applications/
    sed -i "s|Icon=zed|Icon=$HOME/.local/zed.app/share/icons/hicolor/512x512/apps/zed.png|g" ~/.local/share/applications/zed.desktop
    sed -i "s|Exec=zed|Exec=$HOME/.local/zed.app/bin/zed|g" ~/.local/share/applications/zed.desktop

    if which zed >/dev/null 2>&1; then
    else
        echo "To run zed from your terminal, you must add ~/.local/bin to your PATH"
        exit 1
    fi

    ~/.local/bin/zed
}

macos() {
    echo "Downloading Zed.dmg..."
    curl "https://zed.dev/api/releases/$channel/latest/Zed-$arch.dmg" > "$temp/Zed-$arch.dmg"
    hdiutil attach -quiet "$temp/Zed-$arch.dmg" -mountpoint "$temp/mount"
    app="$(cd "$temp/mount/"; echo *.app)"
    echo "Installing $app"
    if [[ -d "/Applications/$app" ]]; then
        echo "Removing existing $app"
        rm -rf "/Applications/$app"
    fi
    ditto -v "$temp/mount/$app" "/Applications/$app"
    hdiutil detach -quiet "$temp/mount"
    open "/Applications/$app"
}

main "$@"
