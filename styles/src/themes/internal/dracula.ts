import chroma from "chroma-js";
import { colorRamp, createColorScheme } from "../common/ramps";

const name = "Dracula";
const author = "Base16 port by Mike Barkmin (http://github.com/mikebarkmin)";
const url = "http://github.com/dracula"

// `name-[light|dark]`, isLight, color ramps
export const dark = createColorScheme(`${name}`, false, {
  neutral: chroma
    .scale([
      "#282936",
      "#3a3c4e",
      "#4d4f68",
      "#626483",
      "#62d6e8",
      "#e9e9f4",
      "#f1f2f8",
      "#f7f7fb",
    ]),
  red: colorRamp(chroma("#ea51b2")),
  orange: colorRamp(chroma("#b45bcf")),
  yellow: colorRamp(chroma("#ebff87")),
  green: colorRamp(chroma("#00f769")),
  cyan: colorRamp(chroma("#a1efe4")),
  blue: colorRamp(chroma("#62d6e8")),
  violet: colorRamp(chroma("#b45bcf")),
  magenta: colorRamp(chroma("#00f769")),
});