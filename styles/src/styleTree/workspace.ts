import { ColorScheme } from "../theme/colorScheme"
import { withOpacity } from "../theme/color"
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
import { interactive } from "../element"

import { titlebar } from "./titlebar"
export default function workspace(colorScheme: ColorScheme): any {
    const layer = colorScheme.lowest
    const isLight = colorScheme.isLight

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
            keyboardHint: interactive({
                base: {
                    ...text(layer, "sans", "variant", { size: "sm" }),
                    padding: {
                        top: 3,
                        left: 8,
                        right: 8,
                        bottom: 3,
                    },
                    cornerRadius: 8,
                },
                state: {
                    hovered: {
                        ...text(layer, "sans", "active", { size: "sm" }),
                    },
                },
            }),

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
        zoomedBackground: {
            cursor: "Arrow",
            background: isLight
                ? withOpacity(background(colorScheme.lowest), 0.8)
                : withOpacity(background(colorScheme.highest), 0.6),
        },
        zoomedPaneForeground: {
            margin: 16,
            shadow: colorScheme.modalShadow,
            border: border(colorScheme.lowest, { overlay: true }),
        },
        zoomedPanelForeground: {
            margin: 16,
            border: border(colorScheme.lowest, { overlay: true }),
        },
        dock: {
            left: {
                border: border(layer, { right: true }),
            },
            bottom: {
                border: border(layer, { top: true }),
            },
            right: {
                border: border(layer, { left: true }),
            },
        },
        paneDivider: {
            color: borderColor(layer),
            width: 1,
        },
        statusBar: statusBar(colorScheme),
        titlebar: titlebar(colorScheme),
        toolbar: {
            height: 34,
            background: background(colorScheme.highest),
            border: border(colorScheme.highest, { bottom: true }),
            itemSpacing: 8,
            navButton: interactive({
                base: {
                    color: foreground(colorScheme.highest, "on"),
                    iconWidth: 12,
                    buttonWidth: 24,
                    cornerRadius: 6,
                },
                state: {
                    hovered: {
                        color: foreground(colorScheme.highest, "on", "hovered"),
                        background: background(
                            colorScheme.highest,
                            "on",
                            "hovered"
                        ),
                    },
                    disabled: {
                        color: foreground(
                            colorScheme.highest,
                            "on",
                            "disabled"
                        ),
                    },
                },
            }),
            padding: { left: 8, right: 8, top: 4, bottom: 4 },
        },
        breadcrumbHeight: 24,
        breadcrumbs: interactive({
            base: {
                ...text(colorScheme.highest, "sans", "variant"),
                cornerRadius: 6,
                padding: {
                    left: 6,
                    right: 6,
                },
            },
            state: {
                hovered: {
                    color: foreground(colorScheme.highest, "on", "hovered"),
                    background: background(
                        colorScheme.highest,
                        "on",
                        "hovered"
                    ),
                },
            },
        }),
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
        dropTargetOverlayColor: withOpacity(foreground(layer, "variant"), 0.5),
    }
}
