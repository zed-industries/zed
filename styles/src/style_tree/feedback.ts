import { background, border, text } from "./components"
import { interactive } from "../element"
import { useTheme } from "../theme"

export default function feedback(): any {
    const theme = useTheme()

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
                disabled: {
                    ...text(theme.highest, "mono", "on", "disabled"),
                    background: background(theme.highest, "on", "disabled"),
                    border: border(theme.highest, "on", "disabled"),
                },
            },
        }),
        button_margin: 8,
        info_text_default: text(theme.highest, "sans", "default", {
            size: "xs",
        }),
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
