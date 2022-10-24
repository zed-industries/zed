import chroma from "chroma-js";
import { colorRamp, createColorScheme } from "./common/ramps";

const name = "One Dark";
const author = "simurai";
const url = "https://github.com/atom/atom/tree/master/packages/one-dark-ui";
const license = {
  type: "MIT",
  url: "https://github.com/atom/atom/blob/master/packages/one-dark-ui/LICENSE.md",
};

export const dark = createColorScheme(`${name}`, false, {
  neutral: chroma
    .scale([
      "#282c34",
      "#353b45",
      "#3e4451",
      "#545862",
      "#565c64",
      "#abb2bf",
      "#b6bdca",
      "#c8ccd4",
    ])
    .domain([0.05, 0.22, 0.25, 0.45, 0.62, 0.8, 0.9, 1]),

  red: colorRamp(chroma("#e06c75")),
  orange: colorRamp(chroma("#d19a66")),
  yellow: colorRamp(chroma("#e5c07b")),
  green: colorRamp(chroma("#98c379")),
  cyan: colorRamp(chroma("#56b6c2")),
  blue: colorRamp(chroma("#61afef")),
  violet: colorRamp(chroma("#c678dd")),
  magenta: colorRamp(chroma("#be5046")),
});
