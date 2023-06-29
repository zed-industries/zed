import { SingleColorToken } from "@tokens-studio/types"
import { Layer, Style, StyleSet } from "../color_scheme"
import { color_token } from "./token"

interface StyleToken {
    background: SingleColorToken
    border: SingleColorToken
    foreground: SingleColorToken
}

interface StyleSetToken {
    default: StyleToken
    active: StyleToken
    disabled: StyleToken
    hovered: StyleToken
    pressed: StyleToken
    inverted: StyleToken
}

export interface LayerToken {
    base: StyleSetToken
    variant: StyleSetToken
    on: StyleSetToken
    accent: StyleSetToken
    positive: StyleSetToken
    warning: StyleSetToken
    negative: StyleSetToken
}

export const style_token = (style: Style, name: string): StyleToken => {
    const token = {
        background: color_token(`${name}Background`, style.background),
        border: color_token(`${name}Border`, style.border),
        foreground: color_token(`${name}Foreground`, style.foreground),
    }

    return token
}

export const style_set_token = (
    style_set: StyleSet,
    name: string
): StyleSetToken => {
    const token: StyleSetToken = {} as StyleSetToken

    for (const style in style_set) {
        const s = style as keyof StyleSet
        token[s] = style_token(style_set[s], `${name}${style}`)
    }

    return token
}

export const layer_token = (layer: Layer, name: string): LayerToken => {
    const token: LayerToken = {} as LayerToken

    for (const style_set in layer) {
        const s = style_set as keyof Layer
        token[s] = style_set_token(layer[s], `${name}${style_set}`)
    }

    return token
}
