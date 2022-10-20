import chroma from "chroma-js";
import { colorRamp, createColorScheme } from "./common/ramps";

const name = "One";
const author = "";
const url = "";
const license = {
  type: "",
  url: "",
};

export const dark = createColorScheme(`${name} Dark`, false, {
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

export const light = createColorScheme(`${name} Light`, true, {
  neutral: chroma
    .scale([
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
