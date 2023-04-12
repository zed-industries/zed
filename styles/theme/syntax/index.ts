import * as font from "@/theme/font"
import { Highlight } from "@/theme/highlight"
import { Color } from "@/theme/color"
import { Border } from "@/theme/border"
import { defaultSyntax } from "@/theme/syntax/defaultSyntax"

export interface SyntaxStyle {
    color: Color
    weight: font.Weight
    underline: Border | true | null
    italic: boolean
    highlight: Highlight | null
}

export interface SyntaxStyleTypes {
    attribute: SyntaxStyle
    boolean: SyntaxStyle
    comment: SyntaxStyle
    "comment.doc": SyntaxStyle
    constant: SyntaxStyle
    "constant.builtin"?: SyntaxStyle
    constructor: SyntaxStyle | Function
    embedded: SyntaxStyle
    emphasis: SyntaxStyle
    "emphasis.strong": SyntaxStyle
    enum: SyntaxStyle
    function: SyntaxStyle
    "function.builtin"?: SyntaxStyle
    "function.definition"?: SyntaxStyle
    "function.method"?: SyntaxStyle
    "function.method.builtin"?: SyntaxStyle
    "function.special.definition"?: SyntaxStyle
    keyword: SyntaxStyle
    label: SyntaxStyle
    linkText: SyntaxStyle
    linkUri: SyntaxStyle
    number: SyntaxStyle
    operator: SyntaxStyle
    preproc: SyntaxStyle
    predictive: SyntaxStyle
    primary: SyntaxStyle
    property: SyntaxStyle
    punctuation: SyntaxStyle
    "punctuation.bracket": SyntaxStyle
    "punctuation.delimiter": SyntaxStyle
    "punctuation.list_marker": SyntaxStyle
    "punctuation.special": SyntaxStyle
    string: SyntaxStyle
    "string.escape"?: SyntaxStyle
    "string.regex"?: SyntaxStyle
    "string.special": SyntaxStyle
    "string.special.symbol"?: SyntaxStyle
    tag: SyntaxStyle
    "text.literal": SyntaxStyle
    title: SyntaxStyle
    type: SyntaxStyle
    "type.builtin"?: SyntaxStyle
    variant: SyntaxStyle
    variable: SyntaxStyle
    "variable.special"?: SyntaxStyle
}

export type OptionalSyntaxStyles = Partial<Syntax>
export type Syntax = Record<keyof SyntaxStyleTypes, SyntaxStyle>

export const buildSyntax = (
    defaultSyntax: Syntax,
    themeSyntax: OptionalSyntaxStyles
): Syntax => {
    return {
        ...defaultSyntax,
        ...themeSyntax,
    }
}

export { defaultSyntax }
