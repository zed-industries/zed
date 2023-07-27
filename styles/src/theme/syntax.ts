import deepmerge from "deepmerge"
import { font_weights, ThemeConfigInputSyntax, RampSet } from "../common"
import { Syntax, SyntaxHighlightStyle, allSyntaxKeys } from "../types/syntax"

// Apply defaults to any missing syntax properties that are not defined manually
function apply_defaults(
    ramps: RampSet,
    syntax_highlights: Partial<Syntax>
): Syntax {
    const restKeys: (keyof Syntax)[] = allSyntaxKeys.filter(
        (key) => !syntax_highlights[key]
    )

    const completeSyntax: Syntax = {} as Syntax

    const defaults: SyntaxHighlightStyle = {
        color: ramps.neutral(1).hex(),
    }

    for (const key of restKeys) {
        {
            completeSyntax[key] = {
                ...defaults,
            }
        }
    }

    const mergedBaseSyntax = Object.assign(completeSyntax, syntax_highlights)

    return mergedBaseSyntax
}

// Merge the base syntax with the theme syntax overrides
// This is a deep merge, so any nested properties will be merged as well
// This allows for a theme to only override a single property of a syntax highlight style
const merge_syntax = (
    baseSyntax: Syntax,
    theme_syntax_overrides: ThemeConfigInputSyntax
): Syntax => {
    return deepmerge<Syntax, ThemeConfigInputSyntax>(
        baseSyntax,
        theme_syntax_overrides,
        {
            arrayMerge: (destinationArray, sourceArray) => [
                ...destinationArray,
                ...sourceArray,
            ],
        }
    )
}

/** Returns a complete Syntax object of the combined styles of a theme's syntax overrides and the default syntax styles */
export const syntaxStyle = (
    ramps: RampSet,
    theme_syntax_overrides: ThemeConfigInputSyntax
): Syntax => {
    const syntax_highlights: Partial<Syntax> = {
        comment: { color: ramps.neutral(0.71).hex() },
        "comment.doc": { color: ramps.neutral(0.71).hex() },
        primary: { color: ramps.neutral(1).hex() },
        emphasis: { color: ramps.blue(0.5).hex() },
        "emphasis.strong": {
            color: ramps.blue(0.5).hex(),
            weight: font_weights.bold,
        },
        link_uri: { color: ramps.green(0.5).hex(), underline: true },
        link_text: { color: ramps.orange(0.5).hex(), italic: true },
        "text.literal": { color: ramps.orange(0.5).hex() },
        punctuation: { color: ramps.neutral(0.86).hex() },
        "punctuation.bracket": { color: ramps.neutral(0.86).hex() },
        "punctuation.special": { color: ramps.neutral(0.86).hex() },
        "punctuation.delimiter": { color: ramps.neutral(0.86).hex() },
        "punctuation.list_marker": { color: ramps.neutral(0.86).hex() },
        string: { color: ramps.orange(0.5).hex() },
        "string.special": { color: ramps.orange(0.5).hex() },
        "string.special.symbol": { color: ramps.orange(0.5).hex() },
        "string.escape": { color: ramps.neutral(0.71).hex() },
        "string.regex": { color: ramps.orange(0.5).hex() },
        "method.constructor": { color: ramps.blue(0.5).hex() },
        type: { color: ramps.cyan(0.5).hex() },
        label: { color: ramps.blue(0.5).hex() },
        attribute: { color: ramps.blue(0.5).hex() },
        property: { color: ramps.blue(0.5).hex() },
        constant: { color: ramps.green(0.5).hex() },
        keyword: { color: ramps.blue(0.5).hex() },
        operator: { color: ramps.orange(0.5).hex() },
        number: { color: ramps.green(0.5).hex() },
        boolean: { color: ramps.green(0.5).hex() },
        function: { color: ramps.yellow(0.5).hex() },
    }

    const baseSyntax = apply_defaults(ramps, syntax_highlights)
    const mergedSyntax = merge_syntax(baseSyntax, theme_syntax_overrides)
    return mergedSyntax
}
