import chroma from "chroma-js";
import { colorRamp, createColorScheme } from "../common/ramps";

const name = "Tokyo Night Storm";
const author = "ghifarit53";
const url = "https://github.com/ghifarit53/tokyonight-vim";

// `name-[light|dark]`, isLight, color ramps
export const dark = createColorScheme(`${name}`, false, {
  neutral: chroma.scale([
    "#24283B",
    "#16161E",
    "#343A52",
    "#444B6A",
    "#787C99",
    "#A9B1D6",
    "#CBCCD1",
    "#D5D6DB",
  ]),
  red: colorRamp(chroma("#C0CAF5")),
  orange: colorRamp(chroma("#A9B1D6")),
  yellow: colorRamp(chroma("#0DB9D7")),
  green: colorRamp(chroma("#9ECE6A")),
  cyan: colorRamp(chroma("#B4F9F8")),
  blue: colorRamp(chroma("#2AC3DE")),
  violet: colorRamp(chroma("#BB9AF7")),
  magenta: colorRamp(chroma("#F7768E")),
});
