import chroma from "chroma-js";
import { colorRamp, createColorScheme } from "../common/ramps";

const name = "Brush Trees";
const author = "Abraham White <abelincoln.white@gmail.com>";
const url = "https://github.com/WhiteAbeLincoln/base16-brushtrees-scheme";
const license = {
  type: "MIT",
  url: "https://github.com/WhiteAbeLincoln/base16-brushtrees-scheme/blob/master/LICENSE"
}

export const dark = createColorScheme(`${name} Dark`, false, {
  neutral: chroma.scale([
    "#485867",
    "#5A6D7A",
    "#6D828E",
    "#8299A1",
    "#98AFB5",
    "#B0C5C8",
    "#C9DBDC",
    "#E3EFEF",
  ]),
  red: colorRamp(chroma("#b38686")),
  orange: colorRamp(chroma("#d8bba2")),
  yellow: colorRamp(chroma("#aab386")),
  green: colorRamp(chroma("#87b386")),
  cyan: colorRamp(chroma("#86b3b3")),
  blue: colorRamp(chroma("#868cb3")),
  violet: colorRamp(chroma("#b386b2")),
  magenta: colorRamp(chroma("#b39f9f")),
});

export const mirage = createColorScheme(`${name} Mirage`, false, {
  neutral: chroma.scale([
    "#485867",
    "#5A6D7A",
    "#6D828E",
    "#8299A1",
    "#98AFB5",
    "#B0C5C8",
    "#C9DBDC",
    "#E3EFEF",
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

export const light = createColorScheme(`${name} Light`, true, {
  neutral: chroma.scale([
    "#1A1F29",
    "#242936",
    "#5C6773",
    "#828C99",
    "#ABB0B6",
    "#F8F9FA",
    "#F3F4F5",
    "#FAFAFA",
  ]),
  red: colorRamp(chroma("#b38686")),
  orange: colorRamp(chroma("#d8bba2")),
  yellow: colorRamp(chroma("#aab386")),
  green: colorRamp(chroma("#87b386")),
  cyan: colorRamp(chroma("#86b3b3")),
  blue: colorRamp(chroma("#868cb3")),
  violet: colorRamp(chroma("#b386b2")),
  magenta: colorRamp(chroma("#b39f9f")),
});
