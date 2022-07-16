import chroma from "chroma-js";
import { colorRamp, createTheme } from "./common/base16";

const name = "one";
const author = "Daniel Pfeifer (http://github.com/purpleKarrot)";
const url =
  "https://github.com/purpleKarrot/base16-one-light-scheme/blob/master/one-light.yaml";

const base00 = "#090a0b";
const base01 = "#202227";
const base02 = "#383a42";
const base03 = "#a0a1a7";
const base04 = "#696c77";
const base05 = "#a0a1a7";
const base06 = "#e5e5e6";
const base07 = "#f0f0f1";
const base08 = "#fafafa";
const base09 = "#d75f00";
const base0A = "#c18401";
const base0B = "#50a14f";
const base0C = "#0184bc";
const base0D = "#4078f2";
const base0E = "#a626a4";
const base0F = "#986801";

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

export const light = createTheme(`${name}-light`, true, ramps);
