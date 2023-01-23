import chroma from "chroma-js";
import { colorRamp, createColorScheme } from "../common/ramps";

const name = "Atelier Dune";
const author = "atelierbram";
const url = "https://atelierbram.github.io/syntax-highlighting/atelier-schemes/dune/";
const license = {
  type: "MIT",
  url: "https://github.com/atelierbram/syntax-highlighting/blob/master/LICENSE",
};

const ramps = {
  neutral: chroma.scale([
    "#20201d",
    "#292824",
    "#6e6b5e",
    "#7d7a68",
    "#999580",
    "#a6a28c",
    "#e8e4cf",
    "#fefbec",
  ]),
  red: colorRamp(chroma("#d73737")),
  orange: colorRamp(chroma("#b65611")),
  yellow: colorRamp(chroma("#ae9513")),
  green: colorRamp(chroma("#60ac39")),
  cyan: colorRamp(chroma("#1fad83")),
  blue: colorRamp(chroma("#6684e1")),
  violet: colorRamp(chroma("#b854d4")),
  magenta: colorRamp(chroma("#d43552")),
};

export const dark = createColorScheme(`${name} Dark`, false, ramps);
export const light = createColorScheme(`${name} Light`, true, ramps);
