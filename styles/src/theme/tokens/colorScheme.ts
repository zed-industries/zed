import { SingleBoxShadowToken, SingleColorToken, SingleOtherToken, TokenTypes, TokenTypographyValue } from "@tokens-studio/types"
import { ColorScheme, Shadow, ThemeSyntax } from "../colorScheme"
import { LayerToken, layerToken } from "./layer"
import { PlayersToken, playersToken } from "./players"

interface ColorSchemeTokens {
    name: SingleOtherToken
    appearance: SingleOtherToken
    lowest: LayerToken
    middle: LayerToken
    highest: LayerToken
    players: PlayersToken
    popoverShadow: SingleBoxShadowToken
    modalShadow: SingleBoxShadowToken
    syntax?: ThemeSyntaxToken
}

const createShadowToken = (shadow: Shadow, tokenName: string): SingleBoxShadowToken => {
    return {
        name: tokenName,
        type: TokenTypes.BOX_SHADOW,
        value: `${shadow.offset[0]}px ${shadow.offset[1]}px ${shadow.blur}px 0px ${shadow.color}`
    };
};

const popoverShadowToken = (colorScheme: ColorScheme): SingleBoxShadowToken => {
    const shadow = colorScheme.popoverShadow;
    return createShadowToken(shadow, "popoverShadow");
};

const modalShadowToken = (colorScheme: ColorScheme): SingleBoxShadowToken => {
    const shadow = colorScheme.modalShadow;
    return createShadowToken(shadow, "modalShadow");
};

interface SyntaxHighlightStyleToken {
    color: SingleColorToken
    weight: TokenTypographyValue['fontWeight']
    underline: TokenTypographyValue['textDecoration']
    italic: SingleOtherToken
}

// TODO: Implement exporting syntax tokens
type ThemeSyntaxToken = Record<keyof ThemeSyntax, SyntaxHighlightStyleToken>

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
    }
}
