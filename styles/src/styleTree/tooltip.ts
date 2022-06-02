import Theme from "../themes/common/theme";
import { backgroundColor, border, shadow, text } from "./components";

export default function tooltip(theme: Theme) {
  return {
    background: backgroundColor(theme, 500),
    border: border(theme, "secondary"),
    padding: { top: 4, bottom: 4, left: 8, right: 8 },
    margin: { top: 6, left: 6 },
    shadow: shadow(theme),
    cornerRadius: 6,
    text: text(theme, "sans", "secondary", { size: "xs", weight: "bold" }),
    keystroke: {
      background: backgroundColor(theme, "on500"),
      cornerRadius: 4,
      margin: { left: 6 },
      padding: { left: 3, right: 3 },
      ...text(theme, "mono", "muted", { size: "xs", weight: "bold" })
    },
    maxTextWidth: 200,
  }
}