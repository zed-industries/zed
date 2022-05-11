import Theme from "../themes/theme";
import { panel } from "./app";
import { backgroundColor, border, borderColor, iconColor, player, text } from "./components";

export default function contactsPanel(theme: Theme) {
  const nameMargin = 8;
  const sidePadding = 12;

  const projectRow = {
    guestAvatarSpacing: 4,
    height: 24,
    guestAvatar: {
      cornerRadius: 8,
      width: 14,
    },
    name: {
      ...text(theme, "mono", "placeholder", { size: "sm" }),
      margin: {
        left: nameMargin,
        right: 6,
      },
    },
    guests: {
      margin: {
        left: nameMargin,
        right: nameMargin,
      }
    },
    padding: {
      left: sidePadding,
      right: sidePadding,
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
      margin: {
        left: sidePadding,
        right: sidePadding,
      }
    },
    userQueryEditorHeight: 32,
    addContactButton: {
      margin: { left: 6, right: 12 },
      color: iconColor(theme, "primary"),
      buttonWidth: 8,
      iconWidth: 8,
    },
    rowHeight: 28,
    headerRow: {
      ...text(theme, "mono", "secondary", { size: "sm" }),
      margin: { top: 14 },
      padding: {
        left: sidePadding,
        right: sidePadding,
      },
      active: {
        ...text(theme, "mono", "primary", { size: "sm" }),
        background: backgroundColor(theme, 100, "active"),
      }
    },
    contactRow: {
      padding: {
        left: sidePadding,
        right: sidePadding
      },
      active: {
        background: backgroundColor(theme, 100, "active"),
      }
    },
    treeBranch: {
      color: borderColor(theme, "active"),
      width: 1,
      hover: {
        color: borderColor(theme, "active"),
      },
      active: {
        color: borderColor(theme, "active"),
      }
    },
    contactAvatar: {
      cornerRadius: 10,
      width: 18,
    },
    contactUsername: {
      ...text(theme, "mono", "primary", { size: "sm" }),
      margin: {
        left: nameMargin,
      },
    },
    contactButtonSpacing: nameMargin,
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
      ...projectRow,
      background: backgroundColor(theme, 300),
      name: {
        ...projectRow.name,
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
      ...projectRow,
      background: backgroundColor(theme, 300),
      name: {
        ...projectRow.name,
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
