import Theme from "../themes/common/theme";
import { border, modalShadow } from "./components";

export default function terminal(theme: Theme) {
  let colors = {
    black: theme.ramps.neutral(0).hex(),
    red: theme.ramps.red(0.5).hex(),
    green: theme.ramps.green(0.5).hex(),
    yellow: theme.ramps.yellow(0.5).hex(),
    blue: theme.ramps.blue(0.5).hex(),
    magenta: theme.ramps.magenta(0.5).hex(),
    cyan: theme.ramps.cyan(0.5).hex(),
    white: theme.ramps.neutral(7).hex(),
    brightBlack: theme.ramps.neutral(2).hex(),
    brightRed: theme.ramps.red(0.25).hex(),
    brightGreen: theme.ramps.green(0.25).hex(),
    brightYellow: theme.ramps.yellow(0.25).hex(),
    brightBlue: theme.ramps.blue(0.25).hex(),
    brightMagenta: theme.ramps.magenta(0.25).hex(),
    brightCyan: theme.ramps.cyan(0.25).hex(),
    brightWhite: theme.ramps.neutral(7).hex(),
    foreground: theme.ramps.neutral(7).hex(),
    background: theme.ramps.neutral(0).hex(),
    modalBackground: theme.ramps.neutral(1).hex(),
    cursor: theme.ramps.neutral(7).hex(),
    dimBlack: theme.ramps.neutral(7).hex(),
    dimRed: theme.ramps.red(0.75).hex(),
    dimGreen: theme.ramps.green(0.75).hex(),
    dimYellow: theme.ramps.yellow(0.75).hex(),
    dimBlue: theme.ramps.blue(0.75).hex(),
    dimMagenta: theme.ramps.magenta(0.75).hex(),
    dimCyan: theme.ramps.cyan(0.75).hex(),
    dimWhite: theme.ramps.neutral(5).hex(),
    brightForeground: theme.ramps.neutral(7).hex(),
    dimForeground: theme.ramps.neutral(0).hex(),
  };

  return {
    colors,
    modalContainer: {
      background: colors.modalBackground,
      cornerRadius: 8,
      padding: 8,
      margin: 25,
      border: border(theme, "primary"),
      shadow: modalShadow(theme),
    }
  };
}