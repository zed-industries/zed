import Theme from "../themes/common/theme";
import { backgroundColor, border, borderColor, popoverShadow, text } from "./components";

export default function contactsPopover(theme: Theme) {
  const sidePadding = 12;
  return {
    background: backgroundColor(theme, 300, "base"),
    cornerRadius: 6,
    padding: { top: 6 },
    margin: { top: -6 },
    shadow: popoverShadow(theme),
    border: border(theme, "primary"),
    width: 300,
    height: 400,
    inviteRowHeight: 28,
    inviteRow: {
      padding: {
        left: sidePadding,
        right: sidePadding,
      },
      border: { top: true, width: 1, color: borderColor(theme, "primary") },
      text: text(theme, "sans", "secondary", { size: "sm" }),
      hover: {
        text: text(theme, "sans", "active", { size: "sm" }),
      },
    },
  }
}
