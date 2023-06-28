import { ColorScheme } from "../theme/colorScheme"
import { background, border, text } from "./components"
import { interactive, toggleable } from "../element"
export default function dropdownMenu(colorScheme: ColorScheme): any {
    const layer = colorScheme.middle

    return {
        rowHeight: 30,
        background: background(layer),
        border: border(layer),
        shadow: colorScheme.popoverShadow,
        header: interactive({
            base: {
                ...text(layer, "sans", { size: "sm" }),
                secondaryText: text(layer, "sans", {
                    size: "sm",
                    color: "#aaaaaa",
                }),
                secondaryTextSpacing: 10,
                padding: { left: 8, right: 8, top: 2, bottom: 2 },
                cornerRadius: 6,
                background: background(layer, "on"),
            },
            state: {
                hovered: {
                    background: background(layer, "hovered"),
                },
                clicked: {
                    background: background(layer, "pressed"),
                },
            },
        }),
        sectionHeader: {
            ...text(layer, "sans", { size: "sm" }),
            padding: { left: 8, right: 8, top: 8, bottom: 8 },
        },
        item: toggleable({
            base: interactive({
                base: {
                    ...text(layer, "sans", { size: "sm" }),
                    secondaryTextSpacing: 10,
                    secondaryText: text(layer, "sans", { size: "sm" }),
                    padding: { left: 18, right: 18, top: 2, bottom: 2 },
                },
                state: {
                    hovered: {
                        background: background(layer, "hovered"),
                        ...text(layer, "sans", "hovered", { size: "sm" }),
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
                },
            },
        }),
    }
}
