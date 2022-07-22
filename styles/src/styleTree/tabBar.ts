import Theme from "../themes/common/theme";
import { iconColor, text, border, backgroundColor } from "./components";

export default function tabBar(theme: Theme) {
  const height = 32;

  const tab = {
    height,
    background: backgroundColor(theme, 300),
    border: border(theme, "primary", {
      left: true,
      bottom: true,
      overlay: true,
    }),
    iconClose: iconColor(theme, "muted"),
    iconCloseActive: iconColor(theme, "active"),
    iconConflict: iconColor(theme, "warning"),
    iconDirty: iconColor(theme, "info"),
    iconWidth: 8,
    spacing: 8,
    text: text(theme, "sans", "secondary", { size: "sm" }),
    padding: {
      left: 8,
      right: 8,
    },
    description: {
      margin: { left: 6, top: 1 },
      ...text(theme, "sans", "muted", { size: "2xs" })
    }
  };

  const activePaneActiveTab = {
    ...tab,
    background: backgroundColor(theme, 500),
    text: text(theme, "sans", "active", { size: "sm" }),
    border: {
      ...tab.border,
    },
  };

  const inactivePaneInactiveTab = {
    ...tab,
    background: backgroundColor(theme, 300),
    text: text(theme, "sans", "muted", { size: "sm" }),
  };

  const inactivePaneActiveTab = {
    ...tab,
    background: backgroundColor(theme, 500),
    text: text(theme, "sans", "secondary", { size: "sm" }),
    border: {
      ...tab.border,
    },
  }

  return {
    height,
    background: backgroundColor(theme, 300),
    border: border(theme, "primary", {
      left: true,
      bottom: true,
      overlay: true,
    }),
    activePane: {
      activeTab: activePaneActiveTab,
      inactiveTab: tab,
    },
    inactivePane: {
      activeTab: inactivePaneActiveTab,
      inactiveTab: inactivePaneInactiveTab,
    },
    paneButton: {
      color: iconColor(theme, "secondary"),
      border: {
        ...activePaneActiveTab.border,
      },
      iconWidth: 12,
      buttonWidth: activePaneActiveTab.height,
      hover: {
        color: iconColor(theme, "active"),
        background: backgroundColor(theme, 300),
      },
    },
  }
}