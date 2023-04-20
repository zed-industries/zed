import { Border, Theme } from "@/theme"
import { TextStyle } from "@theme/text"
import {
    ElementIntensities,
    Intensity,
    calculateIntensity,
    useElementIntensities,
} from "../intensity"
import { Padding, Margin } from "@theme/properties"

export interface ContainerStyle {
    background?: string
    margin?: Margin
    padding?: Padding
    borderRadius?: number
    border?: Border
    width: number | "auto"
    height: number | "auto"
}

const blankContainer: ContainerStyle = {
    width: "auto",
    height: "auto",
}

export const container: Record<string, ContainerStyle> = {
    blank: blankContainer,
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

export interface InteractiveToggleableContainer<T = InteractiveContainer> {
    inactive: T
    active: T
}

type State = "default" | "hovered" | "pressed"

type ContainerColors = {
    bg: Intensity
    border: Intensity
    fg: Intensity
}

export type StateIntensity = ContainerColors
export type StateIntensities = Record<State, StateIntensity>

export function buildStates(
    theme: Theme,
    startingIntensity: ElementIntensities,
): StateIntensities {
    const light = theme.appearance === "light"
    const multiplier = light ? 1 : 1.2;
    const stepSize = 5;
    const startingOffset = light ? 5 : 12;
    const intensitySteps = [0, 1, 2, 3].map(step => multiplier * stepSize * step + startingOffset);

    const scaleFactor = theme.intensity.scaleFactor

    const scaledIntensitySteps = intensitySteps.map(
        (intensity) => intensity * scaleFactor
    )

    const resolvedIntensity = useElementIntensities(theme, startingIntensity)

    const defaultState: StateIntensity = {
        bg: resolvedIntensity.bg,
        border: resolvedIntensity.border,
        fg: resolvedIntensity.fg,
    }

    const elementStates = {
        default: defaultState,
        hovered: buildState(defaultState, scaledIntensitySteps[1]),
        pressed: buildState(defaultState, scaledIntensitySteps[2]),
    }

    return elementStates
}

export function buildState(
    startingIntensity: StateIntensity,
    change: number
): StateIntensity {
    const stateIntensity: StateIntensity = {
        bg: calculateIntensity(startingIntensity.bg, change),
        border: calculateIntensity(startingIntensity.border, change),
        fg: calculateIntensity(startingIntensity.fg, change),
    }

    return stateIntensity
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
