import deepmerge from "deepmerge"
import { fontWeights } from "../../common"

const defaultSyntaxHighlightStyle: Omit<SyntaxHighlightStyle, "color"> = {
    weight: fontWeights.normal,
    underline: false,
    italic: false,
}

function buildDefaultSyntax(colorScheme: ColorScheme): Syntax {
    // Make a temporary object that is allowed to be missing
    // the "color" property for each style
    const syntax: {
        [key: string]: Omit<SyntaxHighlightStyle, "color">
    } = {}

    // then spread the default to each style
    for (const key of Object.keys({} as Syntax)) {
        syntax[key as keyof Syntax] = {
            ...defaultSyntaxHighlightStyle,
        }
    }

    const color = {
        primary: colorScheme.ramps.neutral(1).hex(),
        comment: colorScheme.ramps.neutral(0.71).hex(),
        punctuation: colorScheme.ramps.neutral(0.86).hex(),
        predictive: colorScheme.ramps.neutral(0.57).hex(),
        emphasis: colorScheme.ramps.blue(0.5).hex(),
        string: colorScheme.ramps.orange(0.5).hex(),
        function: colorScheme.ramps.yellow(0.5).hex(),
        type: colorScheme.ramps.cyan(0.5).hex(),
        constructor: colorScheme.ramps.blue(0.5).hex(),
        variant: colorScheme.ramps.blue(0.5).hex(),
        property: colorScheme.ramps.blue(0.5).hex(),
        enum: colorScheme.ramps.orange(0.5).hex(),
        operator: colorScheme.ramps.orange(0.5).hex(),
        number: colorScheme.ramps.green(0.5).hex(),
        boolean: colorScheme.ramps.green(0.5).hex(),
        constant: colorScheme.ramps.green(0.5).hex(),
        keyword: colorScheme.ramps.blue(0.5).hex(),
    }

    // Then assign colors and use Syntax to enforce each style getting it's own color
    const defaultSyntax: Syntax = {
        ...syntax,
        comment: {
            color: color.comment,
        },
        "comment.doc": {
            color: color.comment,
        },
        primary: {
            color: color.primary,
        },
        predictive: {
            color: color.predictive,
        },
        emphasis: {
            color: color.emphasis,
        },
        "emphasis.strong": {
            color: color.emphasis,
            weight: fontWeights.bold,
        },
        title: {
            color: color.primary,
            weight: fontWeights.bold,
        },
        linkUri: {
            color: colorScheme.ramps.green(0.5).hex(),
            underline: true,
        },
        linkText: {
            color: colorScheme.ramps.orange(0.5).hex(),
            italic: true,
        },
        "text.literal": {
            color: color.string,
        },
        punctuation: {
            color: color.punctuation,
        },
        "punctuation.bracket": {
            color: color.punctuation,
        },
        "punctuation.delimiter": {
            color: color.punctuation,
        },
        "punctuation.special": {
            color: colorScheme.ramps.neutral(0.86).hex(),
        },
        "punctuation.list_marker": {
            color: color.punctuation,
        },
        string: {
            color: color.string,
        },
        "string.special": {
            color: color.string,
        },
        "string.special.symbol": {
            color: color.string,
        },
        "string.special.regex": {
            color: color.string,
        },
        "string.escape": {
            color: color.comment,
        },
        "string.regex": {
            color: color.string,
        },
        constructor: {
            color: colorScheme.ramps.blue(0.5).hex(),
        },
        variant: {
            color: colorScheme.ramps.blue(0.5).hex(),
        },
        type: {
            color: color.type,
        },
        "type.builtin": {
            color: color.type,
        },
        "variable.builtin": {
            color: colorScheme.ramps.blue(0.5).hex(),
        },
        "variable.special": {
            color: colorScheme.ramps.blue(0.7).hex(),
        },
        label: {
            color: colorScheme.ramps.blue(0.5).hex(),
        },
        tag: {
            color: colorScheme.ramps.blue(0.5).hex(),
        },
        attribute: {
            color: colorScheme.ramps.blue(0.5).hex(),
        },
        property: {
            color: colorScheme.ramps.blue(0.5).hex(),
        },
        constant: {
            color: color.constant,
        },
        keyword: {
            color: color.keyword,
        },
        enum: {
            color: color.enum,
        },
        operator: {
            color: color.operator,
        },
        number: {
            color: color.number,
        },
        boolean: {
            color: color.boolean,
        },
        "constant.builtin": {
            color: color.constant,
        },
        function: {
            color: color.function,
        },
        "function.builtin": {
            color: color.function,
        },
        "function.call": {
            color: color.function,
        },
        "function.definition": {
            color: color.function,
        },
        "function.special.definition": {
            color: color.function,
        },
        "function.method": {
            color: color.function,
        },
        "function.method.builtin": {
            color: color.function,
        },
        preproc: {
            color: color.primary,
        },
        embedded: {
            color: color.primary,
        },
    }

    console.log(JSON.stringify(defaultSyntax, null, 2))

    return defaultSyntax
}

function mergeSyntax(defaultSyntax: Syntax, colorScheme: ColorScheme): Syntax {
    if (!colorScheme.syntax) {
        return defaultSyntax
    }

    return deepmerge<Syntax, Partial<ThemeSyntax>>(
        defaultSyntax,
        colorScheme.syntax,
        {
            arrayMerge: (destinationArray, sourceArray) => [
                ...destinationArray,
                ...sourceArray,
            ],
        }
    )
}

import {
    ColorScheme,
    Syntax,
    SyntaxHighlightStyle,
    ThemeSyntax,
} from "./colorScheme"

export function buildSyntax(colorScheme: ColorScheme): Syntax {
    const defaultSyntax: Syntax = buildDefaultSyntax(colorScheme)

    const syntax = mergeSyntax(defaultSyntax, colorScheme)

    return syntax
}
