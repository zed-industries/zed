import Theme from "../themes/theme";
import picker from "./picker";
import { backgroundColor, text } from "./components";

export default function contactFinder(theme: Theme) {
  return {
    ...picker(theme),
    rowHeight: 28,
    contactAvatar: {
      cornerRadius: 10,
      width: 18,
    },
    contactUsername: {
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
