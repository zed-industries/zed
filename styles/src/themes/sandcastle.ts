import chroma from "chroma-js";
import { colorRamp, createTheme } from "./common/base16";

const name = "sandcastle";

const ramps = {
  neutral: chroma.scale([
    "#282c34",
    "#2c323b",
    "#3e4451",
    "#665c54",
    "#928374",
    "#a89984",
    "#d5c4a1",
    "#fdf4c1",
  ]),
  red: colorRamp(chroma("#83a598")),
  orange: colorRamp(chroma("#a07e3b")),
  yellow: colorRamp(chroma("#a07e3b")),
  green: colorRamp(chroma("#528b8b")),
  cyan: colorRamp(chroma("#83a598")),
  blue: colorRamp(chroma("#83a598")),
  violet: colorRamp(chroma("#d75f5f")),
  magenta: colorRamp(chroma("#a87322")),
};

export const dark = createTheme(`${name}`, false, ramps);
