import Theme from "../themes/theme";
import { backgroundColor, border, iconColor, text } from "./components";

export default function workspace(theme: Theme) {
    const signInPrompt = {
        ...text(theme, "sans", "secondary"),
        size: 13,
        underline: true,
        padding: {
            right: 8,
        },
    };

    const tab = {
        height: 32,
        background: backgroundColor(theme, 300),
        iconClose: iconColor(theme, "muted"),
        iconCloseActive: iconColor(theme, "active"),
        iconConflict: iconColor(theme, "warning"),
        iconDirty: iconColor(theme, "info"),
        iconWidth: 8,
        spacing: 10,
        text: text(theme, "mono", "secondary", { size: "sm" }),
        border: border(theme, "primary", {
            left: true,
            bottom: true,
            overlay: true,
        }),
        padding: {
            left: 12,
            right: 12,
        },
    };

    const activeTab = {
        ...tab,
        background: backgroundColor(theme, 500),
        text: text(theme, "mono", "active", { size: "sm" }),
        border: {
            ...tab.border,
            bottom: false,
        },
    };

    const sidebarItem = {
        height: 32,
        iconColor: iconColor(theme, "secondary"),
        iconSize: 18,
    };
    const sidebar = {
        width: 30,
        background: backgroundColor(theme, 300),
        border: border(theme, "primary", { right: true }),
        item: sidebarItem,
        activeItem: {
            ...sidebarItem,
            iconColor: iconColor(theme, "active"),
        },
        resizeHandle: {
            background: border(theme, "primary").color,
            padding: {
                left: 1,
            },
        },
    };

    return {
        background: backgroundColor(theme, 300),
        leaderBorderOpacity: 0.7,
        leaderBorderWidth: 2.0,
        tab,
        activeTab,
        leftSidebar: {
            ...sidebar,
            border: border(theme, "primary", { right: true }),
        },
        rightSidebar: {
            ...sidebar,
            border: border(theme, "primary", { left: true }),
        },
        paneDivider: {
            color: border(theme, "secondary").color,
            width: 1,
        },
        status_bar: {
            height: 24,
            itemSpacing: 8,
            padding: {
                left: 6,
                right: 6,
            },
            cursorPosition: text(theme, "sans", "muted"),
            diagnosticMessage: text(theme, "sans", "muted"),
            lspMessage: text(theme, "sans", "muted"),
        },
        titlebar: {
            avatarWidth: 18,
            height: 32,
            background: backgroundColor(theme, 100),
            shareIconColor: iconColor(theme, "secondary"),
            shareIconActiveColor: iconColor(theme, "feature"),
            title: text(theme, "sans", "primary"),
            avatar: {
                cornerRadius: 10,
                border: {
                    color: "#00000088",
                    width: 1,
                },
            },
            avatarRibbon: {
                height: 3,
                width: 12,
                // TODO: The background for this ideally should be 
                // set with a token, not hardcoded in rust
            },
            border: border(theme, "primary", { bottom: true }),
            signInPrompt,
            hoveredSignInPrompt: {
                ...signInPrompt,
                ...text(theme, "sans", "active"),
                size: 13,
            },
            offlineIcon: {
                color: iconColor(theme, "secondary"),
                width: 16,
                padding: {
                    right: 4,
                },
            },
            outdatedWarning: {
                ...text(theme, "sans", "warning"),
                size: 13,
            },
        },
        toolbar: {
            height: 34,
            background: backgroundColor(theme, 500),
            border: border(theme, "secondary", { bottom: true }),
            itemSpacing: 8,
            padding: { left: 16, right: 8, top: 4, bottom: 4 },
        },
        breadcrumbs: {
            ...text(theme, "mono", "secondary"),
            padding: { left: 6 },
        },
        disconnectedOverlay: {
            ...text(theme, "sans", "active"),
            background: "#000000aa",
        },
    };
}
