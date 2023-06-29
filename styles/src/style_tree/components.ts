import { font_families, font_sizes, FontWeight } from "../common"
import { Layer, Styles, StyleSets, Style } from "../theme/color_scheme"

function is_style_set(key: any): key is StyleSets {
    return [
        "base",
        "variant",
        "on",
        "accent",
        "positive",
        "warning",
        "negative",
    ].includes(key)
}

function is_style(key: any): key is Styles {
    return [
        "default",
        "active",
        "disabled",
        "hovered",
        "pressed",
        "inverted",
    ].includes(key)
}
function get_style(
    layer: Layer,
    possible_style_set_or_style?: any,
    possible_style?: any
): Style {
    let style_set: StyleSets = "base"
    let style: Styles = "default"
    if (is_style_set(possible_style_set_or_style)) {
        style_set = possible_style_set_or_style
    } else if (is_style(possible_style_set_or_style)) {
        style = possible_style_set_or_style
    }

    if (is_style(possible_style)) {
        style = possible_style
    }

    return layer[style_set][style]
}

export function background(layer: Layer, style?: Styles): string
export function background(
    layer: Layer,
    style_set?: StyleSets,
    style?: Styles
): string
export function background(
    layer: Layer,
    style_set_or_styles?: StyleSets | Styles,
    style?: Styles
): string {
    return get_style(layer, style_set_or_styles, style).background
}

export function border_color(layer: Layer, style?: Styles): string
export function border_color(
    layer: Layer,
    style_set?: StyleSets,
    style?: Styles
): string
export function border_color(
    layer: Layer,
    style_set_or_styles?: StyleSets | Styles,
    style?: Styles
): string {
    return get_style(layer, style_set_or_styles, style).border
}

export function foreground(layer: Layer, style?: Styles): string
export function foreground(
    layer: Layer,
    style_set?: StyleSets,
    style?: Styles
): string
export function foreground(
    layer: Layer,
    style_set_or_styles?: StyleSets | Styles,
    style?: Styles
): string {
    return get_style(layer, style_set_or_styles, style).foreground
}

export interface TextStyle extends Object {
    family: keyof typeof font_families
    color: string
    size: number
    weight?: FontWeight
    underline?: boolean
}

export interface TextProperties {
    size?: keyof typeof font_sizes
    weight?: FontWeight
    underline?: boolean
    color?: string
    features?: FontFeatures
}

interface FontFeatures {
    /** Contextual Alternates: Applies a second substitution feature based on a match of a character pattern within a context of surrounding patterns */
    calt?: boolean
    /** Case-Sensitive Forms: Shifts various punctuation marks up to a position that works better with all-capital sequences */
    case?: boolean
    /** Capital Spacing: Adjusts inter-glyph spacing for all-capital text */
    cpsp?: boolean
    /** Fractions: Replaces figures separated by a slash with diagonal fractions */
    frac?: boolean
    /** Standard Ligatures: Replaces a sequence of glyphs with a single glyph which is preferred for typographic purposes */
    liga?: boolean
    /** Oldstyle Figures: Changes selected figures from the default or lining style to oldstyle form. */
    onum?: boolean
    /** Ordinals: Replaces default alphabetic glyphs with the corresponding ordinal forms for use after figures */
    ordn?: boolean
    /** Proportional Figures: Replaces figure glyphs set on uniform (tabular) widths with corresponding glyphs set on proportional widths */
    pnum?: boolean
    /** Stylistic set 01 */
    ss01?: boolean
    /** Stylistic set 02 */
    ss02?: boolean
    /** Stylistic set 03 */
    ss03?: boolean
    /** Stylistic set 04 */
    ss04?: boolean
    /** Stylistic set 05 */
    ss05?: boolean
    /** Stylistic set 06 */
    ss06?: boolean
    /** Stylistic set 07 */
    ss07?: boolean
    /** Stylistic set 08 */
    ss08?: boolean
    /** Stylistic set 09 */
    ss09?: boolean
    /** Stylistic set 10 */
    ss10?: boolean
    /** Stylistic set 11 */
    ss11?: boolean
    /** Stylistic set 12 */
    ss12?: boolean
    /** Stylistic set 13 */
    ss13?: boolean
    /** Stylistic set 14 */
    ss14?: boolean
    /** Stylistic set 15 */
    ss15?: boolean
    /** Stylistic set 16 */
    ss16?: boolean
    /** Stylistic set 17 */
    ss17?: boolean
    /** Stylistic set 18 */
    ss18?: boolean
    /** Stylistic set 19 */
    ss19?: boolean
    /** Stylistic set 20 */
    ss20?: boolean
    /** Subscript: Replaces default glyphs with subscript glyphs */
    subs?: boolean
    /** Superscript: Replaces default glyphs with superscript glyphs */
    sups?: boolean
    /** Swash: Replaces default glyphs with swash glyphs for stylistic purposes */
    swsh?: boolean
    /** Titling: Replaces default glyphs with titling glyphs for use in large-size settings */
    titl?: boolean
    /** Tabular Figures: Replaces figure glyphs set on proportional widths with corresponding glyphs set on uniform (tabular) widths */
    tnum?: boolean
    /** Slashed Zero: Replaces default zero with a slashed zero for better distinction between "0" and "O" */
    zero?: boolean
}

