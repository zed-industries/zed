import chroma from "chroma-js";
import { colorRamp, createColorScheme } from "../common/ramps";

const name = "Twilight";
const author = "David Hart (https://github.com/hartbit)";
const url = "";

// `name-[light|dark]`, isLight, color ramps
export const dark = createColorScheme(`${name}`, false, {
  neutral: chroma.scale([
    "#1e1e1e",
    "#323537",
    "#464b50",
    "#5f5a60",
    "#838184",
    "#a7a7a7",
    "#c3c3c3",
    "#ffffff",
  ]),
  red: colorRamp(chroma("#cf6a4c")),
  orange: colorRamp(chroma("#cda869")),
  yellow: colorRamp(chroma("#f9ee98")),
  green: colorRamp(chroma("#8f9d6a")),
  cyan: colorRamp(chroma("#afc4db")),
  blue: colorRamp(chroma("#7587a6")),
  violet: colorRamp(chroma("#9b859d")),
  magenta: colorRamp(chroma("#9b703f")),
});
