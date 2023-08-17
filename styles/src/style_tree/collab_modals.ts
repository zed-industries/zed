import { useTheme } from "../theme"
import { background, border, foreground, text } from "./components"
import picker from "./picker"
import { input } from "../component/input"
import contact_finder from "./contact_finder"
import { tab } from "../component/tab"
import { icon_button } from "../component/icon_button"

export default function channel_modal(): any {
    const theme = useTheme()

    const SPACING = 12 as const
    const BUTTON_OFFSET = 6 as const
    const ITEM_HEIGHT = 36 as const

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

    const member_icon_style = icon_button({
        variant: "ghost",
        size: "sm",
    }).default

    return {
        contact_finder: contact_finder(),
        tabbed_modal: {
            tab_button: tab({ layer: theme.middle }),
            row_height: ITEM_HEIGHT,
            header: {
                background: background(theme.lowest),
                border: border(theme.middle, { "bottom": true, "top": false, left: false, right: false }),
                padding: {
                    top: SPACING,
                    left: SPACING - BUTTON_OFFSET,
                    right: SPACING - BUTTON_OFFSET,
                },
                corner_radii: {
                    top_right: 12,
                    top_left: 12,
                }
            },
            body: {
                background: background(theme.middle),
                padding: {
                    top: SPACING - 4,
                    left: SPACING,
                    right: SPACING,
                    bottom: SPACING,

                },
                corner_radii: {
                    bottom_right: 12,
                    bottom_left: 12,
                }
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
            // FIXME: due to a bug in the picker's size calculation, this must be 600
            max_height: 600,
            max_width: 540,
            title: {
                ...text(theme.middle, "sans", "on", { size: "lg" }),
                padding: {
                    left: BUTTON_OFFSET,
                }
            },
            picker: {
                empty_container: {},
                item: {
                    ...picker_style.item,
                    margin: { left: SPACING, right: SPACING },
                },
                no_matches: picker_style.no_matches,
                input_editor: picker_input,
                empty_input_editor: picker_input,
                header: picker_style.header,
                footer: picker_style.footer,
            },
        },
        channel_modal: {
            // This is used for the icons that are rendered to the right of channel Members in both UIs
            member_icon: member_icon_style,
            // This is used for the icons that are rendered to the right of channel invites in both UIs
            invitee_icon: member_icon_style,
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
}
