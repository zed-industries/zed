import Theme from "../themes/common/theme";
import picker from "./picker";
import { backgroundColor, border, iconColor, player, text } from "./components";

export default function contactFinder(theme: Theme) {
  const sideMargin = 6;
  const contactButton = {
    background: backgroundColor(theme, 100),
    color: iconColor(theme, "primary"),
    iconWidth: 8,
    buttonWidth: 16,
    cornerRadius: 8,
  };

  return {
    picker: {
      item: {
        ...picker(theme).item,
        margin: { left: sideMargin, right: sideMargin }
      },
      empty: picker(theme).empty,
      inputEditor: {
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
          left: sideMargin,
          right: sideMargin,
        }
      }
    },
    rowHeight: 28,
    contactAvatar: {
      cornerRadius: 10,
      width: 18,
    },
    contactUsername: {
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
  };
}
