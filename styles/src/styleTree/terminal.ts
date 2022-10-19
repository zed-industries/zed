import { Elevation } from "../themes/common/colorScheme";

export default function terminal(elevation: Elevation, cursor_color: string) {
  /**
   * Colors are controlled per-cell in the terminal grid.
   * Cells can be set to any of these more 'theme-capable' colors
   * or can be set directly with RGB values.
   * Here are the common interpretations of these names:
   * https://en.wikipedia.org/wiki/ANSI_escape_code#Colors
   */
  return {
    black: elevation.ramps.neutral(0).hex(),
    red: elevation.ramps.red(0.5).hex(),
    green: elevation.ramps.green(0.5).hex(),
    yellow: elevation.ramps.yellow(0.5).hex(),
    blue: elevation.ramps.blue(0.5).hex(),
    magenta: elevation.ramps.magenta(0.5).hex(),
    cyan: elevation.ramps.cyan(0.5).hex(),
    white: elevation.ramps.neutral(1).hex(),
    brightBlack: elevation.ramps.neutral(0.4).hex(),
    brightRed: elevation.ramps.red(0.25).hex(),
    brightGreen: elevation.ramps.green(0.25).hex(),
    brightYellow: elevation.ramps.yellow(0.25).hex(),
    brightBlue: elevation.ramps.blue(0.25).hex(),
    brightMagenta: elevation.ramps.magenta(0.25).hex(),
    brightCyan: elevation.ramps.cyan(0.25).hex(),
    brightWhite: elevation.ramps.neutral(1).hex(),
    /**
     * Default color for characters
     */
    foreground: elevation.ramps.neutral(1).hex(),
    /**
     * Default color for the rectangle background of a cell
     */
    background: elevation.ramps.neutral(0).hex(),
    modalBackground: elevation.ramps.neutral(0.1).hex(),
    /**
     * Default color for the cursor
     */
    cursor: cursor_color,
    dimBlack: elevation.ramps.neutral(1).hex(),
    dimRed: elevation.ramps.red(0.75).hex(),
    dimGreen: elevation.ramps.green(0.75).hex(),
    dimYellow: elevation.ramps.yellow(0.75).hex(),
    dimBlue: elevation.ramps.blue(0.75).hex(),
    dimMagenta: elevation.ramps.magenta(0.75).hex(),
    dimCyan: elevation.ramps.cyan(0.75).hex(),
    dimWhite: elevation.ramps.neutral(0.6).hex(),
    brightForeground: elevation.ramps.neutral(1).hex(),
    dimForeground: elevation.ramps.neutral(0).hex(),
  };
}
