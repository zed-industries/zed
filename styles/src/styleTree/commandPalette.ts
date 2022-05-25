import Theme from "../themes/common/theme";
import { text, backgroundColor, border } from "./components";

export default function commandPalette(theme: Theme) {
  return {
    keystrokeSpacing: 8,
    key: {
      text: text(theme, "mono", "secondary", { size: "xs" }),
      cornerRadius: 4,
      background: backgroundColor(theme, "on300"),
      border: border(theme, "secondary"),
      padding: {
        top: 2,
        bottom: 2,
        left: 8,
        right: 8,
      },
      margin: {
        left: 2
      },
      active: {
        text: text(theme, "mono", "active", { size: "xs" }),
      }
    }
  }
}
