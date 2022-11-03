import chroma from "chroma-js";
import { Meta } from "./common/colorScheme";
import { colorRamp, createColorScheme } from "./common/ramps";
import { SyntaxHighlightStyle } from "./common/theme";
import { SyntaxOverrides, ThemeConfig } from "./common/themeConfig";

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
  red: colorRamp(chroma("#e06c75")),
  orange: colorRamp(chroma("#d19a66")),
  yellow: colorRamp(chroma("#e5c07b")),
  green: colorRamp(chroma("#98c379")),
  cyan: colorRamp(chroma("#56b6c2")),
  blue: colorRamp(chroma("#61afef")),
  violet: colorRamp(chroma("#c678dd")),
  magenta: colorRamp(chroma("#be5046")),
};

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
    type: { color: color.cyan },
    property: { color: color.red },
    number: { color: color.orange },
    string: { color: color.green },
    keyword: { color: color.purple },
    boolean: { color: color.orange },
    punctuation: { color: color.white },
    operator: { color: color.teal }
  },
};

export const dark = createColorScheme(
  `${theme.meta.name}`,
  false,
  ramps,
  theme
);
