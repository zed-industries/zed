import chroma from "chroma-js";
import { colorRamp, createColorScheme } from "../common/ramps";

const name = "Atelier Heath";
const author = "Bram de Haan (http://atelierbramdehaan.nl)";
const url = "";

// `name-[light|dark]`, isLight, color ramps
export const dark = createColorScheme(`${name} Dark`, false, {
  neutral: chroma.scale([
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

export const light = createColorScheme(`${name} Light`, true, {
  neutral: chroma.scale([
    "#161b1d",
    "#1f292e",
    "#516d7b",
    "#5a7b8c",
    "#7195a8",
    "#7ea2b4",
    "#c1e4f6",
    "#ebf8ff",
  ]),
  red: colorRamp(chroma("#d22d72")),
  orange: colorRamp(chroma("#935c25")),
  yellow: colorRamp(chroma("#8a8a0f")),
  green: colorRamp(chroma("#568c3b")),
  cyan: colorRamp(chroma("#2d8f6f")),
  blue: colorRamp(chroma("#257fad")),
  violet: colorRamp(chroma("#6b6bb8")),
  magenta: colorRamp(chroma("#b72dd2")),
});
