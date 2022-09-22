import { ColorScheme } from "../themes/common/colorScheme";
import {
  background,
  border,
  borderColor,
  foreground,
  text,
} from "./components";

export default function contactsPanel(colorScheme: ColorScheme) {
  const nameMargin = 8;
  const sidePadding = 12;

  let layer = colorScheme.lowest.middle;

  const projectRow = {
    guestAvatarSpacing: 4,
    height: 24,
    guestAvatar: {
      cornerRadius: 8,
      width: 14,
    },
    name: {
      ...text(layer, "mono", { size: "sm" }),
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

  const contactButton = {
    background: background(layer, "on"),
    color: foreground(layer, "on"),
    iconWidth: 8,
    buttonWidth: 16,
    cornerRadius: 8,
  };

  return {
    background: background(layer),
    padding: { top: 12, bottom: 0 },
    userQueryEditor: {
      background: background(layer, "on"),
      cornerRadius: 6,
      text: text(layer, "mono", "on"),
      placeholderText: text(layer, "mono", "on", "disabled", { size: "sm" }),
      selection: colorScheme.players[0],
      border: border(layer, "on"),
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
    userQueryEditorHeight: 32,
    addContactButton: {
      margin: { left: 6, right: 12 },
      color: foreground(layer, "on"),
      buttonWidth: 16,
      iconWidth: 16,
    },
    privateButton: {
      iconWidth: 12,
      color: foreground(layer, "on"),
      cornerRadius: 5,
      buttonWidth: 12,
    },
    rowHeight: 28,
    sectionIconSize: 8,
    headerRow: {
      ...text(layer, "mono", { size: "sm" }),
      margin: { top: 14 },
      padding: {
        left: sidePadding,
        right: sidePadding,
      },
      active: {
        ...text(layer, "mono", "active", { size: "sm" }),
        background: background(layer, "active"),
      },
    },
    contactRow: {
      padding: {
        left: sidePadding,
        right: sidePadding,
      },
      active: {
        background: background(layer, "active"),
      },
    },
    treeBranch: {
      color: borderColor(layer),
      width: 1,
      hover: {
        color: borderColor(layer, "hovered"),
      },
      active: {
        color: borderColor(layer, "active"),
      },
    },
    contactAvatar: {
      cornerRadius: 10,
      width: 18,
    },
    contactUsername: {
      ...text(layer, "mono", { size: "sm" }),
      margin: {
        left: nameMargin,
      },
    },
    contactButtonSpacing: nameMargin,
    contactButton: {
      ...contactButton,
      hover: {
        background: background(layer, "hovered"),
      },
    },
    disabledButton: {
      ...contactButton,
      background: background(layer, "on"),
      color: foreground(layer, "on"),
    },
    projectRow: {
      ...projectRow,
      background: background(layer, "on"),
      name: {
        ...projectRow.name,
        ...text(layer, "mono", { size: "sm" }),
      },
      hover: {
        background: background(layer, "hovered"),
      },
      active: {
        background: background(layer, "active"),
      },
    },
    inviteRow: {
      padding: {
        left: sidePadding,
        right: sidePadding,
      },
      border: border(layer, { top: true }),
      text: text(layer, "sans", { size: "sm" }),
      hover: {
        text: text(layer, "sans", "hovered", { size: "sm" }),
      },
    },
  };
}
