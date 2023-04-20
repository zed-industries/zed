type Font = "Zed Mono" | "Zed Sans"

export interface Families {
    mono: Font
    sans: Font
    ui: Font
    terminal: Font
}

export const family: Families = {
    mono: "Zed Mono",
    sans: "Zed Sans",
    ui: "Zed Sans",
    terminal: "Zed Mono",
}

export interface Sizes {
    xs: number
    sm: number
    md: number
    lg: number
    xl: number
}

export const size: Sizes = {
    xs: 0.75,
    sm: 0.875,
    md: 1,
    lg: 1.125,
    xl: 1.25,
}

export type Weight = 400 | 700

export interface Weights {
    regular: Weight
    bold: Weight
}

export const weight: Weights = {
    regular: 400,
    bold: 700,
}

export interface TextStyle {
    family: Font
    size: number
    weight: Weight
    color: string
    lineHeight: number
}

const textDefaults = {
    family: family.sans,
    size: size.md,
    weight: weight.regular,
    lineHeight: 1,
}

export function useText(
    color: string,
    family: Font = textDefaults.family,
    size: number = textDefaults.size,
    weight: Weight = textDefaults.weight,
    lineHeight: number = textDefaults.lineHeight,
): TextStyle {
    return {
        family,
        size,
        weight,
        color,
        lineHeight,
    }
}
