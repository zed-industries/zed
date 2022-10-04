import chroma from "chroma-js";
import { colorRamp, createColorScheme } from "./common/ramps";

const name = "abruzzo";

const ramps = {
  neutral: chroma.scale([
    "#1b0d05",
    "#2c1e18",
    "#654035",
    "#9d5e4a",
    "#b37354",
    "#c1825a",
    "#dda66e",
    "#fbf3e2",
  ]).domain([
    0,
    0.15,
    0.35,
    0.5,
    0.6,
    0.75,
    0.85,
    1
  ]),
  red: colorRamp(chroma("#e594c4")),
  orange: colorRamp(chroma("#d9e87e")),
  yellow: colorRamp(chroma("#fd9d83")),
  green: colorRamp(chroma("#96adf7")),
  cyan: colorRamp(chroma("#fc798f")),
  blue: colorRamp(chroma("#BCD0F5")),
  violet: colorRamp(chroma("#dac5eb")),
  magenta: colorRamp(chroma("#c1a3ef")),
};

export const dark = createColorScheme(`${name}`, false, ramps);
// export const light = createTheme(`${name}-light`, true, ramps);
