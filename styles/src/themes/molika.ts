import chroma from "chroma-js";
import { colorRamp, createTheme } from "./common/base16";

const name = "molika";

const ramps = {
  neutral: chroma.scale([
    "#161616",
    "#2f2f2f",
    "#4a4a4a",
    "#676767",
    "#868686",
    "#a6a6a6",
    "#c7c7c7",
    "#feffff",
  ]),
  red: colorRamp(chroma("#bd9eff")),
  orange: colorRamp(chroma("#bde271")),
  yellow: colorRamp(chroma("#bd9eff")),
  green: colorRamp(chroma("#bde271")),
  cyan: colorRamp(chroma("#5ed6fe")),
  blue: colorRamp(chroma("#fa7fac")),
  violet: colorRamp(chroma("#fff27f")),
  magenta: colorRamp(chroma("#00bdff")),
};

export const dark = createTheme(`${name}`, false, ramps);
// export const light = createTheme(`${name}-light`, true, ramps);
