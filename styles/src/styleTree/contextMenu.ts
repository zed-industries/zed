import { ColorScheme } from "../themes/common/colorScheme";
import {
  background,
  border,
  borderColor,
  text,
} from "./components";

export default function contextMenu(colorScheme: ColorScheme) {
  let elevation = colorScheme.middle;
  let layer = elevation.bottom;
  return {
    background: background(layer),
    cornerRadius: 6,
    padding: 6,
    shadow: elevation.shadow,
    border: border(layer),
    keystrokeMargin: 30,
    item: {
      iconSpacing: 8,
      iconWidth: 14,
      padding: { left: 4, right: 4, top: 2, bottom: 2 },
      cornerRadius: 6,
      label: text(layer, "sans", { size: "sm" }),
      keystroke: {
        ...text(layer, "sans", "variant", { size: "sm", weight: "bold" }),
        padding: { left: 3, right: 3 },
      },
      hover: {
        background: background(layer, "hovered"),
        text: text(layer, "sans", "hovered", { size: "sm" }),
      },
      active: {
        background: background(layer, "active"),
        text: text(layer, "sans", "active", { size: "sm" }),
      },
      activeHover: {
        background: background(layer, "active"),
        text: text(layer, "sans", "active", { size: "sm" }),
      },
    },
    separator: {
      background: borderColor(layer),
      margin: { top: 2, bottom: 2 },
    },
  };
}
