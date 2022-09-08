import chroma from "chroma-js";
import { colorRamp, createTheme } from "../common/base16";

const name = "cave-experiments";

const ramps = {
  neutral: chroma.scale([
    "#555555",
    "#555555",
    "#555555",
    "#555555",
    "#555555",
    "#555555",
    "#555555",
    "#555555",
  ]),
  red: colorRamp(chroma("#555555")),
  orange: colorRamp(chroma("#555555")),
  yellow: colorRamp(chroma("#555555")),
  green: colorRamp(chroma("#555555")),
  cyan: colorRamp(chroma("#555555")),
  blue: colorRamp(chroma("#555555")),
  violet: colorRamp(chroma("#555555")),
  magenta: colorRamp(chroma("#555555")),
};

export const dark = createTheme(`${name}-dark`, false, ramps);
export const light = createTheme(`${name}-light`, true, ramps);
