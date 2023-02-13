import bezier from "bezier-easing";
import chroma from "chroma-js";
import { Color, ColorFamily, ColorFamilyConfig, ColorScale } from "../types";
import { percentageToNormalized } from "./convert";
import { curve } from "./curve";

// Re-export interface in a more standard format
export type EasingFunction = bezier.EasingFunction;

/**
 * Generates a color, outputs it in multiple formats, and returns a variety of useful metadata.
 *
 * @param {EasingFunction} hueEasing - An easing function for the hue component of the color.
 * @param {EasingFunction} saturationEasing - An easing function for the saturation component of the color.
 * @param {EasingFunction} lightnessEasing - An easing function for the lightness component of the color.
 * @param {ColorFamilyConfig} family - Configuration for the color family.
 * @param {number} step - The current step.
 * @param {number} steps - The total number of steps in the color scale.
 *
 * @returns {Color} The generated color, with its calculated contrast against black and white, as well as its LCH values, RGBA array, hexadecimal representation, and a flag indicating if it is light or dark.
 */
function generateColor(
  hueEasing: EasingFunction,
  saturationEasing: EasingFunction,
  lightnessEasing: EasingFunction,
  family: ColorFamilyConfig,
  step: number,
  steps: number
) {
  const { hue, saturation, lightness } = family.color;

  const stepHue = hueEasing(step / steps) * (hue.end - hue.start) + hue.start;
  const stepSaturation =
    saturationEasing(step / steps) * (saturation.end - saturation.start) +
    saturation.start;
  const stepLightness =
    lightnessEasing(step / steps) * (lightness.end - lightness.start) +
    lightness.start;

  const color = chroma.hsl(
    stepHue,
    percentageToNormalized(stepSaturation),
    percentageToNormalized(stepLightness)
  );

  const contrast = {
    black: {
      value: chroma.contrast(color, "black"),
      aaPass: chroma.contrast(color, "black") >= 4.5,
      aaaPass: chroma.contrast(color, "black") >= 7,
    },
    white: {
      value: chroma.contrast(color, "white"),
      aaPass: chroma.contrast(color, "white") >= 4.5,
      aaaPass: chroma.contrast(color, "white") >= 7,
    },
  };

  const lch = color.lch();
  const rgba = color.rgba();
  const hex = color.hex();

  // 55 is a magic number. It's the lightness value at which we consider a color to be "light".
  // It was picked by eye with some testing. We might want to use a more scientific approach in the future.
  const isLight = lch[0] > 55;

  const result: Color = {
    step,
    lch,
    hex,
    rgba,
    contrast,
    isLight,
  };

  return result;
}

/**
 * Generates a color scale based on a color family configuration.
 *
 * @param {ColorFamilyConfig} config - The configuration for the color family.
 * @param {Boolean} inverted - Specifies whether the color scale should be inverted or not.
 *
 * @returns {ColorScale} The generated color scale.
 *
 * @example
 * ```ts
 * const colorScale = generateColorScale({
 *   name: "blue",
 *   color: {
 *     hue: {
 *       start: 210,
 *       end: 240,
 *       curve: "easeInOut"
 *     },
 *     saturation: {
 *       start: 100,
 *       end: 100,
 *       curve: "easeInOut"
 *     },
 *     lightness: {
 *       start: 50,
 *       end: 50,
 *       curve: "easeInOut"
 *     }
 *   }
 * });
 * ```
 */

export function generateColorScale(
  config: ColorFamilyConfig,
  inverted: Boolean = false
) {
  const { hue, saturation, lightness } = config.color;

  // 101 steps means we get values from 0-100
  const NUM_STEPS = 101;

  const hueEasing = curve(hue.curve, inverted);
  const saturationEasing = curve(saturation.curve, inverted);
  const lightnessEasing = curve(lightness.curve, inverted);

  let scale: ColorScale = {
    colors: [],
    values: [],
  };

  for (let i = 0; i < NUM_STEPS; i++) {
    const color = generateColor(
      hueEasing,
      saturationEasing,
      lightnessEasing,
      config,
      i,
      NUM_STEPS
    );

    scale.colors.push(color);
    scale.values.push(color.hex);
  }

  return scale;
}

/** Generates a color family with a scale and an inverted scale. */
export function generateColorFamily(config: ColorFamilyConfig) {
  const scale = generateColorScale(config, false);
  const invertedScale = generateColorScale(config, true);

  const family: ColorFamily = {
    name: config.name,
    scale,
    invertedScale,
  };

  return family;
}
