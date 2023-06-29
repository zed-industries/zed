import { ColorScheme } from "../theme/color_scheme"
import {
    background,
    border,
    border_color,
    foreground,
    text,
} from "./components"
import { interactive, toggleable } from "../element"
export default function contacts_panel(theme: ColorScheme): any {
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

    return {
        background: background(layer),
        padding: { top: 12 },
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
                left: 6,
            },
        },
        user_query_editor_height: 33,
        add_contact_button: {
            margin: { left: 6, right: 12 },
            color: foreground(layer, "on"),
            button_width: 28,
            icon_width: 16,
        },
        row_height: 28,
        section_icon_size: 8,
        header_row: toggleable({
            base: interactive({
                base: {
                    ...text(layer, "mono", { size: "sm" }),
                    margin: { top: 14 },
                    padding: {
                        left: side_padding,
                        right: side_padding,
                    },
                    background: background(layer, "default"), // posiewic: breaking change
                },
                state: {
                    hovered: {
                        background: background(layer, "hovered"),
                    },
                    clicked: {
                        background: background(layer, "pressed"),
                    },
                }, // hack, we want headerRow to be interactive for whatever reason. It probably shouldn't be interactive in the first place.
            }),
            state: {
                active: {
                    default: {
                        ...text(layer, "mono", "active", { size: "sm" }),
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
        contact_row: {
            inactive: {
                default: {
                    padding: {
                        left: side_padding,
                        right: side_padding,
                    },
                },
            },
            active: {
                default: {
                    background: background(layer, "active"),
                    padding: {
                        left: side_padding,
                        right: side_padding,
                    },
                },
            },
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
                    background: background(layer),
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
    }
}
