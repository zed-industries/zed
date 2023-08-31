import { with_opacity } from "../theme/color"
import { background, border, text } from "./components"
import { interactive, toggleable } from "../element"
import { useTheme } from "../theme"

export default function picker(): any {
    const theme = useTheme()

    const container = {
        background: background(theme.lowest),
        border: border(theme.lowest),
        shadow: theme.modal_shadow,
        corner_radius: 12,
        padding: {
            bottom: 4,
        },
    }
    const input_editor = {
        placeholder_text: text(theme.lowest, "sans", "on", "disabled"),
        selection: theme.players[0],
        text: text(theme.lowest, "mono", "on"),
        border: border(theme.lowest, { bottom: true }),
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
    const empty_input_editor: any = { ...input_editor }
    delete empty_input_editor.border
    delete empty_input_editor.margin

    return {
        ...container,
        empty_container: {
            ...container,
            padding: {},
        },
        item: toggleable({
            base: interactive({
                base: {
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
                    corner_radius: 8,
                    text: text(theme.lowest, "sans", "variant"),
                    highlight_text: text(theme.lowest, "sans", "accent", {
                        weight: "bold",
                    }),
                },
                state: {
                    hovered: {
                        background: with_opacity(
                            background(theme.lowest, "hovered"),
                            0.5
                        ),
                    },
                    clicked: {
                        background: with_opacity(
                            background(theme.lowest, "pressed"),
                            0.5
                        ),
                    },
                },
            }),
            state: {
                active: {
                    default: {
                        background: with_opacity(
                            background(theme.lowest, "base", "active"),
                            0.5
                        ),
                    },
                    hovered: {
                        background: with_opacity(
                            background(theme.lowest, "hovered"),
                            0.5
                        ),
                    },
                    clicked: {
                        background: with_opacity(
                            background(theme.lowest, "pressed"),
                            0.5
                        ),
                    },
                },
            },
        }),

        input_editor,
        empty_input_editor,
        no_matches: {
            text: text(theme.lowest, "sans", "variant"),
            padding: {
                bottom: 8,
                left: 16,
                right: 16,
                top: 8,
            },
        },
        header: {
            text: text(theme.lowest, "sans", "variant", { size: "xs" }),

            margin: {
                top: 1,
                left: 8,
                right: 8,
            },
        },
        footer: interactive({
            base: {
                text: text(theme.lowest, "sans", "base", { size: "xs" }),
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
                corner_radius: 8,
                background: with_opacity(
                    background(theme.lowest, "active"),
                    0.5
                ),
            },
            state: {
                hovered: {
                    background: with_opacity(
                        background(theme.lowest, "hovered"),
                        0.5
                    ),
                },
                clicked: {
                    background: with_opacity(
                        background(theme.lowest, "pressed"),
                        0.5
                    ),
                },
            },
        }),
    }
}
