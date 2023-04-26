import { useIntensityColor } from "./color"
import { Theme } from "./config"
import { Intensity } from "./intensity"

function getColor(
    theme: Theme,
    colorKey: keyof Theme["colors"],
    intensity: Intensity
): string {
    return useIntensityColor(theme, colorKey, intensity)
}

interface ColorFunctions {
    [colorKey: string]: (intensity: Intensity) => string
}

export function useColors(theme: Theme): ColorFunctions {
    const functions: ColorFunctions = {}
    for (const colorKey in theme.colors) {
        const key = colorKey as keyof Theme["colors"]
        functions[key] = (intensity: Intensity) =>
            getColor(theme, key, intensity)
    }
    return functions
}
