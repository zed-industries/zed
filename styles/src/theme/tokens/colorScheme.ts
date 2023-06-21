import {
    SingleBoxShadowToken,
    SingleColorToken,
    SingleOtherToken,
    TokenTypes,
} from "@tokens-studio/types"
import {
    ColorScheme,
    Shadow,
    SyntaxHighlightStyle,
    ThemeSyntax,
} from "../colorScheme"
import { LayerToken, layerToken } from "./layer"
import { PlayersToken, playersToken } from "./players"
import { colorToken } from "./token"
import { Syntax } from "../syntax"
import editor from "../../styleTree/editor"

interface ColorSchemeTokens {
    name: SingleOtherToken
    appearance: SingleOtherToken
    lowest: LayerToken
    middle: LayerToken
    highest: LayerToken
    players: PlayersToken
    popoverShadow: SingleBoxShadowToken
    modalShadow: SingleBoxShadowToken
    syntax?: Partial<ThemeSyntaxColorTokens>
}

const createShadowToken = (
    shadow: Shadow,
    tokenName: string
): SingleBoxShadowToken => {
    return {
        name: tokenName,
        type: TokenTypes.BOX_SHADOW,
        value: `${shadow.offset[0]}px ${shadow.offset[1]}px ${shadow.blur}px 0px ${shadow.color}`,
    }
}

const popoverShadowToken = (colorScheme: ColorScheme): SingleBoxShadowToken => {
    const shadow = colorScheme.popoverShadow
    return createShadowToken(shadow, "popoverShadow")
}

const modalShadowToken = (colorScheme: ColorScheme): SingleBoxShadowToken => {
    const shadow = colorScheme.modalShadow
    return createShadowToken(shadow, "modalShadow")
}

type ThemeSyntaxColorTokens = Record<keyof ThemeSyntax, SingleColorToken>

function syntaxHighlightStyleColorTokens(
    syntax: Syntax
): ThemeSyntaxColorTokens {
    const styleKeys = Object.keys(syntax) as (keyof Syntax)[]

    return styleKeys.reduce((acc, styleKey) => {
        // Hack: The type of a style could be "Function"
        // This can happen because we have a "constructor" property on the syntax object
        // and a "constructor" property on the prototype of the syntax object
        // To work around this just assert that the type of the style is not a function
        if (!syntax[styleKey] || typeof syntax[styleKey] === "function")
            return acc
        const { color } = syntax[styleKey] as Required<SyntaxHighlightStyle>
        return { ...acc, [styleKey]: colorToken(styleKey, color) }
    }, {} as ThemeSyntaxColorTokens)
}

const syntaxTokens = (
    colorScheme: ColorScheme
): ColorSchemeTokens["syntax"] => {
    const syntax = editor(colorScheme).syntax

    return syntaxHighlightStyleColorTokens(syntax)
}

export function colorSchemeTokens(colorScheme: ColorScheme): ColorSchemeTokens {
    return {
        name: {
            name: "themeName",
            value: colorScheme.name,
            type: TokenTypes.OTHER,
        },
        appearance: {
            name: "themeAppearance",
            value: colorScheme.isLight ? "light" : "dark",
            type: TokenTypes.OTHER,
        },
        lowest: layerToken(colorScheme.lowest, "lowest"),
        middle: layerToken(colorScheme.middle, "middle"),
        highest: layerToken(colorScheme.highest, "highest"),
        popoverShadow: popoverShadowToken(colorScheme),
        modalShadow: modalShadowToken(colorScheme),
        players: playersToken(colorScheme),
        syntax: syntaxTokens(colorScheme),
    }
}
