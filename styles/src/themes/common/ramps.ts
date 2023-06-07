import chroma, { Color, Scale } from "chroma-js"
import { RampSet } from "./colorScheme"
import {
    ThemeConfigInputColors,
    ThemeConfigInputColorsKeys,
} from "../../themeConfig"

export function colorRamp(color: Color): Scale {
    let endColor = color.desaturate(1).brighten(5)
    let startColor = color.desaturate(1).darken(4)
    return chroma.scale([startColor, color, endColor]).mode("lab")
}

/**
 * Chromajs mutates the underlying ramp when you call domain. This causes problems because
    we now store the ramps object in the theme so that we can pull colors out of them.
    So instead of calling domain and storing the result, we have to construct new ramps for each
    theme so that we don't modify the passed in ramps.
    This combined with an error in the type definitions for chroma js means we have to cast the colors
    function to any in order to get the colors back out from the original ramps.
 * @param isLight 
 * @param colorRamps 
 * @returns 
 */
export function getRamps(
    isLight: boolean,
    colorRamps: ThemeConfigInputColors
): RampSet {
    const ramps: RampSet = {} as any
    const colorsKeys = Object.keys(colorRamps) as ThemeConfigInputColorsKeys[]

    if (isLight) {
        for (const rampName of colorsKeys) {
            ramps[rampName] = chroma.scale(
                colorRamps[rampName].colors(100).reverse()
            )
        }
        ramps.neutral = chroma.scale(colorRamps.neutral.colors(100).reverse())
    } else {
        for (const rampName of colorsKeys) {
            ramps[rampName] = chroma.scale(colorRamps[rampName].colors(100))
        }
        ramps.neutral = chroma.scale(colorRamps.neutral.colors(100))
    }

    return ramps
}
