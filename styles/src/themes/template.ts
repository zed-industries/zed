/**
 * To create a new theme duplicate this file and move into templates
 **/

import chroma from "chroma-js";
import { colorRamp, createTheme } from "./common/base16";

/**
 * Theme Name
 *
 * What the theme will be called in the UI
 * Also used to generate filenames, etc
 **/

const name = "themeName";

/**
 * Theme Colors
 *
 * Zed themes are based on [base16](https://github.com/chriskempson/base16)
 * The first 8 colors ("Neutrals") are used to construct the UI background, panels, etc.
 * The latter 8 colors ("Accents") are used for syntax themes, semantic colors, and UI states.
 **/

/**
 * Color Ramps
 *
 * We use (chroma-js)[https://gka.github.io/chroma.js/] to minipulate color in themes and to build color ramps.
 *
 * You can use chroma-js operations on the ramps here.
 * For example, you could use chroma.scale(...).correctLightness if your color ramps seem washed out near the ends.
 **/

// TODO: Express accents without refering to them directly by color name.
// See common/base16.ts for where color tokens are used.

const ramps = {
  neutral: chroma.scale([
    "#19171c", // Dark: darkest backgrounds, inputs | Light: Lightest text, active states
    "#26232a",
    "#585260",
    "#655f6d",
    "#7e7887",
    "#8b8792",
    "#e2dfe7",
    "#efecf4", // Light: darkest backgrounds, inputs | Dark: Lightest text, active states
  ]),
  red: colorRamp(chroma("#be4678")), // Errors
  orange: colorRamp(chroma("#aa573c")),
  yellow: colorRamp(chroma("#a06e3b")), // Warnings
  green: colorRamp(chroma("#2a9292")), // Positive
  cyan: colorRamp(chroma("#398bc6")), // Player 1 (Host)
  blue: colorRamp(chroma("#576ddb")), // Info
  violet: colorRamp(chroma("#955ae7")),
  magenta: colorRamp(chroma("#bf40bf")),
};

/**
 * Theme Variants
 *
 * Currently we only support (and require) dark and light themes
 * Eventually you will be able to have only a light or dark theme,
 * and define other variants here.
 *
 * createTheme([name], [isLight], [arrayOfRamps])
 **/

export const dark = createTheme(`${name}-dark`, false, ramps);
export const light = createTheme(`${name}-light`, true, ramps);
