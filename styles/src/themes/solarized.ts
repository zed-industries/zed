import chroma from "chroma-js";
import { colorRamp, createTheme } from "./base16";

const name = "solarized";

const ramps = {
  neutral: chroma.scale(["#002b36", "#073642", "#586e75", "#657b83", "#839496", "#93a1a1", "#eee8d5", "#fdf6e3"]),
  red: colorRamp(chroma("#dc322f")),
  orange: colorRamp(chroma("#cb4b16")),
  yellow: colorRamp(chroma("#b58900")),
  green: colorRamp(chroma("#859900")),
  cyan: colorRamp(chroma("#2aa198")),
  blue: colorRamp(chroma("#268bd2")),
  violet: colorRamp(chroma("#6c71c4")),
  magenta: colorRamp(chroma("#d33682")),
}

export const dark = createTheme(`${name}-dark`, false, ramps);
export const light = createTheme(`${name}-light`, true, ramps);