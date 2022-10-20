import chroma from "chroma-js";
import { colorRamp, createColorScheme } from "../common/ramps";

const name = "Monokai";
const author = "Wimer Hazenberg (http://www.monokai.nl)";
const url = "";

// `name-[light|dark]`, isLight, color ramps
export const dark = createColorScheme(`${name}`, false, {
  neutral: chroma.scale([
    "#272822",
    "#383830",
    "#49483e",
    "#75715e",
    "#a59f85",
    "#f8f8f2",
    "#f5f4f1",
    "#f9f8f5",
  ]),
  red: colorRamp(chroma("#f92672")),
  orange: colorRamp(chroma("#fd971f")),
  yellow: colorRamp(chroma("#f4bf75")),
  green: colorRamp(chroma("#a6e22e")),
  cyan: colorRamp(chroma("#a1efe4")),
  blue: colorRamp(chroma("#66d9ef")),
  violet: colorRamp(chroma("#ae81ff")),
  magenta: colorRamp(chroma("#cc6633")),
});
