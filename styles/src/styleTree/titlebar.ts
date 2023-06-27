import { ColorScheme } from "../common";
import { icon_button, toggleable_icon_button } from "../component/icon_button"
import { toggleable_text_button } from "../component/text_button"
import { interactive, toggleable } from "../element"
import { withOpacity } from "../theme/color";
import { background, border, foreground, text } from "./components";

const ITEM_SPACING = 8

interface SpacingProps {
    container_height: number;
    spacing: number;
}

function build_spacing(
    container_height: number,
    element_height: number,
    spacing: number
) {
    return {
        group: spacing * 2,
        item: spacing / 2,
        marginY: (container_height - element_height) / 2,
        marginX: (container_height - element_height) / 2,
    }
}

function mac_os_controls(theme: ColorScheme, { container_height, spacing }: SpacingProps) {
    return {}
}

function project_info(theme: ColorScheme, { container_height, spacing }: SpacingProps) {
    return {}
}

function collaboration_stacks(theme: ColorScheme, { container_height, spacing }: SpacingProps) {
    return {}
}

function sharing_controls(theme: ColorScheme, { container_height, spacing }: SpacingProps) {
    return {}
}

function call_controls(theme: ColorScheme, { container_height, spacing }: SpacingProps) {
    return {}
}

const titlebarButton = (theme: ColorScheme) => toggleable({
    base: interactive({
        base: {
            cornerRadius: 6,
            height: 24,
            width: 24,
            padding: {
                top: 4,
                bottom: 4,
                left: 4,
                right: 4,
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
    }
});

/**
* Opens the User Menu when toggled
*
* When logged in shows the user's avatar and a chevron,
* When logged out only shows a chevron.
*/
function userMenuButton(theme: ColorScheme, online: boolean) {
    const button = toggleable({
        base: interactive({
            base: {
                cornerRadius: 6,
                height: 19,
                width: online ? 36 : 23,
                padding: {
                    top: 2,
                    bottom: 2,
                    left: 6,
                    right: 6,
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
        }
    });

    return {
        user_menu: button,
        avatar: {
            icon_width: 16,
            icon_height: 16,
            cornerRadius: 4,
            outer_corner_radius: 0,
            outer_width: 0,
            outerWidth: 16,
            outerCornerRadius: 16
        },
        icon: {
            margin: {
                left: online ? 2 : 0,
            },
            width: 11,
            height: 11,
            color: foreground(theme.lowest)
        }
    }
}

export function titlebar(theme: ColorScheme) {
    const avatarWidth = 18
    const avatarOuterWidth = avatarWidth + 4
    const followerAvatarWidth = 14
    const followerAvatarOuterWidth = followerAvatarWidth + 4

    return {
        ITEM_SPACING,
        facePileSpacing: 2,
        height: 33, // 32px + 1px border. It's important the content area of the titlebar is evenly sized to vertically center avatar images.
        background: background(theme.lowest),
        border: border(theme.lowest, { bottom: true }),
        padding: {
            left: 80,
            right: ITEM_SPACING,
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
            width: 11,
            // TODO: Chore: Make avatarRibbon colors driven by the theme rather than being hard coded.
        },

        // Sign in buttom
        sign_in_button: toggleable_text_button(theme, {}),

        // Offline Indicator
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

        // Notice that the collaboration server is out of date
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
                right: ITEM_SPACING
            },
        }),

        toggle_microphone_button: toggleable_icon_button(theme, {
            margin: {
                left: ITEM_SPACING,
                right: ITEM_SPACING / 2
            },
            active_color: 'negative'
        }),

        toggle_speakers_button: toggleable_icon_button(theme, {
            margin: {
                left: ITEM_SPACING / 2,
                right: ITEM_SPACING / 2
            },
        }),

        screen_share_button: toggleable_icon_button(theme, {
            margin: {
                left: ITEM_SPACING / 2,
                right: ITEM_SPACING
            },
            active_color: 'accent'
        }),

        toggle_contacts_button: toggleable_icon_button(theme, {
            margin: {
                left: ITEM_SPACING,
                right: ITEM_SPACING / 2
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
        user_menu: {
            userMenuButtonOnline: userMenuButton(theme, true),
            userMenuButtonOffline: userMenuButton(theme, false),
        }
    }
}
