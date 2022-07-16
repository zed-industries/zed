import chroma from "chroma-js";
import { colorRamp, createTheme } from "./common/base16";

const name = "one";
const author = "Chris Kempson (http://chriskempson.com)";
const url =
  "https://github.com/chriskempson/base16-vim/blob/master/colors/base16-onedark.vim";

const base00 = "#282c34";
const base01 = "#353b45";
const base02 = "#3e4451";
const base03 = "#545862";
const base04 = "#565c64";
const base05 = "#abb2bf";
const base06 = "#b6bdca";
const base07 = "#c8ccd4";
const base08 = "#e06c75";
const base09 = "#d19a66";
const base0A = "#e5c07b";
const base0B = "#98c379";
const base0C = "#56b6c2";
const base0D = "#61afef";
const base0E = "#c678dd";
const base0F = "#be5046";

const ramps = {
  neutral: chroma.scale([
    base00,
    base01,
    base02,
    base03,
    base04,
    base05,
    base06,
    base07,
  ]),
  red: colorRamp(chroma(base08)),
  orange: colorRamp(chroma(base09)),
  yellow: colorRamp(chroma(base0A)),
  green: colorRamp(chroma(base0B)),
  cyan: colorRamp(chroma(base0C)),
  blue: colorRamp(chroma(base0D)),
  violet: colorRamp(chroma(base0E)),
  magenta: colorRamp(chroma(base0F)),
};

export const dark = createTheme(`${name}-dark`, false, ramps);
