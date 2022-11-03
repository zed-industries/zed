import chroma from "chroma-js";
import { colorRamp, createColorScheme } from "../common/ramps";
import { ThemeConfig } from "../common/themeConfig";

const name = "Zed Pro";
const author = "Nate Butler"
const url = "https://github.com/iamnbutler"
const license = {
  type: "?",
  url: "?",
};

const red = "#DC604F"
const violet = "#5874C1"
const cyan = "#215050"

const ramps = {
  neutral: chroma
    .scale([
      "#101010",
      "#1C1C1C",
      "#212121",
      "#2D2D2D",
      "#B9B9B9",
      "#DADADA",
      "#E6E6E6",
      "#FFFFFF",
    ])
    .domain([0, 0.1, 0.2, 0.3, 0.7, 0.8, 0.9, 1]),
  red: colorRamp(chroma("#DC604F")),
  orange: colorRamp(chroma("#DE782F")),
  yellow: colorRamp(chroma("#E0B750")),
  green: colorRamp(chroma("#2A643D")),
  cyan: colorRamp(chroma("#215050")),
  blue: colorRamp(chroma("#2F6DB7")),
  violet: colorRamp(chroma("#5874C1")),
  magenta: colorRamp(chroma("#DE9AB8")),
};

export const theme: ThemeConfig = {
  meta: {
    name: "Zed Pro",
    author: "Nate Butler",
    url: "https://github.com/iamnbutler",
    license: {
      type: "MIT",
      url: "",
    }
  },
  color: ramps,
  syntax: {
    primary: "#E6E6E6",
    comment: violet,
    punctuation: "#E6E6E6",
    constant: cyan,
    keyword: red,
  }
}

export const dark = createColorScheme(`${name} Dark`, false, ramps, theme);
export const light = createColorScheme(`${name} Light`, true, ramps, theme);
