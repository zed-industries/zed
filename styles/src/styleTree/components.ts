import { fontFamilies, fontSizes, FontWeight } from "../common"
import { Layer, Styles, StyleSets, Style } from "../themes/common/colorScheme"

function isStyleSet(key: any): key is StyleSets {
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

function isStyle(key: any): key is Styles {
    return [
        "default",
        "active",
        "disabled",
        "hovered",
        "pressed",
        "inverted",
    ].includes(key)
}
function getStyle(
    layer: Layer,
    possibleStyleSetOrStyle?: any,
    possibleStyle?: any
): Style {
    let styleSet: StyleSets = "base"
    let style: Styles = "default"
    if (isStyleSet(possibleStyleSetOrStyle)) {
        styleSet = possibleStyleSetOrStyle
    } else if (isStyle(possibleStyleSetOrStyle)) {
        style = possibleStyleSetOrStyle
    }

    if (isStyle(possibleStyle)) {
        style = possibleStyle
    }

    return layer[styleSet][style]
}

export function background(layer: Layer, style?: Styles): string
export function background(
    layer: Layer,
    styleSet?: StyleSets,
    style?: Styles
): string
export function background(
    layer: Layer,
    styleSetOrStyles?: StyleSets | Styles,
    style?: Styles
): string {
    return getStyle(layer, styleSetOrStyles, style).background
}

export function borderColor(layer: Layer, style?: Styles): string
export function borderColor(
    layer: Layer,
    styleSet?: StyleSets,
    style?: Styles
): string
export function borderColor(
    layer: Layer,
    styleSetOrStyles?: StyleSets | Styles,
    style?: Styles
): string {
    return getStyle(layer, styleSetOrStyles, style).border
}

export function foreground(layer: Layer, style?: Styles): string
export function foreground(
    layer: Layer,
    styleSet?: StyleSets,
    style?: Styles
): string
export function foreground(
    layer: Layer,
    styleSetOrStyles?: StyleSets | Styles,
    style?: Styles
): string {
    return getStyle(layer, styleSetOrStyles, style).foreground
}

interface Text {
    family: keyof typeof fontFamilies
    color: string
    size: number
    weight?: FontWeight
    underline?: boolean
}

export interface TextProperties {
    size?: keyof typeof fontSizes
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
    fontFamily: keyof typeof fontFamilies,
    styleSet: StyleSets,
    style: Styles,
    properties?: TextProperties
): Text
export function text(
    layer: Layer,
    fontFamily: keyof typeof fontFamilies,
    styleSet: StyleSets,
    properties?: TextProperties
): Text
export function text(
    layer: Layer,
    fontFamily: keyof typeof fontFamilies,
    style: Styles,
    properties?: TextProperties
): Text
export function text(
    layer: Layer,
    fontFamily: keyof typeof fontFamilies,
    properties?: TextProperties
): Text
export function text(
    layer: Layer,
    fontFamily: keyof typeof fontFamilies,
    styleSetStyleOrProperties?: StyleSets | Styles | TextProperties,
    styleOrProperties?: Styles | TextProperties,
    properties?: TextProperties
) {
    let style = getStyle(layer, styleSetStyleOrProperties, styleOrProperties)

    if (typeof styleSetStyleOrProperties === "object") {
        properties = styleSetStyleOrProperties
    }
    if (typeof styleOrProperties === "object") {
        properties = styleOrProperties
    }

    let size = fontSizes[properties?.size || "sm"]
    let color = properties?.color || style.foreground

    return {
        family: fontFamilies[fontFamily],
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
    styleSet: StyleSets,
    style: Styles,
    properties?: BorderProperties
): Border
export function border(
    layer: Layer,
    styleSet: StyleSets,
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
    styleSetStyleOrProperties?: StyleSets | Styles | BorderProperties,
    styleOrProperties?: Styles | BorderProperties,
    properties?: BorderProperties
): Border {
    let style = getStyle(layer, styleSetStyleOrProperties, styleOrProperties)

    if (typeof styleSetStyleOrProperties === "object") {
        properties = styleSetStyleOrProperties
    }
    if (typeof styleOrProperties === "object") {
        properties = styleOrProperties
    }

    return {
        color: style.border,
        width: 1,
        ...properties,
    }
}


export function svg(color: string, asset: String, width: Number, height: Number) {
    return {
        color,
        asset,
        dimensions: {
            width,
            height,
        }
    }
}
