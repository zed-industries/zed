import Theme from "../themes/theme";
import { panel } from "./app";
import {
    backgroundColor,
    border,
    player,
    shadow,
    text,
    TextColor
} from "./components";

export default function chatPanel(theme: Theme) {
    function channelSelectItem(
        theme: Theme,
        textColor: TextColor,
        hovered: boolean
    ) {
        return {
            name: text(theme, "sans", textColor),
            padding: 4,
            hash: {
                ...text(theme, "sans", "muted"),
                margin: {
                    right: 8,
                },
            },
            background: hovered ? backgroundColor(theme, 300, "hovered") : undefined,
            cornerRadius: hovered ? 6 : 0,
        };
    }

    const message = {
        body: text(theme, "sans", "secondary"),
        timestamp: text(theme, "sans", "muted", { size: "sm" }),
        padding: {
            bottom: 6,
        },
        sender: {
            ...text(theme, "sans", "primary", { weight: "bold" }),
            margin: {
                right: 8,
            },
        },
    };

    return {
        ...panel,
        channelName: text(theme, "sans", "primary", { weight: "bold" }),
        channelNameHash: {
            ...text(theme, "sans", "muted"),
            padding: {
                right: 8,
            },
        },
        channelSelect: {
            header: {
                ...channelSelectItem(theme, "primary", false),
                padding: {
                    bottom: 4,
                    left: 0,
                },
            },
            item: channelSelectItem(theme, "secondary", false),
            hoveredItem: channelSelectItem(theme, "secondary", true),
            activeItem: channelSelectItem(theme, "primary", false),
            hoveredActiveItem: channelSelectItem(theme, "primary", true),
            menu: {
                background: backgroundColor(theme, 500),
                cornerRadius: 6,
                padding: 4,
                border: border(theme, "primary"),
                shadow: shadow(theme),
            },
        },
        signInPrompt: text(theme, "sans", "secondary", { underline: true }),
        hoveredSignInPrompt: text(theme, "sans", "primary", { underline: true }),
        message,
        pendingMessage: {
            ...message,
            body: {
                ...message.body,
                color: theme.textColor.muted.value,
            },
            sender: {
                ...message.sender,
                color: theme.textColor.muted.value,
            },
            timestamp: {
                ...message.timestamp,
                color: theme.textColor.muted.value,
            },
        },
        inputEditor: {
            background: backgroundColor(theme, 500),
            cornerRadius: 6,
            text: text(theme, "mono", "primary"),
            placeholderText: text(theme, "mono", "placeholder", { size: "sm" }),
            selection: player(theme, 1).selection,
            border: border(theme, "secondary"),
            padding: {
                bottom: 7,
                left: 8,
                right: 8,
                top: 7,
            },
        },
    };
}
