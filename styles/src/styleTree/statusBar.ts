import Theme from "../themes/common/theme";
import { backgroundColor, border, iconColor, text } from "./components";
import { workspaceBackground } from "./workspace";

export default function statusBar(theme: Theme) {
  const statusContainer = {
    cornerRadius: 6,
    padding: { top: 3, bottom: 3, left: 6, right: 6 }
  }

  const diagnosticStatusContainer = {
    cornerRadius: 6,
    padding: { top: 1, bottom: 1, left: 6, right: 6 }
  }

  return {
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
      ...diagnosticStatusContainer,
      iconSpacing: 4,
      iconWidth: 14,
      height: 18,
      message: text(theme, "sans", "muted"),
      iconColor: iconColor(theme, "muted"),
      hover: {
        message: text(theme, "sans", "primary"),
        iconColor: iconColor(theme, "primary"),
        background: backgroundColor(theme, 300, "hovered"),
      }
    },
    diagnosticMessage: {
      ...text(theme, "sans", "muted"),
      hover: text(theme, "sans", "secondary"),
    },
    diagnosticSummary: {
      height: 16,
      iconWidth: 14,
      iconSpacing: 2,
      summarySpacing: 6,
      text: text(theme, "sans", "primary", { size: "sm" }),
      iconColorOk: iconColor(theme, "secondary"),
      iconColorWarning: iconColor(theme, "warning"),
      iconColorError: iconColor(theme, "error"),
      containerOk: {
        cornerRadius: 6,
        padding: { top: 3, bottom: 3, left: 7, right: 7 },
      },
      containerWarning: {
        ...diagnosticStatusContainer,
        background: backgroundColor(theme, "warning"),
        border: border(theme, "warning"),
      },
      containerError: {
        ...diagnosticStatusContainer,
        background: backgroundColor(theme, "error"),
        border: border(theme, "error"),
      },
      hover: {
        iconColorOk: iconColor(theme, "primary"),
        containerOk: {
          cornerRadius: 6,
          padding: { top: 3, bottom: 3, left: 7, right: 7 },
          background: backgroundColor(theme, 300, "hovered"),
        },
        containerWarning: {
          ...diagnosticStatusContainer,
          background: backgroundColor(theme, "warning", "hovered"),
          border: border(theme, "warning"),
        },
        containerError: {
          ...diagnosticStatusContainer,
          background: backgroundColor(theme, "error", "hovered"),
          border: border(theme, "error"),
        }
      },
    },
    sidebarButtons: {
      groupLeft: {},
      groupRight: {},
      item: {
        ...statusContainer,
        iconSize: 14,
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
      badge: {
        cornerRadius: 3,
        padding: 2,
        margin: { bottom: -1, right: -1 },
        border: { width: 1, color: workspaceBackground(theme) },
        background: iconColor(theme, "feature"),
      }
    }
  }
}
