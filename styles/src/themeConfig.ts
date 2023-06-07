import { Scale, Color } from "chroma-js"
import { Syntax } from "./themes/common/syntax"

interface ThemeMeta {
    /** The name of the theme */
    name: string
    /** The theme's appearance. Either `light` or `dark`. */
    appearance: ThemeAppearance
    /** The author of the theme
     *
     * Ideally formatted as `Full Name <email>`
     *
     * Example: `John Doe <john@doe.com>`
     */
    author: string
    /** SPDX License string
     *
     * Example: `MIT`
     */
    licenseType?: string | ThemeLicenseType
    licenseUrl?: string
    licenseFile: string
    themeUrl?: string
}

export interface ThemeConfigInputColors {
    neutral: Scale<Color>
    red: Scale<Color>
    orange: Scale<Color>
    yellow: Scale<Color>
    green: Scale<Color>
    cyan: Scale<Color>
    blue: Scale<Color>
    violet: Scale<Color>
    magenta: Scale<Color>
}

export type ThemeConfigInputColorsKeys = keyof ThemeConfigInputColors

/** Allow any part of a syntax highlight style to be overriden by the theme
 *
 * Example:
 * ```ts
 * override: {
 *   syntax: {
 *     boolean: {
 *       underline: true,
 *     },
 *   },
 * }
 * ```
 */
export type ThemeConfigInputSyntax = Partial<Syntax>

interface ThemeConfigOverrides {
    syntax: ThemeConfigInputSyntax
}

type ThemeConfigProperties = ThemeMeta & {
    inputColor: ThemeConfigInputColors
    override: ThemeConfigOverrides
}

// This should be the format a theme is defined as
export type ThemeConfig = {
    [K in keyof ThemeConfigProperties]: ThemeConfigProperties[K]
}

interface ThemeColors {
    neutral: string[]
    red: string[]
    orange: string[]
    yellow: string[]
    green: string[]
    cyan: string[]
    blue: string[]
    violet: string[]
    magenta: string[]
}

type ThemeSyntax = Required<Syntax>

export type ThemeProperties = ThemeMeta & {
    color: ThemeColors
    syntax: ThemeSyntax
}

// This should be a theme after all its properties have been resolved
export type Theme = {
    [K in keyof ThemeProperties]: ThemeProperties[K]
}

export enum ThemeAppearance {
    Light = "light",
    Dark = "dark",
}

export enum ThemeLicenseType {
    MIT = "MIT",
    Apache2 = "Apache License 2.0",
}

export type ThemeFamilyItem =
    | ThemeConfig
    | { light: ThemeConfig; dark: ThemeConfig }

type ThemeFamilyProperties = Partial<Omit<ThemeMeta, "name" | "appearance">> & {
    name: string
    default: ThemeFamilyItem
    variants: {
        [key: string]: ThemeFamilyItem
    }
}

// Idea: A theme family is a collection of themes that share the same name
// For example, a theme family could be `One Dark` and have a `light` and `dark` variant
// The Ayu family could have `light`, `mirage`, and `dark` variants

type ThemeFamily = {
    [K in keyof ThemeFamilyProperties]: ThemeFamilyProperties[K]
}

/** The collection of all themes
 *
 * Example:
 * ```ts
 * {
 *   one_dark,
 *   one_light,
 *     ayu: {
 *     name: 'Ayu',
 *     default: 'ayu_mirage',
 *     variants: {
 *       light: 'ayu_light',
 *       mirage: 'ayu_mirage',
 *       dark: 'ayu_dark',
 *     },
 *   },
 *  ...
 * }
 * ```
 */
export type ThemeIndex = Record<string, ThemeFamily | ThemeConfig>
