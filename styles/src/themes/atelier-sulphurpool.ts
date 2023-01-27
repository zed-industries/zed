import chroma from "chroma-js";
import { Meta } from "./common/colorScheme";
import { colorRamp, createColorScheme } from "./common/ramps";

const name = "Atelier Sulphurpool";

const ramps = {
  neutral: chroma
    .scale([
      "#202746",
      "#293256",
      "#5e6687",
      "#6b7394",
      "#898ea4",
      "#979db4",
      "#dfe2f1",
      "#f5f7ff",
    ])
    .domain([0, 0.2, 0.38, 0.45, 0.65, 0.7, 0.85, 1]),
  red: colorRamp(chroma("#c94922")),
  orange: colorRamp(chroma("#c76b29")),
  yellow: colorRamp(chroma("#c08b30")),
  green: colorRamp(chroma("#ac9739")),
  cyan: colorRamp(chroma("#22a2c9")),
  blue: colorRamp(chroma("#3d8fd1")),
  violet: colorRamp(chroma("#6679cc")),
  magenta: colorRamp(chroma("#9c637a")),
};

export const dark = createColorScheme(`${name} Dark`, false, ramps);
export const light = createColorScheme(`${name} Light`, true, ramps);

export const meta: Meta = {
  name,
  author: "atelierbram",
  license: {
    SPDX: "MIT",
    https_url: "https://raw.githubusercontent.com/atelierbram/syntax-highlighting/master/LICENSE",
    license_checksum: "6c2353bb9dd0b7b211364d98184ab482e54f40f611eda0c02974c3a1f9e6193c"
  },
  url: "https://atelierbram.github.io/syntax-highlighting/atelier-schemes/dune/"
}