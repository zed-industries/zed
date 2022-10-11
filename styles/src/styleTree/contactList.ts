import Theme from "../themes/common/theme";
import { backgroundColor, border, borderColor, iconColor, player, text } from "./components";

export default function contactList(theme: Theme) {
  const nameMargin = 8;
  const sidePadding = 12;

  const contactButton = {
    background: backgroundColor(theme, 100),
    color: iconColor(theme, "primary"),
    iconWidth: 8,
    buttonWidth: 16,
    cornerRadius: 8,
  };
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
      },
    },
    padding: {
      left: sidePadding,
      right: sidePadding,
    },
  };

  return {
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
      },
    },
    userQueryEditorHeight: 33,
    addContactButton: {
      margin: { left: 6, right: 12 },
      color: iconColor(theme, "primary"),
      buttonWidth: 16,
      iconWidth: 16,
    },
    rowHeight: 28,
    sectionIconSize: 8,
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
      },
    },
    leaveCall: {
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
    contactRow: {
      padding: {
        left: sidePadding,
        right: sidePadding,
      },
      active: {
        background: backgroundColor(theme, 100, "active"),
      },
    },
    contactAvatar: {
      cornerRadius: 10,
      width: 18,
    },
    contactStatusFree: {
      cornerRadius: 4,
      padding: 4,
      margin: { top: 12, left: 12 },
      background: iconColor(theme, "ok"),
    },
    contactStatusBusy: {
      cornerRadius: 4,
      padding: 4,
      margin: { top: 12, left: 12 },
      background: iconColor(theme, "error"),
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
        background: backgroundColor(theme, "on300", "hovered"),
      },
    },
    disabledButton: {
      ...contactButton,
      background: backgroundColor(theme, 100),
      color: iconColor(theme, "muted"),
    },
    inviteRow: {
      padding: {
        left: sidePadding,
        right: sidePadding,
      },
      border: { top: true, width: 1, color: borderColor(theme, "primary") },
      text: text(theme, "sans", "secondary", { size: "sm" }),
      hover: {
        text: text(theme, "sans", "active", { size: "sm" }),
      },
    },
    callingIndicator: {
      ...text(theme, "mono", "muted", { size: "xs" })
    },
    treeBranch: {
      color: borderColor(theme, "active"),
      width: 1,
      hover: {
        color: borderColor(theme, "active"),
      },
      active: {
        color: borderColor(theme, "active"),
      },
    },
    projectRow: {
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
      },
    },
  }
}
