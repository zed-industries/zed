import chroma, { Color, Scale } from "chroma-js"
import { RampSet } from "./create_theme"
import {
    ThemeConfigInputColors,
    ThemeConfigInputColorsKeys,
} from "./theme_config"

export function color_ramp(color: Color): Scale {
    const end_color = color.desaturate(1).brighten(5)
    const start_color = color.desaturate(1).darken(4)
    return chroma.scale([start_color, color, end_color]).mode("lab")
}

/**
 * Chromajs mutates the underlying ramp when you call domain. This causes problems because
    we now store the ramps object in the theme so that we can pull colors out of them.
    So instead of calling domain and storing the result, we have to construct new ramps for each
    theme so that we don't modify the passed in ramps.
    This combined with an error in the type definitions for chroma js means we have to cast the colors
    function to any in order to get the colors back out from the original ramps.
 * @param is_light
 * @param color_ramps
 * @returns
 */
export function get_ramps(
    is_light: boolean,
    color_ramps: ThemeConfigInputColors
): RampSet {
    const ramps: RampSet = {} as any // eslint-disable-line @typescript-eslint/no-explicit-any
    const color_keys = Object.keys(color_ramps) as ThemeConfigInputColorsKeys[]

    if (is_light) {
        for (const ramp_name of color_keys) {
            ramps[ramp_name] = chroma.scale(
                color_ramps[ramp_name].colors(100).reverse()
            )
        }
        ramps.neutral = chroma.scale(color_ramps.neutral.colors(100).reverse())
    } else {
        for (const ramp_name of color_keys) {
            ramps[ramp_name] = chroma.scale(color_ramps[ramp_name].colors(100))
        }
        ramps.neutral = chroma.scale(color_ramps.neutral.colors(100))
    }

    return ramps
}
