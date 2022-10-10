import chroma from "chroma-js";
import { colorRamp, createColorScheme } from "./common/ramps";

const name = "brush-tree";

const ramps = {
  neutral: chroma
    .scale([
      "#485867",
      "#5A6D7A",
      "#6D828E",
      "#8299A1",
      "#98AFB5",
      "#B0C5C8",
      "#C9DBDC",
      "#E3EFEF",
    ])
    .domain([0, 0.17, 0.32, 0.48, 0.6, 0.715, 0.858, 1]),
  red: colorRamp(chroma("#b38686")),
  orange: colorRamp(chroma("#d8bba2")),
  yellow: colorRamp(chroma("#aab386")),
  green: colorRamp(chroma("#87b386")),
  cyan: colorRamp(chroma("#86b3b3")),
  blue: colorRamp(chroma("#868cb3")),
  violet: colorRamp(chroma("#b386b2")),
  magenta: colorRamp(chroma("#b39f9f")),
};

export const dark = createColorScheme(`${name}-dark`, false, ramps);
export const light = createColorScheme(`${name}-light`, true, ramps);
