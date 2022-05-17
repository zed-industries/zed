import chroma from "chroma-js";
import { createTheme } from "./base16";

const name = "cave";

const colors = {
  "red": chroma("#be4678"),
  "orange": chroma("#aa573c"),
  "yellow": chroma("#a06e3b"),
  "green": chroma("#2a9292"),
  "cyan": chroma("#398bc6"),
  "blue": chroma("#576ddb"),
  "violet": chroma("#955ae7"),
  "magenta": chroma("#bf40bf"),
};

const ramps = {
  neutral: chroma.scale(["#19171c", "#26232a", "#585260", "#655f6d", "#7e7887", "#8b8792", "#e2dfe7", "#efecf4"]),
  red: chroma.scale([colors.red.darken(3), colors.red, colors.red.brighten(3)]),
  orange: chroma.scale([colors.orange.darken(3), colors.orange, colors.orange.brighten(3)]),
  yellow: chroma.scale([colors.yellow.darken(3), colors.yellow, colors.yellow.brighten(3)]),
  green: chroma.scale([colors.green.darken(3), colors.green, colors.green.brighten(3)]),
  cyan: chroma.scale([colors.cyan.darken(3), colors.cyan, colors.cyan.brighten(3)]),
  blue: chroma.scale([colors.blue.darken(3), colors.blue, colors.blue.brighten(3)]),
  violet: chroma.scale([colors.violet.darken(3), colors.violet, colors.violet.brighten(3)]),
  magenta: chroma.scale([colors.magenta.darken(3), colors.magenta, colors.magenta.brighten(3)]),
}

export const dark = createTheme(`${name}-dark`, false, ramps);
export const light = createTheme(`${name}-light`, true, ramps);