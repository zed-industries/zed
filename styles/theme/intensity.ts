import chroma from "chroma-js"
import { Theme, ThemeConfig } from "./config"

export function hexToIntensity(hex: string): number {
    const hsl = chroma(hex).hsl()

    // Round intensity up so that we never end up with a value of 0
    const intensity = Math.ceil(hsl[2] * 100)
    return intensity
}

interface Intensity {
    min: number
    max: number
}

export function buildThemeIntensity(themeConfig: ThemeConfig): Intensity {
    const neutral = themeConfig.colors.neutral
    const appearance = themeConfig.appearance // "light" or "dark"

    if (appearance === 'light' && Array.isArray(neutral)) {
        neutral.reverse()
    }

    let firstColor = neutral[0]
    let lastColor = neutral[neutral.length - 1]

    let minIntensity = hexToIntensity(chroma(firstColor).hex())
    let maxIntensity = hexToIntensity(chroma(lastColor).hex())

    if (appearance === 'light') {
        [minIntensity, maxIntensity] = [maxIntensity, minIntensity]
    }

    console.log('firstColor:', firstColor)
    console.log('lastColor:', lastColor)
    console.log('minIntensity:', minIntensity)
    console.log('maxIntensity:', maxIntensity)

    if (minIntensity < 1) {
        throw new Error(
            `Intensity ${minIntensity} too low. Intensity must be between 1 and 100`
        )
    }

    if (maxIntensity > 100) {
        throw new Error(
            `Intensity ${maxIntensity} too high. Intensity must be between 1 and 100`
        )
    }

    if (minIntensity > maxIntensity) {
        throw new Error("Min intensity must be less than max intensity")
    }

    const intensity: Intensity = {
        min: minIntensity,
        max: maxIntensity,
    }

    return intensity
}


export function normalizeIntensity(theme: Theme): Theme {
    const normalizedIntensity = {
        min: (theme.intensity.min / 100) * 100,
        max: (theme.intensity.max / 100) * 100,
    }

    return {
        ...theme,
        intensity: normalizedIntensity,
    }
}

interface StateIntensities {
    default: number
    hovered: number
    pressed: number
    active: number
}

function buildStateIntensities(
    theme: Theme,
    baseIntensity: number
): StateIntensities {
    const isLightTheme = theme.appearance === "light"
    const intensitySteps = isLightTheme ? [0, 3, 6, 9] : [0, 15, 20, 25]

    const calculateIntensity = (intensity: number, change: number): number => {
        let newIntensity = intensity + change
        if (newIntensity > 100) {
            // If the new intensity is too high, change the direction and use the same change value
            newIntensity = intensity - change
        }
        return Math.min(Math.max(newIntensity, 1), 100)
    }

    const stateIntensities: StateIntensities = {
        default: baseIntensity,
        hovered: calculateIntensity(baseIntensity, intensitySteps[1]),
        pressed: calculateIntensity(baseIntensity, intensitySteps[2]),
        active: calculateIntensity(baseIntensity, intensitySteps[3]),
    }

    return stateIntensities
}
