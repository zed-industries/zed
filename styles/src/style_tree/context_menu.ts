import { ColorScheme } from "../theme/color_scheme"
import { background, border, borderColor, text } from "./components"
import { interactive, toggleable } from "../element"

export default function context_menu(colorScheme: ColorScheme): any {
    const layer = colorScheme.middle
    return {
        background: background(layer),
        cornerRadius: 10,
        padding: 4,
        shadow: colorScheme.popoverShadow,
        border: border(layer),
        keystrokeMargin: 30,
        item: toggleable({
            base: interactive({
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
                    clicked: {
                        background: background(layer, "pressed"),
                    },
                },
            }),
            state: {
                active: {
                    default: {
                        background: background(layer, "active"),
                    },
                    hovered: {
                        background: background(layer, "hovered"),
                    },
                    clicked: {
                        background: background(layer, "pressed"),
                    },
                },
            },
        }),

        separator: {
            background: borderColor(layer),
            margin: { top: 2, bottom: 2 },
        },
    }
}
