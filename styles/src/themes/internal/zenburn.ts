import chroma from "chroma-js";
import { colorRamp, createColorScheme } from "../common/ramps";

const name = "Zenburn";
const author = "elnawe";
const url = "https://github.com/elnawe/base16-zenburn-scheme";
const license = {
  type: "?",
  url: "https://github.com/elnawe/base16-zenburn-scheme/blob/master/zenburn.yaml",
};

// `name-[light|dark]`, isLight, color ramps
export const dark = createColorScheme(`${name}`, false, {
  neutral: chroma.scale([
    "#383838",
    "#404040",
    "#606060",
    "#6f6f6f",
    "#808080",
    "#dcdccc",
    "#c0c0c0",
    "#ffffff",
  ]),
  red: colorRamp(chroma("#dca3a3")),
  orange: colorRamp(chroma("#dfaf8f")),
  yellow: colorRamp(chroma("#e0cf9f")),
  green: colorRamp(chroma("#5f7f5f")),
  cyan: colorRamp(chroma("#93e0e3")),
  blue: colorRamp(chroma("#7cb8bb")),
  violet: colorRamp(chroma("#dc8cc3")),
  magenta: colorRamp(chroma("#000000")),
});
