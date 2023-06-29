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
} from "../color_scheme"
import { LayerToken, layerToken } from "./layer"
import { PlayersToken, playersToken } from "./players"
import { colorToken } from "./token"
import { Syntax } from "../syntax"
import editor from "../../style_tree/editor"

interface ColorSchemeTokens {
    name: SingleOtherToken
    appearance: SingleOtherToken
    lowest: LayerToken
    middle: LayerToken
    highest: LayerToken
    players: PlayersToken
    popover_shadow: SingleBoxShadowToken
    modal_shadow: SingleBoxShadowToken
    syntax?: Partial<ThemeSyntaxColorTokens>
}

const create_shadow_token = (
    shadow: Shadow,
    token_name: string
): SingleBoxShadowToken => {
    return {
        name: token_name,
        type: TokenTypes.BOX_SHADOW,
        value: `${shadow.offset[0]}px ${shadow.offset[1]}px ${shadow.blur}px 0px ${shadow.color}`,
    }
}

const popover_shadow_token = (theme: ColorScheme): SingleBoxShadowToken => {
    const shadow = theme.popover_shadow
    return create_shadow_token(shadow, "popover_shadow")
}

const modal_shadow_token = (theme: ColorScheme): SingleBoxShadowToken => {
    const shadow = theme.modal_shadow
    return create_shadow_token(shadow, "modal_shadow")
}

type ThemeSyntaxColorTokens = Record<keyof ThemeSyntax, SingleColorToken>

function syntax_highlight_style_color_tokens(
    syntax: Syntax
): ThemeSyntaxColorTokens {
    const style_keys = Object.keys(syntax) as (keyof Syntax)[]

    return style_keys.reduce((acc, style_key) => {
        // Hack: The type of a style could be "Function"
        // This can happen because we have a "constructor" property on the syntax object
        // and a "constructor" property on the prototype of the syntax object
        // To work around this just assert that the type of the style is not a function
        if (!syntax[style_key] || typeof syntax[style_key] === "function")
            return acc
        const { color } = syntax[style_key] as Required<SyntaxHighlightStyle>
        return { ...acc, [style_key]: colorToken(style_key, color) }
    }, {} as ThemeSyntaxColorTokens)
}

const syntax_tokens = (
    theme: ColorScheme
): ColorSchemeTokens["syntax"] => {
    const syntax = editor(theme).syntax

    return syntax_highlight_style_color_tokens(syntax)
}

export function theme_tokens(theme: ColorScheme): ColorSchemeTokens {
    return {
        name: {
            name: "themeName",
            value: theme.name,
            type: TokenTypes.OTHER,
        },
        appearance: {
            name: "themeAppearance",
            value: theme.is_light ? "light" : "dark",
            type: TokenTypes.OTHER,
        },
        lowest: layerToken(theme.lowest, "lowest"),
        middle: layerToken(theme.middle, "middle"),
        highest: layerToken(theme.highest, "highest"),
        popover_shadow: popover_shadow_token(theme),
        modal_shadow: modal_shadow_token(theme),
        players: playersToken(theme),
        syntax: syntax_tokens(theme),
    }
}
