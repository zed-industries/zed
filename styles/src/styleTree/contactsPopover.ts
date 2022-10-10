import Theme from "../themes/common/theme";
import { backgroundColor, border, popoverShadow } from "./components";

export default function contactsPopover(theme: Theme) {
  return {
    background: backgroundColor(theme, 300, "base"),
    cornerRadius: 6,
    padding: { top: 6 },
    shadow: popoverShadow(theme),
    border: border(theme, "primary"),
    width: 250,
    height: 300,
  }
}
