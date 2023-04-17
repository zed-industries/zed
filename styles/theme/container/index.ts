import { Border, Theme } from "@/theme"
import { TextStyle } from "@/theme/font"
import { Intensity, numberToIntensity } from "../intensity"

export type Margin = [number, number, number, number]
export type Padding = [number, number, number, number]

export interface ContainerStyle {
    background: string
    margin: Margin
    padding: Padding
    borderRadius: number
    border: Border
    width: number
    height: number
}

export enum IconSize {
    "Small" = 7,
    "Medium" = 11,
    "Large" = 15,
}

export enum BorderRadius {
    "Medium" = 4,
}

export interface IconStyle {
    color: string
    size: IconSize
}

export interface ContainedText {
    container: ContainerStyle
    text: TextStyle
}

export interface ContainedIcon {
    container: ContainerStyle
    icon: IconStyle
}

export interface ContainedTextWithIcon extends ContainedText {
    icon: IconStyle
}

export type InteractiveState =
    | ContainedIcon
    | ContainedText
    | ContainedTextWithIcon

export interface InteractiveContainer<T = InteractiveState> {
    default: T
    hovered: T
    pressed: T
}

export interface StateIntensities {
    default: Intensity
    hovered: Intensity
    pressed: Intensity
    active: Intensity
}

export function buildStateIntensities(
    theme: Theme,
    baseIntensity: number,
    scaleFactor: number
): StateIntensities {
    const isLightTheme = theme.appearance === "light"
    const intensitySteps = isLightTheme ? [0, 5, 10, 15] : [0, 12, 18, 24]
    const defaultIntensity = numberToIntensity(baseIntensity)

    const scaledIntensitySteps = intensitySteps.map(
        (intensity) => intensity * scaleFactor
    )

    const calculateIntensity = (
        intensity: number,
        change: number
    ): Intensity => {
        let newIntensity = intensity + change
        if (newIntensity > 100) {
            // If the new intensity is too high, change the direction and use the same change value
            newIntensity = intensity - change
        }

        // Round the ouput to ensure it is a valid intensity
        const finalIntensity = Math.ceil(
            Math.min(Math.max(newIntensity, 1), 100)
        )

        return numberToIntensity(finalIntensity)
    }

    const stateIntensities: StateIntensities = {
        default: defaultIntensity,
        hovered: calculateIntensity(defaultIntensity, scaledIntensitySteps[1]),
        pressed: calculateIntensity(defaultIntensity, scaledIntensitySteps[2]),
        active: calculateIntensity(defaultIntensity, scaledIntensitySteps[3]),
    }

    return stateIntensities
}

export const checkContrast = (
    name: string,
    background: Intensity,
    foreground: Intensity
) => {
    const contrast = foreground / background

    if (contrast < 4.5) {
        console.log(`Constrast on ${name} may be too low: ${contrast}`)
    }

    if (contrast < 3) {
        throw new Error(`Constrast on ${name} is too low: ${contrast}`)
    }
}
