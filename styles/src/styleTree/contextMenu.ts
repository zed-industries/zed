import Theme from "../themes/common/theme";
import { shadow, text } from "./components";

export default function contextMenu(theme: Theme) {
  return {
    background: "#ff0000",
    // background: backgroundColor(theme, 300, "base"),
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
    },
    separator: {
      background: "#00ff00"
    }
  }
}