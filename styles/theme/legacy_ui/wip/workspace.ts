import { ColorScheme } from "../themes/common/colorScheme"
import { withOpacity } from "../utils/color"
import {
    background,
    border,
    borderColor,
    foreground,
    svg,
    text,
} from "./components"
import statusBar from "./statusBar"
import tabBar from "./tabBar"

export default function workspace(colorScheme: ColorScheme) {
    const layer = colorScheme.lowest
    const itemSpacing = 8
    const titlebarButton = {
        cornerRadius: 6,
        padding: {
            top: 1,
            bottom: 1,
            left: 8,
            right: 8,
        },
        ...text(layer, "sans", "variant", { size: "xs" }),
        background: background(layer, "variant"),
        border: border(layer),
        hover: {
            ...text(layer, "sans", "variant", "hovered", { size: "xs" }),
            background: background(layer, "variant", "hovered"),
            border: border(layer, "variant", "hovered"),
        },
        clicked: {
            ...text(layer, "sans", "variant", "pressed", { size: "xs" }),
            background: background(layer, "variant", "pressed"),
            border: border(layer, "variant", "pressed"),
        },
        active: {
            ...text(layer, "sans", "variant", "active", { size: "xs" }),
            background: background(layer, "variant", "active"),
            border: border(layer, "variant", "active"),
        },
    }
    const avatarWidth = 18
    const avatarOuterWidth = avatarWidth + 4
    const followerAvatarWidth = 14
    const followerAvatarOuterWidth = followerAvatarWidth + 4

    return {
        background: background(colorScheme.lowest),
        blankPane: {
            logoContainer: {
                width: 256,
                height: 256,
            },
            logo: svg(
                withOpacity("#000000", colorScheme.isLight ? 0.6 : 0.8),
                "icons/logo_96.svg",
                256,
                256
            ),

            logoShadow: svg(
                withOpacity(
                    colorScheme.isLight
                        ? "#FFFFFF"
                        : colorScheme.lowest.base.default.background,
                    colorScheme.isLight ? 1 : 0.6
                ),
                "icons/logo_96.svg",
                256,
                256
            ),
            keyboardHints: {
                margin: {
                    top: 96,
                },
                cornerRadius: 4,
            },
            keyboardHint: {
                ...text(layer, "sans", "variant", { size: "sm" }),
                padding: {
                    top: 3,
                    left: 8,
                    right: 8,
                    bottom: 3,
                },
                cornerRadius: 8,
                hover: {
                    ...text(layer, "sans", "active", { size: "sm" }),
                },
            },
            keyboardHintWidth: 320,
        },
        joiningProjectAvatar: {
            cornerRadius: 40,
            width: 80,
        },
        joiningProjectMessage: {
            padding: 12,
            ...text(layer, "sans", { size: "lg" }),
        },
        externalLocationMessage: {
            background: background(colorScheme.middle, "accent"),
            border: border(colorScheme.middle, "accent"),
            cornerRadius: 6,
            padding: 12,
            margin: { bottom: 8, right: 8 },
            ...text(colorScheme.middle, "sans", "accent", { size: "xs" }),
        },
        leaderBorderOpacity: 0.7,
        leaderBorderWidth: 2.0,
        tabBar: tabBar(colorScheme),
        modal: {
            margin: {
                bottom: 52,
                top: 52,
            },
            cursor: "Arrow",
        },
        sidebar: {
            initialSize: 240,
            border: border(layer, { left: true, right: true }),
        },
        paneDivider: {
            color: borderColor(layer),
            width: 1,
        },
        statusBar: statusBar(colorScheme),
        titlebar: {
            itemSpacing,
            facePileSpacing: 2,
            height: 33, // 32px + 1px border. It's important the content area of the titlebar is evenly sized to vertically center avatar images.
            background: background(layer),
            border: border(layer, { bottom: true }),
            padding: {
                left: 80,
                right: itemSpacing,
            },

            // Project
            title: text(layer, "sans", "variant"),

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
                width: 12,
                // TODO: Chore: Make avatarRibbon colors driven by the theme rather than being hard coded.
            },

            // Sign in buttom
            // FlatButton, Variant
            signInPrompt: {
                margin: {
                    left: itemSpacing,
                },
                ...titlebarButton,
            },

            // Offline Indicator
            offlineIcon: {
                color: foreground(layer, "variant"),
                width: 16,
                margin: {
                    left: itemSpacing,
                },
                padding: {
                    right: 4,
                },
            },

            // Notice that the collaboration server is out of date
            outdatedWarning: {
                ...text(layer, "sans", "warning", { size: "xs" }),
                background: withOpacity(background(layer, "warning"), 0.3),
                border: border(layer, "warning"),
                margin: {
                    left: itemSpacing,
                },
                padding: {
                    left: 8,
                    right: 8,
                },
                cornerRadius: 6,
            },
            callControl: {
                cornerRadius: 6,
                color: foreground(layer, "variant"),
                iconWidth: 12,
                buttonWidth: 20,
                hover: {
                    background: background(layer, "variant", "hovered"),
                    color: foreground(layer, "variant", "hovered"),
                },
            },
            toggleContactsButton: {
                margin: { left: itemSpacing },
                cornerRadius: 6,
                color: foreground(layer, "variant"),
                iconWidth: 14,
                buttonWidth: 20,
                active: {
                    background: background(layer, "variant", "active"),
                    color: foreground(layer, "variant", "active"),
                },
                clicked: {
                    background: background(layer, "variant", "pressed"),
                    color: foreground(layer, "variant", "pressed"),
                },
                hover: {
                    background: background(layer, "variant", "hovered"),
                    color: foreground(layer, "variant", "hovered"),
                },
            },
            userMenuButton: {
                buttonWidth: 20,
                iconWidth: 12,
                ...titlebarButton,
            },
            toggleContactsBadge: {
                cornerRadius: 3,
                padding: 2,
                margin: { top: 3, left: 3 },
                border: border(layer),
                background: foreground(layer, "accent"),
            },
            shareButton: {
                ...titlebarButton,
            },
        },

        toolbar: {
            height: 34,
            background: background(colorScheme.highest),
            border: border(colorScheme.highest, { bottom: true }),
            itemSpacing: 8,
            navButton: {
                color: foreground(colorScheme.highest, "on"),
                iconWidth: 12,
                buttonWidth: 24,
                cornerRadius: 6,
                hover: {
                    color: foreground(colorScheme.highest, "on", "hovered"),
                    background: background(
                        colorScheme.highest,
                        "on",
                        "hovered"
                    ),
                },
                disabled: {
                    color: foreground(colorScheme.highest, "on", "disabled"),
                },
            },
            padding: { left: 8, right: 8, top: 4, bottom: 4 },
        },
        breadcrumbHeight: 24,
        breadcrumbs: {
            ...text(colorScheme.highest, "sans", "variant"),
            cornerRadius: 6,
            padding: {
                left: 6,
                right: 6,
            },
            hover: {
                color: foreground(colorScheme.highest, "on", "hovered"),
                background: background(colorScheme.highest, "on", "hovered"),
            },
        },
        disconnectedOverlay: {
            ...text(layer, "sans"),
            background: withOpacity(background(layer), 0.8),
        },
        notification: {
            margin: { top: 10 },
            background: background(colorScheme.middle),
            cornerRadius: 6,
            padding: 12,
            border: border(colorScheme.middle),
            shadow: colorScheme.popoverShadow,
        },
        notifications: {
            width: 400,
            margin: { right: 10, bottom: 10 },
        },
        dock: {
            initialSizeRight: 640,
            initialSizeBottom: 304,
            wash_color: withOpacity(background(colorScheme.highest), 0.5),
            panel: {
                border: border(colorScheme.middle),
            },
            maximized: {
                margin: 32,
                border: border(colorScheme.highest, { overlay: true }),
                shadow: colorScheme.modalShadow,
            },
        },
        dropTargetOverlayColor: withOpacity(foreground(layer, "variant"), 0.5),
    }
}
