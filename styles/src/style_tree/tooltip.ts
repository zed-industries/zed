import { useTheme } from "../theme"
import { background, border, text } from "./components"

export default function tooltip(): any {
    const theme = useTheme()

    return {
        background: background(theme.middle),
        border: border(theme.middle),
        padding: { top: 4, bottom: 4, left: 8, right: 8 },
        margin: { top: 6, left: 6 },
        shadow: theme.popover_shadow,
        corner_radius: 6,
        text: text(theme.middle, "sans", { size: "xs" }),
        keystroke: {
            background: background(theme.middle, "on"),
            corner_radius: 4,
            margin: { left: 6 },
            padding: { left: 4, right: 4 },
            ...text(theme.middle, "mono", "on", { size: "xs", weight: "bold" }),
        },
        max_text_width: 200,
    }
}
