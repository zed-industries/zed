import { ColorScheme } from "../themes/common/colorScheme";
import { panel } from "./app";
import { background, foreground, text } from "./components";

export default function projectPanel(colorScheme: ColorScheme) {
  let layer = colorScheme.lowest.middle;
  return {
    ...panel,
    padding: { left: 12, right: 12, top: 6, bottom: 6 },
    indentWidth: 8,
    entry: {
      height: 24,
      iconColor: foreground(layer, "on"),
      iconSize: 8,
      iconSpacing: 8,
      text: text(layer, "mono", "on", { size: "sm" }),
      hover: {
        background: background(layer, "on", "hovered"),
      },
      active: {
        background: background(layer, "base", "active"),
        text: text(layer, "mono", "base", "active", { size: "sm" }),
      },
      activeHover: {
        background: background(layer, "base", "hovered"),
        text: text(layer, "mono", "base", "active", { size: "sm" }),
      },
    },
    cutEntryFade: 0.4,
    ignoredEntryFade: 0.6,
    filenameEditor: {
      background: background(layer, "on"),
      text: text(layer, "mono", "on", "active", { size: "sm" }),
      selection: colorScheme.players[0],
    },
  };
}
