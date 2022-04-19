import Theme from "../themes/theme";
import { text, backgroundColor, border } from "./components";

export default function commandPalette(theme: Theme) {
  return {
    keystrokeSpacing: 8,
    key: {
      text: text(theme, "mono", "primary", { size: "xs" }),
      cornerRadius: 3,
      background: backgroundColor(theme, "info", "base"),
      border: border(theme, "info"),
      padding: {
        left: 3,
        right: 3,
      },
      margin: {
        left: 3
      },
    }
  }
}
