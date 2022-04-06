import Theme from "../themes/theme";
import { backgroundColor, border, player, text } from "./components";

export default function search(theme: Theme) {
  const optionButton = {
    ...text(theme, "mono", "secondary"),
    background: backgroundColor(theme, 300),
    cornerRadius: 6,
    border: border(theme, "primary"),
    margin: {
      left: 1,
      right: 1,
    },
    padding: {
      bottom: 1,
      left: 6,
      right: 6,
      top: 1,
    },
  };

  const editor = {
    background: backgroundColor(theme, 500),
    cornerRadius: 6,
    minWidth: 200,
    maxWidth: 500,
    placeholderText: text(theme, "mono", "placeholder"),
    selection: player(theme, 1).selection,
    text: text(theme, "mono", "primary"),
    border: border(theme, "secondary"),
    margin: {
      right: 5,
    },
    padding: {
      top: 3,
      bottom: 3,
      left: 14,
      right: 14,
    },
  };

  return {
    matchBackground: theme.editor.highlight.match.value,
    tabIconSpacing: 4,
    tabIconWidth: 14,
    activeHoveredOptionButton: {
      ...optionButton,
      background: backgroundColor(theme, 100),
    },
    activeOptionButton: {
      ...optionButton,
      background: backgroundColor(theme, 100),
    },
    editor,
    hoveredOptionButton: {
      ...optionButton,
      background: backgroundColor(theme, 100),
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
        left: 2,
        right: 2,
      },
    },
    resultsStatus: {
      ...text(theme, "mono", "primary"),
      size: 18,
    },
  };
}
