#!/usr/bin/env sh
set -eu

# Uninstalls Zaply that was installed using the install.sh script

check_remaining_installations() {
    platform="$(uname -s)"
    if [ "$platform" = "Darwin" ]; then
        # Check for any Zaply variants in /Applications
        remaining=$(ls -d /Applications/Zaply*.app 2>/dev/null | wc -l)
        [ "$remaining" -eq 0 ]
    else
        # Check for any Zaply variants in ~/.local
        remaining=$(ls -d "$HOME/.local/zaply"*.app 2>/dev/null | wc -l)
        [ "$remaining" -eq 0 ]
    fi
}

prompt_remove_preferences() {
    printf "Do you want to keep your Zaply preferences? [Y/n] "
    read -r response
    case "$response" in
        [nN]|[nN][oO])
            rm -rf "$HOME/.config/zaply"
            echo "Preferences removed."
            ;;
        *)
            echo "Preferences kept."
            ;;
    esac
}

main() {
    platform="$(uname -s)"
    channel="${ZAPLY_CHANNEL:-stable}"

    if [ "$platform" = "Darwin" ]; then
        platform="macos"
    elif [ "$platform" = "Linux" ]; then
        platform="linux"
    else
        echo "Unsupported platform $platform"
        exit 1
    fi

    "$platform"

    echo "Zaply has been uninstalled"
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
        appid="dev.zaply.Zaply"
        db_suffix="stable"
        ;;
      nightly)
        appid="dev.zaply.Zaply-Nightly"
        db_suffix="nightly"
        ;;
      preview)
        appid="dev.zaply.Zaply-Preview"
        db_suffix="preview"
        ;;
      dev)
        appid="dev.zaply.Zaply-Dev"
        db_suffix="dev"
        ;;
      *)
        echo "Unknown release channel: ${channel}. Using stable app ID."
        appid="dev.zaply.Zaply"
        db_suffix="stable"
        ;;
    esac

    # Remove the app directory
    rm -rf "$HOME/.local/zaply$suffix.app"

    # Remove the binary symlink
    rm -f "$HOME/.local/bin/zaply"

    # Remove the .desktop file
    rm -f "$HOME/.local/share/applications/${appid}.desktop"

    # Remove the database directory for this channel
    rm -rf "$HOME/.local/share/zaply/db/0-$db_suffix"

    # Remove socket file
    rm -f "$HOME/.local/share/zaply/zaply-$db_suffix.sock"

    # Remove the entire Zaply directory if no installations remain
    if check_remaining_installations; then
        rm -rf "$HOME/.local/share/zaply"
        prompt_remove_preferences
    fi

    rm -rf $HOME/.zaply_server
}

macos() {
    app="Zaply.app"
    db_suffix="stable"
    app_id="dev.zaply.Zaply"
    case "$channel" in
      nightly)
        app="Zaply Nightly.app"
        db_suffix="nightly"
        app_id="dev.zaply.Zaply-Nightly"
        ;;
      preview)
        app="Zaply Preview.app"
        db_suffix="preview"
        app_id="dev.zaply.Zaply-Preview"
        ;;
      dev)
        app="Zaply Dev.app"
        db_suffix="dev"
        app_id="dev.zaply.Zaply-Dev"
        ;;
    esac

    # Remove the app bundle
    if [ -d "/Applications/$app" ]; then
        rm -rf "/Applications/$app"
    fi

    # Remove the binary symlink
    rm -f "$HOME/.local/bin/zaply"

    # Remove the database directory for this channel
    rm -rf "$HOME/Library/Application Support/Zaply/db/0-$db_suffix"

    # Remove app-specific files and directories
    rm -rf "$HOME/Library/Application Support/com.apple.sharedfilelist/com.apple.LSSharedFileList.ApplicationRecentDocuments/$app_id.sfl"*
    rm -rf "$HOME/Library/Caches/$app_id"
    rm -rf "$HOME/Library/HTTPStorages/$app_id"
    rm -rf "$HOME/Library/Preferences/$app_id.plist"
    rm -rf "$HOME/Library/Saved Application State/$app_id.savedState"

    # Remove the entire Zaply directory if no installations remain
    if check_remaining_installations; then
        rm -rf "$HOME/Library/Application Support/Zaply"
        rm -rf "$HOME/Library/Logs/Zaply"

        prompt_remove_preferences
    fi

    rm -rf $HOME/.zaply_server
}

main "$@"
