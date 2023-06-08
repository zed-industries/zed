import chroma from "chroma-js"
export * from "./theme"
export { chroma }

export const fontFamilies = {
    sans: "Zed Sans",
    mono: "Zed Mono",
}

export const fontSizes = {
    "3xs": 8,
    "2xs": 10,
    xs: 12,
    sm: 14,
    md: 16,
    lg: 18,
    xl: 20,
}

export type FontWeight =
    | "thin"
    | "extra_light"
    | "light"
    | "normal"
    | "medium"
    | "semibold"
    | "bold"
    | "extra_bold"
    | "black"

export const fontWeights: { [key: string]: FontWeight } = {
    thin: "thin",
    extra_light: "extra_light",
    light: "light",
    normal: "normal",
    medium: "medium",
    semibold: "semibold",
    bold: "bold",
    extra_bold: "extra_bold",
    black: "black",
}

export const sizes = {
    px: 1,
    xs: 2,
    sm: 4,
    md: 6,
    lg: 8,
    xl: 12,
}
