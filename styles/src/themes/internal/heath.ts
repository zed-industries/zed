import chroma from "chroma-js";
import { colorRamp, createColorScheme } from "../common/ramps";

const name = "Atelier Heath";
const author = "Bram de Haan (http://atelierbramdehaan.nl)";
const url = ""

// `name-[light|dark]`, isLight, color ramps
export const dark = createColorScheme(`${name}`, false, {
  neutral: chroma
    .scale([
      "#1b181b",
      "#292329",
      "#695d69",
      "#776977",
      "#9e8f9e",
      "#ab9bab",
      "#d8cad8",
      "#f7f3f7",
    ]),
  red: colorRamp(chroma("#ca402b")),
  orange: colorRamp(chroma("#a65926")),
  yellow: colorRamp(chroma("#bb8a35")),
  green: colorRamp(chroma("#918b3b")),
  cyan: colorRamp(chroma("#159393")),
  blue: colorRamp(chroma("#516aec")),
  violet: colorRamp(chroma("#7b59c0")),
  magenta: colorRamp(chroma("#cc33cc")),
});