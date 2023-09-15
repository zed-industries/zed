import { background, border, foreground, text } from "./components"
import { interactive } from "../element"
import { useTheme } from "../theme"

export default function simple_message_notification(): any {
    const theme = useTheme()

    const header_padding = 8

    return {
        message: {
            ...text(theme.middle, "sans", { size: "xs" }),
            margin: { left: header_padding, right: header_padding },
        },
        action_message: interactive({
            base: {
                ...text(theme.middle, "sans", { size: "xs" }),
                border: border(theme.middle, "active"),
                corner_radius: 4,
                padding: {
                    top: 3,
                    bottom: 3,
                    left: 7,
                    right: 7,
                },

                margin: { left: header_padding, top: 6, bottom: 6 },
            },
            state: {
                hovered: {
                    ...text(theme.middle, "sans", "default", { size: "xs" }),
                    background: background(theme.middle, "hovered"),
                    border: border(theme.middle, "active"),
                },
            },
        }),
        dismiss_button: interactive({
            base: {
                color: foreground(theme.middle),
                icon_width: 14,
                icon_height: 14,
                button_width: 14,
                button_height: 14,
            },
            state: {
                hovered: {
                    color: foreground(theme.middle, "hovered"),
                },
            },
        }),
    }
}
