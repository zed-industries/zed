import {
    background,
    border,
    border_color,
    foreground,
    text,
} from "./components"
import { interactive, toggleable } from "../element"
import { useTheme } from "../theme"
import channel_modal from "./channel_modal"
import { icon_button, toggleable_icon_button } from "../component/icon_button"


export default function contacts_panel(): any {
    const theme = useTheme()

    const name_margin = 8
    const side_padding = 12

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
            ...text(layer, "mono", { size: "sm" }),
            margin: {
                left: name_margin,
                right: 6,
            },
        },
        guests: {
            margin: {
                left: name_margin,
                right: name_margin,
            },
        },
        padding: {
            left: side_padding,
            right: side_padding,
        },
    }

    const headerButton = toggleable({
        state: {
            inactive: interactive({
                base: {
                    corner_radius: 6,
                    padding: {
                        top: 2,
                        bottom: 2,
                        left: 4,
                        right: 4,
                    },
                    icon_width: 14,
                    icon_height: 14,
                    button_width: 20,
                    button_height: 16,
                    color: foreground(layer, "on"),
                },
                state: {
                    default: {
                    },
                    hovered: {
                        background: background(layer, "base", "hovered"),
                    },
                    clicked: {
                        background: background(layer, "base", "pressed"),
                    },
                },
            }),
            active: interactive({
                base: {
                    corner_radius: 6,
                    padding: {
                        top: 2,
                        bottom: 2,
                        left: 4,
                        right: 4,
                    },
                    icon_width: 14,
                    icon_height: 14,
                    button_width: 20,
                    button_height: 16,
                    color: foreground(layer, "on"),
                },
                state: {
                    default: {
                        background: background(layer, "base", "active"),
                    },
                    clicked: {
                        background: background(layer, "base", "active"),
                    },
                },
            }),
        },
    })


    return {
        channel_modal: channel_modal(),
        log_in_button: interactive({
            base: {
                background: background(theme.middle),
                border: border(theme.middle, "active"),
                corner_radius: 4,
                margin: {
                    top: 16,
                    left: 16,
                    right: 16,
                },
                padding: {
                    top: 3,
                    bottom: 3,
                    left: 7,
                    right: 7,
                },
                ...text(theme.middle, "sans", "default", { size: "sm" }),
            },
            state: {
                hovered: {
                    ...text(theme.middle, "sans", "default", { size: "sm" }),
                    background: background(theme.middle, "hovered"),
                    border: border(theme.middle, "active"),
                },
                clicked: {
                    ...text(theme.middle, "sans", "default", { size: "sm" }),
                    background: background(theme.middle, "pressed"),
                    border: border(theme.middle, "active"),
                },
            },
        }),
        background: background(layer),
        padding: {
            top: 12,
        },
        user_query_editor: {
            background: background(layer, "on"),
            corner_radius: 6,
            text: text(layer, "mono", "on"),
            placeholder_text: text(layer, "mono", "on", "disabled", {
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
                left: side_padding,
                right: side_padding,
            },
        },
        channel_hash: {
            color: foreground(layer, "on"),
            width: 14,
        },
        user_query_editor_height: 33,
        add_contact_button: headerButton,
        add_channel_button: headerButton,
        leave_call_button: headerButton,
        row_height: 28,
        channel_indent: 10,
        section_icon_size: 8,
        header_row: {
            ...text(layer, "mono", { size: "sm", weight: "bold" }),
            margin: { top: 14 },
            padding: {
                left: side_padding,
                right: side_padding,
            },
        },
        subheader_row: toggleable({
            base: interactive({
                base: {
                    ...text(layer, "mono", { size: "sm" }),
                    padding: {
                        left: side_padding,
                        right: side_padding,
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
                        ...text(layer, "mono", "active", { size: "sm" }),
                        background: background(layer, "active"),
                    },
                    clicked: {
                        background: background(layer, "pressed"),
                    },
                },
            },
        }),
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
                        left: side_padding,
                        right: side_padding,
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
                        ...text(layer, "mono", "active", { size: "sm" }),
                        background: background(layer, "active"),
                    },
                    clicked: {
                        background: background(layer, "pressed"),
                    },
                },
            },
        }),
        list_empty_state: {
            ...text(layer, "ui_sans", "variant", { size: "sm" }),
            padding: side_padding
        },
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
            ...text(layer, "mono", { size: "sm" }),
            margin: {
                left: name_margin,
            },
        },
        contact_button_spacing: name_margin,
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
                    // background: background(layer),
                    icon: {
                        margin: { left: name_margin },
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
                left: name_margin,
            }
        }
    }
}
