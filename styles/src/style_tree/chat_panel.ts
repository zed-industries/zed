import {
    background,
    border,
    border_color,
    foreground,
    text,
} from "./components"
import { interactive, toggleable } from "../element"
import { useTheme } from "../theme"
import collab_modals from "./collab_modals"
import { icon_button, toggleable_icon_button } from "../component/icon_button"
import { indicator } from "../component/indicator"

export default function contacts_panel(): any {
    const theme = useTheme()

    const CHANNEL_SPACING = 4 as const
    const NAME_MARGIN = 6 as const
    const SPACING = 12 as const
    const INDENT_SIZE = 8 as const
    const ITEM_HEIGHT = 28 as const

    const layer = theme.middle

    const input_editor = {
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
        margin: {
            left: SPACING,
            right: SPACING,
        },
    }

    const channel_name = {
        padding: {
            top: 4,
            bottom: 4,
            left: 4,
            right: 4,
        },
        hash: {
            ...text(layer, "sans", "base"),
        },
        name: text(layer, "sans", "base"),
    }

    return {
        background: background(layer),
        channel_select: {
            header: channel_name,
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
        input_editor,
        message: {
            body: text(layer, "sans", "base"),
            sender: {
                padding: {
                    left: 4,
                    right: 4,
                },
                ...text(layer, "sans", "base", "disabled"),
            },
            timestamp: text(layer, "sans", "base"),
        },
        pending_message: {
            body: text(layer, "sans", "base"),
            sender: {
                padding: {
                    left: 4,
                    right: 4,
                },
                ...text(layer, "sans", "base", "disabled"),
            },
            timestamp: text(layer, "sans", "base"),
        },
        sign_in_prompt: {
            default: text(layer, "sans", "base"),
        },
        timestamp: {
            body: text(layer, "sans", "base"),
            sender: {
                padding: {
                    left: 4,
                    right: 4,
                },
                ...text(layer, "sans", "base", "disabled"),
            }
        }
    }
}
