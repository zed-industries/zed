import { useColors } from "./colors"
import * as color from "./color"
import { Theme } from "./config"
import { Intensity } from "./intensity"

type BorderStyle = "solid" | "dashed" | "dotted" | "double" | "wavy"

// TODO: Update borders in Rust to allow strokw width per-side
// TODO: Update borders in Rust to allow setting the border style from the theme
export interface BorderOptions {
    /** A color family from the Theme */
    color: keyof color.Scales
    width: number
    style: BorderStyle
    inset: boolean
    side: "all" | "top" | "bottom" | "left" | "right"
}

interface Border {
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
    side: "all",
    inset: false,
}

export function border(
    theme: Theme,
    intensity: Intensity,
    options?: Partial<BorderOptions>,
): Border {
    const themeColor = useColors(theme)

    const mergedOptions = {
        ...DEFAULT_BORDER_OPTIONS,
        ...options,
    }

    const side = {
        top: mergedOptions.side === "all" || mergedOptions.side === "top",
        bottom: mergedOptions.side === "all" || mergedOptions.side === "bottom",
        left: mergedOptions.side === "all" || mergedOptions.side === "left",
        right: mergedOptions.side === "all" || mergedOptions.side === "right",
    }

    const color = options.color ? themeColor[options.color](intensity) : themeColor.neutral(intensity)

    const border: Border = {
        color: color,
        width: mergedOptions.width,
        overlay: mergedOptions.inset,
        ...side
    }

    return border
}
