import Theme from "../themes/theme";
import { backgroundColor, iconColor, text } from "./components";

export default function incomingRequestNotification(theme: Theme): Object {
  return {
    headerAvatar: {
      height: 12,
      width: 12,
      cornerRadius: 6,
    },
    headerMessage: {
      ...text(theme, "sans", "primary", { size: "xs" }),
      margin: { left: 4 }
    },
    headerHeight: 18,
    bodyMessage: {
      ...text(theme, "sans", "secondary", { size: "xs" }),
      margin: { top: 6, bottom: 6 },
    },
    button: {
      ...text(theme, "sans", "primary", { size: "xs" }),
      background: backgroundColor(theme, "on300"),
      padding: 4,
      cornerRadius: 6,
      margin: { left: 6 },
    },
    dismissButton: {
      color: iconColor(theme, "secondary"),
      iconWidth: 8,
      iconHeight: 8,
      buttonWidth: 8,
      buttonHeight: 8,
    }
  }
}