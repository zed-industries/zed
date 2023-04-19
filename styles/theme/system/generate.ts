import bezier from "bezier-easing"
import chroma from "chroma-js"
import { useCurve } from "./curves"
import { Color, ColorFamily, ColorFamilyConfig, ColorScale } from "./color"

/** Converts a percentage scale value (0-100) to normalized scale (0-1) value. */
export function percentageToNormalized(value: number) {
    const normalized = value / 100
    return normalized
}

/** Converts a normalized scale (0-1) value to a percentage scale (0-100) value. */
export function normalizedToPercetage(value: number) {
    const percentage = value * 100
    return percentage
}

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
    hueEasing: bezier.EasingFunction,
    saturationEasing: bezier.EasingFunction,
    lightnessEasing: bezier.EasingFunction,
    family: ColorFamilyConfig,
    step: number,
    steps: number
): Color {
    const { hue, saturation, lightness } = family.color

    const stepHue = hueEasing(step / steps) * (hue.end - hue.start) + hue.start
    const stepSaturation =
        saturationEasing(step / steps) * (saturation.end - saturation.start) +
        saturation.start
    const stepLightness =
        lightnessEasing(step / steps) * (lightness.end - lightness.start) +
        lightness.start

    const color = chroma.hsl(
        stepHue,
        percentageToNormalized(stepSaturation),
        percentageToNormalized(stepLightness)
    )

    const lch = color.lch()
    const rgba = color.rgba()
    const hex = color.hex()

    // 55 is a magic number. It's the lightness value at which we consider a color to be "light".
    // It was picked by eye with some testing. We might want to use a more scientific approach in the future.
    const isLight = lch[0] > 55

    const result: Color = {
        step,
        lch,
        hex,
        rgba,
        isLight,
    }

    return result
}

/**
 * Generates a color scale based on a color family configuration.
 *
 * @param {ColorFamilyConfig} config - The configuration for the color family.
 * @param {Boolean} inverted - Specifies whether the color scale should be inverted or not.
 *
 * @returns {ColorScale} The generated color scale.
 *
 * Example:
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
): ColorScale {
    const { hue, saturation, lightness } = config.color

    // 101 steps means we get values from 0-100
    const NUM_STEPS = 101

    const hueEasing = useCurve(hue.curve, inverted)
    const saturationEasing = useCurve(saturation.curve, inverted)
    const lightnessEasing = useCurve(lightness.curve, inverted)

    let scale: ColorScale = {
        colors: [],
        values: [],
    }

    for (let i = 0; i < NUM_STEPS; i++) {
        const color = generateColor(
            hueEasing,
            saturationEasing,
            lightnessEasing,
            config,
            i,
            NUM_STEPS
        )

        scale.colors.push(color)
        scale.values.push(color.hex)
    }

    return scale
}

/** Generates a color family with a scale and an inverted scale. */
export function generateColorFamily(config: ColorFamilyConfig) {
    const scale = generateColorScale(config, false)
    const invertedScale = generateColorScale(config, true)

    const family: ColorFamily = {
        name: config.name,
        scale,
        invertedScale,
    }

    return family
}
