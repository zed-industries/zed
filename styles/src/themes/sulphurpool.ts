import { createTheme } from "./base16";
import { color } from "../tokens";

const name = "sulphurpool";

const neutrals = [
  color("#202746"),
  color("#293256"),
  color("#5e6687"),
  color("#6b7394"),
  color("#898ea4"),
  color("#979db4"),
  color("#dfe2f1"),
  color("#f5f7ff"),
]

const colors = {
  "red": color("#c94922"),
  "orange": color("#c76b29"),
  "yellow": color("#c08b30"),
  "green": color("#ac9739"),
  "cyan": color("#22a2c9"),
  "blue": color("#3d8fd1"),
  "violet": color("#6679cc"),
  "magenta": color("#9c637a"),
};

export const dark = createTheme(`${name}-dark`, false, neutrals, colors);
export const light = createTheme(`${name}-light`, true, neutrals, colors);