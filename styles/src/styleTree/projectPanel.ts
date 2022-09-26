import { ColorScheme } from "../themes/common/colorScheme";
import { background, foreground, text } from "./components";

export default function projectPanel(colorScheme: ColorScheme) {
  let layer = colorScheme.lowest.middle;
  return {
    background: background(layer),
    padding: { left: 12, right: 12, top: 6, bottom: 6 },
    indentWidth: 8,
    entry: {
      height: 24,
      iconColor: foreground(layer, "variant"),
      iconSize: 8,
      iconSpacing: 8,
      text: text(layer, "mono", "on", { size: "sm" }),
      hover: {
        background: background(layer, "on", "hovered"),
      },
      active: {
        background: background(layer, "info", "active"),
        text: text(layer, "mono", "active", { size: "sm" }),
      },
      activeHover: {
        background: background(layer, "info", "hovered"),
        text: text(layer, "mono", "active", { size: "sm" }),
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
