#!/usr/bin/env sh
set -eu

# Downloads the latest tarball from https://zed.dev/releases and unpacks it
# into ~/.local/. If you'd prefer to do this manually, instructions are at
# https://zed.dev/docs/linux.

main() {
    platform="$(uname -s)"
    arch="$(uname -m)"
    channel="${ZED_CHANNEL:-stable}"
    temp="$(mktemp -d "/tmp/zed-XXXXXX")"

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

    if [ "$(command -v zed)" = "$HOME/.local/bin/zed" ]; then
        echo "Zed has been installed. Run with 'zed'"
    else
        echo "To run Zed from your terminal, you must add ~/.local/bin to your PATH"
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

        echo "To run Zed now, '~/.local/bin/zed'"
    fi
}

linux() {
    if [ -n "${ZED_BUNDLE_PATH:-}" ]; then
        cp "$ZED_BUNDLE_PATH" "$temp/zed-linux-$arch.tar.gz"
    else
        echo "Downloading Zed"
        curl "https://zed.dev/api/releases/$channel/latest/zed-linux-$arch.tar.gz" > "$temp/zed-linux-$arch.tar.gz"
    fi

    suffix=""
    if [ "$channel" != "stable" ]; then
        suffix="-$channel"
    fi

    appid=""
    case "$channel" in
      stable)
        appid="dev.zed.Zed"
        ;;
      nightly)
        appid="dev.zed.Zed-Nightly"
        ;;
      preview)
        appid="dev.zed.Zed-Preview"
        ;;
      dev)
        appid="dev.zed.Zed-Dev"
        ;;
      *)
        echo "Unknown release channel: ${channel}. Using stable app ID."
        appid="dev.zed.Zed"
        ;;
    esac

    # Unpack
    rm -rf "$HOME/.local/zed$suffix.app"
    mkdir -p "$HOME/.local/zed$suffix.app"
    tar -xzf "$temp/zed-linux-$arch.tar.gz" -C "$HOME/.local/"

    # Setup ~/.local directories
    mkdir -p "$HOME/.local/bin" "$HOME/.local/share/applications"

    # Link the binary
    if [ -f "$HOME/.local/zed$suffix.app/bin/zed" ]; then
        ln -sf "$HOME/.local/zed$suffix.app/bin/zed" "$HOME/.local/bin/zed"
    else
        # support for versions before 0.139.x.
        ln -sf "$HOME/.local/zed$suffix.app/bin/cli" "$HOME/.local/bin/zed"
    fi

    # Copy .desktop file
    desktop_file_path="$HOME/.local/share/applications/${appid}.desktop"
    cp "$HOME/.local/zed$suffix.app/share/applications/zed$suffix.desktop" "${desktop_file_path}"
    sed -i "s|Icon=zed|Icon=$HOME/.local/zed$suffix.app/share/icons/hicolor/512x512/apps/zed.png|g" "${desktop_file_path}"
    sed -i "s|Exec=zed|Exec=$HOME/.local/zed$suffix.app/bin/zed|g" "${desktop_file_path}"

    setup_polkit
}

setup_polkit() {
    # Check if polkit is installed
    if ! command -v pkexec >/dev/null 2>&1; then
        echo
        echo "Note: 'pkexec' not detected. You won't be able to edit files owned by root in Zed."
        echo
        echo "Warning: Do NOT run Zed directly with sudo. Doing so may cause files to become unusable, which requires manual fixing."
        echo
        echo "See https://zed.dev/docs/linux#root for more information."
        echo
        return 0
    fi

    # Prompt user
    echo
    printf "Configure polkit to allow Zed to request elevated permissions when editing files owned by root? [Y/n] "
    read -r response
    response=${response:-Y} # Default to Yes if empty
    echo

    case "$response" in
        [Yy]*)            
            LIBEXEC_DIR="/usr/libexec/zed"
            ELEVATE_SCRIPT="$LIBEXEC_DIR/elevate.sh"
            POLKIT_DIR="/usr/share/polkit-1/actions"

            ELEVATE_SCRIPT_CONTENT='#!/bin/bash
eval "$@"'

            POLICY_CONTENT='<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE policyconfig PUBLIC
"-//freedesktop//DTD PolicyKit Policy Configuration 1.0//EN"
"http://www.freedesktop.org/standards/PolicyKit/1/policyconfig.dtd">
<policyconfig>
  <action id="dev.zed.Zed">
    <description>Run Zed with elevated privileges</description>
    <message>Zed needs temporary elevated access to make changes. Please enter your password.</message>
    <defaults>
      <allow_any>auth_admin</allow_any>
      <allow_inactive>auth_admin</allow_inactive>
      <allow_active>auth_admin</allow_active>
    </defaults>
    <annotate key="org.freedesktop.policykit.exec.path">/usr/libexec/zed/elevate.sh</annotate>
  </action>
</policyconfig>'

            # We only request sudo access now
            if sudo mkdir -p "$LIBEXEC_DIR" && \
                echo "$ELEVATE_SCRIPT_CONTENT" | sudo tee "$ELEVATE_SCRIPT" > /dev/null && \
                sudo chmod 755 "$ELEVATE_SCRIPT" && \
                sudo mkdir -p "$POLKIT_DIR" && \
                echo "$POLICY_CONTENT" | sudo tee "$POLKIT_DIR/dev.zed.Zed.policy" > /dev/null && \
                sudo chmod 644 "$POLKIT_DIR/dev.zed.Zed.policy"; then
                echo "Successfully configured polkit for Zed"
            else
                echo "Failed to configure polkit for Zed"
            fi
            ;;
        *)
            echo "Skipping"
            ;;
    esac

    echo
}

macos() {
    echo "Downloading Zed"
    curl "https://zed.dev/api/releases/$channel/latest/Zed-$arch.dmg" > "$temp/Zed-$arch.dmg"
    hdiutil attach -quiet "$temp/Zed-$arch.dmg" -mountpoint "$temp/mount"
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
    ln -sf "/Applications/$app/Contents/MacOS/cli" "$HOME/.local/bin/zed"
}

main "$@"
