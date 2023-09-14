import {
    background,
    border,
    text,
} from "./components"
import { useTheme } from "../theme"

export default function chat_panel(): any {
    const theme = useTheme()
    const layer = theme.middle

    const SPACING = 12 as const

    const channel_name = {
        padding: {
            // top: 4,
            bottom: 4,
            // left: 4,
            right: 4,
        },
        hash: {
            ...text(layer, "sans", "base"),
        },
        name: text(layer, "sans", "base"),
    }

    return {
        background: background(layer),
        padding: {
            top: SPACING,
            bottom: SPACING,
            left: SPACING,
            right: SPACING,
        },
        channel_select: {
            header: { ...channel_name },
            item: channel_name,
            active_item: channel_name,
            hovered_item: channel_name,
            hovered_active_item: channel_name,
            menu: {
                padding: {
                    top: 10,
                    bottom: 10,
                }
            }
        },
        input_editor: {
            background: background(layer, "on"),
            corner_radius: 6,
            text: text(layer, "sans", "base"),
            placeholder_text: text(layer, "sans", "base", "disabled", {
                size: "xs",
            }),
            selection: theme.players[0],
            border: border(layer, "on"),
            padding: {
                bottom: 4,
                left: 8,
                right: 8,
                top: 4,
            },
        },
        message: {
            body: text(layer, "sans", "base"),
            sender: {
                margin: {
                    right: 8,
                },
                ...text(layer, "sans", "base", { weight: "bold" }),
            },
            timestamp: text(layer, "sans", "base", "disabled"),
            margin: { bottom: SPACING }
        },
        pending_message: {
            body: text(layer, "sans", "base"),
            sender: {
                margin: {
                    right: 8,
                },
                ...text(layer, "sans", "base", "disabled"),
            },
            timestamp: text(layer, "sans", "base"),
        },
        sign_in_prompt: {
            default: text(layer, "sans", "base"),
        }
    }
}
