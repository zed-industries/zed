import chroma from "chroma-js";
import { colorRamp, createColorScheme } from "../common/ramps";

const name = "Dracula";
const author = "zenorocha";
const url = "https://github.com/dracula/dracula-theme";
const license = {
  type: "MIT",
  url: "https://github.com/dracula/dracula-theme/blob/master/LICENSE",
};

export const dark = createColorScheme(`${name}`, false, {
  neutral: chroma.scale([
    "#282A36",
    "#3a3c4e",
    "#4d4f68",
    "#626483",
    "#62d6e8",
    "#e9e9f4",
    "#f1f2f8",
    "#f8f8f2",
  ]),
  red: colorRamp(chroma("#ff5555")),
  orange: colorRamp(chroma("#ffb86c")),
  yellow: colorRamp(chroma("#f1fa8c")),
  green: colorRamp(chroma("#50fa7b")),
  cyan: colorRamp(chroma("#8be9fd")),
  blue: colorRamp(chroma("#6272a4")),
  violet: colorRamp(chroma("#bd93f9")),
  magenta: colorRamp(chroma("#00f769")),
});
