import chroma from "chroma-js";
import { Meta } from "./common/colorScheme";
import { colorRamp, createColorScheme } from "./common/ramps";
import { ThemeConfig } from "./common/themeConfig";

const color = {
  white: "#ACB2BE",
  grey: "#5D636F",
  red: "#D07277",
  orange: "#C0966B",
  yellow: "#DFC184",
  green: "#A1C181",
  teal: "#6FB4C0",
  blue: "#74ADE9",
  purple: "#B478CF",
};

const ramps = {
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
  red: colorRamp(chroma(color.red)),
  orange: colorRamp(chroma(color.orange)),
  yellow: colorRamp(chroma(color.yellow)),
  green: colorRamp(chroma(color.green)),
  cyan: colorRamp(chroma(color.teal)),
  blue: colorRamp(chroma(color.blue)),
  violet: colorRamp(chroma(color.purple)),
  magenta: colorRamp(chroma("#be5046")),
};

export const theme: ThemeConfig = {
  meta: {
    name: "One Dark",
    author: "simurai",
    url: "https://github.com/atom/atom/tree/master/packages/one-dark-ui",
    license: {
      type: "MIT",
      url: "https://github.com/atom/atom/blob/master/packages/one-dark-ui/LICENSE.md",
    },
  },
  color: ramps,
  syntax: {
    primary: { color: color.white },
    comment: { color: color.grey },
    function: { color: color.blue },
    type: { color: color.teal },
    property: { color: color.red },
    number: { color: color.orange },
    string: { color: color.green },
    keyword: { color: color.purple },
    boolean: { color: color.orange },
    punctuation: { color: color.white },
    operator: { color: color.teal }
  },
  override: {}
};

export const variants = [
  createColorScheme(
    `${theme.meta.name} Alpha`,
    false,
    ramps,
    theme
  ),
  createColorScheme(
    `${theme.meta.name} Beta`,
    false,
    ramps,
    theme
  ),
  createColorScheme(
    `${theme.meta.name} Gamma`,
    false,
    ramps,
    theme
  ),
  createColorScheme(
    `${theme.meta.name} Delta`,
    false,
    ramps,
    theme
  ),
  createColorScheme(
    `${theme.meta.name} Foo`,
    false,
    ramps,
    theme
  ),
  createColorScheme(
    `${theme.meta.name} Baz`,
    false,
    ramps,
    theme
  )
]