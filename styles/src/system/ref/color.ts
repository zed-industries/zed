import * as chroma from "chroma-js";

// Colors should use the LCH color space.
// https://www.w3.org/TR/css-color-4/#lch-colors

const base = {
  black: chroma.lch(0, 0, 0),
  white: chroma.lch(150, 0, 0),
  gray: {
    light: chroma.lch(96, 0, 0),
    mid: chroma.lch(55, 0, 0),
    dark: chroma.lch(10, 0, 0),
  },
  red: {
    light: chroma.lch(96, 4, 31),
    mid: chroma.lch(55, 77, 31),
    dark: chroma.lch(10, 24, 31),
  },
};

export const black = base.black;
export const white = base.white;

export const gray = chroma.scale([
  base.gray.light,
  base.gray.mid,
  base.gray.dark,
]);
export const red = chroma.scale([base.red.light, base.red.mid, base.red.dark]);
