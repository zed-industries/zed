import { ColorScheme } from "../themes/common/colorScheme"
import { withOpacity } from "../utils/color"
import { background, border, text } from "./components"

export default function picker(colorScheme: ColorScheme) {
    let layer = colorScheme.lowest
    const container = {
        background: background(layer),
        border: border(layer),
        shadow: colorScheme.modalShadow,
        cornerRadius: 12,
        padding: {
            bottom: 4,
        },
    }
    const inputEditor = {
        placeholderText: text(layer, "sans", "on", "disabled"),
        selection: colorScheme.players[0],
        text: text(layer, "mono", "on"),
        border: border(layer, { bottom: true }),
        padding: {
            bottom: 8,
            left: 16,
            right: 16,
            top: 8,
        },
        margin: {
            bottom: 4,
        },
    }
    const emptyInputEditor = { ...inputEditor }
    delete emptyInputEditor.border
    delete emptyInputEditor.margin

    return {
        ...container,
        emptyContainer: {
            ...container,
            padding: {},
        },
        item: {
            padding: {
                bottom: 4,
                left: 12,
                right: 12,
                top: 4,
            },
            margin: {
                top: 1,
                left: 4,
                right: 4,
            },
            cornerRadius: 8,
            text: text(layer, "sans", "variant"),
            highlightText: text(layer, "sans", "accent", { weight: "bold" }),
            active: {
                background: withOpacity(
                    background(layer, "base", "active"),
                    0.5
                ),
                text: text(layer, "sans", "base", "active"),
                highlightText: text(layer, "sans", "accent", {
                    weight: "bold",
                }),
            },
            hover: {
                background: withOpacity(background(layer, "hovered"), 0.5),
            },
        },
        inputEditor,
        emptyInputEditor,
        noMatches: {
            text: text(layer, "sans", "variant"),
            padding: {
                bottom: 8,
                left: 16,
                right: 16,
                top: 8,
            },
        },
    }
}
