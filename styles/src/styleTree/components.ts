import { fontFamilies, fontSizes, FontWeight } from "../common";
import { Layer, Styles, StyleSets } from "../themes/common/colorScheme";

export function background(layer: Layer, styleSet?: StyleSets, style?: Styles): string {
  return layer[styleSet ?? "base"][style ?? "default"].background;
}
export function borderColor(layer: Layer, styleSet?: StyleSets, style?: Styles): string {
  return layer[styleSet ?? "base"][style ?? "default"].border;
}
export function foreground(layer: Layer, styleSet?: StyleSets, style?: Styles): string {
  return layer[styleSet ?? "base"][style ?? "default"].foreground;
}


interface Text {
  family: keyof typeof fontFamilies,
  color: string,
  size: number,
  weight?: FontWeight,
  underline?: boolean,
}

interface TextProperties {
  size?: keyof typeof fontSizes;
  weight?: FontWeight;
  underline?: boolean;
}

export function text(
  layer: Layer,
  fontFamily: keyof typeof fontFamilies,
  styleSet: StyleSets,
  style: Styles,
  properties?: TextProperties
): Text;
export function text(
  layer: Layer,
  fontFamily: keyof typeof fontFamilies,
  styleSet: StyleSets,
  properties?: TextProperties): Text;
export function text(
  layer: Layer,
  fontFamily: keyof typeof fontFamilies,
  properties?: TextProperties): Text;
export function text(
  layer: Layer,
  fontFamily: keyof typeof fontFamilies,
  styleSetOrProperties?: StyleSets | TextProperties,
  styleOrProperties?: Styles | TextProperties,
  properties?: TextProperties
) {
  let styleSet: StyleSets = "base";
  let style: Styles = "default";

  if (typeof styleSetOrProperties === "string") {
    styleSet = styleSetOrProperties
  } else if (styleSetOrProperties !== undefined) {
    properties = styleSetOrProperties;
  }

  if (typeof styleOrProperties === "string") {
    style = styleOrProperties;
  } else if (styleOrProperties !== undefined) {
    properties = styleOrProperties;
  }

  let size = fontSizes[properties?.size || "sm"];
  return {
    family: fontFamilies[fontFamily],
    color: layer[styleSet][style].foreground,
    ...properties,
    size,
  };
}


export interface Border {
  color: string,
  width: number,
  top?: boolean;
  bottom?: boolean;
  left?: boolean;
  right?: boolean;
  overlay?: boolean;
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
  layer: Layer,
  styleSet: StyleSets,
  style: Styles,
  options?: BorderOptions
): Border;
export function border(
  layer: Layer,
  styleSet: StyleSets,
  options?: BorderOptions
): Border;
export function border(
  layer: Layer,
  options?: BorderOptions
): Border;
export function border(
  layer: Layer,
  styleSetOrOptions?: StyleSets | BorderOptions,
  styleOrOptions?: Styles | BorderOptions,
  options?: BorderOptions
): Border {
  let styleSet: StyleSets = "base";
  let style: Styles = "default";

  if (typeof styleSetOrOptions === "string") {
    styleSet = styleSetOrOptions
  } else if (styleSetOrOptions !== undefined) {
    options = styleSetOrOptions;
  }

  if (typeof styleOrOptions === "string") {
    style = styleOrOptions;
  } else if (styleOrOptions !== undefined) {
    options = styleOrOptions;
  }

  return {
    color: layer[styleSet][style].border,
    width: 1,
    ...options,
  };
}