import {
    background,
    border,
    border_color,
    foreground,
    text,
} from "./components"
import { interactive, toggleable } from "../element"
import { useTheme } from "../theme"
import collab_modals from "./collab_modals"
import { text_button } from "../component/text_button"
import { toggleable_icon_button } from "../component/icon_button"

export default function contacts_panel(): any {
    const theme = useTheme()

    const NAME_MARGIN = 6 as const
    const SPACING = 12 as const
    const INDENT_SIZE = 8 as const
    const ITEM_HEIGHT = 28 as const

    const layer = theme.middle

    const contact_button = {
        background: background(layer, "on"),
        color: foreground(layer, "on"),
        icon_width: 8,
        button_width: 16,
        corner_radius: 8,
    }

    const project_row = {
        guest_avatar_spacing: 4,
        height: 24,
        guest_avatar: {
            corner_radius: 8,
            width: 14,
        },
        name: {
            ...text(layer, "ui_sans", { size: "sm" }),
            margin: {
                left: NAME_MARGIN,
                right: 4,
            },
        },
        guests: {
            margin: {
                left: NAME_MARGIN,
                right: NAME_MARGIN,
            },
        },
        padding: {
            left: SPACING,
            right: SPACING,
        },
    }

    const icon_style = {
        color: foreground(layer, "variant"),
        width: 14,
    }

    const header_icon_button = toggleable_icon_button(theme, {
        layer: theme.middle,
        variant: "ghost",
    })

    const subheader_row = toggleable({
        base: interactive({
            base: {
                ...text(layer, "ui_sans", { size: "sm" }),
                padding: {
                    left: SPACING,
                    right: SPACING,
                },
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
        state: {
            active: {
                default: {
                    ...text(layer, "ui_sans", "active", { size: "sm" }),
                    background: background(layer, "active"),
                },
                clicked: {
                    background: background(layer, "pressed"),
                },
            },
        },
    })

    const filter_input = {
        background: background(layer, "on"),
        corner_radius: 6,
        text: text(layer, "ui_sans", "base"),
        placeholder_text: text(layer, "ui_sans", "base", "disabled", {
            size: "xs",
        }),
        selection: theme.players[0],
        border: border(layer, "on"),
        padding: {
            bottom: 4,
            left: 8,
            right: 8,
            top: 4,
        },
        margin: {
            left: SPACING,
            right: SPACING,
        },
    }

    return {
        ...collab_modals(),
        log_in_button: text_button(),
        background: background(layer),
        padding: {
            top: SPACING,
        },
        user_query_editor: filter_input,
        channel_hash: icon_style,
        user_query_editor_height: 33,
        add_contact_button: header_icon_button,
        add_channel_button: header_icon_button,
        leave_call_button: header_icon_button,
        row_height: ITEM_HEIGHT,
        channel_indent: INDENT_SIZE,
        section_icon_size: 14,
        header_row: {
            ...text(layer, "ui_sans", { size: "sm", weight: "bold" }),
            margin: { top: SPACING },
            padding: {
                left: SPACING,
                right: SPACING,
            },
        },
        subheader_row,
        leave_call: interactive({
            base: {
                background: background(layer),
                border: border(layer),
                corner_radius: 6,
                margin: {
                    top: 1,
                },
                padding: {
                    top: 1,
                    bottom: 1,
                    left: 7,
                    right: 7,
                },
                ...text(layer, "sans", "variant", { size: "xs" }),
            },
            state: {
                hovered: {
                    ...text(layer, "sans", "hovered", { size: "xs" }),
                    background: background(layer, "hovered"),
                    border: border(layer, "hovered"),
                },
            },
        }),
        contact_row: toggleable({
            base: interactive({
                base: {
                    padding: {
                        left: SPACING,
                        right: SPACING,
                    },
                },
                state: {
                    clicked: {
                        background: background(layer, "pressed"),
                    },
                },
            }),
            state: {
                inactive: {
                    hovered: {
                        background: background(layer, "hovered"),
                    },
                },
                active: {
                    default: {
                        ...text(layer, "ui_sans", "active", { size: "sm" }),
                        background: background(layer, "active"),
                    },
                    clicked: {
                        background: background(layer, "pressed"),
                    },
                },
            },
        }),
        list_empty_label_container: {
            margin: {
                left: 5,
            }
        },
        list_empty_icon: {
            color: foreground(layer, "on"),
            width: 16,
        },
        list_empty_state: toggleable({
            base: interactive({
                base: {
                    ...text(layer, "ui_sans", "variant", { size: "sm" }),
                    padding: SPACING

                },
                state: {
                    clicked: {
                        background: background(layer, "pressed"),
                    },
                },
            }),
            state: {
                inactive: {
                    hovered: {
                        background: background(layer, "hovered"),
                    },
                },
                active: {
                    default: {
                        ...text(layer, "ui_sans", "active", { size: "sm" }),
                        background: background(layer, "active"),
                    },
                    clicked: {
                        background: background(layer, "pressed"),
                    },
                },
            },
        }),
        contact_avatar: {
            corner_radius: 10,
            width: 18,
        },
        contact_status_free: {
            corner_radius: 4,
            padding: 4,
            margin: { top: 12, left: 12 },
            background: foreground(layer, "positive"),
        },
        contact_status_busy: {
            corner_radius: 4,
            padding: 4,
            margin: { top: 12, left: 12 },
            background: foreground(layer, "negative"),
        },
        contact_username: {
            ...text(layer, "ui_sans", { size: "sm" }),
            margin: {
                left: NAME_MARGIN,
            },
        },
        contact_button_spacing: NAME_MARGIN,
        contact_button: interactive({
            base: { ...contact_button },
            state: {
                hovered: {
                    background: background(layer, "hovered"),
                },
            },
        }),
        disabled_button: {
            ...contact_button,
            background: background(layer, "on"),
            color: foreground(layer, "on"),
        },
        calling_indicator: {
            ...text(layer, "mono", "variant", { size: "xs" }),
        },
        tree_branch: toggleable({
            base: interactive({
                base: {
                    color: border_color(layer),
                    width: 1,
                },
                state: {
                    hovered: {
                        color: border_color(layer),
                    },
                },
            }),
            state: {
                active: {
                    default: {
                        color: border_color(layer),
                    },
                },
            },
        }),
        project_row: toggleable({
            base: interactive({
                base: {
                    ...project_row,
                    icon: {
                        margin: { left: NAME_MARGIN },
                        color: foreground(layer, "variant"),
                        width: 12,
                    },
                    name: {
                        ...project_row.name,
                        ...text(layer, "mono", { size: "sm" }),
                    },
                },
                state: {
                    hovered: {
                        background: background(layer, "hovered"),
                    },
                },
            }),
            state: {
                active: {
                    default: { background: background(layer, "active") },
                },
            },
        }),
        face_overlap: 8,
        channel_editor: {
            padding: {
                left: NAME_MARGIN,
            }
        }
    }
}
