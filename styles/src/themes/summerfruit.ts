import chroma from "chroma-js";
import { colorRamp, createTheme } from "./common/base16";

const name = "summerfruit";

const ramps = {
  neutral: chroma.scale([
    "#151515",
    "#202020",
    "#303030",
    "#505050",
    "#B0B0B0",
    "#D0D0D0",
    "#E0E0E0",
    "#FFFFFF",
  ]),
  red: colorRamp(chroma("#FF0086")),
  orange: colorRamp(chroma("#FD8900")),
  yellow: colorRamp(chroma("#ABA800")),
  green: colorRamp(chroma("#00C918")),
  cyan: colorRamp(chroma("#1FAAAA")),
  blue: colorRamp(chroma("#3777E6")),
  violet: colorRamp(chroma("#AD00A1")),
  magenta: colorRamp(chroma("#CC6633")),
};

export const dark = createTheme(`${name}-dark`, false, ramps);
export const light = createTheme(`${name}-light`, true, ramps);
