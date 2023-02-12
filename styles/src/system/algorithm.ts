// Adapted from @k-vyn/coloralgorithm

import bezier from "bezier-easing";
import chroma, { Scale } from "chroma-js";
import { Curve } from "./curves";
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

  // Roughly  calculate if a color is dark or light
  const isLight = lch[0] > 50;

  const result = {
    step,
    hex,
    lch,
    rgbaArray,
    isLight,
  };

  return result;
}

/** Outputs 101 colors (0-100) */
export function generateColors(props: ColorProps, inverted: boolean) {
  const steps = 101;
  const colors: ColorSet = [];

  const { start, middle, end } = props.color;

  const startColor = typeof start === "string" ? validColor(start) : start;
  const middleColor = typeof middle === "string" ? validColor(middle) : middle;
  const endColor = typeof end === "string" ? validColor(end) : end;

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

export function generateColorsUsingCurve(
  startColor: string,
  endColor: string,
  curve: number[]
) {
  const NUM_STEPS = 101;

  const easing = bezier(curve[0], curve[1], curve[2], curve[3]);
  const curveProgress = [];
  for (let i = 0; i <= NUM_STEPS; i++) {
    curveProgress.push(easing(i / NUM_STEPS));
  }

  const colors: chroma.Color[] = [];
  for (let i = 0; i < NUM_STEPS; i++) {
    // Use HSL as an input as it is easier to construct programatically
    // const color = chroma.hsl();
    const color = chroma.mix(startColor, endColor, curveProgress[i], "lch");
    colors.push(color);
  }

  return colors;
}

export function generateColors2(
  hue: {
    start: number;
    end: number;
    curve: Curve;
  },
  saturation: {
    start: number;
    end: number;
    curve: Curve;
  },
  lightness: {
    start: number;
    end: number;
    curve: Curve;
  }
) {
  const NUM_STEPS = 9;

  const hueEasing = bezier(
    hue.curve.value[0],
    hue.curve.value[1],
    hue.curve.value[2],
    hue.curve.value[3]
  );
  const saturationEasing = bezier(
    saturation.curve.value[0],
    saturation.curve.value[1],
    saturation.curve.value[2],
    saturation.curve.value[3]
  );
  const lightnessEasing = bezier(
    lightness.curve.value[0],
    lightness.curve.value[1],
    lightness.curve.value[2],
    lightness.curve.value[3]
  );

  const colors: chroma.Color[] = [];
  for (let i = 0; i < NUM_STEPS; i++) {
    const hueValue =
      hueEasing(i / NUM_STEPS) * (hue.end - hue.start) + hue.start;
    const saturationValue =
      saturationEasing(i / NUM_STEPS) * (saturation.end - saturation.start) +
      saturation.start;
    const lightnessValue =
      lightnessEasing(i / NUM_STEPS) * (lightness.end - lightness.start) +
      lightness.start;

    const color = chroma.hsl(
      hueValue,
      saturationValue / 100,
      lightnessValue / 100
    );
    colors.push(color);
  }

  const scale = chroma.scale(colors).mode("lch");
  return scale;
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
