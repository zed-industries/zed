import Theme from "../themes/common/theme";
import { iconColor, text } from "./components";

const headerPadding = 8;

export default function updateNotification(theme: Theme): Object {
  return {
    message: {
      ...text(theme, "sans", "primary", { size: "xs" }),
      margin: { left: headerPadding, right: headerPadding }
    },
    actionMessage: {
      ...text(theme, "sans", "secondary", { size: "xs" }),
      margin: { left: headerPadding, top: 6, bottom: 6 },
      hover: {
        color: theme.textColor["active"].value
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