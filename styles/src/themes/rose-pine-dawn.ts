import chroma from "chroma-js";
import { colorRamp, createTheme } from "./common/base16";

const name = "rosé-pine-dawn";

const ramps = {
  neutral: chroma.scale([
    "#26233a",
    "#555169",
    "#575279",
    "#6e6a86",
    "#9893a5",
    "#f2e9de",
    "#fffaf3",
    "#faf4ed",
  ]),
  red: colorRamp(chroma("#1f1d2e")),
  orange: colorRamp(chroma("#b4637a")),
  yellow: colorRamp(chroma("#ea9d34")),
  green: colorRamp(chroma("#d7827e")),
  cyan: colorRamp(chroma("#286983")),
  blue: colorRamp(chroma("#56949f")),
  violet: colorRamp(chroma("#907aa9")),
  magenta: colorRamp(chroma("#c5c3ce")),
};

export const light = createTheme(`${name}`, true, ramps);
