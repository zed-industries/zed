import { ColorScheme } from "../themes/common/colorScheme";
import { withOpacity } from "../utils/color";
import { background, border, foreground, text } from "./components";

export default function search(colorScheme: ColorScheme) {
  let layer = colorScheme.highest;

  // Search input
  const editor = {
    background: background(layer),
    cornerRadius: 8,
    minWidth: 200,
    maxWidth: 500,
    placeholderText: text(layer, "mono", "disabled"),
    selection: colorScheme.players[0],
    text: text(layer, "mono", "default"),
    border: border(layer),
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
    // TODO: Add an activeMatchBackground on the rust side to differenciate between active and inactive
    matchBackground: withOpacity(foreground(layer, "accent"), 0.4),
    tabIconSpacing: 8,
    tabIconWidth: 14,
    optionButton: {
      ...text(layer, "mono", "on"),
      background: background(layer, "on"),
      cornerRadius: 6,
      border: border(layer, "on"),
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
        ...text(layer, "mono", "on", "inverted"),
        background: background(layer, "on", "inverted"),
        border: border(layer, "on", "inverted"),
      },
      clicked: {
        ...text(layer, "mono", "on", "pressed"),
        background: background(layer, "on", "pressed"),
        border: border(layer, "on", "pressed"),
      },
      hover: {
        ...text(layer, "mono", "on", "hovered"),
        background: background(layer, "on", "hovered"),
        border: border(layer, "on", "hovered"),
      },
    },
    editor,
    invalidEditor: {
      ...editor,
      border: border(layer, "negative"),
    },
    matchIndex: {
      ...text(layer, "mono", "variant"),
      padding: 6,
    },
    optionButtonGroup: {
      padding: {
        left: 12,
        right: 12,
      },
    },
    resultsStatus: {
      ...text(layer, "mono", "on"),
      size: 18,
    },
  };
}
