import picker from "./picker";
import { ColorScheme } from "../themes/common/colorScheme";
import { background, foreground } from "./components";

export default function contactFinder(colorScheme: ColorScheme) {
  let layer = colorScheme.highest.top;
  const contactButton = {
    background: background(layer, "variant"),
    color: foreground(layer, "variant"),
    iconWidth: 8,
    buttonWidth: 16,
    cornerRadius: 8,
  };

  return {
    ...picker(colorScheme),
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
        background: background(layer, "variant", "hovered"),
      },
    },
    disabledContactButton: {
      ...contactButton,
      background: background(layer, "disabled"),
      color: foreground(layer, "disabled"),
    },
  };
}
