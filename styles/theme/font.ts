export const family = {
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
