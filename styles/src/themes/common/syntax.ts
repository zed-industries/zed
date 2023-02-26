import deepmerge from "deepmerge"
import { fontWeights } from "../../common";

const defaultSyntaxHighlightStyle: Omit<SyntaxHighlightStyle, "color"> = {
    weight: fontWeights.normal,
    underline: false,
    italic: false
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
            ...defaultSyntaxHighlightStyle
        }
    }

    const color = {
        comment: colorScheme.ramps.neutral(0.71).hex()
    }

    // Then assign colors and use Syntax to enforce each style getting it's own color
    const defaultSyntax: Syntax = {
        ...syntax,
        comment: {
            color: color.comment
        },
        "comment.doc": {
            color: color.comment
        },
        primary: {
            color: colorScheme.ramps.neutral(1).hex()
        },
        predictive: {
            color: colorScheme.ramps.neutral(0.57).hex()
        }
        // TODO: Finish default styles
    }

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

import { ColorScheme, Syntax, SyntaxHighlightStyle, ThemeSyntax } from "./colorScheme";

export function buildSyntax(colorScheme: ColorScheme): Syntax {

    const defaultSyntax: Syntax = buildDefaultSyntax(colorScheme)

    const syntax = mergeSyntax(defaultSyntax, colorScheme)

    return syntax
}