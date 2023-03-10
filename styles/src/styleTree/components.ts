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
