import { useColors } from "./colors"
import { Theme } from "./config"
import { ElementIntensities, useElementIntensities } from "./intensity"

type BorderStyle = "solid" | "dashed" | "dotted" | "double" | "wavy"

export interface Border {
    width: number
    color: string
    style: BorderStyle
    inset: boolean
}

export type BorderOptions = Partial<Border>

export function border(
    theme: Theme,
    intensity: ElementIntensities,
    options?: BorderOptions
): Border {
    const color = useColors(theme)

    const resolvedIntensity = useElementIntensities(theme, intensity)

    const border: Border = {
        width: 1,
        color: color.neutral(resolvedIntensity.border),
        style: "solid",
        inset: false,
        ...options,
    }

    return border
}
