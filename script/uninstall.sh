#!/usr/bin/env sh
set -eu

# Uninstalls CodeOrbit that was installed using the install.sh script

check_remaining_installations() {
    platform="$(uname -s)"
    if [ "$platform" = "Darwin" ]; then
        # Check for any CodeOrbit variants in /Applications
        remaining=$(ls -d /Applications/CodeOrbit*.app 2>/dev/null | wc -l)
        [ "$remaining" -eq 0 ]
    else
        # Check for any CodeOrbit variants in ~/.local
        remaining=$(ls -d "$HOME/.local/CodeOrbit"*.app 2>/dev/null | wc -l)
        [ "$remaining" -eq 0 ]
    fi
}

prompt_remove_preferences() {
    printf "Do you want to keep your CodeOrbit preferences? [Y/n] "
    read -r response
    case "$response" in
        [nN]|[nN][oO])
            rm -rf "$HOME/.config/CodeOrbit"
            echo "Preferences removed."
            ;;
        *)
            echo "Preferences kept."
            ;;
    esac
}

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

    echo "CodeOrbit has been uninstalled"
}

linux() {
    suffix=""
    if [ "$channel" != "stable" ]; then
        suffix="-$channel"
    fi

    appid=""
    db_suffix="stable"
    case "$channel" in
      stable)
        appid="dev.CodeOrbit.CodeOrbit"
        db_suffix="stable"
        ;;
      nightly)
        appid="dev.CodeOrbit.CodeOrbit-Nightly"
        db_suffix="nightly"
        ;;
      preview)
        appid="dev.CodeOrbit.CodeOrbit-Preview"
        db_suffix="preview"
        ;;
      dev)
        appid="dev.CodeOrbit.CodeOrbit-Dev"
        db_suffix="dev"
        ;;
      *)
        echo "Unknown release channel: ${channel}. Using stable app ID."
        appid="dev.CodeOrbit.CodeOrbit"
        db_suffix="stable"
        ;;
    esac

    # Remove the app directory
    rm -rf "$HOME/.local/CodeOrbit$suffix.app"

    # Remove the binary symlink
    rm -f "$HOME/.local/bin/CodeOrbit"

    # Remove the .desktop file
    rm -f "$HOME/.local/share/applications/${appid}.desktop"

    # Remove the database directory for this channel
    rm -rf "$HOME/.local/share/CodeOrbit/db/0-$db_suffix"

    # Remove socket file
    rm -f "$HOME/.local/share/CodeOrbit/CodeOrbit-$db_suffix.sock"

    # Remove the entire CodeOrbit directory if no installations remain
    if check_remaining_installations; then
        rm -rf "$HOME/.local/share/CodeOrbit"
        prompt_remove_preferences
    fi

    rm -rf $HOME/.zed_server
}

macos() {
    app="CodeOrbit.app"
    db_suffix="stable"
    app_id="dev.CodeOrbit.CodeOrbit"
    case "$channel" in
      nightly)
        app="CodeOrbit Nightly.app"
        db_suffix="nightly"
        app_id="dev.CodeOrbit.CodeOrbit-Nightly"
        ;;
      preview)
        app="CodeOrbit Preview.app"
        db_suffix="preview"
        app_id="dev.CodeOrbit.CodeOrbit-Preview"
        ;;
      dev)
        app="CodeOrbit Dev.app"
        db_suffix="dev"
        app_id="dev.CodeOrbit.CodeOrbit-Dev"
        ;;
    esac

    # Remove the app bundle
    if [ -d "/Applications/$app" ]; then
        rm -rf "/Applications/$app"
    fi

    # Remove the binary symlink
    rm -f "$HOME/.local/bin/CodeOrbit"

    # Remove the database directory for this channel
    rm -rf "$HOME/Library/Application Support/CodeOrbit/db/0-$db_suffix"

    # Remove app-specific files and directories
    rm -rf "$HOME/Library/Application Support/com.apple.sharedfilelist/com.apple.LSSharedFileList.ApplicationRecentDocuments/$app_id.sfl"*
    rm -rf "$HOME/Library/Caches/$app_id"
    rm -rf "$HOME/Library/HTTPStorages/$app_id"
    rm -rf "$HOME/Library/Preferences/$app_id.plist"
    rm -rf "$HOME/Library/Saved Application State/$app_id.savedState"

    # Remove the entire CodeOrbit directory if no installations remain
    if check_remaining_installations; then
        rm -rf "$HOME/Library/Application Support/CodeOrbit"
        rm -rf "$HOME/Library/Logs/CodeOrbit"

        prompt_remove_preferences
    fi

    rm -rf $HOME/.zed_server
}

main "$@"
