
import { ColorScheme } from "../themes/common/colorScheme";
import { border, background, foreground, text } from "./components";


export default function welcome(colorScheme: ColorScheme) {
  let layer = colorScheme.highest;

  // TODO
  let checkboxBase = {
    cornerRadius: 4,
    padding: {
      left: 3,
      right: 3,
      top: 3,
      bottom: 3,
    },
    shadow: colorScheme.popoverShadow,
    border: border(layer),
    margin: {
      left: 8,
      right: 8,
      top: 5,
      bottom: 5
    },
  };

  return {
    button: {
      background: background(layer),
      border: border(layer),
      cornerRadius: 6,
      margin: {
        top: 1,
      },
      padding: {
        top: 1,
        bottom: 1,
        left: 7,
        right: 7,
      },
      ...text(layer, "sans", "variant", { size: "xs" }),
      hover: {
        ...text(layer, "sans", "hovered", { size: "xs" }),
        background: background(layer, "hovered"),
        border: border(layer, "hovered"),
      },
    },
    checkbox: {
      width: 12,
      height: 12,
      icon: "icons/check_12.svg",
      iconColor: foreground(layer, "on"),
      default: {
        ...checkboxBase,
        background: background(layer, "default"),
        border: {
          color: foreground(layer, "hovered"),
          width: 1,
        }
      },
      checked: {
        ...checkboxBase,
        background: background(layer, "hovered"),
        border: {
          color: foreground(layer, "hovered"),
          width: 1,
        }
      },
      hovered: {
        ...checkboxBase,
        background: background(layer, "hovered"),

        border: {
          color: foreground(layer, "hovered"),
          width: 1,
        }
      },
      hoveredAndChecked: {
        ...checkboxBase,
        background: background(layer, "hovered"),
        border: {
          color: foreground(layer, "hovered"),
          width: 1,
        }
      }
    }
  }
}