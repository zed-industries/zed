import Theme from "../themes/common/theme";
import { backgroundColor, border, shadow, text } from "./components";

export default function tooltip(theme: Theme) {
  return {
    background: backgroundColor(theme, 500),
    border: border(theme, "primary"),
    padding: 6,
    margin: { top: 8, left: 8 },
    shadow: shadow(theme),
    cornerRadius: 6,
    ...text(theme, "sans", "primary", { size: "xs" })
  }
}