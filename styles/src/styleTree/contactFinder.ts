import picker from "./picker";
import { ColorScheme } from "../themes/common/colorScheme";
import { background, foreground } from "./components";

export default function contactFinder(colorScheme: ColorScheme) {
  let layer = colorScheme.middle.bottom;
  const contactButton = {
    background: background(layer),
    color: foreground(layer),
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
        background: background(layer, "base", "hovered"),
      },
    },
    disabledContactButton: {
      ...contactButton,
      background: background(layer, "base", "disabled"),
      color: foreground(layer, "base", "disabled"),
    },
  };
}
