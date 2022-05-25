import Theme from "../themes/common/theme";
import { backgroundColor, shadow, text } from "./components";

export default function contextMenu(theme: Theme) {
  return {
    background: backgroundColor(theme, 300, "base"),
    cornerRadius: 6,
    padding: {
      bottom: 2,
      left: 6,
      right: 6,
      top: 2,
    },
    shadow: shadow(theme),
    item: {
      label: text(theme, "sans", "secondary", { size: "sm" }),
      keystroke: {
        margin: { left: 60 },
        ...text(theme, "sans", "muted", { size: "sm", weight: "bold" })
      },
    },
    separator: {
      background: "#00ff00"
    },
  }
}