import Theme from "../themes/theme";
import { backgroundColor, border, player, text } from "./components";

export default function search(theme: Theme) {

  // Search input
  const editor = {
    background: backgroundColor(theme, 500),
    cornerRadius: 8,
    minWidth: 200,
    maxWidth: 500,
    placeholderText: text(theme, "mono", "placeholder"),
    selection: player(theme, 1).selection,
    text: text(theme, "mono", "active"),
    border: border(theme, "secondary"),
    margin: {
      right: 12,
    },
    padding: {
      top: 3,
      bottom: 3,
      left: 12,
      right: 8,
    },
  };

  return {
    matchBackground: theme.editor.highlight.match.value,
    tabIconSpacing: 8,
    tabIconWidth: 14,
    optionButton: {
      ...text(theme, "mono", "secondary"),
      background: backgroundColor(theme, "on500"),
      cornerRadius: 6,
      border: border(theme, "secondary"),
      margin: {
        right: 4,
      },
      padding: {
        bottom: 2,
        left: 10,
        right: 10,
        top: 2,
      },
      active: {
        ...text(theme, "mono", "active"),
        background: backgroundColor(theme, "on500", "active"),
        border: border(theme, "muted"),
      },
      hover: {
        ...text(theme, "mono", "active"),
        background: backgroundColor(theme, "on500", "hovered"),
        border: border(theme, "muted"),
      }
    },
    editor,
    invalidEditor: {
      ...editor,
      border: border(theme, "error"),
    },
    matchIndex: {
      ...text(theme, "mono", "muted"),
      padding: 6,
    },
    optionButtonGroup: {
      padding: {
        left: 12,
        right: 12,
      },
    },
    resultsStatus: {
      ...text(theme, "mono", "primary"),
      size: 18,
    },
  };
}
