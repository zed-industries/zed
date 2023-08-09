import chroma from "chroma-js"
export * from "./theme"
export { chroma }

export const font_families = {
    ui_sans: "IBM Plex Sans",
    sans: "Zed Sans",
    mono: "Zed Mono",
}

export const font_sizes = {
    "2xs": 10,
    xs: 12,
    sm: 14,
    md: 16,
    lg: 18,
}

export type FontWeight = "normal" | "bold"

export const font_weights: { [key: string]: FontWeight } = {
    normal: "normal",
    bold: "bold",
}

export const sizes = {
    px: 1,
    xs: 2,
    sm: 4,
    md: 6,
    lg: 8,
    xl: 12,
}
