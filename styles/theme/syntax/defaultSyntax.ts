import { Syntax, SyntaxStyle } from "@/theme/syntax/syntax";
import * as font from "@/theme/font";
import { Color, chroma } from "@/theme/color";

const TEMP_COLOR: Color = chroma("red");

export const baseSyntaxStyle: SyntaxStyle = {
    color: chroma("black"),
    weight: font.weight.regular,
    underline: null,
    italic: false,
    highlight: null,
};

const defaultColors: Record<keyof Syntax, Color> = {
    attribute: TEMP_COLOR,
    boolean: TEMP_COLOR,
    comment: TEMP_COLOR,
    "comment.doc": TEMP_COLOR,
    constant: TEMP_COLOR,
    "constant.builtin": TEMP_COLOR,
    constructor: TEMP_COLOR,
    embedded: TEMP_COLOR,
    emphasis: TEMP_COLOR,
    "emphasis.strong": TEMP_COLOR,
    enum: TEMP_COLOR,
    function: TEMP_COLOR,
    "function.builtin": TEMP_COLOR,
    "function.definition": TEMP_COLOR,
    "function.method": TEMP_COLOR,
    "function.method.builtin": TEMP_COLOR,
    "function.special.definition": TEMP_COLOR,
    keyword: TEMP_COLOR,
    label: TEMP_COLOR,
    linkText: TEMP_COLOR,
    linkUri: TEMP_COLOR,
    number: TEMP_COLOR,
    operator: TEMP_COLOR,
    preproc: TEMP_COLOR,
    predictive: TEMP_COLOR,
    primary: TEMP_COLOR,
    property: TEMP_COLOR,
    punctuation: TEMP_COLOR,
    "punctuation.bracket": TEMP_COLOR,
    "punctuation.delimiter": TEMP_COLOR,
    "punctuation.list_marker": TEMP_COLOR,
    "punctuation.special": TEMP_COLOR,
    string: TEMP_COLOR,
    "string.escape": TEMP_COLOR,
    "string.regex": TEMP_COLOR,
    "string.special": TEMP_COLOR,
    "string.special.symbol": TEMP_COLOR,
    tag: TEMP_COLOR,
    "text.literal": TEMP_COLOR,
    title: TEMP_COLOR,
    type: TEMP_COLOR,
    "type.builtin": TEMP_COLOR,
    variant: TEMP_COLOR,
    variable: TEMP_COLOR,
    "variable.special": TEMP_COLOR,
};

function buildDefaultSyntaxColors(): Syntax {
    const defaultSyntax = Object.keys(defaultColors).reduce((acc, key) => {
        acc[key as keyof Syntax] = { ...baseSyntaxStyle, color: TEMP_COLOR };
        return acc;
    }, {} as Record<keyof Syntax, SyntaxStyle>);

    return defaultSyntax;
}

let defaultSyntaxColors = buildDefaultSyntaxColors();

// Deal with specific non-color style defaults
defaultSyntaxColors["emphasis.strong"].weight = font.weight.bold;
defaultSyntaxColors["linkUri"].underline = true;
defaultSyntaxColors["linkText"].italic = true;

export const defaultSyntax: Syntax = defaultSyntaxColors;
