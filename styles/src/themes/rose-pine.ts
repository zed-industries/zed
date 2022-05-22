import chroma from "chroma-js";
import { colorRamp, createTheme } from "./common/base16";

const name = "rosé-pine";

const ramps = {
  neutral: chroma.scale([
    "#191724",
    "#1f1d2e",
    "#26233a",
    "#555169",
    "#6e6a86",
    "#e0def4",
    "#f0f0f3",
    "#c5c3ce",
  ]),
  red: colorRamp(chroma("#e2e1e7")),
  orange: colorRamp(chroma("#eb6f92")),
  yellow: colorRamp(chroma("#f6c177")),
  green: colorRamp(chroma("#ebbcba")),
  cyan: colorRamp(chroma("#31748f")),
  blue: colorRamp(chroma("#0CA793")),
  violet: colorRamp(chroma("#8A3FA6")),
  magenta: colorRamp(chroma("#C74DED")),
};

export const dark = createTheme(`${name}`, false, ramps);
