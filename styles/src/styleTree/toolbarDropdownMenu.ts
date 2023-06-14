import { ColorScheme } from "../theme/colorScheme"
import { background, border, text } from "./components"

export default function dropdownMenu(colorScheme: ColorScheme) {
    let layer = colorScheme.middle

    return {
        rowHeight: 30,
        background: background(layer),
        border: border(layer),
        shadow: colorScheme.popoverShadow,
        header: {
            ...text(layer, "sans", { size: "sm" }),
            secondaryText: text(layer, "sans", { size: "sm", color: "#aaaaaa" }),
            secondaryTextSpacing: 10,
            padding: { left: 8, right: 8, top: 2, bottom: 2 },
            cornerRadius: 6,
            background: background(layer, "on"),
            border: border(layer, "on", { overlay: true }),
            hover: {
                background: background(layer, "hovered"),
                ...text(layer, "sans", "hovered", { size: "sm" }),
            }
        },
        sectionHeader: {
            ...text(layer, "sans", { size: "sm" }),
            padding: { left: 8, right: 8, top: 8, bottom: 8 },
        },
        item: {
            ...text(layer, "sans", { size: "sm" }),
            secondaryTextSpacing: 10,
            secondaryText: text(layer, "sans", { size: "sm" }),
            padding: { left: 18, right: 18, top: 2, bottom: 2 },
            hover: {
                background: background(layer, "hovered"),
                ...text(layer, "sans", "hovered", { size: "sm" }),
            },
            active: {
                background: background(layer, "active"),
            },
            activeHover: {
                background: background(layer, "active"),
            },
        },
    }
}
