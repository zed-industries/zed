import { Syntax, SyntaxStyle } from "@/theme/syntax/syntax"
import * as font from "@/theme/font"
import { Color } from "@/theme/color"
import { Highlight } from "@/theme/highlight"
import { defaultSyntax } from "@/theme/syntax/defaultSyntax"
import deepmerge from "deepmerge"

// WIP MERGE SYNTAX

export type OptionalSyntaxStyles = Partial<Syntax>

function buildSyntaxStyle(color: Color, highlight?: Highlight): SyntaxStyle {
    const style: SyntaxStyle = {
        color: color,
        weight: font.weight.regular,
        underline: null,
        italic: false,
        highlight: highlight && highlight,
    }

    return style
}

export const buildSyntax = (
    defaultSyntax: Syntax,
    themeSyntax: OptionalSyntaxStyles
): Syntax => {
    return {
        ...defaultSyntax,
        ...themeSyntax,
    }
}
