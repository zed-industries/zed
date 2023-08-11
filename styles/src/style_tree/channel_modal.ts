import { useTheme } from "../theme"
import { interactive, toggleable } from "../element"
import { background, border, foreground, text } from "./components"
import picker from "./picker"
import { input } from "../component/input"
import { toggleable_text_button } from "../component/text_button"

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

    const picker_input = input()

    return {
        header: {
            background: background(theme.middle, "accent"),
            border: border(theme.middle, { "bottom": true, "top": false, left: false, right: false }),
        },
        body: {
            background: background(theme.middle),
        },
        modal: {
            background: background(theme.middle),
            shadow: theme.modal_shadow,
            corner_radius: 12,
            padding: {
                bottom: 0,
                left: 0,
                right: 0,
                top: 0,
            },

        },
        // This is used for the icons that are rendered to the right of channel Members in both UIs
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
        // This is used for the icons that are rendered to the right of channel invites in both UIs
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
        member_tag: {
            ...text(theme.middle, "sans", { size: "xs" }),
            border: border(theme.middle, "active"),
            background: background(theme.middle),
            margin: {
                left: 8,
            },
            padding: {
                left: 4,
                right: 4,
            }
        },
        max_height: 400,
        max_width: 540,
        title: {
            ...text(theme.middle, "sans", "on", { size: "lg" }),
            padding: {
                left: 6,
            }
        },
        mode_button: toggleable_text_button(theme, {
            variant: "ghost",
            layer: theme.middle,
            active_color: "accent",
            margin: {
                top: 8,
                bottom: 8,
                right: 4
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
