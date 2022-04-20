import Theme from "../themes/theme";
import { backgroundColor, border, player, text } from "./components";

export default function search(theme: Theme) {
  const optionButton = {
    ...text(theme, "mono", "secondary"),
    background: backgroundColor(theme, "on500"),
    cornerRadius: 4,
    border: border(theme, "secondary"),
    margin: {
      left: 2,
      right: 2,
    },
    padding: {
      bottom: 3,
      left: 8,
      right: 8,
      top: 3,
    },
  };

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
      right: 6,
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
    activeHoveredOptionButton: {
      ...optionButton,
      ...text(theme, "mono", "active"),
      background: backgroundColor(theme, "on500", "active"),
      border: border(theme, "muted"),
    },
    activeOptionButton: {
      ...optionButton,
      ...text(theme, "mono", "active"),
      background: backgroundColor(theme, "on500", "active"),
      border: border(theme, "muted"),
    },
    editor,
    hoveredOptionButton: {
      ...optionButton,
      ...text(theme, "mono", "active"),
      border: border(theme, "muted"),
    },
    invalidEditor: {
      ...editor,
      border: border(theme, "error"),
    },
    matchIndex: {
      ...text(theme, "mono", "muted"),
      padding: 6,
    },
    optionButton,
    optionButtonGroup: {
      padding: {
        left: 4,
        right: 4,
      },
    },
    resultsStatus: {
      ...text(theme, "mono", "primary"),
      size: 18,
    },
  };
}
