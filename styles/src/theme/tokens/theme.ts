import {
    SingleBoxShadowToken,
    SingleColorToken,
    SingleOtherToken,
    TokenTypes,
} from "@tokens-studio/types"
import { Shadow } from "../create_theme"
import { LayerToken, layer_token } from "./layer"
import { PlayersToken, players_token } from "./players"
import { color_token } from "./token"
import editor from "../../style_tree/editor"
import { useTheme } from "../../../src/common"
import { Syntax, SyntaxHighlightStyle } from "../../types/syntax"

interface ThemeTokens {
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

const popover_shadow_token = (): SingleBoxShadowToken => {
    const theme = useTheme()
    const shadow = theme.popover_shadow
    return create_shadow_token(shadow, "popover_shadow")
}

const modal_shadow_token = (): SingleBoxShadowToken => {
    const theme = useTheme()
    const shadow = theme.modal_shadow
    return create_shadow_token(shadow, "modal_shadow")
}

type ThemeSyntaxColorTokens = Record<keyof Syntax, SingleColorToken>

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
        return { ...acc, [style_key]: color_token(style_key, color) }
    }, {} as ThemeSyntaxColorTokens)
}

const syntax_tokens = (): ThemeTokens["syntax"] => {
    const syntax = editor().syntax

    return syntax_highlight_style_color_tokens(syntax)
}

export function theme_tokens(): ThemeTokens {
    const theme = useTheme()

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
        lowest: layer_token(theme.lowest, "lowest"),
        middle: layer_token(theme.middle, "middle"),
        highest: layer_token(theme.highest, "highest"),
        popover_shadow: popover_shadow_token(),
        modal_shadow: modal_shadow_token(),
        players: players_token(),
        syntax: syntax_tokens(),
    }
}
