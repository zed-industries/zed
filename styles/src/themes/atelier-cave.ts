import chroma from "chroma-js";
import { Meta } from "./common/colorScheme";
import { colorRamp, createColorScheme } from "./common/ramps";

const name = "Atelier Cave";

export const dark = createColorScheme(`${name} Dark`, false, {
  neutral: chroma
    .scale([
      "#19171c",
      "#26232a",
      "#585260",
      "#655f6d",
      "#7e7887",
      "#8b8792",
      "#e2dfe7",
      "#efecf4",
    ])
    .domain([0, 0.15, 0.45, 0.6, 0.65, 0.7, 0.85, 1]),
  red: colorRamp(chroma("#be4678")),
  orange: colorRamp(chroma("#aa573c")),
  yellow: colorRamp(chroma("#a06e3b")),
  green: colorRamp(chroma("#2a9292")),
  cyan: colorRamp(chroma("#398bc6")),
  blue: colorRamp(chroma("#576ddb")),
  violet: colorRamp(chroma("#955ae7")),
  magenta: colorRamp(chroma("#bf40bf")),
});

export const light = createColorScheme(`${name} Light`, true, {
  neutral: chroma
    .scale([
      "#19171c",
      "#26232a",
      "#585260",
      "#655f6d",
      "#7e7887",
      "#8b8792",
      "#e2dfe7",
      "#efecf4",
    ])
    .correctLightness(),
  red: colorRamp(chroma("#be4678")),
  orange: colorRamp(chroma("#aa573c")),
  yellow: colorRamp(chroma("#a06e3b")),
  green: colorRamp(chroma("#2a9292")),
  cyan: colorRamp(chroma("#398bc6")),
  blue: colorRamp(chroma("#576ddb")),
  violet: colorRamp(chroma("#955ae7")),
  magenta: colorRamp(chroma("#bf40bf")),
});


export const meta: Meta = {
  name,
  author: "atelierbram",
  license: {
    SPDX: "MIT",
    https_url: "https://raw.githubusercontent.com/atelierbram/syntax-highlighting/master/LICENSE",
    license_checksum: "6c2353bb9dd0b7b211364d98184ab482e54f40f611eda0c02974c3a1f9e6193c"
  },
  url: "https://atelierbram.github.io/syntax-highlighting/atelier-schemes/cave/"
}