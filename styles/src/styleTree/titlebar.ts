import { ColorScheme } from "../common"
import { icon_button, toggleable_icon_button } from "../component/icon_button"
import { toggleable_text_button } from "../component/text_button"
import { interactive, toggleable } from "../element"
import { withOpacity } from "../theme/color"
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
        marginY: (container_height - element_height) / 2,
        marginX: (container_height - element_height) / 2,
    }
}

function call_controls(theme: ColorScheme) {
    const button_height = 18

    const space = build_spacing(TITLEBAR_HEIGHT, button_height, ITEM_SPACING)
    const marginY = {
        top: space.marginY,
        bottom: space.marginY,
    }

    return {
        toggle_microphone_button: toggleable_icon_button(theme, {
            margin: {
                ...marginY,
                left: space.group,
                right: space.half_item,
            },
            active_color: "negative",
        }),

        toggle_speakers_button: toggleable_icon_button(theme, {
            margin: {
                ...marginY,
                left: space.half_item,
                right: space.half_item,
            },
        }),

        screen_share_button: toggleable_icon_button(theme, {
            margin: {
                ...marginY,
                left: space.half_item,
                right: space.group,
            },
            active_color: "accent",
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
function user_menu(theme: ColorScheme) {
    const button_height = 18

    const space = build_spacing(TITLEBAR_HEIGHT, button_height, ITEM_SPACING)

    const build_button = ({ online }: { online: boolean }) => {
        const button = toggleable({
            base: interactive({
                base: {
                    cornerRadius: 6,
                    height: button_height,
                    width: online ? 37 : 24,
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
        userMenuButtonOnline: build_button({ online: true }),
        userMenuButtonOffline: build_button({ online: false }),
    }
}

export function titlebar(theme: ColorScheme) {
    const avatarWidth = 15
    const avatarOuterWidth = avatarWidth + 4
    const followerAvatarWidth = 14
    const followerAvatarOuterWidth = followerAvatarWidth + 4

    return {
        item_spacing: ITEM_SPACING,
        facePileSpacing: 2,
        height: TITLEBAR_HEIGHT,
        background: background(theme.lowest),
        border: border(theme.lowest, { bottom: true }),
        padding: {
            left: 80,
            right: 0,
        },

        // Project
        title: text(theme.lowest, "sans", "variant"),
        highlight_color: text(theme.lowest, "sans", "active").color,

        // Collaborators
        leaderAvatar: {
            width: avatarWidth,
            outerWidth: avatarOuterWidth,
            cornerRadius: avatarWidth / 2,
            outerCornerRadius: avatarOuterWidth / 2,
        },
        followerAvatar: {
            width: followerAvatarWidth,
            outerWidth: followerAvatarOuterWidth,
            cornerRadius: followerAvatarWidth / 2,
            outerCornerRadius: followerAvatarOuterWidth / 2,
        },
        inactiveAvatarGrayscale: true,
        followerAvatarOverlap: 8,
        leaderSelection: {
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
            cornerRadius: 6,
        },
        avatarRibbon: {
            height: 3,
            width: 14,
            // TODO: Chore: Make avatarRibbon colors driven by the theme rather than being hard coded.
        },

        sign_in_button: toggleable_text_button(theme, {}),
        offlineIcon: {
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
        outdatedWarning: {
            ...text(theme.lowest, "sans", "warning", { size: "xs" }),
            background: withOpacity(background(theme.lowest, "warning"), 0.3),
            border: border(theme.lowest, "warning"),
            margin: {
                left: ITEM_SPACING,
            },
            padding: {
                left: 8,
                right: 8,
            },
            cornerRadius: 6,
        },

        leave_call_button: icon_button(theme, {
            margin: {
                left: ITEM_SPACING / 2,
                right: ITEM_SPACING,
            },
        }),

        ...call_controls(theme),

        toggle_contacts_button: toggleable_icon_button(theme, {
            margin: {
                left: ITEM_SPACING,
            },
        }),

        // Jewel that notifies you that there are new contact requests
        toggleContactsBadge: {
            cornerRadius: 3,
            padding: 2,
            margin: { top: 3, left: 3 },
            border: border(theme.lowest),
            background: foreground(theme.lowest, "accent"),
        },
        shareButton: toggleable_text_button(theme, {}),
        user_menu: user_menu(theme),
    }
}
