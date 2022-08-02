import Theme from "../themes/common/theme";
import { backgroundColor, border, iconColor, text } from "./components";
import { workspaceBackground } from "./workspace";

export default function statusBar(theme: Theme) {
  const statusContainer = {
    cornerRadius: 6,
    padding: { top: 3, bottom: 3, left: 6, right: 6 },
  };

  const diagnosticStatusContainer = {
    cornerRadius: 6,
    padding: { top: 1, bottom: 1, left: 6, right: 6 },
  };

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
    cursorPosition: text(theme, "sans", "secondary"),
    autoUpdateProgressMessage: text(theme, "sans", "secondary"),
    autoUpdateDoneMessage: text(theme, "sans", "secondary"),
    lspStatus: {
      ...diagnosticStatusContainer,
      iconSpacing: 4,
      iconWidth: 14,
      height: 18,
      message: text(theme, "sans", "secondary"),
      iconColor: iconColor(theme, "muted"),
      hover: {
        message: text(theme, "sans", "primary"),
        iconColor: iconColor(theme, "primary"),
        background: backgroundColor(theme, 300, "hovered"),
      },
    },
    diagnosticMessage: {
      ...text(theme, "sans", "secondary"),
      hover: text(theme, "sans", "active"),
    },
    feedback: {
      ...text(theme, "sans", "secondary"),
      hover: text(theme, "sans", "active"),
    },
    diagnosticSummary: {
      height: 16,
      iconWidth: 16,
      iconSpacing: 2,
      summarySpacing: 6,
      text: text(theme, "sans", "primary", { size: "sm" }),
      iconColorOk: iconColor(theme, "muted"),
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
        iconColorOk: iconColor(theme, "active"),
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
        },
      },
    },
    sidebarButtons: {
      groupLeft: {},
      groupRight: {},
      item: {
        ...statusContainer,
        iconSize: 16,
        iconColor: iconColor(theme, "muted"),
        hover: {
          iconColor: iconColor(theme, "active"),
          background: backgroundColor(theme, 300, "hovered"),
        },
        active: {
          iconColor: iconColor(theme, "active"),
          background: backgroundColor(theme, 300, "active"),
        },
      },
      badge: {
        cornerRadius: 3,
        padding: 2,
        margin: { bottom: -1, right: -1 },
        border: { width: 1, color: workspaceBackground(theme) },
        background: iconColor(theme, "feature"),
      },
    },
  };
}
