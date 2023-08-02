import { with_opacity } from "../theme/color"
import { background, border, foreground, text } from "./components"
import { interactive, toggleable } from "../element"
import { useTheme } from "../theme"

export default function search(): any {
    const theme = useTheme()

    // Search input
    const editor = {
        background: background(theme.highest),
        corner_radius: 8,
        min_width: 200,
        max_width: 500,
        placeholder_text: text(theme.highest, "mono", "disabled"),
        selection: theme.players[0],
        text: text(theme.highest, "mono", "default"),
        border: border(theme.highest),
        margin: {
            right: 12,
        },
        padding: {
            top: 3,
            bottom: 3,
            left: 10,
            right: 4,
        },
    }

    const include_exclude_editor = {
        ...editor,
        min_width: 100,
        max_width: 250,
    }

    return {
        // TODO: Add an activeMatchBackground on the rust side to differentiate between active and inactive
        match_background: with_opacity(
            foreground(theme.highest, "accent"),
            0.4
        ),
        option_button: toggleable({
            base: interactive({
                base: {
                    ...text(theme.highest, "mono", "on"),
                    background: background(theme.highest, "on"),

                    border: border(theme.highest, "on"),

                    padding: {
                        bottom: 6,
                        left: 6,
                        right: 6,
                        top: 6,
                    },
                },
                state: {
                    hovered: {
                        ...text(theme.highest, "mono", "on", "hovered"),
                        background: background(theme.highest, "on", "hovered"),
                        border: border(theme.highest, "on", "hovered"),
                    },
                    clicked: {
                        ...text(theme.highest, "mono", "on", "pressed"),
                        background: background(theme.highest, "on", "pressed"),
                        border: border(theme.highest, "on", "pressed"),
                    },
                },
            }),
            state: {
                active: {
                    default: {
                        ...text(theme.highest, "mono", "accent"),
                    },
                    hovered: {
                        ...text(theme.highest, "mono", "accent", "hovered"),
                    },
                    clicked: {
                        ...text(theme.highest, "mono", "accent", "pressed"),
                    },
                },
            },
        }),
        action_button: interactive({
            base: {
                ...text(theme.highest, "mono", "on"),
                background: background(theme.highest, "on"),
                corner_radius: 6,
                border: border(theme.highest, "on"),
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
                    ...text(theme.highest, "mono", "on", "hovered"),
                    background: background(theme.highest, "on", "hovered"),
                    border: border(theme.highest, "on", "hovered"),
                },
                clicked: {
                    ...text(theme.highest, "mono", "on", "pressed"),
                    background: background(theme.highest, "on", "pressed"),
                    border: border(theme.highest, "on", "pressed"),
                },
            },
        }),
        editor,
        invalid_editor: {
            ...editor,
            border: border(theme.highest, "negative"),
        },
        include_exclude_editor,
        invalid_include_exclude_editor: {
            ...include_exclude_editor,
            border: border(theme.highest, "negative"),
        },
        match_index: {
            ...text(theme.highest, "mono", "variant"),
            padding: {
                left: 6,
            },
        },
        option_button_group: {
            padding: {
                left: 12,
                right: 12,
            },
        },
        include_exclude_inputs: {
            ...text(theme.highest, "mono", "variant"),
            padding: {
                right: 6,
            },
        },
        major_results_status: {
            ...text(theme.highest, "mono", "on"),
            size: 15,
        },
        minor_results_status: {
            ...text(theme.highest, "mono", "variant"),
            size: 13,
        },
        dismiss_button: interactive({
            base: {
                color: foreground(theme.highest, "variant"),
                icon_width: 12,
                button_width: 14,
                padding: {
                    left: 10,
                    right: 10,
                },
            },
            state: {
                hovered: {
                    color: foreground(theme.highest, "hovered"),
                },
                clicked: {
                    color: foreground(theme.highest, "pressed"),
                },
            },
        }),
        editor_icon: {
            icon: {
                color: foreground(theme.highest, "variant"),
                asset: "icons/magnifying_glass_12.svg",
                dimensions: {
                    width: 12,
                    height: 12,
                }
            },
            container: {
                margin: { right: 6 },
                padding: { left: 4 }
            }
        }
    }
}
