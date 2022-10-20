import chroma from "chroma-js";
import { colorRamp, createColorScheme } from "../common/ramps";

const name = "Seti UI";
const author = "jesseweed";
const url = "";

// `name-[light|dark]`, isLight, color ramps
export const dark = createColorScheme(`${name}`, false, {
  neutral: chroma.scale([
    "#151718",
    "#262B30",
    "#1E2326",
    "#41535B",
    "#43a5d5",
    "#d6d6d6",
    "#eeeeee",
    "#ffffff",
  ]),
  red: colorRamp(chroma("#Cd3f45")),
  orange: colorRamp(chroma("#db7b55")),
  yellow: colorRamp(chroma("#e6cd69")),
  green: colorRamp(chroma("#9fca56")),
  cyan: colorRamp(chroma("#55dbbe")),
  blue: colorRamp(chroma("#55b5db")),
  violet: colorRamp(chroma("#a074c4")),
  magenta: colorRamp(chroma("#8a553f")),
});
