import Theme from "../themes/theme";
import picker from "./picker";
import { backgroundColor, border, player, text } from "./components";

export default function contactFinder(theme: Theme) {
  return {
    ...picker(theme),
    maxWidth: 540.,
    maxHeight: 420.,
    queryEditor: {
      background: backgroundColor(theme, 500),
      cornerRadius: 6,
      text: text(theme, "mono", "primary"),
      placeholderText: text(theme, "mono", "placeholder", { size: "sm" }),
      selection: player(theme, 1).selection,
      border: border(theme, "secondary"),
      padding: {
        bottom: 4,
        left: 8,
        right: 8,
        top: 4,
      },
    },
    rowHeight: 28,
    contactAvatar: {
      cornerRadius: 10,
      width: 18,
    },
    contactUsername: {
      ...text(theme, "mono", "primary", { size: "sm" }),
      padding: {
        left: 8,
      },
    },
    contactButton: {
      ...text(theme, "mono", "primary", { size: "sm" }),
      background: backgroundColor(theme, 100),
      cornerRadius: 12,
      padding: { left: 7, right: 7 }
    },
  }
}
