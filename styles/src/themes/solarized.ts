import { createTheme } from "./base16";
import { color } from "../tokens";

const name = "solarized";

const neutrals = [
  color("#002b36"),
  color("#073642"),
  color("#586e75"),
  color("#657b83"),
  color("#839496"),
  color("#93a1a1"),
  color("#eee8d5"),
  color("#fdf6e3"),
];

const colors = {
  "red": color("#dc322f"),
  "orange": color("#cb4b16"),
  "yellow": color("#b58900"),
  "green": color("#859900"),
  "cyan": color("#2aa198"),
  "blue": color("#268bd2"),
  "violet": color("#6c71c4"),
  "magenta": color("#d33682"),
};

export const dark = createTheme(`${name}-dark`, false, neutrals, colors);
export const light = createTheme(`${name}-light`, true, neutrals, colors);