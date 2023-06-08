import picker from "./picker"
import { ColorScheme } from "../theme/colorScheme"
import { background, border, foreground, text } from "./components"

export default function contactFinder(colorScheme: ColorScheme): any {
    let layer = colorScheme.middle

    const sideMargin = 6
    const contactButton = {
        background: background(layer, "variant"),
        color: foreground(layer, "variant"),
        iconWidth: 8,
        buttonWidth: 16,
        cornerRadius: 8,
    }

    const pickerStyle = picker(colorScheme)
    const pickerInput = {
        background: background(layer, "on"),
        cornerRadius: 6,
        text: text(layer, "mono"),
        placeholderText: text(layer, "mono", "on", "disabled", { size: "xs" }),
        selection: colorScheme.players[0],
        border: border(layer),
        padding: {
            bottom: 4,
            left: 8,
            right: 8,
            top: 4,
        },
        margin: {
            left: sideMargin,
            right: sideMargin,
        },
    }

    return {
        picker: {
            emptyContainer: {},
            item: {
                ...pickerStyle.item,
                margin: { left: sideMargin, right: sideMargin },
            },
            noMatches: pickerStyle.noMatches,
            inputEditor: pickerInput,
            emptyInputEditor: pickerInput,
        },
        rowHeight: 28,
        contactAvatar: {
            cornerRadius: 10,
            width: 18,
        },
        contactUsername: {
            padding: {
                left: 8,
            },
        },
        contactButton: {
            ...contactButton,
            hover: {
                background: background(layer, "variant", "hovered"),
            },
        },
        disabledContactButton: {
            ...contactButton,
            background: background(layer, "disabled"),
            color: foreground(layer, "disabled"),
        },
    }
}
