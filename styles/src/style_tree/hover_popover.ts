import { ColorScheme } from "../theme/color_scheme"
import { background, border, foreground, text } from "./components"

export default function hover_popover(theme: ColorScheme): any {
    const base_container = {
        background: background(theme.middle),
        corner_radius: 8,
        padding: {
            left: 8,
            right: 8,
            top: 4,
            bottom: 4,
        },
        shadow: theme.popover_shadow,
        border: border(theme.middle),
        margin: {
            left: -8,
        },
    }

    return {
        container: base_container,
        info_container: {
            ...base_container,
            background: background(theme.middle, "accent"),
            border: border(theme.middle, "accent"),
        },
        warning_container: {
            ...base_container,
            background: background(theme.middle, "warning"),
            border: border(theme.middle, "warning"),
        },
        error_container: {
            ...base_container,
            background: background(theme.middle, "negative"),
            border: border(theme.middle, "negative"),
        },
        block_style: {
            padding: { top: 4 },
        },
        prose: text(theme.middle, "sans", { size: "sm" }),
        diagnostic_source_highlight: {
            color: foreground(theme.middle, "accent"),
        },
        highlight: theme.ramps.neutral(0.5).alpha(0.2).hex(), // TODO: blend was used here. Replace with something better
    }
}
