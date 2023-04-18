import deepmerge from "deepmerge"
import { FontWeight, fontWeights } from "../../common"
import { ColorScheme } from "./colorScheme"
import chroma from "chroma-js"

export interface SyntaxHighlightStyle {
    color: string
    weight?: FontWeight
    underline?: boolean
    italic?: boolean
}

export interface Syntax {
    // == Text Styles ====== /
    comment: SyntaxHighlightStyle
    // elixir: doc comment
    "comment.doc": SyntaxHighlightStyle
    primary: SyntaxHighlightStyle
    predictive: SyntaxHighlightStyle

    // === Formatted Text ====== /
    emphasis: SyntaxHighlightStyle
    "emphasis.strong": SyntaxHighlightStyle
    title: SyntaxHighlightStyle
    linkUri: SyntaxHighlightStyle
    linkText: SyntaxHighlightStyle
    /** md: indented_code_block, fenced_code_block, code_span */
    "text.literal": SyntaxHighlightStyle

    // == Punctuation ====== /
    punctuation: SyntaxHighlightStyle
    /** Example: `(`, `[`, `{`...*/
    "punctuation.bracket": SyntaxHighlightStyle
    /**., ;*/
    "punctuation.delimiter": SyntaxHighlightStyle
    // js, ts: ${, } in a template literal
    // yaml: *, &, ---, ...
    "punctuation.special": SyntaxHighlightStyle
    // md: list_marker_plus, list_marker_dot, etc
    "punctuation.list_marker": SyntaxHighlightStyle

    // == Strings ====== /

    string: SyntaxHighlightStyle
    // css: color_value
    // js: this, super
    // toml: offset_date_time, local_date_time...
    "string.special": SyntaxHighlightStyle
    // elixir: atom, quoted_atom, keyword, quoted_keyword
    // ruby: simple_symbol, delimited_symbol...
    "string.special.symbol"?: SyntaxHighlightStyle
    // elixir, python, yaml...: escape_sequence
    "string.escape"?: SyntaxHighlightStyle
    // Regular expressions
    "string.regex"?: SyntaxHighlightStyle

    // == Types ====== /
    // We allow Function here because all JS objects literals have this property
    constructor: SyntaxHighlightStyle | Function
    variant: SyntaxHighlightStyle
    type: SyntaxHighlightStyle
    // js: predefined_type
    "type.builtin"?: SyntaxHighlightStyle

    // == Values
    variable: SyntaxHighlightStyle
    // this, ...
    // css: -- (var(--foo))
    // lua: self
    "variable.special"?: SyntaxHighlightStyle
    // c: statement_identifier,
    label: SyntaxHighlightStyle
    // css: tag_name, nesting_selector, universal_selector...
    tag: SyntaxHighlightStyle
    // css: attribute, pseudo_element_selector (tag_name),
    attribute: SyntaxHighlightStyle
    // css: class_name, property_name, namespace_name...
    property: SyntaxHighlightStyle
    // true, false, null, nullptr
    constant: SyntaxHighlightStyle
    // css: @media, @import, @supports...
    // js: declare, implements, interface, keyof, public...
    keyword: SyntaxHighlightStyle
    // note: js enum is currently defined as a keyword
    enum: SyntaxHighlightStyle
    // -, --, ->, !=, &&, ||, <=...
    operator: SyntaxHighlightStyle
    number: SyntaxHighlightStyle
    boolean: SyntaxHighlightStyle
    // elixir: __MODULE__, __DIR__, __ENV__, etc
    // go: nil, iota
    "constant.builtin"?: SyntaxHighlightStyle

    // == Functions ====== /

    function: SyntaxHighlightStyle
    // lua: assert, error, loadfile, tostring, unpack...
    "function.builtin"?: SyntaxHighlightStyle
    // go: call_expression, method_declaration
    // js: call_expression, method_definition, pair (key, arrow function)
    // rust: function_item name: (identifier)
    "function.definition"?: SyntaxHighlightStyle
    // rust: macro_definition name: (identifier)
    "function.special.definition"?: SyntaxHighlightStyle
    "function.method"?: SyntaxHighlightStyle
    // ruby: identifier/"defined?" // Nate note: I don't fully understand this one.
    "function.method.builtin"?: SyntaxHighlightStyle

    // == Unsorted ====== /
    // lua: hash_bang_line
    preproc: SyntaxHighlightStyle
    // elixir, python: interpolation (ex: foo in ${foo})
    // js: template_substitution
    embedded: SyntaxHighlightStyle
}

export type ThemeSyntax = Partial<Syntax>

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

    const light = colorScheme.isLight

    // then spread the default to each style
    for (const key of Object.keys({} as Syntax)) {
        syntax[key as keyof Syntax] = {
            ...defaultSyntaxHighlightStyle,
        }
    }

    // Mix the neutral and blue colors to get a
    // predictive color distinct from any other color in the theme
    const predictive = chroma.mix(
        colorScheme.ramps.neutral(0.4).hex(),
        colorScheme.ramps.blue(0.4).hex(),
        0.45,
        "lch"
    ).hex()

    const color = {
        primary: colorScheme.ramps.neutral(1).hex(),
        comment: colorScheme.ramps.neutral(0.71).hex(),
        punctuation: colorScheme.ramps.neutral(0.86).hex(),
        predictive: predictive,
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
        variable: {
            color: color.primary,
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
        function: {
            color: color.function,
        },
        preproc: {
            color: color.primary,
        },
        embedded: {
            color: color.primary,
        },
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

export function buildSyntax(colorScheme: ColorScheme): Syntax {
    const defaultSyntax: Syntax = buildDefaultSyntax(colorScheme)

    const syntax = mergeSyntax(defaultSyntax, colorScheme)

    return syntax
}
