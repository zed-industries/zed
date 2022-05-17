import Theme from "../themes/common/theme";
import { backgroundColor, iconColor, text } from "./components";

const avatarSize = 12;
const headerPadding = 8;

export default function contactNotification(theme: Theme): Object {
  return {
    headerAvatar: {
      height: avatarSize,
      width: avatarSize,
      cornerRadius: 6,
    },
    headerMessage: {
      ...text(theme, "sans", "primary", { size: "xs" }),
      margin: { left: headerPadding, right: headerPadding }
    },
    headerHeight: 18,
    bodyMessage: {
      ...text(theme, "sans", "secondary", { size: "xs" }),
      margin: { left: avatarSize + headerPadding, top: 6, bottom: 6 },
    },
    button: {
      ...text(theme, "sans", "primary", { size: "xs" }),
      background: backgroundColor(theme, "on300"),
      padding: 4,
      cornerRadius: 6,
      margin: { left: 6 },
      hover: {
        background: backgroundColor(theme, "on300", "hovered")
      }
    },
    dismissButton: {
      color: iconColor(theme, "secondary"),
      iconWidth: 8,
      iconHeight: 8,
      buttonWidth: 8,
      buttonHeight: 8,
      hover: {
        color: iconColor(theme, "primary")
      }
    }
  }
}