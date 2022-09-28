import { ColorScheme } from "../themes/common/colorScheme";
import { withOpacity } from "../utils/color";
import { text, border, background, foreground } from "./components";

export default function tabBar(colorScheme: ColorScheme) {
  const height = 32;

  let elevation = colorScheme.lowest;
  let activeLayerActiveTab = elevation.top;
  let activeLayerInactiveTab = elevation.middle;
  let inactiveLayerActiveTab = elevation.middle;
  let inactiveLayerInactiveTab = elevation.bottom;

  const tab = {
    height,
    text: text(activeLayerInactiveTab, "sans", "variant", { size: "sm" }),
    background: background(activeLayerInactiveTab),
    border: border(activeLayerInactiveTab, {
      right: true,
      bottom: true,
      overlay: true,
    }),
    padding: {
      left: 8,
      right: 12,
    },
    spacing: 8,

    // Close icons
    iconWidth: 8,
    iconClose: foreground(activeLayerInactiveTab, "variant"),
    iconCloseActive: foreground(activeLayerInactiveTab),

    // Indicators
    iconConflict: foreground(activeLayerInactiveTab, "warning"),
    iconDirty: foreground(activeLayerInactiveTab, "info"),

    // When two tabs of the same name are open, a label appears next to them
    description: {
      margin: { left: 8 },
      ...text(activeLayerInactiveTab, "sans", "disabled", { size: "2xs" })
    }
  };

  const activePaneActiveTab = {
    ...tab,
    background: background(activeLayerActiveTab),
    text: text(activeLayerActiveTab, "sans", { size: "sm" }),
    border: {
      ...tab.border,
      bottom: false
    },
  };

  const inactivePaneInactiveTab = {
    ...tab,
    background: background(inactiveLayerInactiveTab),
    text: text(inactiveLayerInactiveTab, "sans", "variant", { size: "sm" }),
  };

  const inactivePaneActiveTab = {
    ...tab,
    background: background(inactiveLayerActiveTab),
    text: text(inactiveLayerActiveTab, "sans", "variant", { size: "sm" }),
    border: {
      ...tab.border,
      bottom: false
    },
  }

  const draggedTab = {
    ...activePaneActiveTab,
    background: withOpacity(tab.background, 0.8),
    border: undefined as any,
    shadow: elevation.above.shadow,
  }

  return {
    height,
    background: background(activeLayerInactiveTab),
    dropTargetOverlayColor: withOpacity(foreground(activeLayerInactiveTab), 0.6),
    activePane: {
      activeTab: activePaneActiveTab,
      inactiveTab: tab,
    },
    inactivePane: {
      activeTab: inactivePaneActiveTab,
      inactiveTab: inactivePaneInactiveTab,
    },
    draggedTab,
    paneButton: {
      color: foreground(activeLayerInactiveTab, "variant"),
      iconWidth: 12,
      buttonWidth: activePaneActiveTab.height,
      hover: {
        color: foreground(activeLayerInactiveTab, "hovered"),
      },
    },
    paneButtonContainer: {
      background: tab.background,
      border: {
        ...tab.border,
        right: false,
      }
    }
  }
}