import chroma from "chroma-js";
import { colorRamp, createColorScheme } from "../common/ramps";

// Ashes scheme for the Base16 Builder (https://github.com/chriskempson/base16-builder)
const name = "Embers";
const author = "Jannik Siebert (https://github.com/janniks)";
const url = ""

// `name-[light|dark]`, isLight, color ramps
export const dark = createColorScheme(`${name}`, false, {
  neutral: chroma
    .scale([
      "#16130F",
      "#2C2620",
      "#433B32",
      "#5A5047",
      "#8A8075",
      "#A39A90",
      "#BEB6AE",
      "#DBD6D1",
    ]),
  red: colorRamp(chroma("#826D57")),
  orange: colorRamp(chroma("#828257")),
  yellow: colorRamp(chroma("#6D8257")),
  green: colorRamp(chroma("#57826D")),
  cyan: colorRamp(chroma("#576D82")),
  blue: colorRamp(chroma("#6D5782")),
  violet: colorRamp(chroma("#82576D")),
  magenta: colorRamp(chroma("#825757")),
});