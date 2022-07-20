import Theme from "../themes/common/theme";
import { withOpacity } from "../utils/color";
import {
  backgroundColor,
  border,
  iconColor,
  modalShadow,
  text,
} from "./components";
import statusBar from "./statusBar";

export function workspaceBackground(theme: Theme) {
  return backgroundColor(theme, 300);
}

export default function workspace(theme: Theme) {
  const tab = {
    height: 32,
    background: workspaceBackground(theme),
    iconClose: iconColor(theme, "muted"),
    iconCloseActive: iconColor(theme, "active"),
    iconConflict: iconColor(theme, "warning"),
    iconDirty: iconColor(theme, "info"),
    iconWidth: 8,
    spacing: 8,
    text: text(theme, "sans", "secondary", { size: "sm" }),
    border: border(theme, "primary", {
      left: true,
      bottom: true,
      overlay: true,
    }),
    padding: {
      left: 8,
      right: 8,
    },
    description: {
      margin: { left: 6, top: 1 },
      ...text(theme, "sans", "muted", { size: "2xs" })
    }
  };

  const activeTab = {
    ...tab,
    background: backgroundColor(theme, 500),
    text: text(theme, "sans", "active", { size: "sm" }),
    border: {
      ...tab.border,
      bottom: false,
    },
  };

  const titlebarPadding = 6;

  return {
    background: backgroundColor(theme, 300),
    joiningProjectAvatar: {
      cornerRadius: 40,
      width: 80,
    },
    joiningProjectMessage: {
      padding: 12,
      ...text(theme, "sans", "primary", { size: "lg" }),
    },
    leaderBorderOpacity: 0.7,
    leaderBorderWidth: 2.0,
    tab,
    activeTab,
    paneButton: {
      color: iconColor(theme, "secondary"),
      border: {
        ...tab.border,
      },
      iconWidth: 12,
      buttonWidth: tab.height,
      hover: {
        color: iconColor(theme, "active"),
        background: backgroundColor(theme, 300),
      },
    },
    modal: {
      margin: {
        bottom: 52,
        top: 52,
      },
      cursor: "Arrow",
    },
    sidebarResizeHandle: {
      background: border(theme, "primary").color,
      padding: {
        left: 1,
      },
    },
    paneDivider: {
      color: border(theme, "secondary").color,
      width: 1,
    },
    statusBar: statusBar(theme),
    titlebar: {
      avatarWidth: 18,
      avatarMargin: 8,
      height: 33,
      background: backgroundColor(theme, 100),
      padding: {
        left: 80,
        right: titlebarPadding,
      },
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
        // TODO: The background for this ideally should be
        // set with a token, not hardcoded in rust
      },
      border: border(theme, "primary", { bottom: true, overlay: true }),
      signInPrompt: {
        background: backgroundColor(theme, 100),
        border: border(theme, "secondary"),
        cornerRadius: 6,
        margin: {
          top: 1,
        },
        padding: {
          top: 1,
          bottom: 1,
          left: 7,
          right: 7,
        },
        ...text(theme, "sans", "secondary", { size: "xs" }),
        hover: {
          ...text(theme, "sans", "active", { size: "xs" }),
          background: backgroundColor(theme, "on300", "hovered"),
          border: border(theme, "primary"),
        },
      },
      offlineIcon: {
        color: iconColor(theme, "secondary"),
        width: 16,
        margin: {
          left: titlebarPadding,
        },
        padding: {
          right: 4,
        },
      },
      outdatedWarning: {
        ...text(theme, "sans", "warning", { size: "xs" }),
        background: backgroundColor(theme, "warning"),
        border: border(theme, "warning"),
        margin: {
          left: titlebarPadding,
        },
        padding: {
          left: 6,
          right: 6,
        },
        cornerRadius: 6,
      },
    },
    toolbar: {
      height: 34,
      background: backgroundColor(theme, 500),
      border: border(theme, "secondary", { bottom: true }),
      itemSpacing: 8,
      navButton: {
        color: iconColor(theme, "primary"),
        iconWidth: 12,
        buttonWidth: 24,
        cornerRadius: 6,
        hover: {
          color: iconColor(theme, "active"),
          background: backgroundColor(theme, 300),
        },
        disabled: {
          color: withOpacity(iconColor(theme, "muted"), 0.6),
        },
      },
      padding: { left: 8, right: 8, top: 4, bottom: 4 },
    },
    breadcrumbs: {
      ...text(theme, "mono", "secondary"),
      padding: { left: 6 },
    },
    disconnectedOverlay: {
      ...text(theme, "sans", "active"),
      background: withOpacity(theme.backgroundColor[500].base, 0.8),
    },
    notification: {
      margin: { top: 10 },
      background: backgroundColor(theme, 300),
      cornerRadius: 6,
      padding: 12,
      border: border(theme, "primary"),
      shadow: modalShadow(theme),
    },
    notifications: {
      width: 400,
      margin: { right: 10, bottom: 10 },
    },
  };
}
