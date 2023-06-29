import { ColorScheme } from "../theme/color_scheme"
import { background, border, text } from "./components"

export default function tooltip(colorScheme: ColorScheme): any {
    const layer = colorScheme.middle
    return {
        background: background(layer),
        border: border(layer),
        padding: { top: 4, bottom: 4, left: 8, right: 8 },
        margin: { top: 6, left: 6 },
        shadow: colorScheme.popover_shadow,
        corner_radius: 6,
        text: text(layer, "sans", { size: "xs" }),
        keystroke: {
            background: background(layer, "on"),
            corner_radius: 4,
            margin: { left: 6 },
            padding: { left: 4, right: 4 },
            ...text(layer, "mono", "on", { size: "xs", weight: "bold" }),
        },
        maxTextWidth: 200,
    }
}
