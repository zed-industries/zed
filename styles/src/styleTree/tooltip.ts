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
    ...text(theme, "sans", "secondary", { size: "xs", weight: "bold" })
  }
}