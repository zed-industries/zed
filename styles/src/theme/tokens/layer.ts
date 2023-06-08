import { SingleColorToken } from "@tokens-studio/types";
import { Layer, Style, StyleSet } from "../colorScheme";
import { colorToken } from "./token";

interface StyleToken {
    background: SingleColorToken,
    border: SingleColorToken,
    foreground: SingleColorToken,
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

export const styleToken = (style: Style, name: string): StyleToken => {
    const token = {
        background: colorToken(`${name}Background`, style.background),
        border: colorToken(`${name}Border`, style.border),
        foreground: colorToken(`${name}Foreground`, style.foreground),
    }

    return token
}

export const styleSetToken = (styleSet: StyleSet, name: string): StyleSetToken => {
    const token: StyleSetToken = {} as StyleSetToken;

    for (const style in styleSet) {
        const s = style as keyof StyleSet;
        token[s] = styleToken(styleSet[s], `${name}${style}`);
    }

    return token;
}

export const layerToken = (layer: Layer, name: string): LayerToken => {
    const token: LayerToken = {} as LayerToken;

    for (const styleSet in layer) {
        const s = styleSet as keyof Layer;
        token[s] = styleSetToken(layer[s], `${name}${styleSet}`);
    }

    return token;
}
