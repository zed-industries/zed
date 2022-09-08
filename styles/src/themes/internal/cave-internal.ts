import chroma from "chroma-js";
import { colorRamp, createTheme } from "../common/base16";

const name = "cave-internal";

const ramps = {
  neutral: chroma.scale([
    "#111111",
    "#222222",
    "#333333",
    "#444444",
    "#555555",
    "#888888",
    "#999999",
    "#000000",
  ]),
  red: colorRamp(chroma("#aaaaaa")),
  orange: colorRamp(chroma("#bbbbbb")),
  yellow: colorRamp(chroma("#cccccc")),
  green: colorRamp(chroma("#dddddd")),
  cyan: colorRamp(chroma("#eeeeee")),
  blue: colorRamp(chroma("#ffffff")),
  violet: colorRamp(chroma("#ababab")),
  magenta: colorRamp(chroma("#cdcdcd")),
};

export const dark = createTheme(`${name}-dark`, false, ramps);
export const light = createTheme(`${name}-light`, true, ramps);
