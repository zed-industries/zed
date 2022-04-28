import Theme from "../themes/theme";
import { backgroundColor, border, iconColor, text } from "./components";

export default function workspace(theme: Theme) {
  const tab = {
    height: 32,
    background: backgroundColor(theme, 300),
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

  const diagnosticSummary = {
    cornerRadius: 6,
    padding: { left: 6, right: 6 },
    ...text(theme, "sans", "primary", { size: "sm" }),
  };

  return {
    background: backgroundColor(theme, 300),
    leaderBorderOpacity: 0.7,
    leaderBorderWidth: 2.0,
    tab,
    activeTab,
    modal: {
      margin: {
        bottom: 52,
        top: 52,
      },
      cursor: "Arrow"
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
    statusBar: {
      height: 30,
      itemSpacing: 8,
      padding: {
        top: 1,
        bottom: 1,
        left: 6,
        right: 6,
      },
      border: border(theme, "primary", { top: true, overlay: true }),
      cursorPosition: text(theme, "sans", "muted"),
      autoUpdateProgressMessage: text(theme, "sans", "muted"),
      autoUpdateDoneMessage: text(theme, "sans", "muted"),
      lspStatus: {
        iconSpacing: 4,
        iconWidth: 14,
        height: 18,
        cornerRadius: 6,
        padding: { left: 6, right: 6 },
        message: text(theme, "sans", "muted"),
        iconColor: iconColor(theme, "muted"),
        hover: {
          message: text(theme, "sans", "primary"),
          iconColor: iconColor(theme, "primary"),
          background: backgroundColor(theme, 300, "hovered"),
        }
      },
      diagnostics: {
        height: 16,
        summaryOk: {
          ...diagnosticSummary,
          hover: {
            background: backgroundColor(theme, 300, "hovered"),
          }
        },
        summaryWarning: {
          ...diagnosticSummary,
          background: backgroundColor(theme, "warning"),
          border: border(theme, "warning"),
          hover: {
            background: backgroundColor(theme, "warning", "hovered"),
          }
        },
        summaryError: {
          ...diagnosticSummary,
          background: backgroundColor(theme, "error"),
          border: border(theme, "error"),
          hover: {
            background: backgroundColor(theme, "error", "hovered"),
          }
        },
        message: {
          ...text(theme, "sans", "muted"),
          hover: text(theme, "sans", "secondary"),
        },
        iconColorOk: iconColor(theme, "secondary"),
        iconColorWarning: iconColor(theme, "warning"),
        iconColorError: iconColor(theme, "error"),
        iconWidth: 14,
        iconSpacing: 2,
        summarySpacing: 6,
      },
      sidebarButtons: {
        groupLeft: {},
        groupRight: {},
        item: {
          iconSize: 14,
          padding: { top: 3, bottom: 3, left: 6, right: 6 },
          cornerRadius: 6,
          iconColor: iconColor(theme, "secondary"),
          hover: {
            iconColor: iconColor(theme, "primary"),
            background: backgroundColor(theme, 300, "hovered"),
          },
          active: {
            iconColor: iconColor(theme, "active"),
            background: backgroundColor(theme, 300, "active"),
          }
        },
      },
    },
    titlebar: {
      avatarWidth: 18,
      height: 32,
      background: backgroundColor(theme, 100),
      padding: {
        left: 80,
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
      border: border(theme, "primary", { bottom: true }),
      signInPrompt: {
        border: border(theme, "primary"),
        cornerRadius: 6,
        margin: {
          top: 1,
          right: 6,
        },
        padding: {
          left: 6,
          right: 6,
        },
        ...text(theme, "sans", "secondary", { size: "xs" }),
        hover: text(theme, "sans", "active", { size: "xs" }),
      },
      offlineIcon: {
        color: iconColor(theme, "secondary"),
        width: 16,
        padding: {
          right: 4,
        },
      },
      shareIcon: {
        cornerRadius: 6,
        margin: { top: 3, bottom: 2 },
        color: iconColor(theme, "secondary"),
        hover: {
          background: backgroundColor(theme, 100, "hovered"),
          color: iconColor(theme, "secondary"),
        },
        active: {
          background: backgroundColor(theme, 100, "active"),
          color: iconColor(theme, "active"),
        },
        activeHover: {
          background: backgroundColor(theme, 100, "hovered"),
          color: iconColor(theme, "active"),
        }
      },
      outdatedWarning: {
        ...text(theme, "sans", "warning"),
        size: 13,
        margin: { right: 6 }
      },
    },
    toolbar: {
      height: 34,
      background: backgroundColor(theme, 500),
      border: border(theme, "secondary", { bottom: true }),
      itemSpacing: 8,
      padding: { left: 16, right: 8, top: 4, bottom: 4 },
    },
    breadcrumbs: {
      ...text(theme, "mono", "secondary"),
      padding: { left: 6 },
    },
    disconnectedOverlay: {
      ...text(theme, "sans", "active"),
      background: "#000000aa",
    },
  };
}
