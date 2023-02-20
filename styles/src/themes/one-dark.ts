import chroma from "chroma-js";
import { Meta } from "./common/colorScheme";
import { colorRamp, createColorScheme } from "./common/ramps";

const name = "One Dark";

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

export const meta: Meta = {
  name,
  author: "simurai",
  license: {
    SPDX: "MIT",
    https_url: "https://raw.githubusercontent.com/atom/atom/master/packages/one-light-ui/LICENSE.md",
    license_checksum: "d5af8fc171f6f600c0ab4e7597dca398dda80dbe6821ce01cef78e859e7a00f8"
  },
  url: "https://github.com/atom/atom/tree/master/packages/one-dark-ui"
}
