import chroma from "chroma-js";
import { colorRamp, createColorScheme } from "../common/ramps";

const name = "Nord";
const author = "arcticicestudio";
const url = "";

// `name-[light|dark]`, isLight, color ramps
export const dark = createColorScheme(`${name}`, false, {
  neutral: chroma.scale([
    "#2E3440",
    "#3B4252",
    "#434C5E",
    "#4C566A",
    "#D8DEE9",
    "#E5E9F0",
    "#ECEFF4",
    "#8FBCBB",
  ]),
  red: colorRamp(chroma("#88C0D0")),
  orange: colorRamp(chroma("#81A1C1")),
  yellow: colorRamp(chroma("#5E81AC")),
  green: colorRamp(chroma("#BF616A")),
  cyan: colorRamp(chroma("#D08770")),
  blue: colorRamp(chroma("#EBCB8B")),
  violet: colorRamp(chroma("#A3BE8C")),
  magenta: colorRamp(chroma("#B48EAD")),
});
