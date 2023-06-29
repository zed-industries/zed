import { ColorScheme } from "../theme/color_scheme"
import { background, border, text } from "./components"
import { interactive } from "../element"

export default function feedback(theme: ColorScheme): any {
    return {
        submit_button: interactive({
            base: {
                ...text(theme.highest, "mono", "on"),
                background: background(theme.highest, "on"),
                corner_radius: 6,
                border: border(theme.highest, "on"),
                margin: {
                    right: 4,
                },
                padding: {
                    bottom: 2,
                    left: 10,
                    right: 10,
                    top: 2,
                },
            },
            state: {
                clicked: {
                    ...text(theme.highest, "mono", "on", "pressed"),
                    background: background(theme.highest, "on", "pressed"),
                    border: border(theme.highest, "on", "pressed"),
                },
                hovered: {
                    ...text(theme.highest, "mono", "on", "hovered"),
                    background: background(theme.highest, "on", "hovered"),
                    border: border(theme.highest, "on", "hovered"),
                },
            },
        }),
        button_margin: 8,
        info_text_default: text(theme.highest, "sans", "default", { size: "xs" }),
        link_text_default: text(theme.highest, "sans", "default", {
            size: "xs",
            underline: true,
        }),
        link_text_hover: text(theme.highest, "sans", "hovered", {
            size: "xs",
            underline: true,
        }),
    }
}
