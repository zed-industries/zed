import chroma from "chroma-js";
import { colorRamp, createTheme } from "./common/base16";

const name = "molika";

const ramps = {
  neutral: chroma.scale(["#161616", "#161616", "#676767", "#676767", "#c7c7c7", "#c7c7c7", "#feffff", "#feffff"]),
  red: colorRamp(chroma("#fa7fac")),
  orange: colorRamp(chroma("#e5da72")),
  yellow: colorRamp(chroma("#fff27f")),
  green: colorRamp(chroma("#bde271")),
  cyan: colorRamp(chroma("#5ed6fe")),
  blue: colorRamp(chroma("#00bdff")),
  violet: colorRamp(chroma("#9a37ff")),
  magenta: colorRamp(chroma("#bd9eff")),
}

export const dark = createTheme(`${name}-dark`, false, ramps);
export const light = createTheme(`${name}-light`, true, ramps);