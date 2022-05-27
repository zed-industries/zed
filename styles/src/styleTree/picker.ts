import Theme from "../themes/common/theme";
import { backgroundColor, border, player, modalShadow, text } from "./components";

export default function picker(theme: Theme) {
  return {
    background: backgroundColor(theme, 300),
    cornerRadius: 8,
    padding: 8,
    item: {
      padding: {
        bottom: 4,
        left: 12,
        right: 12,
        top: 4,
      },
      cornerRadius: 8,
      text: text(theme, "sans", "secondary"),
      highlightText: text(theme, "sans", "feature", { weight: "bold" }),
      active: {
        background: backgroundColor(theme, 300, "active"),
        text: text(theme, "sans", "primary"),
      },
      hover: {
        background: backgroundColor(theme, 300, "hovered"),
      }
    },
    border: border(theme, "primary"),
    empty: {
      text: text(theme, "sans", "placeholder"),
      padding: {
        bottom: 4,
        left: 12,
        right: 12,
        top: 8,
      },
    },
    inputEditor: {
      background: backgroundColor(theme, 500),
      cornerRadius: 8,
      placeholderText: text(theme, "sans", "placeholder"),
      selection: player(theme, 1).selection,
      text: text(theme, "mono", "primary"),
      border: border(theme, "secondary"),
      padding: {
        bottom: 7,
        left: 16,
        right: 16,
        top: 7,
      },
    },
    shadow: modalShadow(theme),
  };
}
