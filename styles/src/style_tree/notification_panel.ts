import { background, text } from "./components"
import { icon_button } from "../component/icon_button"
import { useTheme } from "../theme"
import { interactive } from "../element"

export default function chat_panel(): any {
    const theme = useTheme()
    const layer = theme.middle

    return {
        background: background(layer),
        avatar: {
            icon_width: 24,
            icon_height: 24,
            corner_radius: 4,
            outer_width: 24,
            outer_corner_radius: 16,
        },
        read_text: text(layer, "sans", "base"),
        unread_text: text(layer, "sans", "base"),
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
        timestamp: text(layer, "sans", "base", "disabled"),
        avatar_container: {
            padding: {
                right: 6,
                left: 2,
                top: 2,
                bottom: 2,
            }
        },
        list: {

        },
        icon_button: icon_button({
            variant: "ghost",
            color: "variant",
            size: "sm",
        }),
        sign_in_prompt: {
            default: text(layer, "sans", "base"),
        }
    }
}
