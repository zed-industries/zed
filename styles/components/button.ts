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
    const isLightTheme = theme.appearance === "light";
    const intensitySteps = isLightTheme ? [0, 3, 6, 9] : [0, 15, 20, 25];
    const defaultIntensity = numberToIntensity(baseIntensity);

    const scaledIntensitySteps = intensitySteps.map(
        (intensity) => intensity * scaleFactor
    );

    const calculateIntensity = (intensity: number, change: number): Intensity => {
        let newIntensity = intensity + change;
        if (newIntensity > 100) {
            // If the new intensity is too high, change the direction and use the same change value
            newIntensity = intensity - change;
        }
        return numberToIntensity(Math.min(Math.max(newIntensity, 1), 100));
    };

    const stateIntensities: StateIntensities = {
        default: defaultIntensity,
        hovered: calculateIntensity(defaultIntensity, scaledIntensitySteps[1]),
        pressed: calculateIntensity(defaultIntensity, scaledIntensitySteps[2]),
        active: calculateIntensity(defaultIntensity, scaledIntensitySteps[3]),
    };

    return stateIntensities;
}

export function buttonWithIconStyle(theme: Theme): InteractiveContainer<ContainedIcon> {
    const color = useColors(theme)
    const bgIntensity = buildStateIntensities(theme, 26, theme.intensity.scaleFactor);
    const borderIntensity = buildStateIntensities(theme, 32, theme.intensity.scaleFactor);
    const fgIntensity = buildStateIntensities(theme, 100, theme.intensity.scaleFactor);

    const button = (
        state: keyof StateIntensities,
    ) => {
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
            }
        }
    }

    return {
        default: button("default"),
        hovered: button("hovered"),
        pressed: button("pressed"),
    }
}
