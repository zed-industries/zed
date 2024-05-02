#!/usr/bin/env bash
set -euo pipefail

main() {
    platform="$(uname -s)"
    arch="$(uname -m)"
    channel="${ZED_CHANNEL:-stable}"
    temp="$(mktemp -d "/tmp/zed-XXXXX")"

    if [[ $platform == "Darwin" ]]; then
        platform="macos"
    elif [[ $platform == "Linux" ]]; then
        platform="linux"
        channel="${ZED_CHANNEL:-preview}"
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
    echo "Downloading Zed"
    curl "https://zed.dev/api/releases/$channel/latest/zed-linux-$arch.tar.gz" > "$temp/zed-linux-$arch.tar.gz"

    suffix=""
    if [[ $channel != "stable" ]]; then
        suffix="-$channel"
    fi

    rm -rf "$HOME/.local/zed$suffix.app"
    mkdir -p "$HOME/.local/zed$suffix.app"
    tar -xzf "$temp/zed-linux-$arch.tar.gz" -C "$HOME/.local/"

    mkdir -p "$HOME/.local/bin" "$HOME/.local/share/applications"
    ln -sf ~/.local/zed$suffix.app/bin/zed ~/.local/bin/
    cp ~/.local/zed$suffix.app/share/applications/zed$suffix.desktop ~/.local/share/applications/
    sed -i "s|Icon=zed|Icon=$HOME/.local/zed$suffix.app/share/icons/hicolor/512x512/apps/zed.png|g" ~/.local/share/applications/zed$suffix.desktop
    sed -i "s|Exec=zed|Exec=$HOME/.local/zed$suffix.app/bin/zed|g" ~/.local/share/applications/zed$suffix.desktop

    if which zed >/dev/null 2>&1; then
        echo "Zed has been installed. Run with 'zed'"
    else
        echo "To run zed from your terminal, you must add ~/.local/bin to your PATH"
        echo "Run:"
        echo "   echo 'export PATH=\$HOME/.local/bin:\$PATH' >> ~/.bashrc"
        echo "   source ~/.bashrc"
        echo "To run zed now, '~/.local/bin/zed'"
    fi
}

macos() {
    echo "Downloading Zed"
    curl "https://zed.dev/api/releases/$channel/latest/Zed-$arch.dmg" > "$temp/Zed-$arch.dmg"
    hdiutil attach -quiet "$temp/Zed-$arch.dmg" -mountpoint "$temp/mount"
    app="$(cd "$temp/mount/"; echo *.app)"
    echo "Installing $app"
    if [[ -d "/Applications/$app" ]]; then
        echo "Removing existing $app"
        rm -rf "/Applications/$app"
    fi
    ditto "$temp/mount/$app" "/Applications/$app"
    hdiutil detach -quiet "$temp/mount"

    echo "Zed has been installed. Run with 'open /Applications/$app'"
}

main "$@"
