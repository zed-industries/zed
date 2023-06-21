import { ColorScheme } from "../theme/colorScheme"
import { withOpacity } from "../theme/color"
import { text, border, background, foreground } from "./components"
import { interactive, toggleable } from "../element"

export default function tabBar(colorScheme: ColorScheme) {
    const height = 32

    let activeLayer = colorScheme.highest
    let layer = colorScheme.middle

    const tab = {
        height,
        text: text(layer, "sans", "variant", { size: "sm" }),
        background: background(layer),
        border: border(layer, {
            right: true,
            bottom: true,
            overlay: true,
        }),
        padding: {
            left: 8,
            right: 12,
        },
        spacing: 8,

        // Tab type icons (e.g. Project Search)
        typeIconWidth: 14,

        // Close icons
        closeIconWidth: 8,
        iconClose: foreground(layer, "variant"),
        iconCloseActive: foreground(layer, "hovered"),

        // Indicators
        iconConflict: foreground(layer, "warning"),
        iconDirty: foreground(layer, "accent"),

        // When two tabs of the same name are open, a label appears next to them
        description: {
            margin: { left: 8 },
            ...text(layer, "sans", "disabled", { size: "2xs" }),
        },
    }

    const activePaneActiveTab = {
        ...tab,
        background: background(activeLayer),
        text: text(activeLayer, "sans", "active", { size: "sm" }),
        border: {
            ...tab.border,
            bottom: false,
        },
    }

    const inactivePaneInactiveTab = {
        ...tab,
        background: background(layer),
        text: text(layer, "sans", "variant", { size: "sm" }),
    }

    const inactivePaneActiveTab = {
        ...tab,
        background: background(activeLayer),
        text: text(layer, "sans", "variant", { size: "sm" }),
        border: {
            ...tab.border,
            bottom: false,
        },
    }

    const draggedTab = {
        ...activePaneActiveTab,
        background: withOpacity(tab.background, 0.9),
        border: undefined as any,
        shadow: colorScheme.popoverShadow,
    }

    return {
        height,
        background: background(layer),
        activePane: {
            activeTab: activePaneActiveTab,
            inactiveTab: tab,
        },
        inactivePane: {
            activeTab: inactivePaneActiveTab,
            inactiveTab: inactivePaneInactiveTab,
        },
        draggedTab,
        paneButton: toggleable({
            base: interactive({
                base: {
                    color: foreground(layer, "variant"),
                    iconWidth: 12,
                    buttonWidth: activePaneActiveTab.height,
                },
                state: {
                    hovered: {
                        color: foreground(layer, "hovered"),
                    },
                    clicked: {
                        color: foreground(layer, "pressed"),
                    },
                },
            }),
            state: {
                active: {
                    default: {
                        color: foreground(layer, "accent"),
                    },
                    hovered: {
                        color: foreground(layer, "hovered"),
                    },
                    clicked: {
                        color: foreground(layer, "pressed"),
                    },
                },
            },
        }),
        paneButtonContainer: {
            background: tab.background,
            border: {
                ...tab.border,
                right: false,
            },
        },
    }
}
