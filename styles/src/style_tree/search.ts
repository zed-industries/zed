import { ColorScheme } from "../theme/color_scheme"
import { withOpacity } from "../theme/color"
import { background, border, foreground, text } from "./components"
import { interactive, toggleable } from "../element"

export default function search(colorScheme: ColorScheme): any {
    const layer = colorScheme.highest

    // Search input
    const editor = {
        background: background(layer),
        corner_radius: 8,
        minWidth: 200,
        maxWidth: 500,
        placeholderText: text(layer, "mono", "disabled"),
        selection: colorScheme.players[0],
        text: text(layer, "mono", "default"),
        border: border(layer),
        margin: {
            right: 12,
        },
        padding: {
            top: 3,
            bottom: 3,
            left: 12,
            right: 8,
        },
    }

    const includeExcludeEditor = {
        ...editor,
        minWidth: 100,
        maxWidth: 250,
    }

    return {
        // TODO: Add an activeMatchBackground on the rust side to differentiate between active and inactive
        matchBackground: withOpacity(foreground(layer, "accent"), 0.4),
        optionButton: toggleable({
            base: interactive({
                base: {
                    ...text(layer, "mono", "on"),
                    background: background(layer, "on"),
                    corner_radius: 6,
                    border: border(layer, "on"),
                    margin: {
                        right: 4,
                    },
                    padding: {
                        bottom: 2,
                        left: 10,
                        right: 10,
                        top: 2,
                    },
                },
                state: {
                    hovered: {
                        ...text(layer, "mono", "on", "hovered"),
                        background: background(layer, "on", "hovered"),
                        border: border(layer, "on", "hovered"),
                    },
                    clicked: {
                        ...text(layer, "mono", "on", "pressed"),
                        background: background(layer, "on", "pressed"),
                        border: border(layer, "on", "pressed"),
                    },
                },
            }),
            state: {
                active: {
                    default: {
                        ...text(layer, "mono", "accent"),
                    },
                    hovered: {
                        ...text(layer, "mono", "accent", "hovered"),
                    },
                    clicked: {
                        ...text(layer, "mono", "accent", "pressed"),
                    },
                },
            },
        }),
        editor,
        invalidEditor: {
            ...editor,
            border: border(layer, "negative"),
        },
        includeExcludeEditor,
        invalidIncludeExcludeEditor: {
            ...includeExcludeEditor,
            border: border(layer, "negative"),
        },
        matchIndex: {
            ...text(layer, "mono", "variant"),
            padding: {
                left: 6,
            },
        },
        optionButtonGroup: {
            padding: {
                left: 12,
                right: 12,
            },
        },
        includeExcludeInputs: {
            ...text(layer, "mono", "variant"),
            padding: {
                right: 6,
            },
        },
        resultsStatus: {
            ...text(layer, "mono", "on"),
            size: 18,
        },
        dismissButton: interactive({
            base: {
                color: foreground(layer, "variant"),
                icon_width: 12,
                button_width: 14,
                padding: {
                    left: 10,
                    right: 10,
                },
            },
            state: {
                hovered: {
                    color: foreground(layer, "hovered"),
                },
                clicked: {
                    color: foreground(layer, "pressed"),
                },
            },
        }),
    }
}
