import { useColors } from "./colors"
import * as color from "./color"
import { Theme } from "./config"
import { Intensity, resolveThemeColorIntensity } from "./intensity"

type BorderStyle = "solid" | "dashed" | "dotted" | "double" | "wavy"

// TODO: Update borders in Rust to allow strokw width per-side
// TODO: Update borders in Rust to allow setting the border style from the theme
export interface BorderOptions {
    /** A color family from the Theme */
    color: keyof color.Scales
    width: number
    style: BorderStyle
    inset: boolean
    position: "all" | "top" | "bottom" | "left" | "right"
}

export interface Border {
    color: string
    width: number
    top?: boolean
    bottom?: boolean
    left?: boolean
    right?: boolean
    // TODO: Rename overlay -> inset in Rust to align with more common terminology
    // Until then we remap the name in the border function
    overlay?: boolean
}

const DEFAULT_BORDER_OPTIONS: Partial<BorderOptions> = {
    width: 1,
    style: "solid",
    position: "all",
    inset: false,
}

const DEFAULT_BORDER_INTENSITY: Intensity = 100

export function border(
    theme: Theme,
    intensity: Intensity,
    options?: Partial<BorderOptions>,
): Border {
    if (!intensity) {
        intensity = DEFAULT_BORDER_INTENSITY
    }

    const themeColor = useColors(theme)
    const resolvedColorIntensity = resolveThemeColorIntensity(theme, intensity)
    const DEFAULT_COLOR = themeColor.neutral(resolvedColorIntensity)

    const mergedOptions = {
        ...DEFAULT_BORDER_OPTIONS,
        ...options,
    }

    // If options are provided, we use the provided color intensity
    // Otherwise we use the default color intensity
    const color = (options?.color && themeColor[options.color](intensity)) || DEFAULT_COLOR;

    const position = {
        top: mergedOptions.position === "all" || mergedOptions.position === "top",
        bottom: mergedOptions.position === "all" || mergedOptions.position === "bottom",
        left: mergedOptions.position === "all" || mergedOptions.position === "left",
        right: mergedOptions.position === "all" || mergedOptions.position === "right",
    }

    const border: Border = {
        color: color,
        width: mergedOptions.width,
        overlay: mergedOptions.inset,
        ...position
    }

    return border
}
