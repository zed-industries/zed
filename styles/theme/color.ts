import chroma from "chroma-js"
import { Theme, ThemeConfig } from "./config"
import { Intensity } from "./intensity"

export type Color = chroma.Color
export type Scale = chroma.Scale
export type Scales = Record<keyof ThemeConfig["colors"], Scale>

type UIScales = Record<keyof ThemeConfig["colors"], string[]>

export { chroma }

function buildScaleFromSingleColor(color: Color): Scale {
    // TODO: Do something to generate a ramp from a single color
    const scale = chroma.scale([
        color.darken(1),
        color.darken(0.5),
        color,
        color.brighten(0.5),
        color.brighten(1),
    ])
    return scale
}

export function buildThemeScales(themeConfig: ThemeConfig): UIScales {
    const scales: Scales = {} as Scales
    for (const [colorName, colorValue] of Object.entries(themeConfig.colors)) {
        const name = colorName as keyof ThemeConfig["colors"]

        scales[name] = Array.isArray(colorValue)
            ? chroma.scale(colorValue)
            : buildScaleFromSingleColor(chroma(colorValue))
    }

    const scaleArrays: UIScales = {} as UIScales

    for (const [colorName, scale] of Object.entries(scales)) {
        const name = colorName as keyof ThemeConfig["colors"]
        scaleArrays[name] = scale.colors(100)
    }

    return scaleArrays
}

export type UIColors = Record<keyof Theme["color"], Intensity>
export type UIColor = keyof UIColors

export function useIntensityColor(
    theme: Theme,
    color_family: UIColor,
    intensity: Intensity
): string {
    if (intensity < 1 || intensity > 100) {
        throw new Error(
            `useIntensityColor: Intensity must be between 1 and 100, received ${intensity}`
        )
    }

    // Adjust intensity to be 0-indexed
    const adjusted_intensity = intensity - 1
    const scale = theme.color[color_family]
    const c = scale[adjusted_intensity]
    return c
}
