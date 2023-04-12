import chroma from "chroma-js"
import { ThemeSyntax } from "@/src/themes/common/syntax"

interface Author {
    name: string
    email: string
    handle: string
}

type License = "MIT" | "Apache-2.0" | "GPL-3.0" | "Unlicense"

enum Appearance {
    Dark,
    Light,
}

export type InputColor = string | string[]

interface RequiredThemeProperties {
    name: string
    appearance: Appearance
    author: string | Author
    license: License
    colors: {
        neutral: InputColor
        accent: InputColor
        error: InputColor
        info: InputColor
        warning: InputColor
        success: InputColor
    }
}

interface OptionalThemeProperties {
    url: string
    syntax: ThemeSyntax
}

interface CalculatedThemeProperties {
    intensity: {
        min: number
        max: number
    }
}

export type ThemeConfig = RequiredThemeProperties &
    Partial<OptionalThemeProperties>
export type Theme = ThemeConfig & CalculatedThemeProperties

const lightThemeConfig: ThemeConfig = {
    appearance: "light",
    calculatedIntensity: {
        min: 5,
        max: 88,
    },
}
const darkThemeConfig: ThemeConfig = {
    appearance: "dark",
    calculatedIntensity: {
        min: 1,
        max: 92,
    },
}

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

export function normalizeIntensity(themeConfig: ThemeConfig): ThemeConfig {
    const normalizedIntensity = {
        min: (themeConfig.calculatedIntensity.min / 100) * 100,
        max: (themeConfig.calculatedIntensity.max / 100) * 100,
    }

    return {
        ...themeConfig,
        calculatedIntensity: normalizedIntensity,
    }
}

interface StateIntensities {
    default: number
    hovered: number
    pressed: number
    active: number
}

function buildStateIntensities(
    themeConfig: ThemeConfig,
    baseIntensity: number
): StateIntensities {
    const isLightTheme = themeConfig.appearance === "light"
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

console.log(buildStateIntensities(lightThemeConfig, 50))
// { default: 50, hovered: 53, pressed: 56, active: 59 }

console.log(buildStateIntensities(darkThemeConfig, 50))
// { default: 50, hovered: 65, pressed: 70, active: 75 }

interface Button {
    background: StateIntensities
    label: {
        text: string
        color: StateIntensities
    }
    icon: {
        intensity: StateIntensities
    }
}

function contrastRatio(intensity1: number, intensity2: number): number {
    const [intensityLighter, intensityDarker] =
        intensity1 > intensity2
            ? [intensity1, intensity2]
            : [intensity2, intensity1]
    return (intensityLighter + 0.5) / (intensityDarker + 0.5)
}

function hasSufficientContrast(
    intensity1: number,
    intensity2: number,
    minContrast: number
): boolean {
    return contrastRatio(intensity1, intensity2) >= minContrast
}

function createButton(
    themeConfig: ThemeConfig,
    labelText: string,
    backgroundIntensity: number,
    labelIntensity: number,
    iconIntensity: number
): Button | null {
    const backgroundStates = buildStateIntensities(
        themeConfig,
        backgroundIntensity
    )
    const labelStates = buildStateIntensities(themeConfig, labelIntensity)
    const iconStates = buildStateIntensities(themeConfig, iconIntensity)

    // Ensure sufficient contrast for all states
    const minContrast = 3
    const states = ["default", "hovered", "pressed", "active"] as const
    for (const state of states) {
        if (
            !hasSufficientContrast(
                backgroundStates[state],
                labelStates[state],
                minContrast
            ) ||
            !hasSufficientContrast(
                backgroundStates[state],
                iconStates[state],
                minContrast
            )
        ) {
            console.warn(
                `Insufficient contrast for state "${state}". Please adjust intensities.`
            )
            return null
        }
    }

    const button: Button = {
        background: backgroundStates,
        label: {
            text: labelText,
            color: labelStates,
        },
        icon: {
            intensity: iconStates,
        },
    }

    return button
}

const lightButton = createButton(lightThemeConfig, "Click me!", 50, 100, 100)
console.log(lightButton)
// {
//   background: { default: 50, hovered: 53, pressed: 56, active: 59 },
//   label: { text: 'Click me!', color: { default: 100, hovered: 100, pressed: 100, active: 100 } },
//   icon: { intensity: { default: 100, hovered: 100, pressed: 100, active: 100 } }
// }

const darkButton = createButton(darkThemeConfig, "Click me!", 50, 1, 1)
console.log(darkButton)
// {
//   background: { default: 50, hovered: 65, pressed: 70, active: 75 },
//   label: { text: 'Click me!', color: { default: 1, hovered: 1, pressed: 1, active: 1 } },
//   icon: { intensity: { default: 1, hovered: 1, pressed: 1, active: 1 } }
// }
