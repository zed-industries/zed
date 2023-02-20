import chroma from "chroma-js";
import { colorRamp, createColorScheme } from "../common/ramps";

const name = "Abruzzo";
const author = "slightknack <hey@isaac.sh>";
const url = "https://github.com/slightknack";
const license = {
  type: "",
  url: ""
}

export const dark = createColorScheme(`${name}`, false, {
  neutral: chroma.scale([
    "#1b0d05",
    "#2c1e18",
    "#654035",
    "#9d5e4a",
    "#b37354",
    "#c1825a",
    "#dda66e",
    "#fbf3e2",
  ]),
  red: colorRamp(chroma("#e594c4")),
  orange: colorRamp(chroma("#d9e87e")),
  yellow: colorRamp(chroma("#fd9d83")),
  green: colorRamp(chroma("#96adf7")),
  cyan: colorRamp(chroma("#fc798f")),
  blue: colorRamp(chroma("#BCD0F5")),
  violet: colorRamp(chroma("#dac5eb")),
  magenta: colorRamp(chroma("#c1a3ef")),
});
