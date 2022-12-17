import { background, border, foreground, text } from "../components";
import { Layer } from "../../themes/common/colorScheme";

export function label_button(label: String, tooltipText: String, layer: Layer) {
  return {
    label,
    tooltipText,
    container: general_button(layer)
  }
}

export function icon_button(icon: String, tooltipText: String, layer: Layer) {
  return {
    icon: general_button_icon(icon, layer),
    tooltipText,
    container: general_button(layer)
  }
}

export function icon_label_button(icon: String, label: String, tooltipText: String, layer: Layer) {
  return {
    icon: general_button_icon(icon, layer),
    label,
    tooltipText,
    container: general_button(layer)
  }
}

function general_button(layer: Layer) {

  // NOTE FOR NATE: These values are all copied from 'search.optionButton', and so are wrong for a lot of places!
  return {
    ...text(layer, "mono", "on"),
    background: background(layer, "on"),
    cornerRadius: 6,
    border: border(layer, "on"),
    margin: {
      right: 4,
    },
    padding: {
      bottom: 2,
      left: 10,
      right: 10,
      top: 2,
    },
    active: {
      ...text(layer, "mono", "on", "inverted"),
      background: background(layer, "on", "inverted"),
      border: border(layer, "on", "inverted"),
    },
    clicked: {
      ...text(layer, "mono", "on", "pressed"),
      background: background(layer, "on", "pressed"),
      border: border(layer, "on", "pressed"),
    },
    hover: {
      ...text(layer, "mono", "on", "hovered"),
      background: background(layer, "on", "hovered"),
      border: border(layer, "on", "hovered"),
    },
  }
}

function general_button_icon(location: String, layer: Layer) {
  return {
    color: foreground(layer, "variant"),
    size: 16,
    location
  }
}