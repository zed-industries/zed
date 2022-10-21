import chroma from "chroma-js";
import { colorRamp, createColorScheme } from "../common/ramps";

const name = "Ayu";
const author = "Konstantin Pschera <me@kons.ch>";
const url = "https://github.com/ayu-theme/ayu-colors";
const license = {
  type: "MIT",
  url: "https://github.com/ayu-theme/ayu-colors/blob/master/license"
}

export const dark = createColorScheme(`${name} Mirage`, false, {
  neutral: chroma.scale([
    "#171B24",
    "#1F2430",
    "#242936",
    "#707A8C",
    "#8A9199",
    "#CCCAC2",
    "#D9D7CE",
    "#F3F4F5",
  ]),
  red: colorRamp(chroma("#F28779")),
  orange: colorRamp(chroma("#FFAD66")),
  yellow: colorRamp(chroma("#FFD173")),
  green: colorRamp(chroma("#D5FF80")),
  cyan: colorRamp(chroma("#95E6CB")),
  blue: colorRamp(chroma("#5CCFE6")),
  violet: colorRamp(chroma("#D4BFFF")),
  magenta: colorRamp(chroma("#F29E74")),
});
