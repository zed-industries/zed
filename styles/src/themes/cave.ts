import { createTheme } from "./base16";
import { color } from "../tokens";

const name = "cave";

const neutrals = [
  color("#19171c"),
  color("#26232a"),
  color("#585260"),
  color("#655f6d"),
  color("#7e7887"),
  color("#8b8792"),
  color("#e2dfe7"),
  color("#efecf4"),
];

const colors = {
  "red": color("#be4678"),
  "orange": color("#aa573c"),
  "yellow": color("#a06e3b"),
  "green": color("#2a9292"),
  "cyan": color("#398bc6"),
  "blue": color("#576ddb"),
  "violet": color("#955ae7"),
  "magenta": color("#bf40bf"),
};

export const dark = createTheme(`${name}-dark`, false, neutrals, colors);
export const light = createTheme(`${name}-light`, true, neutrals, colors);