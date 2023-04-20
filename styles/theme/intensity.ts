import chroma from "chroma-js"
import { Theme, ThemeConfig } from "./config"

export function hexToIntensity(hex: string): Intensity {
    const hsl = chroma(hex).hsl()

    // Round intensity up so that we never end up with a value of 0
    const intensity = Math.ceil(hsl[2] * 100)

    if (intensity < 1 || intensity > 100)
        throw new Error(
            `Intensity ${intensity} out of range. Intensity must be between 1 and 100`
        )

    return intensity as Intensity
}

export function numberToIntensity(number: number): Intensity {
    const i = Math.ceil(Math.min(Math.max(number, 1), 100))

    if (i < 1 || i > 100)
        throw new Error(
            `Intensity ${i} out of range. Intensity must be between 1 and 100`
        )

    return i as Intensity
}

// Dumb but it works
// prettier-ignore
export type Intensity = | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34 | 35 | 36 | 37 | 38 | 39 | 40 | 41 | 42 | 43 | 44 | 45 | 46 | 47 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 58 | 59 | 60 | 61 | 62 | 63 | 64 | 65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 | 91 | 92 | 93 | 94 | 95 | 96 | 97 | 98 | 99 | 100

interface IntensityRange {
    min: Intensity
    max: Intensity
    scaleFactor: number
}

function checkIntensity(number: number | Intensity): Intensity {
    let intensity: Intensity

    if (typeof number === "number") {
        intensity = numberToIntensity(number)
    } else {
        intensity = number
    }

    if (intensity < 1 || intensity > 100) {
        throw new Error(
            `Intensity ${intensity} out of range. Intensity must be between 1 and 100`
        )
    }

    return intensity
}

export function addToIntensity(
    startingIntensity: Intensity,
    intensityToAdd: Intensity
): Intensity {
    checkIntensity(startingIntensity)
    checkIntensity(intensityToAdd)

    let newIntensity = startingIntensity + intensityToAdd

    // Bounce back if we're out of range
    if (newIntensity > 100) {
        newIntensity = startingIntensity - intensityToAdd
    } else if (newIntensity < 1) {
        newIntensity = startingIntensity + Math.abs(intensityToAdd)
    }

    return checkIntensity(newIntensity)
}

export function addToElementIntensities(
    startingIntensity: ElementIntensities<Intensity>,
    intensityToAdd: Intensity
): ElementIntensities<Intensity> {
    return {
        bg: addToIntensity(startingIntensity.bg, intensityToAdd),
        border: addToIntensity(startingIntensity.border, intensityToAdd),
        fg: addToIntensity(startingIntensity.fg, intensityToAdd),
    }
}

export function buildThemeIntensity(themeConfig: ThemeConfig): IntensityRange {
    const neutral = themeConfig.colors.neutral
    const appearance = themeConfig.appearance // "light" or "dark"

    if (appearance === "light" && Array.isArray(neutral)) {
        neutral.reverse()
    }

    const firstColor = neutral[0]
    const lastColor = neutral[neutral.length - 1]

    let minIntensity = hexToIntensity(chroma(firstColor).hex())
    let maxIntensity = hexToIntensity(chroma(lastColor).hex())

    if (appearance === "light") {
        ;[minIntensity, maxIntensity] = [maxIntensity, minIntensity]
    }

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
        throw new Error(
            `${themeConfig.name}: Min intensity must be less than max intensity`
        )
    }

    const intensity: IntensityRange = {
        min: minIntensity,
        max: maxIntensity,
        scaleFactor: calculateScaleFactor(minIntensity, maxIntensity),
    }

    return intensity
}

function calculateScaleFactor(min: number, max: number): number {
    const smallerScaleDifference = Math.abs(max - min)
    const maxDistance = 99
    const scaleFactor = maxDistance / smallerScaleDifference
    return +scaleFactor.toFixed(3)
}

/**
 * Single intensity = same for light and dark
 *
 * Array = [dark intensity, light intensity]
 */
export type ElementIntensity = Intensity | [Intensity, Intensity]

export interface ElementIntensities<T = ElementIntensity> {
    bg: T
    border: T
    fg: T
}

/** Resolves ElementIntensity down to a single Intensity per property based on the theme's appearance
 *
 * If two intensities are provided, the first is used for dark appearance and the second for light appearance
 *
 * If one intensity is provided, it is used for both dark and light appearance
 */
export function useElementIntensities(
    theme: Theme,
    intensity: ElementIntensities<ElementIntensity>
): ElementIntensities<Intensity> {
    const elementIntensities: ElementIntensities<Intensity> = {
        bg: Array.isArray(intensity.bg)
            ? theme.appearance === "light"
                ? intensity.bg[1]
                : intensity.bg[0]
            : intensity.bg,
        border: Array.isArray(intensity.border)
            ? theme.appearance === "light"
                ? intensity.border[1]
                : intensity.border[0]
            : intensity.border,
        fg: Array.isArray(intensity.fg)
            ? theme.appearance === "light"
                ? intensity.fg[1]
                : intensity.fg[0]
            : intensity.fg,
    }

    return { ...intensity, ...elementIntensities }
}

export const calculateIntensity = (
    intensity: number,
    change: number
): Intensity => {
    let newIntensity = intensity + change
    if (newIntensity > 100) {
        // If the new intensity is too high, change the direction and use the same change value
        newIntensity = intensity - change
    }

    const finalIntensity = numberToIntensity(newIntensity)

    return finalIntensity
}
