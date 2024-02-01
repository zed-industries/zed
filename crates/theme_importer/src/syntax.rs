use strum::EnumIter;

#[derive(Debug, PartialEq, Copy, Clone, EnumIter)]
pub enum ZedSyntaxToken {
    Attribute,
    Boolean,
    Comment,
    CommentDoc,
    Constant,
    Constructor,
    Embedded,
    Emphasis,
    EmphasisStrong,
    Enum,
    Function,
    Hint,
    Keyword,
    Label,
    LinkText,
    LinkUri,
    Number,
    Operator,
    Predictive,
    Preproc,
    Primary,
    Property,
    Punctuation,
    PunctuationBracket,
    PunctuationDelimiter,
    PunctuationListMarker,
    PunctuationSpecial,
    String,
    StringEscape,
    StringRegex,
    StringSpecial,
    StringSpecialSymbol,
    Tag,
    TextLiteral,
    Title,
    Type,
    Variable,
    VariableSpecial,
    Variant,
}

impl std::fmt::Display for ZedSyntaxToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ZedSyntaxToken::Attribute => "attribute",
                ZedSyntaxToken::Boolean => "boolean",
                ZedSyntaxToken::Comment => "comment",
                ZedSyntaxToken::CommentDoc => "comment.doc",
                ZedSyntaxToken::Constant => "constant",
                ZedSyntaxToken::Constructor => "constructor",
                ZedSyntaxToken::Embedded => "embedded",
                ZedSyntaxToken::Emphasis => "emphasis",
                ZedSyntaxToken::EmphasisStrong => "emphasis.strong",
                ZedSyntaxToken::Enum => "enum",
                ZedSyntaxToken::Function => "function",
                ZedSyntaxToken::Hint => "hint",
                ZedSyntaxToken::Keyword => "keyword",
                ZedSyntaxToken::Label => "label",
                ZedSyntaxToken::LinkText => "link_text",
                ZedSyntaxToken::LinkUri => "link_uri",
                ZedSyntaxToken::Number => "number",
                ZedSyntaxToken::Operator => "operator",
                ZedSyntaxToken::Predictive => "predictive",
                ZedSyntaxToken::Preproc => "preproc",
                ZedSyntaxToken::Primary => "primary",
                ZedSyntaxToken::Property => "property",
                ZedSyntaxToken::Punctuation => "punctuation",
                ZedSyntaxToken::PunctuationBracket => "punctuation.bracket",
                ZedSyntaxToken::PunctuationDelimiter => "punctuation.delimiter",
                ZedSyntaxToken::PunctuationListMarker => "punctuation.list_marker",
                ZedSyntaxToken::PunctuationSpecial => "punctuation.special",
                ZedSyntaxToken::String => "string",
                ZedSyntaxToken::StringEscape => "string.escape",
                ZedSyntaxToken::StringRegex => "string.regex",
                ZedSyntaxToken::StringSpecial => "string.special",
                ZedSyntaxToken::StringSpecialSymbol => "string.special.symbol",
                ZedSyntaxToken::Tag => "tag",
                ZedSyntaxToken::TextLiteral => "text.literal",
                ZedSyntaxToken::Title => "title",
                ZedSyntaxToken::Type => "type",
                ZedSyntaxToken::Variable => "variable",
                ZedSyntaxToken::VariableSpecial => "variable.special",
                ZedSyntaxToken::Variant => "variant",
            }
        )
    }
}

impl ZedSyntaxToken {
    pub fn fallbacks(&self) -> &[Self] {
        match self {
            ZedSyntaxToken::CommentDoc => &[ZedSyntaxToken::Comment],
            ZedSyntaxToken::Number => &[ZedSyntaxToken::Constant],
            ZedSyntaxToken::VariableSpecial => &[ZedSyntaxToken::Variable],
            ZedSyntaxToken::PunctuationBracket
            | ZedSyntaxToken::PunctuationDelimiter
            | ZedSyntaxToken::PunctuationListMarker
            | ZedSyntaxToken::PunctuationSpecial => &[ZedSyntaxToken::Punctuation],
            ZedSyntaxToken::StringEscape
            | ZedSyntaxToken::StringRegex
            | ZedSyntaxToken::StringSpecial
            | ZedSyntaxToken::StringSpecialSymbol => &[ZedSyntaxToken::String],
            _ => &[],
        }
    }
}
