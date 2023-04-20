import { useColors } from "./colors"
import { Theme } from "./config"
import { Intensity } from "./intensity"

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
    intensity: Intensity,
    options?: BorderOptions
): Border {
    const color = useColors(theme)

    const border: Border = {
        width: 1,
        color: color.neutral(intensity),
        style: "solid",
        inset: false,
        ...options,
    }

    return border
}
