import { ColorScheme } from "../theme/color_scheme"
import { foreground, text } from "./components"
import { interactive } from "../element"


export default function update_notification(theme: ColorScheme): any {
    const header_padding = 8

    return {
        message: {
            ...text(theme.middle, "sans", { size: "xs" }),
            margin: { left: header_padding, right: header_padding },
        },
        action_message: interactive({
            base: {
                ...text(theme.middle, "sans", { size: "xs" }),
                margin: { left: header_padding, top: 6, bottom: 6 },
            },
            state: {
                hovered: {
                    color: foreground(theme.middle, "hovered"),
                },
            },
        }),
        dismiss_button: interactive({
            base: {
                color: foreground(theme.middle),
                icon_width: 8,
                icon_height: 8,
                button_width: 8,
                button_height: 8,
            },
            state: {
                hovered: {
                    color: foreground(theme.middle, "hovered"),
                },
            },
        }),
    }
}
