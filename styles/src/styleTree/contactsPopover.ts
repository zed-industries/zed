import { ColorScheme } from "../themes/common/colorScheme";
import { background, border, text } from "./components";

export default function contactsPopover(colorScheme: ColorScheme) {
  let layer = colorScheme.middle;
  const sidePadding = 12;
  return {
    background: background(layer),
    cornerRadius: 6,
    padding: { top: 6 },
    margin: { top: -6 },
    shadow: colorScheme.popoverShadow,
    border: border(layer),
    width: 300,
    height: 400,
    inviteRowHeight: 28,
    inviteRow: {
      padding: {
        left: sidePadding,
        right: sidePadding,
      },
      border: border(layer, { top: true }),
      text: text(layer, "sans", "variant", { size: "sm" }),
      hover: {
        text: text(layer, "sans", "hovered", { size: "sm" }),
      },
    },
  }
}
