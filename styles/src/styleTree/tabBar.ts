import { ColorScheme } from "../themes/common/colorScheme";
import { withOpacity } from "../utils/color";
import { text, border, background, foreground } from "./components";

export default function tabBar(colorScheme: ColorScheme) {
  const height = 32;

  let elevation = colorScheme.lowest;
  let layer = elevation.middle;

  const tab = {
    height,
    background: background(layer),
    border: border(layer, {
      right: true,
      bottom: true,
      overlay: true,
    }),
    iconClose: foreground(layer),
    iconCloseActive: foreground(layer, "active"),
    iconConflict: foreground(layer, "warning"),
    iconDirty: foreground(layer, "info"),
    iconWidth: 8,
    spacing: 8,
    text: text(layer, "sans", "variant", { size: "sm" }),
    padding: {
      left: 8,
      right: 8,
    },
    description: {
      margin: { left: 6, top: 1 },
      ...text(layer, "sans", "variant", { size: "2xs" })
    }
  };

  const activePaneActiveTab = {
    ...tab,
    background: background(elevation.top),
    text: text(elevation.top, "sans", { size: "sm" }),
    border: {
      ...tab.border,
      bottom: false
    },
  };

  const inactivePaneInactiveTab = {
    ...tab,
    background: background(layer),
    text: text(layer, "sans", "variant", { size: "sm" }),
  };

  const inactivePaneActiveTab = {
    ...tab,
    background: background(elevation.top),
    text: text(elevation.top, "sans", "variant", { size: "sm" }),
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
    background: background(layer),
    dropTargetOverlayColor: withOpacity(foreground(layer), 0.6),
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
      color: foreground(layer),
      iconWidth: 12,
      buttonWidth: activePaneActiveTab.height,
      hover: {
        color: foreground(layer, "hovered"),
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