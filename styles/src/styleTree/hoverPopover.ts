import { ColorScheme } from "../theme/colorScheme"
import { background, border, foreground, text } from "./components"

export default function HoverPopover(colorScheme: ColorScheme) {
    let layer = colorScheme.middle
    let baseContainer = {
        background: background(layer),
        cornerRadius: 8,
        padding: {
            left: 8,
            right: 8,
            top: 4,
            bottom: 4,
        },
        shadow: colorScheme.popoverShadow,
        border: border(layer),
        margin: {
            left: -8,
        },
    }

    return {
        container: baseContainer,
        infoContainer: {
            ...baseContainer,
            background: background(layer, "accent"),
            border: border(layer, "accent"),
        },
        warningContainer: {
            ...baseContainer,
            background: background(layer, "warning"),
            border: border(layer, "warning"),
        },
        errorContainer: {
            ...baseContainer,
            background: background(layer, "negative"),
            border: border(layer, "negative"),
        },
        blockStyle: {
            padding: { top: 4 },
        },
        prose: text(layer, "sans", { size: "sm" }),
        diagnosticSourceHighlight: { color: foreground(layer, "accent") },
        highlight: colorScheme.ramps.neutral(0.5).alpha(0.2).hex(), // TODO: blend was used here. Replace with something better
    }
}
