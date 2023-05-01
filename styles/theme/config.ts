import { InputSyntax } from "@/theme/syntax"

interface Author {
    name: string
    email?: string
    handle?: string
}

type License = "MIT" | "Apache-2.0" | "GPL-3.0" | "Unlicense"

export type InputColor = string | string[]

export type ThemeColor = "neutral" | "accent" | "error" | "info" | "warning" | "success"

type ThemeConfigColors = Record<ThemeColor, InputColor>
export type ThemeColors = Record<ThemeColor, string[]>

export interface ThemeConfig {
    name: string
    appearance: "light" | "dark"
    author: string | Author
    url?: string
    license: License
    colors: ThemeConfigColors
    syntax?: Partial<InputSyntax>
}

export interface CalculatedThemeProperties {
    intensity: {
        min: number
        max: number
        scaleFactor: number
    }
    color: ThemeColors
}

export type Theme = ThemeConfig & CalculatedThemeProperties
