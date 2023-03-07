
import { ColorScheme } from "../themes/common/colorScheme";
import { border, background, foreground, text, TextProperties } from "./components";


export default function welcome(colorScheme: ColorScheme) {
  let layer = colorScheme.highest;

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
      right: 8,
      top: 5,
      bottom: 5
    },
  };
  
  let interactive_text_size: TextProperties = { size: "md" }

  return {
    pageWidth: 450,
    logoSubheading: {
      ...text(layer, "sans", { size: "lg" }),
      margin: {
        top: 10,
        bottom: 7,
      },
    },
    button: {
      background: background(layer),
      border: border(layer, "active"),
      cornerRadius: 4,
      margin: {
        top: 8,
        bottom: 7
      },
      padding: {
        top: 1,
        bottom: 1,
        left: 7,
        right: 7,
      },
      ...text(layer, "sans", "hovered", interactive_text_size),
      hover: {
        ...text(layer, "sans", "hovered", interactive_text_size),
        background: background(layer, "hovered"),
        border: border(layer, "hovered"),
      },
    },
    checkbox: {
      label: {
          ...text(layer, "sans", interactive_text_size),
          // Also supports margin, container, border, etc.
      },
      container: {
        margin: {
          top: 5,
        },
      },
      width: 12,
      height: 12,
      checkIcon: "icons/check_12.svg",
      checkIconColor: foreground(layer, "on"),
      default: {
        ...checkboxBase,
        background: background(layer, "default"),
        border: border(layer, "active")
      },
      checked: {
        ...checkboxBase,
        background: background(layer, "hovered"),
        border: border(layer, "active")
      },
      hovered: {
        ...checkboxBase,
        background: background(layer, "hovered"),
        border: border(layer, "hovered")
      },
      hoveredAndChecked: {
        ...checkboxBase,
        background: background(layer, "hovered"),
        border: border(layer, "active")
      }
    }
  }
}