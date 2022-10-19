import chroma from "chroma-js";
import { colorRamp, createColorScheme } from "../common/ramps";

const name = "Gruvbox";
const author = "Dawid Kurek (dawikur@gmail.com)";
const url = "https://github.com/morhetz/gruvbox"

// `name-[light|dark]`, isLight, color ramps
export const dark = createColorScheme(`${name}-dark-medium`, false, {
  neutral: chroma
    .scale([
      "#282828",
      "#3c3836",
      "#504945",
      "#665c54",
      "#bdae93",
      "#d5c4a1",
      "#ebdbb2",
      "#fbf1c7",
    ]),
  red: colorRamp(chroma("#fb4934")),
  orange: colorRamp(chroma("#fe8019")),
  yellow: colorRamp(chroma("#fabd2f")),
  green: colorRamp(chroma("#b8bb26")),
  cyan: colorRamp(chroma("#8ec07c")),
  blue: colorRamp(chroma("#83a598")),
  violet: colorRamp(chroma("#d3869b")),
  magenta: colorRamp(chroma("#d65d0e")),
});

export const light = createColorScheme(`${name}-light-medium`, true, {
  neutral: chroma
    .scale([
      "#282828",
      "#3c3836",
      "#504945",
      "#665c54",
      "#bdae93",
      "#d5c4a1",
      "#ebdbb2",
      "#fbf1c7",
    ]),
  red: colorRamp(chroma("#9d0006")),
  orange: colorRamp(chroma("#af3a03")),
  yellow: colorRamp(chroma("#b57614")),
  green: colorRamp(chroma("#79740e")),
  cyan: colorRamp(chroma("#427b58")),
  blue: colorRamp(chroma("#076678")),
  violet: colorRamp(chroma("#8f3f71")),
  magenta: colorRamp(chroma("#d65d0e")),
});