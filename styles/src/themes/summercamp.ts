import chroma from "chroma-js";
import { colorRamp, createTheme } from "./common/base16";

const name = "summercamp";

const ramps = {
  neutral: chroma.scale([
    "#1c1810",
    "#2a261c",
    "#3a3527",
    "#3a3527",
    "#5f5b45",
    "#736e55",
    "#bab696",
    "#f8f5de",
  ]),
  red: colorRamp(chroma("#e35142")),
  orange: colorRamp(chroma("#fba11b")),
  yellow: colorRamp(chroma("#f2ff27")),
  green: colorRamp(chroma("#5ceb5a")),
  cyan: colorRamp(chroma("#5aebbc")),
  blue: colorRamp(chroma("#489bf0")),
  violet: colorRamp(chroma("#FF8080")),
  magenta: colorRamp(chroma("#F69BE7")),
};

export const dark = createTheme(`${name}`, false, ramps);
