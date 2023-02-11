// Adapted from @k-vyn/coloralgorithm

import chroma, { Scale } from "chroma-js";
import { ColorFamily, ColorProps, ColorSet } from "./types";

function validColor(color: string) {
  if (chroma.valid(color)) {
    return color;
  } else {
    throw new Error(`Invalid color: ${color}`);
  }
}

function assignColor(scale: Scale, steps: number, step: number) {
  const color = scale(step / steps);
  const lch = color.lch();
  const rgbaArray = color.rgba();
  const hex = color.hex();

  const result = {
    step,
    hex,
    lch,
    rgbaArray,
  };

  return result;
}

/** Outputs 101 colors (0-100) */
export function generateColors(props: ColorProps, inverted: boolean) {
  const steps = 101;
  const colors: ColorSet = [];

  const { start, middle, end } = props.color;

  const startColor = validColor(start);
  const middleColor = validColor(middle);
  const endColor = validColor(end);

  // TODO: Use curve when generating colors

  let scale: Scale;

  if (inverted) {
    scale = chroma.scale([endColor, middleColor, startColor]).mode("lch");
  } else {
    scale = chroma.scale([startColor, middleColor, endColor]).mode("lch");
  }
  for (let i = 0; i < steps; i++) {
    const color = assignColor(scale, steps, i);
    colors.push(color);
  }
  return colors;
}

/** Generates two color ramps:
 * One for for light, and one for dark.
 * By generating two ramps, rather than two default themes, we can use the same reference palette values for tokens in components.
 *
 * Each ramp has 101 colors (0-100)
 */
export function generateColorSet(props: ColorProps) {
  const generatedColors = generateColors(props, false);
  const generatedInvertedColors = generateColors(props, true);

  const colors = generatedColors.map((color) => color.hex);
  const invertedColors = generatedInvertedColors.map((color) => color.hex);

  const result: ColorFamily = {
    name: props.name,
    colors: colors,
    invertedColors: invertedColors,
    colorsMeta: generatedColors,
    invertedMeta: generatedInvertedColors,
  };

  return result;
}
