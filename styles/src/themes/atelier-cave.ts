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
    https_url: "https://atelierbram.mit-license.org/license.txt",
    license_checksum: "f95ce526ef4e7eecf7a832bba0e3451cc1000f9ce63eb01ed6f64f8109f5d0a5"
  },
  url: "https://atelierbram.github.io/syntax-highlighting/atelier-schemes/cave/"
}