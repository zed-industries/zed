import { createTheme } from "./base16";
import { color } from "../tokens";

const name = "molika";

const neutrals = [
  color("#161616"),
  color("#161616"),
  color("#676767"),
  color("#676767"),
  color("#c7c7c7"),
  color("#c7c7c7"),
  color("#feffff"),
  color("#feffff"),
];

const colors = {
  "red": color("#fa7fac"),
  "orange": color("#e5da72"),
  "yellow": color("#fff27f"),
  "green": color("#bde271"),
  "cyan": color("#5ed6fe"),
  "blue": color("#00bdff"),
  "violet": color("#9a37ff"),
  "magenta": color("#bd9eff"),
};

export const dark = createTheme(`${name}-dark`, false, neutrals, colors);
// export const light = createTheme(`${name}-light`, true, neutrals, colors);