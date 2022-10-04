import { ColorScheme } from "../themes/common/colorScheme";
import {
  background,
  border,
  text,
} from "./components";

export default function picker(colorScheme: ColorScheme) {
  let elevation = colorScheme.highest;
  let layer = elevation.top;
  return {
    background: background(layer),
    cornerRadius: 12,
    item: {
      padding: {
        bottom: 4,
        left: 12,
        right: 12,
        top: 4,
      },
      margin: {
        left: 4,
        right: 4
      },
      cornerRadius: 6,
      text: text(layer, "sans", "variant"),
      highlightText: text(layer, "sans", "info", { weight: "bold" }),
      active: {
        background: background(layer, "active"),
        text: text(layer, "sans", "active"),
        highlightText: text(layer, "sans", "info", "active", { weight: "bold" }),
      },
      hover: {
        background: background(layer, "hovered"),
      },
    },
    border: border(layer),
    empty: {
      text: text(layer, "sans", "variant"),
      padding: {
        bottom: 8,
        left: 16,
        right: 16,
        top: 8,
      },
    },
    inputEditor: {
      placeholderText: text(layer, "sans", "on", "disabled"),
      selection: colorScheme.players[0],
      text: text(layer, "mono", "on"),
      border: border(layer, { bottom: true }),
      padding: {
        bottom: 8,
        left: 16,
        right: 16,
        top: 8,
      },
      margin: {
        bottom: 4
      }
    },
    shadow: elevation.shadow,
  };
}
