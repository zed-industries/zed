import Theme from "../themes/common/theme";
import picker from "./picker";
import { backgroundColor, iconColor } from "./components";

export default function contactFinder(theme: Theme) {
  const contactButton = {
    background: backgroundColor(theme, 100),
    color: iconColor(theme, "primary"),
    iconWidth: 8,
    buttonWidth: 16,
    cornerRadius: 8,
  };

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
      ...contactButton,
      hover: {
        background: backgroundColor(theme, 100, "hovered")
      }
    },
    disabledContactButton: {
      ...contactButton,
      background: backgroundColor(theme, 100),
      color: iconColor(theme, "muted"),
    },
  }
}
