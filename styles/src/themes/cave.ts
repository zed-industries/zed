import chroma from "chroma-js";
import { colorRamp, createColorScheme } from "./common/ramps";

const name = "cave";

const ramps = {
  neutral: chroma.scale([
    "#19171c",
    "#26232a",
    "#585260",
    "#655f6d",
    "#7e7887",
    "#8b8792",
    "#e2dfe7",
    "#efecf4",
  ]),
  red: colorRamp(chroma("#be4678")),
  orange: colorRamp(chroma("#aa573c")),
  yellow: colorRamp(chroma("#a06e3b")),
  green: colorRamp(chroma("#2a9292")),
  cyan: colorRamp(chroma("#398bc6")),
  blue: colorRamp(chroma("#576ddb")),
  violet: colorRamp(chroma("#955ae7")),
  magenta: colorRamp(chroma("#bf40bf")),
};

export const dark = createColorScheme(`${name}-dark`, false, ramps);
export const light = createColorScheme(`${name}-light`, true, ramps);
