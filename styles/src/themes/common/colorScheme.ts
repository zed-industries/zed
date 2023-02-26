import { Scale } from "chroma-js"
import { FontWeight } from "../../common"

export interface ColorScheme {
    name: string
    isLight: boolean

    lowest: Layer
    middle: Layer
    highest: Layer

    ramps: RampSet

    popoverShadow: Shadow
    modalShadow: Shadow

    players: Players
    syntax?: Partial<ThemeSyntax>
}

export interface Meta {
    name: string
    author: string
    url: string
    license: License
}

export interface License {
    SPDX: SPDXExpression
    /// A url where we can download the license's text
    https_url: string
    license_checksum: string
}

// License name -> License text
export interface Licenses {
    [key: string]: string
}

// FIXME: Add support for the SPDX expression syntax
export type SPDXExpression = "MIT"

export interface Player {
    cursor: string
    selection: string
}

export interface Players {
    "0": Player
    "1": Player
    "2": Player
    "3": Player
    "4": Player
    "5": Player
    "6": Player
    "7": Player
}

export interface Shadow {
    blur: number
    color: string
    offset: number[]
}

export type StyleSets = keyof Layer
export interface Layer {
    base: StyleSet
    variant: StyleSet
    on: StyleSet
    accent: StyleSet
    positive: StyleSet
    warning: StyleSet
    negative: StyleSet
}

export interface RampSet {
    neutral: Scale
    red: Scale
    orange: Scale
    yellow: Scale
    green: Scale
    cyan: Scale
    blue: Scale
    violet: Scale
    magenta: Scale
}

export type Styles = keyof StyleSet
export interface StyleSet {
    default: Style
    active: Style
    disabled: Style
    hovered: Style
    pressed: Style
    inverted: Style
}

export interface Style {
    background: string
    border: string
    foreground: string
}

export interface SyntaxHighlightStyle {
    color: string
    weight?: FontWeight
    underline?: boolean
    italic?: boolean
}

export interface Syntax {
    // == Text Styles
    primary: SyntaxHighlightStyle
    predictive: SyntaxHighlightStyle
    emphasis: SyntaxHighlightStyle
    "emphasis.strong": SyntaxHighlightStyle
    title: SyntaxHighlightStyle
    linkUri: SyntaxHighlightStyle
    linkText: SyntaxHighlightStyle

    // == General
    comment: SyntaxHighlightStyle

    // == Punctuation
    punctuation: SyntaxHighlightStyle
    /** (, [, {...*/
    "punctuation.bracket": SyntaxHighlightStyle,
    // ., ;
    "punctuation.delimiter": SyntaxHighlightStyle,
    // js, ts: ${, } in a template literal
    // yaml: *, &, ---, ...
    "punctuation.special": SyntaxHighlightStyle,
    // md: list_marker_plus, list_marker_dot, etc
    "punctuation.list_marker": SyntaxHighlightStyle

    // this, ...
    // css: -- (var(--foo))
    // lua: self
    "variable.special": SyntaxHighlightStyle
    // true, false, null, nullptr
    constant: SyntaxHighlightStyle
    // css: @media, @import, @supports...
    // js: declare, implements, interface, keyof, public...
    keyword: SyntaxHighlightStyle
    function: SyntaxHighlightStyle
    type: SyntaxHighlightStyle
    constructor: SyntaxHighlightStyle
    variant: SyntaxHighlightStyle
    // css: class_name, property_name, namespace_name...
    property: SyntaxHighlightStyle
    // note: js enum is currently defined as a keyword
    enum: SyntaxHighlightStyle
    // -, --, ->, !=, &&, ||, <=...
    operator: SyntaxHighlightStyle
    string: SyntaxHighlightStyle
    number: SyntaxHighlightStyle
    boolean: SyntaxHighlightStyle

    // c: statement_identifier, 
    label: SyntaxHighlightStyle,
    // css: tag_name, nesting_selector, universal_selector...
    tag: SyntaxHighlightStyle,
    // css: attribute, pseudo_element_selector (tag_name), 
    attribute: SyntaxHighlightStyle,
    // css: color_value
    // js: this, super
    // racket: regex
    // toml: offset_date_time, local_date_time...
    "string.special": SyntaxHighlightStyle,
    // elixir: atom, quoted_atom, keyword, quoted_keyword
    // ruby: simple_symbol, delimited_symbol...
    "string.special.symbol": SyntaxHighlightStyle
    // ruby: Regular expression
    "string.special.regex": SyntaxHighlightStyle
    // elixir, python, yaml...: escape_sequence
    "string.escape": SyntaxHighlightStyle,
    // Regular expressions
    "string.regex": SyntaxHighlightStyle,
    // elixir: doc comment
    "comment.doc": SyntaxHighlightStyle,
    // js: predefined_type
    "type.builtin": SyntaxHighlightStyle,
    // elixir, python: interpolation (ex: foo in ${foo})
    // js: template_substitution
    embedded: SyntaxHighlightStyle,
    // elixir: __MODULE__, __DIR__, __ENV__, etc
    // go: nil, iota
    "constant.builtin": SyntaxHighlightStyle,
    // go: call_expression, method_declaration
    // js: call_expression, method_definition, pair (key, arrow function)
    "function.method": SyntaxHighlightStyle,
    // ruby: identifier/"defined?" // Nate note: I don't fully understand this one.
    "function.method.builtin": SyntaxHighlightStyle
    // lua: function_call
    "function.call": SyntaxHighlightStyle
    // lua: assert, error, loadfile, tostring, unpack...
    "function.builtin": SyntaxHighlightStyle
    // lua: hash_bang_line
    preproc: SyntaxHighlightStyle

    // md: indented_code_block, fenced_code_block, code_span
    "text.literal": SyntaxHighlightStyle
    // racket: lang_name
    "variable.builtin": SyntaxHighlightStyle
    // rust: function_item name: (identifier)
    "function.definition": SyntaxHighlightStyle
    // rust: macro_definition name: (identifier) 
    "function.special.definition": SyntaxHighlightStyle
}

// HACK: "constructor" as a key in the syntax interface returns an error when a theme tries to use it.
// For now hack around it by omiting constructor as a valid key for overrides.
export type ThemeSyntax = Partial<Omit<Syntax, "constructor">>
