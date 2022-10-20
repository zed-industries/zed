import chroma from "chroma-js";
import { colorRamp, createColorScheme } from "../common/ramps";

const name = "Atelier Seaside";
const author = "Bram de Haan (http://atelierbramdehaan.nl)";
const url = "";

const ramps = {
  neutral: chroma.scale([
    "#131513",
    "#242924",
    "#5e6e5e",
    "#687d68",
    "#809980",
    "#8ca68c",
    "#cfe8cf",
    "#f4fbf4",
  ]),
  red: colorRamp(chroma("#e6193c")),
  orange: colorRamp(chroma("#87711d")),
  yellow: colorRamp(chroma("#98981b")),
  green: colorRamp(chroma("#29a329")),
  cyan: colorRamp(chroma("#1999b3")),
  blue: colorRamp(chroma("#3d62f5")),
  violet: colorRamp(chroma("#ad2bee")),
  magenta: colorRamp(chroma("#e619c3")),
};

export const dark = createColorScheme(`${name} Dark`, false, ramps);
export const light = createColorScheme(`${name} Light`, true, ramps);
