import chroma from "chroma-js";
import { colorRamp, createColorScheme } from "../common/ramps";

const name = "Tokyo";
const author = "folke";
const url = "https://github.com/folke/tokyonight.nvim";
const license = {
  type: "Apache License 2.0",
  url: "https://github.com/folke/tokyonight.nvim/blob/main/LICENSE",
};

// `name-[light|dark]`, isLight, color ramps
export const dark = createColorScheme(`${name} Night`, false, {
  neutral: chroma.scale([
    "#1A1B26",
    "#16161E",
    "#2F3549",
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

export const light = createColorScheme(`${name} Day`, true, {
  neutral: chroma.scale([
    "#1A1B26",
    "#1A1B26",
    "#343B59",
    "#4C505E",
    "#9699A3",
    "#DFE0E5",
    "#CBCCD1",
    "#D5D6DB",
  ]),
  red: colorRamp(chroma("#343B58")),
  orange: colorRamp(chroma("#965027")),
  yellow: colorRamp(chroma("#166775")),
  green: colorRamp(chroma("#485E30")),
  cyan: colorRamp(chroma("#3E6968")),
  blue: colorRamp(chroma("#34548A")),
  violet: colorRamp(chroma("#5A4A78")),
  magenta: colorRamp(chroma("#8C4351")),
});
