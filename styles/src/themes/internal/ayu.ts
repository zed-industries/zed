import chroma from "chroma-js";
import { colorRamp, createColorScheme } from "../common/ramps";

const name = "Ayu";
const author = "Khue Nguyen <Z5483Y@gmail.com>";

export const dark = createColorScheme(`${name} Dark`, false, {
  neutral: chroma.scale([
    "#0F1419",
    "#131721",
    "#272D38",
    "#3E4B59",
    "#BFBDB6",
    "#E6E1CF",
    "#E6E1CF",
    "#F3F4F5",
  ]),
  red: colorRamp(chroma("#F07178")),
  orange: colorRamp(chroma("#FF8F40")),
  yellow: colorRamp(chroma("#FFB454")),
  green: colorRamp(chroma("#B8CC52")),
  cyan: colorRamp(chroma("#95E6CB")),
  blue: colorRamp(chroma("#59C2FF")),
  violet: colorRamp(chroma("#D2A6FF")),
  magenta: colorRamp(chroma("#E6B673")),
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
  red: colorRamp(chroma("#F07178")),
  orange: colorRamp(chroma("#FA8D3E")),
  yellow: colorRamp(chroma("#F2AE49")),
  green: colorRamp(chroma("#86B300")),
  cyan: colorRamp(chroma("#4CBF99")),
  blue: colorRamp(chroma("#36A3D9")),
  violet: colorRamp(chroma("#A37ACC")),
  magenta: colorRamp(chroma("#E6BA7E")),
});
