import Theme from "../themes/common/theme";
import { panel } from "./app";
import { backgroundColor, iconColor, player, shadow, text } from "./components";

export default function projectPanel(theme: Theme) {
  return {
    ...panel,
    padding: { left: 12, right: 12, top: 6, bottom: 6 },
    indentWidth: 20,
    entry: {
      height: 24,
      iconColor: iconColor(theme, "muted"),
      iconSize: 8,
      iconSpacing: 8,
      text: text(theme, "mono", "muted", { size: "sm" }),
      hover: {
        background: backgroundColor(theme, 300, "hovered"),
        text: text(theme, "mono", "primary", { size: "sm" }),
      },
      active: {
        background: backgroundColor(theme, 300, "active"),
        text: text(theme, "mono", "primary", { size: "sm" }),
      },
      activeHover: {
        background: backgroundColor(theme, 300, "hovered"),
        text: text(theme, "mono", "active", { size: "sm" }),
      }
    },
    ignoredEntryFade: 0.6,
    filenameEditor: {
      background: backgroundColor(theme, 500, "active"),
      text: text(theme, "mono", "primary", { size: "sm" }),
      selection: player(theme, 1).selection,
    },
  };
}
