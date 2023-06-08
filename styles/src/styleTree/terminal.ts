import { ColorScheme } from "../theme/colorScheme"

export default function terminal(colorScheme: ColorScheme) {
    /**
     * Colors are controlled per-cell in the terminal grid.
     * Cells can be set to any of these more 'theme-capable' colors
     * or can be set directly with RGB values.
     * Here are the common interpretations of these names:
     * https://en.wikipedia.org/wiki/ANSI_escape_code#Colors
     */
    return {
        black: colorScheme.ramps.neutral(0).hex(),
        red: colorScheme.ramps.red(0.5).hex(),
        green: colorScheme.ramps.green(0.5).hex(),
        yellow: colorScheme.ramps.yellow(0.5).hex(),
        blue: colorScheme.ramps.blue(0.5).hex(),
        magenta: colorScheme.ramps.magenta(0.5).hex(),
        cyan: colorScheme.ramps.cyan(0.5).hex(),
        white: colorScheme.ramps.neutral(1).hex(),
        brightBlack: colorScheme.ramps.neutral(0.4).hex(),
        brightRed: colorScheme.ramps.red(0.25).hex(),
        brightGreen: colorScheme.ramps.green(0.25).hex(),
        brightYellow: colorScheme.ramps.yellow(0.25).hex(),
        brightBlue: colorScheme.ramps.blue(0.25).hex(),
        brightMagenta: colorScheme.ramps.magenta(0.25).hex(),
        brightCyan: colorScheme.ramps.cyan(0.25).hex(),
        brightWhite: colorScheme.ramps.neutral(1).hex(),
        /**
         * Default color for characters
         */
        foreground: colorScheme.ramps.neutral(1).hex(),
        /**
         * Default color for the rectangle background of a cell
         */
        background: colorScheme.ramps.neutral(0).hex(),
        modalBackground: colorScheme.ramps.neutral(0.1).hex(),
        /**
         * Default color for the cursor
         */
        cursor: colorScheme.players[0].cursor,
        dimBlack: colorScheme.ramps.neutral(1).hex(),
        dimRed: colorScheme.ramps.red(0.75).hex(),
        dimGreen: colorScheme.ramps.green(0.75).hex(),
        dimYellow: colorScheme.ramps.yellow(0.75).hex(),
        dimBlue: colorScheme.ramps.blue(0.75).hex(),
        dimMagenta: colorScheme.ramps.magenta(0.75).hex(),
        dimCyan: colorScheme.ramps.cyan(0.75).hex(),
        dimWhite: colorScheme.ramps.neutral(0.6).hex(),
        brightForeground: colorScheme.ramps.neutral(1).hex(),
        dimForeground: colorScheme.ramps.neutral(0).hex(),
    }
}
