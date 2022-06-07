import chroma from "chroma-js";
import { isIPv4 } from "net";
import Theme, { BackgroundColorSet } from "../themes/common/theme";
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
  let size = fontSizes[properties?.size || "sm"].value;
  return {
    family: fontFamilies[fontFamily].value,
    color: theme.textColor[color].value,
    ...properties,
    size,
  };
}
export function textColor(theme: Theme, color: TextColor) {
  return theme.textColor[color].value;
}

export type BorderColor = keyof Theme["borderColor"];
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
  color: BorderColor,
  options?: BorderOptions
) {
  return {
    color: borderColor(theme, color),
    width: 1,
    ...options,
  };
}
export function borderColor(theme: Theme, color: BorderColor) {
  return theme.borderColor[color].value;
}

export type IconColor = keyof Theme["iconColor"];
export function iconColor(theme: Theme, color: IconColor) {
  return theme.iconColor[color].value;
}

export type PlayerIndex = 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8;
export interface Player {
  selection: {
    cursor: Color;
    selection: Color;
  };
}
export function player(
  theme: Theme,
  playerNumber: PlayerIndex,
): Player {
  return {
    selection: {
      cursor: theme.player[playerNumber].cursorColor.value,
      selection: theme.player[playerNumber].selectionColor.value,
    },
  };
}

export type BackgroundColor = keyof Theme["backgroundColor"];
export type BackgroundState = keyof BackgroundColorSet;
export function backgroundColor(
  theme: Theme,
  name: BackgroundColor,
  state?: BackgroundState,
): Color {
  return theme.backgroundColor[name][state || "base"].value;
}

export function modalShadow(theme: Theme) {
  return {
    blur: 16,
    color: theme.shadow.value,
    offset: [0, 2],
  };
}

export function popoverShadow(theme: Theme) {
  return {
    blur: 4,
    color: theme.shadow.value,
    offset: [1, 2],
  };
}
