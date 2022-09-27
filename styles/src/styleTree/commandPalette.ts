import { ColorScheme } from "../themes/common/colorScheme";
import { text, border, background } from "./components";

export default function commandPalette(colorScheme: ColorScheme) {
  let layer = colorScheme.highest.top;
  return {
    keystrokeSpacing: 8,
    key: {
      text: text(layer, "mono", { size: "xs" }),
      cornerRadius: 4,
      background: background(layer, "on"),
      padding: {
        top: 2,
        bottom: 2,
        left: 8,
        right: 8,
      },
      margin: {
        left: 2,
      },
      active: {
        text: text(layer, "mono", "active", { size: "xs" }),
        background: background(layer, "on", "active"),
      },
    },
  };
}
