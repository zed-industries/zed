import { useSurfaceStyle } from "@components/surface"
import { Theme } from "@theme"
import { activeTab, inactiveTab } from "@ui/pane/tab"

export default function tabBar(theme: Theme) {
    const inactive_tab = inactiveTab(theme)
    const active_tab = activeTab(theme)

    const legacy_tab = {
        ...inactive_tab.default.container,
        spacing: inactive_tab.default.flex.spacing,
        typeIconWidth: inactive_tab.default.icon.size,
        closeIconWidth: inactive_tab.default.close.default.icon.size,
        iconClose: inactive_tab.default.close.default.icon.color,
        iconCloseActive: active_tab.default.close.default.icon.color,
        iconConflict: inactive_tab.default.indicator.conflict.color,
        iconDirty: inactive_tab.default.indicator.dirty.color,
        description: {
            margin: inactive_tab.default.description.container.margin,
            ...inactive_tab.default.description.text,
        },
    }

    const TAB_BAR_HEIGHT = 32

    const legacy_styles = {
        height: TAB_BAR_HEIGHT,
        background: useSurfaceStyle(theme, "pane").background,
    }

    return {
        ...legacy_styles,
        activePane: {
            activeTab: legacy_tab,
            inactiveTab: legacy_tab,
        },
        inactivePane: {
            activeTab: legacy_tab,
            inactiveTab: legacy_tab,
        },
        // draggedTab,
        // paneButton: {
        //     color: foreground(layer, "variant"),
        //     iconWidth: 12,
        //     buttonWidth: activePaneActiveTab.height,
        //     hover: {
        //         color: foreground(layer, "hovered"),
        //     },
        // },
        // paneButtonContainer: {
        //     background: tab.background,
        //     border: {
        //         ...tab.border,
        //         right: false,
        //     },
        // },
    }
}

// const tab = {
//     height,
//     text: text(layer, "sans", "variant", { size: "sm" }),
//     background: background(layer),
//     border: border(layer, {
//         right: true,
//         bottom: true,
//         overlay: true,
//     }),
//     padding: {
//         left: 8,
//         right: 12,
//     },
//     spacing: 8,

//     // Tab type icons (e.g. Project Search)
//     typeIconWidth: 14,

//     // Close icons
//     closeIconWidth: 8,
//     iconClose: foreground(layer, "variant"),
//     iconCloseActive: foreground(layer, "hovered"),

//     // Indicators
//     iconConflict: foreground(layer, "warning"),
//     iconDirty: foreground(layer, "accent"),

//     // When two tabs of the same name are open, a label appears next to them
//     description: {
//         margin: { left: 8 },
//         ...text(layer, "sans", "disabled", { size: "2xs" }),
//     },
// }

// const activePaneActiveTab = {
//     ...tab,
//     background: background(activeLayer),
//     text: text(activeLayer, "sans", "active", { size: "sm" }),
//     border: {
//         ...tab.border,
//         bottom: false,
//     },
// }

// const inactivePaneInactiveTab = {
//     ...tab,
//     background: background(layer),
//     text: text(layer, "sans", "variant", { size: "sm" }),
// }

// const inactivePaneActiveTab = {
//     ...tab,
//     background: background(activeLayer),
//     text: text(layer, "sans", "variant", { size: "sm" }),
//     border: {
//         ...tab.border,
//         bottom: false,
//     },
// }

// const draggedTab = {
//     ...activePaneActiveTab,
//     background: withOpacity(tab.background, 0.9),
//     border: undefined as any,
//     shadow: colorScheme.popoverShadow,
// }
