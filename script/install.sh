#!/usr/bin/env sh
set -eu

# Downloads the latest tarball from https://CodeOrbit.dev/releases and unpacks it
# into ~/.local/. If you'd prefer to do this manually, instructions are at
# https://CodeOrbit.dev/docs/linux.

main() {
    platform="$(uname -s)"
    arch="$(uname -m)"
    channel="${ZED_CHANNEL:-stable}"
    temp="$(mktemp -d "/tmp/CodeOrbit-XXXXXX")"

    if [ "$platform" = "Darwin" ]; then
        platform="macos"
    elif [ "$platform" = "Linux" ]; then
        platform="linux"
    else
        echo "Unsupported platform $platform"
        exit 1
    fi

    case "$platform-$arch" in
        macos-arm64* | linux-arm64* | linux-armhf | linux-aarch64)
            arch="aarch64"
            ;;
        macos-x86* | linux-x86* | linux-i686*)
            arch="x86_64"
            ;;
        *)
            echo "Unsupported platform or architecture"
            exit 1
            ;;
    esac

    if command -v curl >/dev/null 2>&1; then
        curl () {
            command curl -fL "$@"
        }
    elif command -v wget >/dev/null 2>&1; then
        curl () {
            wget -O- "$@"
        }
    else
        echo "Could not find 'curl' or 'wget' in your path"
        exit 1
    fi

    "$platform" "$@"

    if [ "$(command -v CodeOrbit)" = "$HOME/.local/bin/CodeOrbit" ]; then
        echo "CodeOrbit has been installed. Run with 'CodeOrbit'"
    else
        echo "To run CodeOrbit from your terminal, you must add ~/.local/bin to your PATH"
        echo "Run:"

        case "$SHELL" in
            *zsh)
                echo "   echo 'export PATH=\$HOME/.local/bin:\$PATH' >> ~/.zshrc"
                echo "   source ~/.zshrc"
                ;;
            *fish)
                echo "   fish_add_path -U $HOME/.local/bin"
                ;;
            *)
                echo "   echo 'export PATH=\$HOME/.local/bin:\$PATH' >> ~/.bashrc"
                echo "   source ~/.bashrc"
                ;;
        esac

        echo "To run CodeOrbit now, '~/.local/bin/CodeOrbit'"
    fi
}

linux() {
    if [ -n "${ZED_BUNDLE_PATH:-}" ]; then
        cp "$ZED_BUNDLE_PATH" "$temp/CodeOrbit-linux-$arch.tar.gz"
    else
        echo "Downloading CodeOrbit"
        curl "https://CodeOrbit.dev/api/releases/$channel/latest/CodeOrbit-linux-$arch.tar.gz" > "$temp/CodeOrbit-linux-$arch.tar.gz"
    fi

    suffix=""
    if [ "$channel" != "stable" ]; then
        suffix="-$channel"
    fi

    appid=""
    case "$channel" in
      stable)
        appid="dev.CodeOrbit.CodeOrbit"
        ;;
      nightly)
        appid="dev.CodeOrbit.CodeOrbit-Nightly"
        ;;
      preview)
        appid="dev.CodeOrbit.CodeOrbit-Preview"
        ;;
      dev)
        appid="dev.CodeOrbit.CodeOrbit-Dev"
        ;;
      *)
        echo "Unknown release channel: ${channel}. Using stable app ID."
        appid="dev.CodeOrbit.CodeOrbit"
        ;;
    esac

    # Unpack
    rm -rf "$HOME/.local/CodeOrbit$suffix.app"
    mkdir -p "$HOME/.local/CodeOrbit$suffix.app"
    tar -xzf "$temp/CodeOrbit-linux-$arch.tar.gz" -C "$HOME/.local/"

    # Setup ~/.local directories
    mkdir -p "$HOME/.local/bin" "$HOME/.local/share/applications"

    # Link the binary
    if [ -f "$HOME/.local/CodeOrbit$suffix.app/bin/CodeOrbit" ]; then
        ln -sf "$HOME/.local/CodeOrbit$suffix.app/bin/CodeOrbit" "$HOME/.local/bin/CodeOrbit"
    else
        # support for versions before 0.139.x.
        ln -sf "$HOME/.local/CodeOrbit$suffix.app/bin/cli" "$HOME/.local/bin/CodeOrbit"
    fi

    # Copy .desktop file
    desktop_file_path="$HOME/.local/share/applications/${appid}.desktop"
    cp "$HOME/.local/CodeOrbit$suffix.app/share/applications/CodeOrbit$suffix.desktop" "${desktop_file_path}"
    sed -i "s|Icon=CodeOrbit|Icon=$HOME/.local/CodeOrbit$suffix.app/share/icons/hicolor/512x512/apps/CodeOrbit.png|g" "${desktop_file_path}"
    sed -i "s|Exec=CodeOrbit|Exec=$HOME/.local/CodeOrbit$suffix.app/bin/CodeOrbit|g" "${desktop_file_path}"
}

macos() {
    echo "Downloading CodeOrbit"
    curl "https://CodeOrbit.dev/api/releases/$channel/latest/CodeOrbit-$arch.dmg" > "$temp/CodeOrbit-$arch.dmg"
    hdiutil attach -quiet "$temp/CodeOrbit-$arch.dmg" -mountpoint "$temp/mount"
    app="$(cd "$temp/mount/"; echo *.app)"
    echo "Installing $app"
    if [ -d "/Applications/$app" ]; then
        echo "Removing existing $app"
        rm -rf "/Applications/$app"
    fi
    ditto "$temp/mount/$app" "/Applications/$app"
    hdiutil detach -quiet "$temp/mount"

    mkdir -p "$HOME/.local/bin"
    # Link the binary
    ln -sf "/Applications/$app/Contents/MacOS/cli" "$HOME/.local/bin/CodeOrbit"
}

main "$@"
