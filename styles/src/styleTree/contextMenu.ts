import { ColorScheme } from "../theme/colorScheme"
import { background, border, borderColor, text } from "./components"
import { interactive } from "../element"
import { toggleable } from "./toggle"

export default function contextMenu(colorScheme: ColorScheme) {
    let layer = colorScheme.middle
    return {
        background: background(layer),
        cornerRadius: 10,
        padding: 4,
        shadow: colorScheme.popoverShadow,
        border: border(layer),
        keystrokeMargin: 30,
        item: toggleable(
            interactive({
                base: {
                    iconSpacing: 8,
                    iconWidth: 14,
                    padding: { left: 6, right: 6, top: 2, bottom: 2 },
                    cornerRadius: 6,
                    label: text(layer, "sans", { size: "sm" }),
                    keystroke: {
                        ...text(layer, "sans", "variant", {
                            size: "sm",
                            weight: "bold",
                        }),
                        padding: { left: 3, right: 3 },
                    },
                },
                state: {
                    hovered: {
                        background: background(layer, "hovered"),
                        label: text(layer, "sans", "hovered", { size: "sm" }),
                        keystroke: {
                            ...text(layer, "sans", "hovered", {
                                size: "sm",
                                weight: "bold",
                            }),
                            padding: { left: 3, right: 3 },
                        },
                    },
                },
            }),
            {
                default: {
                    background: background(layer, "active"),
                },
                hovered: {
                    background: background(layer, "active"),
                },
            }
        ),

        separator: {
            background: borderColor(layer),
            margin: { top: 2, bottom: 2 },
        },
    }
}
