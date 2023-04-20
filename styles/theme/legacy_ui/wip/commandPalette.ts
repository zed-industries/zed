import { ColorScheme } from "../themes/common/colorScheme"
import { withOpacity } from "../utils/color"
import { text, background } from "./components"

export default function commandPalette(colorScheme: ColorScheme) {
    let layer = colorScheme.highest
    return {
        keystrokeSpacing: 8,
        key: {
            text: text(layer, "mono", "variant", "default", { size: "xs" }),
            cornerRadius: 2,
            background: background(layer, "on"),
            padding: {
                top: 1,
                bottom: 1,
                left: 6,
                right: 6,
            },
            margin: {
                top: 1,
                bottom: 1,
                left: 2,
            },
            active: {
                text: text(layer, "mono", "on", "default", { size: "xs" }),
                background: withOpacity(background(layer, "on"), 0.2),
            },
        },
    }
}
