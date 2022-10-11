import chroma from "chroma-js";
import { colorRamp, createColorScheme } from "./common/ramps";

const name = "cave";

export const dark = createColorScheme(`${name}-dark`, false, {
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
    .domain([0, 0.3, 0.45, 0.6, 0.65, 0.7, 0.85, 1]),
  red: colorRamp(chroma("#be4678")),
  orange: colorRamp(chroma("#aa573c")),
  yellow: colorRamp(chroma("#a06e3b")),
  green: colorRamp(chroma("#2a9292")),
  cyan: colorRamp(chroma("#398bc6")),
  blue: colorRamp(chroma("#576ddb")),
  violet: colorRamp(chroma("#955ae7")),
  magenta: colorRamp(chroma("#bf40bf")),
});

export const light = createColorScheme(`${name}-light`, true, {
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
    ]).correctLightness(),
  red: colorRamp(chroma("#be4678")),
  orange: colorRamp(chroma("#aa573c")),
  yellow: colorRamp(chroma("#a06e3b")),
  green: colorRamp(chroma("#2a9292")),
  cyan: colorRamp(chroma("#398bc6")),
  blue: colorRamp(chroma("#576ddb")),
  violet: colorRamp(chroma("#955ae7")),
  magenta: colorRamp(chroma("#bf40bf")),
});