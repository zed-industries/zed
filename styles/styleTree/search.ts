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

  return {
    background: backgroundColor(theme, 300),
    matchBackground: theme.editor.highlight.match,
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
    editor: {
      background: backgroundColor(theme, 500),
      cornerRadius: 6,
      maxWidth: 400,
      placeholderText: text(theme, "mono", "placeholder"),
      selection: player(theme, 1).selection,
      text: text(theme, "mono", "primary"),
      border: border(theme, "primary"),
      margin: {
        bottom: 5,
        left: 5,
        right: 5,
        top: 5,
      },
      padding: {
        bottom: 3,
        left: 13,
        right: 13,
        top: 3,
      },
    },
    hoveredOptionButton: {
      ...optionButton,
      background: backgroundColor(theme, 100),
    },
    invalidEditor: {
      extends: "$search.editor",
      border: border(theme, "error"),
    },
    matchIndex: {
      ...text(theme, "mono", "secondary"),
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
