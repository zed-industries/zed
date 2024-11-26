#!/usr/bin/env sh
set -eu

# Uninstalls Zed that was installed using the install.sh script
# Modeled after install.sh

main() {
    platform="$(uname -s)"
    channel="${ZED_CHANNEL:-stable}"

    if [ "$platform" = "Darwin" ]; then
        platform="macos"
    elif [ "$platform" = "Linux" ]; then
        platform="linux"
    else
        echo "Unsupported platform $platform"
        exit 1
    fi

    "$platform"

    echo "Zed has been uninstalled"
}

linux() {
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

    # Remove the app directory
    rm -rf "$HOME/.local/zed$suffix.app"

    # Remove the binary symlink
    rm -f "$HOME/.local/bin/zed"

    # Remove the .desktop file
    rm -f "$HOME/.local/share/applications/${appid}.desktop"
}

macos() {
    app="Zed.app"
    case "$channel" in
      nightly)
        app="Zed Nightly.app"
        ;;
      preview)
        app="Zed Preview.app"
        ;;
      dev)
        app="Zed Dev.app"
        ;;
    esac

    # Remove the app bundle
    if [ -d "/Applications/$app" ]; then
        rm -rf "/Applications/$app"
    fi

    # Remove the binary symlink
    rm -f "$HOME/.local/bin/zed"
}

main "$@"
