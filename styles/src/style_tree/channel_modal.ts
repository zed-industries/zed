import { useTheme } from "../theme"
import { interactive, toggleable } from "../element"
import { background, border, foreground, text } from "./components"
import picker from "./picker"

export default function channel_modal(): any {
    const theme = useTheme()

    const side_margin = 6
    const contact_button = {
        background: background(theme.middle, "variant"),
        color: foreground(theme.middle, "variant"),
        icon_width: 8,
        button_width: 16,
        corner_radius: 8,
    }

    const picker_style = picker()
    delete picker_style.shadow
    delete picker_style.border

    const picker_input = {
        background: background(theme.middle, "on"),
        corner_radius: 6,
        text: text(theme.middle, "mono"),
        placeholder_text: text(theme.middle, "mono", "on", "disabled", {
            size: "xs",
        }),
        selection: theme.players[0],
        border: border(theme.middle),
        padding: {
            bottom: 4,
            left: 8,
            right: 8,
            top: 4,
        },
        margin: {
            left: side_margin,
            right: side_margin,
        },
    }

    return {
        member_icon: {
            background: background(theme.middle),
            padding: {
                bottom: 4,
                left: 4,
                right: 4,
                top: 4,
            },
            width: 5,
            color: foreground(theme.middle, "accent"),
        },
        invitee_icon: {
            background: background(theme.middle),
            padding: {
                bottom: 4,
                left: 4,
                right: 4,
                top: 4,
            },
            width: 5,
            color: foreground(theme.middle, "accent"),
        },
        remove_member_button: {
            ...text(theme.middle, "sans", { size: "xs" }),
            background: background(theme.middle),
            padding: {
                left: 7,
                right: 7
            }
        },
        cancel_invite_button: {
            ...text(theme.middle, "sans", { size: "xs" }),
            background: background(theme.middle),
        },
        admin_toggle_part: toggleable({
            base: {
                ...text(theme.middle, "sans", { size: "xs" }),
                padding: {
                    left: 7,
                    right: 7,
                },
            },
            state: {
                active: {
                    background: background(theme.middle, "on"),
                }
            }
        }),
        admin_toggle: {
            border: border(theme.middle, "active"),
            background: background(theme.middle),
            margin: {
                right: 8,
            }
        },
        container: {
            background: background(theme.lowest),
            border: border(theme.lowest),
            shadow: theme.modal_shadow,
            corner_radius: 12,
            padding: {
                bottom: 4,
                left: 20,
                right: 20,
                top: 20,
            },
        },
        height: 400,
        header: text(theme.middle, "sans", "on", { size: "lg" }),
        mode_button: toggleable({
            base: interactive({
                base: {
                    ...text(theme.middle, "sans", { size: "xs" }),
                    border: border(theme.middle, "active"),
                    corner_radius: 4,
                    padding: {
                        top: 3,
                        bottom: 3,
                        left: 7,
                        right: 7,
                    },

                    margin: { left: 6, top: 6, bottom: 6 },
                },
                state: {
                    hovered: {
                        ...text(theme.middle, "sans", "default", { size: "xs" }),
                        background: background(theme.middle, "hovered"),
                        border: border(theme.middle, "active"),
                    },
                },
            }),
            state: {
                active: {
                    default: {
                        color: foreground(theme.middle, "accent"),
                    },
                    hovered: {
                        color: foreground(theme.middle, "accent", "hovered"),
                    },
                    clicked: {
                        color: foreground(theme.middle, "accent", "pressed"),
                    },
                },
            }
        }),
        picker: {
            empty_container: {},
            item: {
                ...picker_style.item,
                margin: { left: side_margin, right: side_margin },
            },
            no_matches: picker_style.no_matches,
            input_editor: picker_input,
            empty_input_editor: picker_input,
            header: picker_style.header,
            footer: picker_style.footer,
        },
        row_height: 28,
        contact_avatar: {
            corner_radius: 10,
            width: 18,
        },
        contact_username: {
            padding: {
                left: 8,
            },
        },
        contact_button: {
            ...contact_button,
            hover: {
                background: background(theme.middle, "variant", "hovered"),
            },
        },
        disabled_contact_button: {
            ...contact_button,
            background: background(theme.middle, "disabled"),
            color: foreground(theme.middle, "disabled"),
        },
    }
}
