import Theme from "../themes/theme";
import { backgroundColor, border, iconColor, text } from "./components";

export default function workspace(theme: Theme) {
  const signInPrompt = {
    ...text(theme, "sans", "secondary"),
    size: 13,
    underline: true,
    padding: {
      right: 8,
    },
  };

  const tab = {
    height: 34,
    iconClose: iconColor(theme, "secondary"),
    iconCloseActive: iconColor(theme, "active"),
    iconConflict: iconColor(theme, "warning"),
    iconDirty: iconColor(theme, "info"),
    iconWidth: 8,
    spacing: 10,
    text: text(theme, "mono", "secondary"),
    border: border(theme, "primary", {
      left: true,
      bottom: true,
      overlay: true,
    }),
    padding: {
      left: 12,
      right: 12,
    },
  };

  const activeTab = {
    ...tab,
    background: backgroundColor(theme, 300),
    text: text(theme, "mono", "primary"),
    border: {
      ...tab.border,
      bottom: false,
    },
  };

  const sidebarItem = {
    height: 32,
    iconColor: iconColor(theme, "secondary"),
    iconSize: 18,
  };
  const sidebar = {
    width: 30,
    border: border(theme, "primary", { right: true }),
    item: sidebarItem,
    activeItem: {
      ...sidebarItem,
      iconColor: iconColor(theme, "primary"),
    },
    resizeHandle: {
      background: border(theme, "primary").color,
      padding: {
        left: 1,
      },
    },
  };

  return {
    background: backgroundColor(theme, 500),
    leaderBorderOpacity: 0.7,
    leaderBorderWidth: 2.0,
    tab,
    activeTab,
    leftSidebar: {
      ...sidebar,
      border: border(theme, "primary", { right: true }),
    },
    rightSidebar: {
      ...sidebar,
      border: border(theme, "primary", { left: true }),
    },
    paneDivider: {
      color: border(theme, "primary").color,
      width: 1,
    },
    status_bar: {
      height: 24,
      itemSpacing: 8,
      padding: {
        left: 6,
        right: 6,
      },
      cursorPosition: text(theme, "sans", "muted"),
      diagnosticMessage: text(theme, "sans", "muted"),
      lspMessage: text(theme, "sans", "muted"),
    },
    titlebar: {
      avatarWidth: 18,
      height: 32,
      shareIconColor: iconColor(theme, "secondary"),
      shareIconActiveColor: iconColor(theme, "active"),
      title: text(theme, "sans", "primary"),
      avatar: {
        cornerRadius: 10,
        border: {
          color: "#00000088",
          width: 1,
        },
      },
      avatarRibbon: {
        height: 3,
        width: 12,
      },
      border: border(theme, "primary", { bottom: true }),
      signInPrompt,
      hoveredSignInPrompt: {
        ...signInPrompt,
        ...text(theme, "mono", "active"),
      },
      offlineIcon: {
        color: iconColor(theme, "muted"),
        width: 16,
        padding: {
          right: 4,
        },
      },
      outdatedWarning: {
        ...text(theme, "sans", "muted"),
        size: 13,
      },
    },
    toolbar: {
      height: 44,
    },
  };
}
