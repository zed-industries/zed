import { ColorScheme } from "../themes/common/colorScheme";
import { foreground, text } from "./components";

const headerPadding = 8;

export default function simpleMessageNotification(colorScheme: ColorScheme): Object {
  let layer = colorScheme.middle;
  return {
    message: {
      ...text(layer, "sans", { size: "xs" }),
      margin: { left: headerPadding, right: headerPadding },
    },
    actionMessage: {
      ...text(layer, "sans", { size: "xs" }),
      margin: { left: headerPadding, top: 6, bottom: 6 },
      hover: {
        color: foreground(layer, "hovered"),
      },
    },
    dismissButton: {
      color: foreground(layer),
      iconWidth: 8,
      iconHeight: 8,
      buttonWidth: 8,
      buttonHeight: 8,
      hover: {
        color: foreground(layer, "hovered"),
      },
    },
  };
}
