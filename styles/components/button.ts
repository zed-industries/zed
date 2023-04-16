import { Border, Intensity, Theme, useColors } from "@/theme"
import { numberToIntensity } from "@/theme/intensity"

type Margin = [number, number, number, number]
type Padding = [number, number, number, number]

interface ContainerStyle {
    background: string
    margin: Margin
    padding: Padding
    borderRadius: number
    border: Border
}

enum IconSize {
    "Small" = 7,
    "Medium" = 11,
    "Large" = 15,
}

enum BorderRadius {
    "Medium" = 4,
}

interface TextStyle {
    family: string
    size: number
    weight: number
    color: string
    lineHeight: number
}

interface IconStyle {
    color: string
    size: IconSize
}

interface ContainedText {
    container: ContainerStyle
    text: TextStyle
}

interface ContainedIcon {
    container: ContainerStyle
    icon: IconStyle
}

interface ContainedTextWithIcon extends ContainedText {
    icon: IconStyle
}

type InteractiveState = ContainedIcon | ContainedText | ContainedTextWithIcon

interface InteractiveContainer<T = InteractiveState> {
    default: T
    hovered: T
    pressed: T
}

interface ToggleableInteractiveContainer {
    inactive: InteractiveContainer
    active: InteractiveContainer
}

interface StateIntensities {
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

    console.log(JSON.stringify(stateIntensities, null, 4))

    return stateIntensities
}

const checkContrast = (
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

export function buttonWithIconStyle(
    theme: Theme
): InteractiveContainer<ContainedIcon> {
    const color = useColors(theme)
    const bgIntensity = buildStateIntensities(
        theme,
        12,
        theme.intensity.scaleFactor
    )
    const borderIntensity = buildStateIntensities(
        theme,
        theme.appearance === "light" ? 36 : 24,
        theme.intensity.scaleFactor
    )
    const fgIntensity = buildStateIntensities(
        theme,
        100,
        theme.intensity.scaleFactor
    )

    checkContrast(
        "buttonWithIconStyle",
        bgIntensity.default,
        fgIntensity.default
    )

    const button = (state: keyof StateIntensities): ContainedIcon => {
        return {
            container: {
                background: color.neutral(bgIntensity[state]),
                margin: [0, 0, 0, 0],
                padding: [4, 4, 4, 4],
                borderRadius: BorderRadius.Medium,
                border: {
                    width: 1,
                    color: color.neutral(borderIntensity[state]),
                    style: "solid",
                    inset: false,
                },
            },
            icon: {
                color: color.neutral(fgIntensity[state]),
                size: IconSize.Medium,
            },
        }
    }

    return {
        default: button("default"),
        hovered: button("hovered"),
        pressed: button("pressed"),
    }
}
