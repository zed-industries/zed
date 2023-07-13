import deepmerge from "deepmerge"
import { FontWeight, font_weights, useTheme } from "../common"
import chroma from "chroma-js"

export interface SyntaxHighlightStyle {
    color?: string
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
    hint: SyntaxHighlightStyle

    // === Formatted Text ====== /
    emphasis: SyntaxHighlightStyle
    "emphasis.strong": SyntaxHighlightStyle
    title: SyntaxHighlightStyle
    link_uri: SyntaxHighlightStyle
    link_text: SyntaxHighlightStyle
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
    constructor: SyntaxHighlightStyle | Function // eslint-disable-line  @typescript-eslint/ban-types
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

const default_syntax_highlight_style: Omit<SyntaxHighlightStyle, "color"> = {
    weight: "normal",
    underline: false,
    italic: false,
}

function build_default_syntax(): Syntax {
    const theme = useTheme()

    // Make a temporary object that is allowed to be missing
    // the "color" property for each style
    const syntax: {
        [key: string]: Omit<SyntaxHighlightStyle, "color">
    } = {}

    // then spread the default to each style
    for (const key of Object.keys({} as Syntax)) {
        syntax[key as keyof Syntax] = {
            ...default_syntax_highlight_style,
        }
    }

    // Mix the neutral and blue colors to get a
    // predictive color distinct from any other color in the theme
    const predictive = chroma
        .mix(
            theme.ramps.neutral(0.4).hex(),
            theme.ramps.blue(0.4).hex(),
            0.45,
            "lch"
        )
        .hex()
    // Mix the neutral and green colors to get a
    // hint color distinct from any other color in the theme
    const hint = chroma
        .mix(
            theme.ramps.neutral(0.6).hex(),
            theme.ramps.blue(0.4).hex(),
            0.45,
            "lch"
        )
        .hex()

    const color = {
        primary: theme.ramps.neutral(1).hex(),
        comment: theme.ramps.neutral(0.71).hex(),
        punctuation: theme.ramps.neutral(0.86).hex(),
        predictive: predictive,
        hint: hint,
        emphasis: theme.ramps.blue(0.5).hex(),
        string: theme.ramps.orange(0.5).hex(),
        function: theme.ramps.yellow(0.5).hex(),
        type: theme.ramps.cyan(0.5).hex(),
        constructor: theme.ramps.blue(0.5).hex(),
        variant: theme.ramps.blue(0.5).hex(),
        property: theme.ramps.blue(0.5).hex(),
        enum: theme.ramps.orange(0.5).hex(),
        operator: theme.ramps.orange(0.5).hex(),
        number: theme.ramps.green(0.5).hex(),
        boolean: theme.ramps.green(0.5).hex(),
        constant: theme.ramps.green(0.5).hex(),
        keyword: theme.ramps.blue(0.5).hex(),
    }

    // Then assign colors and use Syntax to enforce each style getting it's own color
    const default_syntax: Syntax = {
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
            italic: true,
        },
        hint: {
            color: color.hint,
            weight: font_weights.bold,
        },
        emphasis: {
            color: color.emphasis,
        },
        "emphasis.strong": {
            color: color.emphasis,
            weight: font_weights.bold,
        },
        title: {
            color: color.primary,
            weight: font_weights.bold,
        },
        link_uri: {
            color: theme.ramps.green(0.5).hex(),
            underline: true,
        },
        link_text: {
            color: theme.ramps.orange(0.5).hex(),
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
            color: theme.ramps.neutral(0.86).hex(),
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
            color: theme.ramps.blue(0.5).hex(),
        },
        variant: {
            color: theme.ramps.blue(0.5).hex(),
        },
        type: {
            color: color.type,
        },
        variable: {
            color: theme.ramps.red(0.5).hex(),
        },
        label: {
            color: theme.ramps.blue(0.5).hex(),
        },
        tag: {
            color: theme.ramps.blue(0.5).hex(),
        },
        attribute: {
            color: theme.ramps.blue(0.5).hex(),
        },
        property: {
            color: theme.ramps.blue(0.5).hex(),
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

    return default_syntax
}

export function build_syntax(): Syntax {
    const theme = useTheme()

    const default_syntax: Syntax = build_default_syntax()

    if (!theme.syntax) {
        return default_syntax
    }

    const syntax = deepmerge<Syntax, Partial<ThemeSyntax>>(
        default_syntax,
        theme.syntax,
        {
            arrayMerge: (destinationArray, sourceArray) => [
                ...destinationArray,
                ...sourceArray,
            ],
        }
    )

    return syntax
}
