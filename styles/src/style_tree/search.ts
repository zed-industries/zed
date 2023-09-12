import { with_opacity } from "../theme/color"
import { background, border, foreground, text } from "./components"
import { interactive, toggleable } from "../element"
import { useTheme } from "../theme"
import { text_button } from "../component/text_button"

const search_results = () => {
    const theme = useTheme()

    return {
        // TODO: Add an activeMatchBackground on the rust side to differentiate between active and inactive
        match_background: with_opacity(
            foreground(theme.highest, "accent"),
            0.4
        ),
    }
}

export default function search(): any {
    const theme = useTheme()
    const SEARCH_ROW_SPACING = 12

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
            right: SEARCH_ROW_SPACING,
        },
        padding: {
            top: 4,
            bottom: 4,
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
        padding: { top: 4, bottom: 4 },

        option_button: toggleable({
            base: interactive({
                base: {
                    icon_width: 14,
                    button_width: 32,
                    color: foreground(theme.highest, "variant"),
                    background: background(theme.highest, "on"),
                    corner_radius: 2,
                    margin: { right: 2 },
                    border: {
                        width: 1,
                        color: background(theme.highest, "on"),
                    },
                    padding: {
                        left: 4,
                        right: 4,
                        top: 4,
                        bottom: 4,
                    },
                },
                state: {
                    hovered: {
                        ...text(theme.highest, "mono", "variant", "hovered"),
                        background: background(theme.highest, "on", "hovered"),
                        border: {
                            width: 1,
                            color: background(theme.highest, "on", "hovered"),
                        },
                    },
                    clicked: {
                        ...text(theme.highest, "mono", "variant", "pressed"),
                        background: background(theme.highest, "on", "pressed"),
                        border: {
                            width: 1,
                            color: background(theme.highest, "on", "pressed"),
                        },
                    },
                },
            }),
            state: {
                active: {
                    default: {
                        icon_width: 14,
                        button_width: 32,
                        color: foreground(theme.highest, "variant"),
                        background: background(theme.highest, "accent"),
                        border: border(theme.highest, "accent"),
                    },
                    hovered: {
                        background: background(
                            theme.highest,
                            "accent",
                            "hovered"
                        ),
                        border: border(theme.highest, "accent", "hovered"),
                    },
                    clicked: {
                        background: background(
                            theme.highest,
                            "accent",
                            "pressed"
                        ),
                        border: border(theme.highest, "accent", "pressed"),
                    },
                },
            },
        }),
        option_button_component: toggleable({
            base: interactive({
                base: {
                    icon_size: 14,
                    color: foreground(theme.highest, "variant"),

                    button_width: 32,
                    background: background(theme.highest, "on"),
                    corner_radius: 2,
                    margin: { right: 2 },
                    border: {
                        width: 1,
                        color: background(theme.highest, "on"),
                    },
                    padding: {
                        left: 4,
                        right: 4,
                        top: 4,
                        bottom: 4,
                    },
                },
                state: {
                    hovered: {
                        ...text(theme.highest, "mono", "variant", "hovered"),
                        background: background(theme.highest, "on", "hovered"),
                        border: {
                            width: 1,
                            color: background(theme.highest, "on", "hovered"),
                        },
                    },
                    clicked: {
                        ...text(theme.highest, "mono", "variant", "pressed"),
                        background: background(theme.highest, "on", "pressed"),
                        border: {
                            width: 1,
                            color: background(theme.highest, "on", "pressed"),
                        },
                    },
                },
            }),
            state: {
                active: {
                    default: {
                        icon_size: 14,
                        button_width: 32,
                        color: foreground(theme.highest, "variant"),
                        background: background(theme.highest, "accent"),
                        border: border(theme.highest, "accent"),
                    },
                    hovered: {
                        background: background(
                            theme.highest,
                            "accent",
                            "hovered"
                        ),
                        border: border(theme.highest, "accent", "hovered"),
                    },
                    clicked: {
                        background: background(
                            theme.highest,
                            "accent",
                            "pressed"
                        ),
                        border: border(theme.highest, "accent", "pressed"),
                    },
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
            ...text(theme.highest, "mono", { size: "sm" }),
            padding: {
                right: SEARCH_ROW_SPACING,
            },
        },
        option_button_group: {
            padding: {
                left: SEARCH_ROW_SPACING,
                right: SEARCH_ROW_SPACING,
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
        // Input Icon
        editor_icon: {
            icon: {
                color: foreground(theme.highest, "disabled"),
                asset: "icons/magnifying_glass.svg",
                dimensions: {
                    width: 14,
                    height: 14,
                },
            },
            container: {
                margin: { right: 4 },
                padding: { left: 1, right: 1 },
            },
        },
        // Toggle group buttons - Text | Regex | Semantic
        mode_button: toggleable({
            base: interactive({
                base: {
                    ...text(theme.highest, "mono", "variant", { size: "sm" }),
                    background: background(theme.highest, "variant"),

                    border: {
                        ...border(theme.highest, "on"),
                        left: false,
                        right: false,
                    },
                    margin: {
                        top: 1,
                        bottom: 1,
                    },
                    padding: {
                        left: 10,
                        right: 10,
                    },
                    corner_radius: 6,
                },
                state: {
                    hovered: {
                        ...text(theme.highest, "mono", "variant", "hovered", {
                            size: "sm",
                        }),
                        background: background(
                            theme.highest,
                            "variant",
                            "hovered"
                        ),
                        border: border(theme.highest, "on", "hovered"),
                    },
                    clicked: {
                        ...text(theme.highest, "mono", "variant", "pressed", {
                            size: "sm",
                        }),
                        background: background(
                            theme.highest,
                            "variant",
                            "pressed"
                        ),
                        border: border(theme.highest, "on", "pressed"),
                    },
                },
            }),
            state: {
                active: {
                    default: {
                        ...text(theme.highest, "mono", "on", { size: "sm" }),
                        background: background(theme.highest, "on"),
                    },
                    hovered: {
                        ...text(theme.highest, "mono", "on", "hovered", {
                            size: "sm",
                        }),
                        background: background(theme.highest, "on", "hovered"),
                    },
                    clicked: {
                        ...text(theme.highest, "mono", "on", "pressed", {
                            size: "sm",
                        }),
                        background: background(theme.highest, "on", "pressed"),
                    },
                },
            },
        }),
        // Next/Previous Match buttons
        // HACK: This is not how disabled elements should be created
        // Disabled elements should use a disabled state of an interactive element, not a toggleable element with the inactive state being disabled
        nav_button: toggleable({
            state: {
                inactive: interactive({
                    base: {
                        background: background(theme.highest, "disabled"),
                        text: text(theme.highest, "mono", "disabled"),
                        corner_radius: 6,
                        border: {
                            ...border(theme.highest, "disabled"),
                            left: false,
                            right: false,
                        },
                        margin: {
                            top: 1,
                            bottom: 1,
                        },
                        padding: {
                            left: 10,
                            right: 10,
                        },
                    },
                    state: {
                        hovered: {},
                    },
                }),
                active: interactive({
                    base: {
                        text: text(theme.highest, "mono", "on"),
                        background: background(theme.highest, "on"),
                        corner_radius: 6,
                        border: {
                            ...border(theme.highest, "on"),
                            left: false,
                            right: false,
                        },
                        margin: {
                            top: 1,
                            bottom: 1,
                        },
                        padding: {
                            left: 10,
                            right: 10,
                        },
                    },
                    state: {
                        hovered: {
                            ...text(theme.highest, "mono", "on", "hovered"),
                            background: background(
                                theme.highest,
                                "on",
                                "hovered"
                            ),
                            border: border(theme.highest, "on", "hovered"),
                        },
                        clicked: {
                            ...text(theme.highest, "mono", "on", "pressed"),
                            background: background(
                                theme.highest,
                                "on",
                                "pressed"
                            ),
                            border: border(theme.highest, "on", "pressed"),
                        },
                    },
                }),
            },
        }),
        search_bar_row_height: 34,
        search_row_spacing: 8,
        option_button_height: 22,
        modes_container: {},
        replace_icon: {
            icon: {
                color: foreground(theme.highest, "disabled"),
                asset: "icons/replace.svg",
                dimensions: {
                    width: 14,
                    height: 14,
                },
            },
            container: {
                margin: { right: 4 },
                padding: { left: 1, right: 1 },
            },
        },
        replace_next_button: interactive({
            base: {
                icon_size: 14,
                color: foreground(theme.highest, "variant"),

                button_width: 32,
                background: background(theme.highest, "on"),
                corner_radius: 2,
                margin: { right: 2 },
                border: {
                    width: 1,
                    color: background(theme.highest, "on"),
                },
                padding: {
                    left: 4,
                    right: 4,
                    top: 4,
                    bottom: 4,
                },
            },
            state: {
                hovered: {
                    ...text(theme.highest, "mono", "variant", "hovered"),
                    background: background(theme.highest, "on", "hovered"),
                    border: {
                        width: 1,
                        color: background(theme.highest, "on", "hovered"),
                    },
                },
                clicked: {
                    ...text(theme.highest, "mono", "variant", "pressed"),
                    background: background(theme.highest, "on", "pressed"),
                    border: {
                        width: 1,
                        color: background(theme.highest, "on", "pressed"),
                    },
                },
            },
        }),
        replace_all_button: interactive({
            base: {
                icon_size: 14,
                color: foreground(theme.highest, "variant"),

                button_width: 32,
                background: background(theme.highest, "on"),
                corner_radius: 2,
                margin: { right: 2 },
                border: {
                    width: 1,
                    color: background(theme.highest, "on"),
                },
                padding: {
                    left: 4,
                    right: 4,
                    top: 4,
                    bottom: 4,
                },
            },
            state: {
                hovered: {
                    ...text(theme.highest, "mono", "variant", "hovered"),
                    background: background(theme.highest, "on", "hovered"),
                    border: {
                        width: 1,
                        color: background(theme.highest, "on", "hovered"),
                    },
                },
                clicked: {
                    ...text(theme.highest, "mono", "variant", "pressed"),
                    background: background(theme.highest, "on", "pressed"),
                    border: {
                        width: 1,
                        color: background(theme.highest, "on", "pressed"),
                    },
                },
            },
        }),
        select_all_button: interactive({
            base: {
                icon_size: 14,
                color: foreground(theme.highest, "variant"),

                button_width: 32,
                background: background(theme.highest, "on"),
                corner_radius: 2,
                margin: { right: 2 },
                border: {
                    width: 1,
                    color: background(theme.highest, "on"),
                },
                padding: {
                    left: 4,
                    right: 4,
                    top: 4,
                    bottom: 4,
                },
            },
            state: {
                hovered: {
                    ...text(theme.highest, "mono", "variant", "hovered"),
                    background: background(theme.highest, "on", "hovered"),
                    border: {
                        width: 1,
                        color: background(theme.highest, "on", "hovered"),
                    },
                },
                clicked: {
                    ...text(theme.highest, "mono", "variant", "pressed"),
                    background: background(theme.highest, "on", "pressed"),
                    border: {
                        width: 1,
                        color: background(theme.highest, "on", "pressed"),
                    },
                },
            },
        }),
        ...search_results(),
    }
}
