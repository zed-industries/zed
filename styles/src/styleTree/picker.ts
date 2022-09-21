import { ColorScheme } from "../themes/common/colorScheme";
import {
  background,
  border,
  text,
} from "./components";

export default function picker(colorScheme: ColorScheme) {
  let elevation = colorScheme.highest;
  let layer = elevation.middle;
  return {
    background: background(layer),
    cornerRadius: 8,
    padding: 8,
    item: {
      padding: {
        bottom: 4,
        left: 12,
        right: 12,
        top: 4,
      },
      cornerRadius: 8,
      text: text(layer, "sans"),
      highlightText: text(layer, "sans", { weight: "bold" }),
      active: {
        background: background(layer, "base", "active"),
        text: text(layer, "sans", "base", "active"),
      },
      hover: {
        background: background(layer, "base", "hovered"),
      },
    },
    border: border(layer),
    empty: {
      text: text(layer, "sans"),
      padding: {
        bottom: 4,
        left: 12,
        right: 12,
        top: 8,
      },
    },
    inputEditor: {
      background: background(layer, "on"),
      cornerRadius: 8,
      placeholderText: text(layer, "sans", "on", "disabled"),
      selection: colorScheme.players[0],
      text: text(layer, "mono", "on"),
      border: border(layer, "on"),
      padding: {
        bottom: 7,
        left: 16,
        right: 16,
        top: 7,
      },
    },
    shadow: elevation.shadow,
  };
}
