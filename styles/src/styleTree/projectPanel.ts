import Theme from "../themes/theme";
import { Color } from "../utils/color";
import { panel } from "./app";
import { backgroundColor, iconColor, text, TextColor } from "./components";

export default function projectPanel(theme: Theme) {
  function entry(theme: Theme, textColor: TextColor, background?: Color) {
    return {
      height: 24,
      background,
      iconColor: iconColor(theme, "muted"),
      iconSize: 8,
      iconSpacing: 8,
      text: text(theme, "mono", textColor, { size: "sm" }),
    };
  }

  return {
    ...panel,
    entry: entry(theme, "muted"),
    hoveredEntry: entry(
      theme,
      "primary",
      backgroundColor(theme, 300, "hovered")
    ),
    selectedEntry: entry(theme, "primary"),
    hoveredSelectedEntry: entry(
      theme,
      "active",
      backgroundColor(theme, 300, "hovered")
    ),
    padding: { left: 12, right: 12, top: 6, bottom: 6 },
  };
}
