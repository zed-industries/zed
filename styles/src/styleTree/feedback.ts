
import { ColorScheme } from "../themes/common/colorScheme";
import { background, border, text } from "./components";

export default function feedback(colorScheme: ColorScheme) {
  let layer = colorScheme.highest;

  return {
    submit_button: {
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
    button_margin: 8,
    info_text: text(layer, "sans", "default", { size: "xs" }),
  };
}
