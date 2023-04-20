import { ColorScheme } from "../themes/common/colorScheme"
import { background, foreground, text } from "./components"

const avatarSize = 12
const headerPadding = 8

export default function contactNotification(colorScheme: ColorScheme): Object {
    let layer = colorScheme.lowest
    return {
        headerAvatar: {
            height: avatarSize,
            width: avatarSize,
            cornerRadius: 6,
        },
        headerMessage: {
            ...text(layer, "sans", { size: "xs" }),
            margin: { left: headerPadding, right: headerPadding },
        },
        headerHeight: 18,
        bodyMessage: {
            ...text(layer, "sans", { size: "xs" }),
            margin: { left: avatarSize + headerPadding, top: 6, bottom: 6 },
        },
        button: {
            ...text(layer, "sans", "on", { size: "xs" }),
            background: background(layer, "on"),
            padding: 4,
            cornerRadius: 6,
            margin: { left: 6 },
            hover: {
                background: background(layer, "on", "hovered"),
            },
        },
        dismissButton: {
            color: foreground(layer, "variant"),
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