export function text(
    layer: Layer,
    font_family: keyof typeof font_families,
    style_set: StyleSets,
    style: Styles,
    properties?: TextProperties
): TextStyle
export function text(
    layer: Layer,
    font_family: keyof typeof font_families,
    style_set: StyleSets,
    properties?: TextProperties
): TextStyle
export function text(
    layer: Layer,
    font_family: keyof typeof font_families,
    style: Styles,
    properties?: TextProperties
): TextStyle
export function text(
    layer: Layer,
    font_family: keyof typeof font_families,
    properties?: TextProperties
): TextStyle
export function text(
    layer: Layer,
    font_family: keyof typeof font_families,
    style_set_style_or_properties?: StyleSets | Styles | TextProperties,
    style_or_properties?: Styles | TextProperties,
    properties?: TextProperties
) {
    const style = get_style(
        layer,
        style_set_style_or_properties,
        style_or_properties
    )

    if (typeof style_set_style_or_properties === "object") {
        properties = style_set_style_or_properties
    }
    if (typeof style_or_properties === "object") {
        properties = style_or_properties
    }

    const size = font_sizes[properties?.size || "sm"]
    const color = properties?.color || style.foreground

    return {
        family: font_families[font_family],
        ...properties,
        color,
        size,
    }
}

export interface Border {
    color: string
    width: number
    top?: boolean
    bottom?: boolean
    left?: boolean
    right?: boolean
    overlay?: boolean
}

export interface BorderProperties {
    width?: number
    top?: boolean
    bottom?: boolean
    left?: boolean
    right?: boolean
    overlay?: boolean
}

export function border(
    layer: Layer,
    style_set: StyleSets,
    style: Styles,
    properties?: BorderProperties
): Border
export function border(
    layer: Layer,
    style_set: StyleSets,
    properties?: BorderProperties
): Border
export function border(
    layer: Layer,
    style: Styles,
    properties?: BorderProperties
): Border
export function border(layer: Layer, properties?: BorderProperties): Border
export function border(
    layer: Layer,
    style_set_or_properties?: StyleSets | Styles | BorderProperties,
    style_or_properties?: Styles | BorderProperties,
    properties?: BorderProperties
): Border {
    const style = get_style(layer, style_set_or_properties, style_or_properties)

    if (typeof style_set_or_properties === "object") {
        properties = style_set_or_properties
    }
    if (typeof style_or_properties === "object") {
        properties = style_or_properties
    }

    return {
        color: style.border,
        width: 1,
        ...properties,
    }
}

export function svg(
    color: string,
    asset: string,
    width: number,
    height: number
) {
    return {
        color,
        asset,
        dimensions: {
            width,
            height,
        },
    }
}
