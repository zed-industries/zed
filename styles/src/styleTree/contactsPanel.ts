import Theme from "../themes/theme";
import { panel } from "./app";
import { backgroundColor, border, borderColor, iconColor, player, text } from "./components";

export default function contactsPanel(theme: Theme) {
  const project = {
    guestAvatarSpacing: 4,
    height: 24,
    guestAvatar: {
      cornerRadius: 8,
      width: 14,
    },
    name: {
      ...text(theme, "mono", "placeholder", { size: "sm" }),
      margin: {
        right: 6,
      },
    },
    padding: {
      left: 8,
    },
  };

  const contactButton = {
    background: backgroundColor(theme, 100),
    color: iconColor(theme, "primary"),
    iconWidth: 8,
    buttonWidth: 16,
    cornerRadius: 8,
  };

  return {
    ...panel,
    userQueryEditor: {
      background: backgroundColor(theme, 500),
      cornerRadius: 6,
      text: text(theme, "mono", "primary"),
      placeholderText: text(theme, "mono", "placeholder", { size: "sm" }),
      selection: player(theme, 1).selection,
      border: border(theme, "secondary"),
      padding: {
        bottom: 4,
        left: 8,
        right: 8,
        top: 4,
      },
    },
    userQueryEditorHeight: 32,
    addContactButton: {
      margin: { left: 6 },
      color: iconColor(theme, "primary"),
      buttonWidth: 8,
      iconWidth: 8,
    },
    headerRow: {
      ...text(theme, "mono", "secondary", { size: "sm" }),
      margin: { top: 8 },
      active: {
        ...text(theme, "mono", "primary", { size: "sm" }),
        background: backgroundColor(theme, 100, "active"),
      }
    },
    contactRow: {
      padding: { left: 8 },
      active: {
        background: backgroundColor(theme, 100, "active"),
      }
    },
    rowHeight: 28,
    treeBranchColor: borderColor(theme, "muted"),
    treeBranchWidth: 1,
    contactAvatar: {
      cornerRadius: 10,
      width: 18,
    },
    contactUsername: {
      ...text(theme, "mono", "primary", { size: "sm" }),
      padding: {
        left: 8,
      },
    },
    contactButton: {
      ...contactButton,
      hover: {
        background: backgroundColor(theme, 100, "hovered"),
      },
    },
    disabledContactButton: {
      ...contactButton,
      background: backgroundColor(theme, 100),
      color: iconColor(theme, "muted"),
    },
    sharedProjectRow: {
      ...project,
      background: backgroundColor(theme, 300),
      cornerRadius: 6,
      name: {
        ...project.name,
        ...text(theme, "mono", "secondary", { size: "sm" }),
      },
      hover: {
        background: backgroundColor(theme, 300, "hovered"),
      },
      active: {
        background: backgroundColor(theme, 300, "active"),
      }
    },
    unsharedProjectRow: {
      ...project,
      background: backgroundColor(theme, 300),
      cornerRadius: 6,
      name: {
        ...project.name,
        ...text(theme, "mono", "secondary", { size: "sm" }),
      },
      hover: {
        background: backgroundColor(theme, 300, "hovered"),
      },
      active: {
        background: backgroundColor(theme, 300, "active"),
      }
    }
  }
}
