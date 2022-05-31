import Theme from "../themes/common/theme";
import { backgroundColor, border, popoverShadow } from "./components";

export default function HoverPopover(theme: Theme) {
  return {
    background: backgroundColor(theme, "on500"),
    cornerRadius: 8,
    padding: {
      left: 8,
      right: 8,
      top: 4,
      bottom: 4
    },
    shadow: popoverShadow(theme),
    border: border(theme, "primary"),
    margin: {
      left: -8,
    },
  }
}