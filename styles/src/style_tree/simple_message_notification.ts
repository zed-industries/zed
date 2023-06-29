import { ColorScheme } from "../theme/color_scheme"
import { background, border, foreground, text } from "./components"
import { interactive } from "../element"

const headerPadding = 8

export default function simple_message_notification(
    colorScheme: ColorScheme
): unknown {
    const layer = colorScheme.middle
    return {
        message: {
            ...text(layer, "sans", { size: "xs" }),
            margin: { left: headerPadding, right: headerPadding },
        },
        actionMessage: interactive({
            base: {
                ...text(layer, "sans", { size: "xs" }),
                border: border(layer, "active"),
                corner_radius: 4,
                padding: {
                    top: 3,
                    bottom: 3,
                    left: 7,
                    right: 7,
                },

                margin: { left: headerPadding, top: 6, bottom: 6 },
            },
            state: {
                hovered: {
                    ...text(layer, "sans", "default", { size: "xs" }),
                    background: background(layer, "hovered"),
                    border: border(layer, "active"),
                },
            },
        }),
        dismissButton: interactive({
            base: {
                color: foreground(layer),
                icon_width: 8,
                iconHeight: 8,
                button_width: 8,
                buttonHeight: 8,
            },
            state: {
                hovered: {
                    color: foreground(layer, "hovered"),
                },
            },
        }),
    }
}
