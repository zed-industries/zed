import { icon_button, text_button, toggleable_icon_button, toggleable_text_button } from "../component"
import { interactive, toggleable } from "../element"
import { useTheme, with_opacity } from "../theme"
import { background, border, foreground, text } from "./components"

const ITEM_SPACING = 8
const TITLEBAR_HEIGHT = 32

function build_spacing(
    container_height: number,
    element_height: number,
    spacing: number
) {
    return {
        group: spacing,
        item: spacing / 2,
        half_item: spacing / 4,
        margin_y: (container_height - element_height) / 2,
        margin_x: (container_height - element_height) / 2,
    }
}

function call_controls() {
    const theme = useTheme()

    const button_height = 18

    const space = build_spacing(TITLEBAR_HEIGHT, button_height, ITEM_SPACING)
    const margin_y = {
        top: space.margin_y,
        bottom: space.margin_y,
    }

    return {
        toggle_microphone_button: toggleable_icon_button({
            margin: {
                ...margin_y,
                left: space.group,
                right: space.half_item,
            },
            active_color: "negative",
            active_background_color: "negative",
        }),

        toggle_speakers_button: toggleable_icon_button({
            margin: {
                ...margin_y,
                left: space.half_item,
                right: space.half_item,
            },
        }),

        screen_share_button: toggleable_icon_button({
            margin: {
                ...margin_y,
                left: space.half_item,
                right: space.group,
            },
            active_color: "accent",
            active_background_color: "accent",
        }),

        muted: foreground(theme.lowest, "negative"),
        speaking: foreground(theme.lowest, "accent"),
    }
}

/**
 * Opens the User Menu when toggled
 *
 * When logged in shows the user's avatar and a chevron,
 * When logged out only shows a chevron.
 */
function user_menu() {
    const theme = useTheme()

    const button_height = 18

    const space = build_spacing(TITLEBAR_HEIGHT, button_height, ITEM_SPACING)

    const build_button = ({ online }: { online: boolean }) => {
        const button = toggleable({
            base: interactive({
                base: {
                    corner_radius: 6,
                    height: button_height,
                    width: 20,
                    padding: {
                        top: 2,
                        bottom: 2,
                        left: 6,
                        right: 6,
                    },
                    margin: {
                        left: space.item,
                        right: space.item,
                    },
                    ...text(theme.lowest, "sans", { size: "xs" }),
                    background: background(theme.lowest),
                },
                state: {
                    hovered: {
                        ...text(theme.lowest, "sans", "hovered", {
                            size: "xs",
                        }),
                        background: background(theme.lowest, "hovered"),
                    },
                    clicked: {
                        ...text(theme.lowest, "sans", "pressed", {
                            size: "xs",
                        }),
                        background: background(theme.lowest, "pressed"),
                    },
                },
            }),
            state: {
                active: {
                    default: {
                        ...text(theme.lowest, "sans", "active", { size: "xs" }),
                        background: background(theme.middle),
                    },
                    hovered: {
                        ...text(theme.lowest, "sans", "active", { size: "xs" }),
                        background: background(theme.middle, "hovered"),
                    },
                    clicked: {
                        ...text(theme.lowest, "sans", "active", { size: "xs" }),
                        background: background(theme.middle, "pressed"),
                    },
                },
            },
        })

        return {
            user_menu: button,
            avatar: {
                icon_width: 16,
                icon_height: 16,
                corner_radius: 4,
                outer_width: 16,
                outer_corner_radius: 16,
            },
            icon: {
                margin: {
                    top: 2,
                    left: online ? space.item : 0,
                    right: space.group,
                    bottom: 2,
                },
                width: 11,
                height: 11,
                color: foreground(theme.lowest),
            },
        }
    }

    return {
        user_menu_button_online: build_button({ online: true }),
        user_menu_button_offline: build_button({ online: false }),
    }
}

export function titlebar(): any {
    const theme = useTheme()

    const avatar_width = 15
    const avatar_outer_width = avatar_width + 4
    const follower_avatar_width = 14
    const follower_avatar_outer_width = follower_avatar_width + 4

    return {
        item_spacing: ITEM_SPACING,
        face_pile_spacing: 2,
        height: TITLEBAR_HEIGHT,
        background: background(theme.lowest),
        border: border(theme.lowest, { bottom: true }),
        padding: {
            left: 80,
            right: 0,
        },
        menu: {
            width: 300,
            height: 400,
        },

        project_menu_button: toggleable_text_button(theme, {
            color: "base"
        }),

        git_menu_button: toggleable_text_button(theme, {
            color: "variant",
        }),

        project_host: text_button({
            text_properties: {
                weight: "bold"
            }
        }),

        // Collaborators
        leader_avatar: {
            width: avatar_width,
            outer_width: avatar_outer_width,
            corner_radius: avatar_width / 2,
            outer_corner_radius: avatar_outer_width / 2,
        },
        follower_avatar: {
            width: follower_avatar_width,
            outer_width: follower_avatar_outer_width,
            corner_radius: follower_avatar_width / 2,
            outer_corner_radius: follower_avatar_outer_width / 2,
        },
        inactive_avatar_grayscale: true,
        follower_avatar_overlap: 8,
        leader_selection: {
            margin: {
                top: 4,
                bottom: 4,
            },
            padding: {
                left: 2,
                right: 2,
                top: 2,
                bottom: 2,
            },
            corner_radius: 6,
        },
        avatar_ribbon: {
            height: 3,
            width: 14,
            // TODO: Chore: Make avatarRibbon colors driven by the theme rather than being hard coded.
        },

        sign_in_button: toggleable_text_button(theme, {}),
        offline_icon: {
            color: foreground(theme.lowest, "variant"),
            width: 16,
            margin: {
                left: ITEM_SPACING,
            },
            padding: {
                right: 4,
            },
        },

        // When the collaboration server is out of date, show a warning
        outdated_warning: {
            ...text(theme.lowest, "sans", "warning", { size: "xs" }),
            background: with_opacity(background(theme.lowest, "warning"), 0.3),
            border: border(theme.lowest, "warning"),
            margin: {
                left: ITEM_SPACING,
            },
            padding: {
                left: 8,
                right: 8,
            },
            corner_radius: 6,
        },

        leave_call_button: icon_button({
            margin: {
                left: ITEM_SPACING / 2,
                right: ITEM_SPACING,
            },
        }),

        ...call_controls(),

        toggle_contacts_button: toggleable_icon_button({
            margin: {
                left: ITEM_SPACING,
            },
        }),

        // Jewel that notifies you that there are new contact requests
        toggle_contacts_badge: {
            corner_radius: 3,
            padding: 2,
            margin: { top: 3, left: 3 },
            border: border(theme.lowest),
            background: foreground(theme.lowest, "accent"),
        },
        share_button: toggleable_text_button(theme, {}),
        user_menu: user_menu(),
    }
}
