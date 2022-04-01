import chroma from "chroma-js";
import Theme, { BackgroundColor } from "../themes/theme";
import { fontFamilies, fontSizes, FontWeight } from "../tokens";
import { Color } from "../utils/color";

export type TextColor = keyof Theme["textColor"];

export function text(
  theme: Theme,
  fontFamily: keyof typeof fontFamilies,
  color: TextColor,
  properties?: {
    size?: keyof typeof fontSizes;
    weight?: FontWeight;
    underline?: boolean;
  }
) {
  const sizeKey = properties?.size || fontFamily === "sans" ? "sm" : "md";
  const size = fontSizes[sizeKey].value;

  return {
    family: fontFamilies[fontFamily].value,
    color: theme.textColor[color].value,
    ...properties,
    size,
  };
}

export interface BorderOptions {
  width?: number;
  top?: boolean;
  bottom?: boolean;
  left?: boolean;
  right?: boolean;
  overlay?: boolean;
}

export function border(
  theme: Theme,
  color: keyof Theme["borderColor"],
  options?: BorderOptions
) {
  return {
    color: borderColor(theme, color),
    width: 1,
    ...options,
  };
}

export function borderColor(theme: Theme, color: keyof Theme["borderColor"]) {
  return theme.borderColor[color].value;
}

export function iconColor(theme: Theme, color: keyof Theme["iconColor"]) {
  return theme.iconColor[color].value;
}

export interface Player {
  selection: {
    cursor: Color;
    selection: Color;
  };
}

export function player(
  theme: Theme,
  playerNumber: 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8
): Player {
  return {
    selection: {
      cursor: theme.player[playerNumber].cursorColor.value,
      selection: theme.player[playerNumber].selectionColor.value,
    },
  };
}

export function backgroundColor(
  theme: Theme,
  name: keyof Theme["backgroundColor"],
  state?: keyof BackgroundColor
): Color {
  return theme.backgroundColor[name][state || "base"].value;
}

export function shadow(theme: Theme) {
  return {
    blur: 16,
    color: chroma("black").alpha(theme.shadowAlpha.value).hex(),
    offset: [0, 2],
  };
}
