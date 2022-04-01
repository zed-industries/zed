import Theme from "../themes/theme";
import { backgroundColor, border, player, shadow, text } from "./components";

export default function selectorModal(theme: Theme): Object {
  const item = {
    padding: {
      bottom: 4,
      left: 16,
      right: 16,
      top: 4,
    },
    cornerRadius: 6,
    text: text(theme, "sans", "secondary"),
    highlightText: text(theme, "sans", "feature", { weight: "bold" }),
  };

  const activeItem = {
    ...item,
    background: backgroundColor(theme, 500, "active"),
    text: text(theme, "sans", "primary"),
  };

  return {
    background: backgroundColor(theme, 500),
    cornerRadius: 6,
    padding: 8,
    item,
    activeItem,
    border: border(theme, "primary"),
    empty: {
      text: text(theme, "sans", "muted"),
      padding: {
        bottom: 4,
        left: 16,
        right: 16,
        top: 8,
      },
    },
    inputEditor: {
      background: backgroundColor(theme, 300),
      corner_radius: 6,
      placeholderText: text(theme, "sans", "placeholder"),
      selection: player(theme, 1).selection,
      text: text(theme, "mono", "primary"),
      border: border(theme, "primary"),
      padding: {
        bottom: 7,
        left: 16,
        right: 16,
        top: 7,
      },
    },
    margin: {
      bottom: 52,
      top: 52,
    },
    shadow: shadow(theme),
  };
}
