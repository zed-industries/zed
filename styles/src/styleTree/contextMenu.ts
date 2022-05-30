import Theme from "../themes/common/theme";
import { backgroundColor, borderColor, shadow, text } from "./components";

export default function contextMenu(theme: Theme) {
  return {
    background: backgroundColor(theme, 300, "base"),
    cornerRadius: 6,
    padding: 6,
    shadow: shadow(theme),
    item: {
      padding: { left: 4, right: 4, top: 2, bottom: 2 },
      cornerRadius: 6,
      label: text(theme, "sans", "secondary", { size: "sm" }),
      keystroke: {
        margin: { left: 60 },
        ...text(theme, "sans", "muted", { size: "sm", weight: "bold" })
      },
      hover: {
        background: backgroundColor(theme, 300, "hovered"),
        text: text(theme, "sans", "primary", { size: "sm" }),
      },
      active: {
        background: backgroundColor(theme, 300, "active"),
        text: text(theme, "sans", "primary", { size: "sm" }),
      },
      activeHover: {
        background: backgroundColor(theme, 300, "hovered"),
        text: text(theme, "sans", "active", { size: "sm" }),
      }
    },
    separator: {
      background: borderColor(theme, "primary"),
      margin: { top: 2, bottom: 2 }
    },
  }
}
