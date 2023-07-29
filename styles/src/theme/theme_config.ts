import { Scale, Color } from "chroma-js"
import { SyntaxHighlightStyle, SyntaxProperty } from "../types/syntax"

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
    license_type?: string | ThemeLicenseType
    license_url?: string
    license_file: string
    theme_url?: string
}

export type ThemeFamilyMeta = Pick<
    ThemeMeta,
    "name" | "author" | "license_type" | "license_url"
>

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
export type ThemeConfigInputSyntax = Partial<
    Record<SyntaxProperty, Partial<SyntaxHighlightStyle>>
>

interface ThemeConfigOverrides {
    syntax: ThemeConfigInputSyntax
}

type ThemeConfigProperties = ThemeMeta & {
    input_color: ThemeConfigInputColors
    override: ThemeConfigOverrides
}

export type ThemeConfig = {
    [K in keyof ThemeConfigProperties]: ThemeConfigProperties[K]
}

export enum ThemeAppearance {
    Light = "light",
    Dark = "dark",
}

export enum ThemeLicenseType {
    MIT = "MIT",
    Apache2 = "Apache License 2.0",
}
