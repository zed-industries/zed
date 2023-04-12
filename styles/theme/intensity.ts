import chroma from "chroma-js"
import { Theme, ThemeConfig } from "./config"

export function hexToIntensity(hex: string): number {
    const hsl = chroma(hex).hsl()
    const intensity = hsl[2] * 100
    return intensity
}

export function rgbToIntensity(rgb: string): number {
    const hsl = chroma(rgb).hsl()
    const intensity = hsl[2] * 100
    return intensity
}

export function hslToIntensity(hsl: string): number {
    const hslArray = chroma(hsl).hsl()
    const intensity = hslArray[2] * 100
    return intensity
}

export function hsbToIntensity(hsb: string): number {
    const hsbArray = hsb.match(/\d+/g).map(Number)
    const hsl = chroma.hsv(hsbArray[0], hsbArray[1], hsbArray[2]).hsl()
    const intensity = hsl[2] * 100
    return intensity
}

interface Intensity {
    min: number
    max: number
}

export function buildThemeIntensity(themeConfig: ThemeConfig): Intensity {
    const neutral = themeConfig.colors.neutral;

    const [firstColor, lastColor] = [neutral[0], neutral[neutral.length - 1]];
    const minIntensity = hexToIntensity(chroma(firstColor).hex());
    const maxIntensity = hexToIntensity(chroma(lastColor).hex());

    if (minIntensity < 1 || maxIntensity > 100) {
        throw new Error("Intensity must be between 1 and 100");
    }

    if (minIntensity > maxIntensity) {
        throw new Error("Min intensity must be less than max intensity");
    }

    if (maxIntensity - maxIntensity > 50) {
        throw new Error("Not enough contrast between lightest and darkest colors");
    }

    const intensity: Intensity = {
        min: minIntensity,
        max: maxIntensity,
    };

    return intensity;
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
