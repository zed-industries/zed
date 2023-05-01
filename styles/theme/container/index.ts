import { Border, Theme } from "@/theme"
import {
    ElementIntensities,
    Intensity,
    calculateIntensity,
    useElementIntensities,
} from "../intensity"
import { Padding, Margin } from "@theme/properties"
import { ContainedText, ContainedTextProps, containedText } from "./containedText"
import { FlexStyle } from "@theme/element/flex"
import { IconStyle } from "@theme/icon"

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

export enum BorderRadius {
    "Medium" = 4,
}

export interface ContainedIcon {
    container: ContainerStyle
    icon: IconStyle
}

export interface ContainedTextWithIcon extends ContainedText {
    icon: IconStyle
}

export type InteractiveState =
    | FlexStyle
    | ContainerStyle
    | ContainedIcon
    | ContainedText
    | ContainedTextWithIcon

export interface Interactive<T = InteractiveState> {
    default: T
    hovered: T
    pressed: T
    dragged?: T
}

export interface InteractiveToggleableContainer<T = Interactive> {
    inactive: T
    active: T
}

export type State = "default" | "hovered" | "pressed"

type ContainerColors = {
    bg: Intensity
    border: Intensity
    fg: Intensity
}

export type StateIntensity = ContainerColors
export type StateIntensities = Record<State, StateIntensity>

export function buildIntensitiesForStates(
    theme: Theme,
    name: string,
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
        default: buildStateIntensity(name, "default", defaultState),
        hovered: buildStateIntensity(name, "hovered", defaultState, scaledIntensitySteps[1]),
        pressed: buildStateIntensity(name, "pressed", defaultState, scaledIntensitySteps[2]),
    }

    return elementStates
}

export function buildStateIntensity(
    componentName: string,
    name: string,
    startingIntensity: StateIntensity,
    change?: number
): StateIntensity {
    if (!change) {
        return startingIntensity
    }

    const stateIntensity: StateIntensity = {
        bg: calculateIntensity(startingIntensity.bg, change),
        border: calculateIntensity(startingIntensity.border, change),
        fg: calculateIntensity(startingIntensity.fg, change),
    }

    const nameForCheck = `${componentName} ${name}`

    checkContrast(nameForCheck, startingIntensity.bg, stateIntensity.fg)

    return stateIntensity
}

export const checkContrast = (
    name: string,
    background: Intensity,
    foreground: Intensity,
    debug?: boolean
) => {
    const foregroundIntensity = Math.max(foreground, background) + 0.05
    const backgroundIntensity = Math.min(foreground, background) + 0.05
    const contrastRatio = foregroundIntensity / backgroundIntensity

    // Return a contrast with 2 decimal places
    const contrast = +contrastRatio.toFixed(2)

    debug && console.log(`Contrast on ${name}: ${contrast}. Foreground: ${foreground}, Background: ${background}`)

    if (contrast < 4.5) {
        console.log(`Constrast on ${name} may be too low: ${contrast}`)
    }

    if (contrast < 3) {
        throw new Error(`Constrast on ${name} is too low: ${contrast}`)
    }
}

export { ContainedText, ContainedTextProps, containedText }
