
import { ColorScheme } from "../themes/common/colorScheme";
import { border, background, text } from "./components";


export default function welcome(colorScheme: ColorScheme) {
  let layer = colorScheme.highest;

  // TODO
  let checkboxBase = {
    cornerRadius: 4,
    padding: {
      left: 8,
      right: 8,
      top: 4,
      bottom: 4,
    },
    shadow: colorScheme.popoverShadow,
    border: border(layer),
    margin: {
      left: -8,
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
      width: 9,
      height: 9,
      default: {
        ...checkboxBase,
        background: colorScheme.ramps.blue(0.5).hex(),
      },
      checked: {
        ...checkboxBase,
        background: colorScheme.ramps.red(0.5).hex(),
      },
      hovered: {
        ...checkboxBase,
        background: colorScheme.ramps.blue(0.5).hex(),

        border: {
          color: colorScheme.ramps.green(0.5).hex(),
          width: 1,
        }
      },
      hoveredAndChecked: {
        ...checkboxBase,
        background: colorScheme.ramps.red(0.5).hex(),
        border: {
          color: colorScheme.ramps.green(0.5).hex(),
          width: 1,
        }
      }
    }
  }
}