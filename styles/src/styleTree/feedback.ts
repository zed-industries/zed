import { ColorScheme } from "../themes/common/colorScheme";
import { background, border, text } from "./components";

export default function feedback(colorScheme: ColorScheme) {
  let layer = colorScheme.middle;
  return {
    feedbackEditor: {
      background: background(layer, "on"),
      cornerRadius: 6,
      text: text(layer, "mono", "on"),
      placeholderText: text(layer, "mono", "on", "disabled", { size: "xs" }),
      selection: colorScheme.players[0],
      border: border(layer, "on"),
      padding: {
        bottom: 4,
        left: 8,
        right: 8,
        top: 4,
      },
      margin: {
        left: 6,
      }
    },
    feedbackPopover: {
      background: background(layer),
      cornerRadius: 6,
      padding: { top: 6 },
      margin: { top: -6 },
      shadow: colorScheme.popoverShadow,
      border: border(layer),
      width: 500,
      height: 400
    }
  }
}
