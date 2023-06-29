import { ColorScheme } from "../theme/color_scheme"
import { foreground, text } from "./components"
import { interactive } from "../element"

const headerPadding = 8

export default function update_notification(colorScheme: ColorScheme): any {
    const layer = colorScheme.middle
    return {
        message: {
            ...text(layer, "sans", { size: "xs" }),
            margin: { left: headerPadding, right: headerPadding },
        },
        actionMessage: interactive({
            base: {
                ...text(layer, "sans", { size: "xs" }),
                margin: { left: headerPadding, top: 6, bottom: 6 },
            },
            state: {
                hovered: {
                    color: foreground(layer, "hovered"),
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
