import { ColorScheme } from "../theme/color_scheme"
import { background, foreground, text } from "./components"
import { interactive } from "../element"

export default function contact_notification(theme: ColorScheme): any {
    const avatar_size = 12
    const header_padding = 8

    return {
        header_avatar: {
            height: avatar_size,
            width: avatar_size,
            corner_radius: 6,
        },
        header_message: {
            ...text(theme.lowest, "sans", { size: "xs" }),
            margin: { left: header_padding, right: header_padding },
        },
        header_height: 18,
        body_message: {
            ...text(theme.lowest, "sans", { size: "xs" }),
            margin: { left: avatar_size + header_padding, top: 6, bottom: 6 },
        },
        button: interactive({
            base: {
                ...text(theme.lowest, "sans", "on", { size: "xs" }),
                background: background(theme.lowest, "on"),
                padding: 4,
                corner_radius: 6,
                margin: { left: 6 },
            },

            state: {
                hovered: {
                    background: background(theme.lowest, "on", "hovered"),
                },
            },
        }),

        dismiss_button: {
            default: {
                color: foreground(theme.lowest, "variant"),
                icon_width: 8,
                icon_height: 8,
                button_width: 8,
                button_height: 8,
                hover: {
                    color: foreground(theme.lowest, "hovered"),
                },
            },
        },
    }
}
