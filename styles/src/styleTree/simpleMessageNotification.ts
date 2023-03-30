import { ColorScheme } from "../themes/common/colorScheme"
import { background, border, foreground, text } from "./components"

const headerPadding = 8

export default function simpleMessageNotification(
    colorScheme: ColorScheme
): Object {
    let layer = colorScheme.middle
    return {
        message: {
            ...text(layer, "sans", { size: "xs" }),
            margin: { left: headerPadding, right: headerPadding },
        },
        actionMessage: {
            ...text(layer, "sans", { size: "xs" }),
            border: border(layer, "active"),
            cornerRadius: 4,
            padding: {
                top: 3,
                bottom: 3,
                left: 7,
                right: 7,
            },


            margin: { left: headerPadding, top: 6, bottom: 6 },
            hover: {
                ...text(layer, "sans", "default", { size: "xs" }),
                background: background(layer, "hovered"),
                border: border(layer, "active"),
            },
        },
        dismissButton: {
            color: foreground(layer),
            iconWidth: 8,
            iconHeight: 8,
            buttonWidth: 8,
            buttonHeight: 8,
            hover: {
                color: foreground(layer, "hovered"),
            },
        },
    }
}
