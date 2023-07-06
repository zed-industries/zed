import { foreground, text } from "./components"
import { interactive } from "../element"
import { useTheme } from "../theme"

export default function update_notification(): any {
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
