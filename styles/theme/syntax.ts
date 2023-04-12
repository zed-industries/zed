import * as font from "@/theme/font"
import { Highlight } from "@/theme/highlight"

export interface SyntaxStyle {
    color: string
    weight: font.Weight
    underline?: boolean
    italic?: boolean
    highlight?: Highlight
}

// prettier-ignore
interface Syntax {
    "attribute": SyntaxStyle,
    "boolean": SyntaxStyle,
    "comment": SyntaxStyle,
    "comment.doc": SyntaxStyle,
    "constant": SyntaxStyle,
    "constant.builtin"?: SyntaxStyle,
    "constructor": SyntaxStyle | Function,
    "embedded": SyntaxStyle,
    "emphasis": SyntaxStyle,
    "emphasis.strong": SyntaxStyle,
    "enum": SyntaxStyle,
    "function": SyntaxStyle,
    "function.builtin"?: SyntaxStyle,
    "function.definition"?: SyntaxStyle,
    "function.method"?: SyntaxStyle,
    "function.method.builtin"?: SyntaxStyle,
    "function.special.definition"?: SyntaxStyle,
    "keyword": SyntaxStyle,
    "label": SyntaxStyle,
    "linkText": SyntaxStyle,
    "linkUri": SyntaxStyle,
    "number": SyntaxStyle,
    "operator": SyntaxStyle,
    "preproc": SyntaxStyle,
    "predictive": SyntaxStyle,
    "primary": SyntaxStyle,
    "property": SyntaxStyle,
    "punctuation": SyntaxStyle,
    "punctuation.bracket": SyntaxStyle,
    "punctuation.delimiter": SyntaxStyle,
    "punctuation.list_marker": SyntaxStyle,
    "punctuation.special": SyntaxStyle,
    "string": SyntaxStyle,
    "string.escape"?: SyntaxStyle,
    "string.regex"?: SyntaxStyle,
    "string.special": SyntaxStyle,
    "string.special.symbol"?: SyntaxStyle,
    "tag": SyntaxStyle,
    "text.literal": SyntaxStyle,
    "title": SyntaxStyle,
    "type": SyntaxStyle,
    "type.builtin"?: SyntaxStyle,
    "variant": SyntaxStyle,
    "variable": SyntaxStyle,
    "variable.special"?: SyntaxStyle,
}

export const syntax: Syntax = {
    // ...
}
