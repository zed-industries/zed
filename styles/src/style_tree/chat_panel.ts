import { background, border, foreground, text } from "./components"
import { icon_button } from "../component/icon_button"
import { useTheme, with_opacity } from "../theme"
import { interactive } from "../element"
import { Color } from "ayu/dist/color"

export default function chat_panel(): any {
    const theme = useTheme()
    const layer = theme.middle

    const SPACING = 12 as const

    const channel_name = {
        padding: {
            left: SPACING,
            right: SPACING,
            top: 4,
            bottom: 4,
        },
        hash: {
            ...text(layer, "sans", "base"),
        },
        name: text(layer, "sans", "base"),
    }

    return {
        background: background(layer),
        avatar: {
            icon_width: 24,
            icon_height: 24,
            corner_radius: 4,
            outer_width: 24,
            outer_corner_radius: 16,
        },
        avatar_container: {
            padding: {
                right: 6,
                left: 2,
                top: 2,
                bottom: 2,
            },
        },
        list: {},
        channel_select: {
            header: {
                ...channel_name,
                border: border(layer, { bottom: true }),
            },
            item: channel_name,
            active_item: {
                ...channel_name,
                background: background(layer, "on", "active"),
            },
            hovered_item: {
                ...channel_name,
                background: background(layer, "on", "hovered"),
            },
            menu: {
                background: background(layer, "on"),
                border: border(layer, { bottom: true }),
            },
        },
        icon_button: icon_button({
            variant: "ghost",
            color: "variant",
            size: "sm",
        }),
        input_editor: {
            background: background(layer, "on"),
            corner_radius: 6,
            text: text(layer, "sans", "base"),
            placeholder_text: text(layer, "sans", "base", "disabled", {
                size: "xs",
            }),
            selection: theme.players[0],
            border: border(layer, "on"),
            margin: {
                left: SPACING,
                right: SPACING,
                bottom: SPACING,
            },
            padding: {
                bottom: 4,
                left: 8,
                right: 8,
                top: 4,
            },
        },

        rich_text: {
            text: text(layer, "sans", "base"),
            code_background: with_opacity(foreground(layer, "accent"), 0.1),
            mention_highlight: { weight: "bold" },
            self_mention_highlight: { weight: "bold" },
            self_mention_background: background(layer, "active"),
        },
        message_sender: {
            margin: {
                right: 8,
            },
            ...text(layer, "sans", "base", { weight: "bold" }),
        },
        message_timestamp: text(layer, "sans", "base", "disabled"),
        message: {
            ...interactive({
                base: {
                    margin: { top: SPACING },
                    padding: {
                        top: 4,
                        bottom: 4,
                        left: SPACING / 2,
                        right: SPACING / 3,
                    },
                },
                state: {
                    hovered: {
                        background: background(layer, "hovered"),
                    },
                },
            }),
        },
        last_message_bottom_spacing: SPACING,
        continuation_message: {
            ...interactive({
                base: {
                    padding: {
                        top: 4,
                        bottom: 4,
                        left: SPACING / 2,
                        right: SPACING / 3,
                    },
                },
                state: {
                    hovered: {
                        background: background(layer, "hovered"),
                    },
                },
            }),
        },
        pending_message: {
            ...interactive({
                base: {
                    padding: {
                        top: 4,
                        bottom: 4,
                        left: SPACING / 2,
                        right: SPACING / 3,
                    },
                },
                state: {
                    hovered: {
                        background: background(layer, "hovered"),
                    },
                },
            }),
        },
        sign_in_prompt: {
            default: text(layer, "sans", "base"),
        },
    }
}
