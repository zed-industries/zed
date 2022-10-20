import chroma from "chroma-js";
import { colorRamp, createColorScheme } from "./common/ramps";

const name = "One Light";
const author = "simurai";
const url = "https://github.com/atom/atom/tree/master/packages/one-light-ui";
const license = {
  type: "MIT",
  url: "https://github.com/atom/atom/blob/master/packages/one-light-ui/LICENSE.md",
};

export const light = createColorScheme(`${name}`, true, {
  neutral: chroma.scale([
      "#090a0b",
      "#202227",
      "#383a42",
      "#696c77",
      "#a0a1a7",
      "#e5e5e6",
      "#f0f0f1",
      "#fafafa",
    ])
    .domain([0.05, 0.22, 0.25, 0.45, 0.62, 0.8, 0.9, 1]),

  red: colorRamp(chroma("#ca1243")),
  orange: colorRamp(chroma("#d75f00")),
  yellow: colorRamp(chroma("#c18401")),
  green: colorRamp(chroma("#50a14f")),
  cyan: colorRamp(chroma("#0184bc")),
  blue: colorRamp(chroma("#4078f2")),
  violet: colorRamp(chroma("#a626a4")),
  magenta: colorRamp(chroma("#986801")),
});
