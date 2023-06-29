import { ColorScheme } from "../theme/color_scheme"
import { background, foreground, text } from "./components"
import { interactive } from "../element"
const avatarSize = 12
const headerPadding = 8

export default function contact_notification(theme: ColorScheme): any {
    return {
        header_avatar: {
            height: avatarSize,
            width: avatarSize,
            corner_radius: 6,
        },
        header_message: {
            ...text(theme.lowest, "sans", { size: "xs" }),
            margin: { left: headerPadding, right: headerPadding },
        },
        header_height: 18,
        body_message: {
            ...text(theme.lowest, "sans", { size: "xs" }),
            margin: { left: avatarSize + headerPadding, top: 6, bottom: 6 },
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
                iconHeight: 8,
                button_width: 8,
                buttonHeight: 8,
                hover: {
                    color: foreground(theme.lowest, "hovered"),
                },
            },
        },
    }
}
