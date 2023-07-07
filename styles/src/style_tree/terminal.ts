import { useTheme } from "../theme"

export default function terminal() {
    const theme = useTheme()

    /**
     * Colors are controlled per-cell in the terminal grid.
     * Cells can be set to any of these more 'theme-capable' colors
     * or can be set directly with RGB values.
     * Here are the common interpretations of these names:
     * https://en.wikipedia.org/wiki/ANSI_escape_code#Colors
     */
    return {
        black: theme.ramps.neutral(0).hex(),
        red: theme.ramps.red(0.5).hex(),
        green: theme.ramps.green(0.5).hex(),
        yellow: theme.ramps.yellow(0.5).hex(),
        blue: theme.ramps.blue(0.5).hex(),
        magenta: theme.ramps.magenta(0.5).hex(),
        cyan: theme.ramps.cyan(0.5).hex(),
        white: theme.ramps.neutral(1).hex(),
        bright_black: theme.ramps.neutral(0.4).hex(),
        bright_red: theme.ramps.red(0.25).hex(),
        bright_green: theme.ramps.green(0.25).hex(),
        bright_yellow: theme.ramps.yellow(0.25).hex(),
        bright_blue: theme.ramps.blue(0.25).hex(),
        bright_magenta: theme.ramps.magenta(0.25).hex(),
        bright_cyan: theme.ramps.cyan(0.25).hex(),
        bright_white: theme.ramps.neutral(1).hex(),
        /**
         * Default color for characters
         */
        foreground: theme.ramps.neutral(1).hex(),
        /**
         * Default color for the rectangle background of a cell
         */
        background: theme.ramps.neutral(0).hex(),
        modal_background: theme.ramps.neutral(0.1).hex(),
        /**
         * Default color for the cursor
         */
        cursor: theme.players[0].cursor,
        dim_black: theme.ramps.neutral(1).hex(),
        dim_red: theme.ramps.red(0.75).hex(),
        dim_green: theme.ramps.green(0.75).hex(),
        dim_yellow: theme.ramps.yellow(0.75).hex(),
        dim_blue: theme.ramps.blue(0.75).hex(),
        dim_magenta: theme.ramps.magenta(0.75).hex(),
        dim_cyan: theme.ramps.cyan(0.75).hex(),
        dim_white: theme.ramps.neutral(0.6).hex(),
        bright_foreground: theme.ramps.neutral(1).hex(),
        dim_foreground: theme.ramps.neutral(0).hex(),
    }
}
