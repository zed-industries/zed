import { ColorScheme } from "../themes/common/colorScheme";
import { withOpacity } from "../utils/color";
import { background, border, foreground, text } from "./components";
import { icon_button, icon_label_button, label_button } from "./design_system/buttons";

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
    previous: label_button("<", "Select Previous Match", layer),
    next: label_button(">", "Select Next Match", layer),
    whole_word: label_button("Word", "Toggle Match Whole Word", layer),
    case_sensitive: label_button("Case", "Toggle Match Case", layer),
    regex: label_button("Regex", "Toggle Use Regular Expression", layer),
  };
}
