use serde::Deserialize;
use strum::EnumIter;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum VsCodeTokenScope {
    One(String),
    Many(Vec<String>),
}

#[derive(Debug, Deserialize)]
pub struct VsCodeTokenColor {
    pub scope: Option<VsCodeTokenScope>,
    pub settings: VsCodeTokenColorSettings,
}

#[derive(Debug, Deserialize)]
pub struct VsCodeTokenColorSettings {
    pub foreground: Option<String>,
    pub background: Option<String>,
    #[serde(rename = "fontStyle")]
    pub font_style: Option<String>,
}

#[derive(Debug, PartialEq, Copy, Clone, EnumIter)]
pub enum ZedSyntaxToken {
    SyntaxAttribute,
    SyntaxBoolean,
    SyntaxComment,
    SyntaxCommentDoc,
    SyntaxConstant,
    SyntaxConstructor,
    SyntaxEmbedded,
    SyntaxEmphasis,
    SyntaxEmphasisStrong,
    SyntaxEnum,
    SyntaxFunction,
    SyntaxHint,
    SyntaxKeyword,
    SyntaxLabel,
    SyntaxLinkText,
    SyntaxLinkUri,
    SyntaxNumber,
    SyntaxOperator,
    SyntaxPredictive,
    SyntaxPreproc,
    SyntaxPrimary,
    SyntaxProperty,
    SyntaxPunctuation,
    SyntaxPunctuationBracket,
    SyntaxPunctuationDelimiter,
    SyntaxPunctuationListMarker,
    SyntaxPunctuationSpecial,
    SyntaxString,
    SyntaxStringEscape,
    SyntaxStringRegex,
    SyntaxStringSpecial,
    SyntaxStringSpecialSymbol,
    SyntaxTag,
    SyntaxTextLiteral,
    SyntaxTitle,
    SyntaxType,
    SyntaxVariable,
    SyntaxVariableSpecial,
    SyntaxVariant,
}

impl std::fmt::Display for ZedSyntaxToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ZedSyntaxToken::*;

        write!(
            f,
            "{}",
            match self {
                SyntaxAttribute => "attribute",
                SyntaxBoolean => "boolean",
                SyntaxComment => "comment",
                SyntaxCommentDoc => "comment.doc",
                SyntaxConstant => "constant",
                SyntaxConstructor => "constructor",
                SyntaxEmbedded => "embedded",
                SyntaxEmphasis => "emphasis",
                SyntaxEmphasisStrong => "emphasis.strong",
                SyntaxEnum => "enum",
                SyntaxFunction => "function",
                SyntaxHint => "hint",
                SyntaxKeyword => "keyword",
                SyntaxLabel => "label",
                SyntaxLinkText => "link_text",
                SyntaxLinkUri => "link_uri",
                SyntaxNumber => "number",
                SyntaxOperator => "operator",
                SyntaxPredictive => "predictive",
                SyntaxPreproc => "preproc",
                SyntaxPrimary => "primary",
                SyntaxProperty => "property",
                SyntaxPunctuation => "punctuation",
                SyntaxPunctuationBracket => "punctuation.bracket",
                SyntaxPunctuationDelimiter => "punctuation.delimiter",
                SyntaxPunctuationListMarker => "punctuation.list_marker",
                SyntaxPunctuationSpecial => "punctuation.special",
                SyntaxString => "string",
                SyntaxStringEscape => "string.escape",
                SyntaxStringRegex => "string.regex",
                SyntaxStringSpecial => "string.special",
                SyntaxStringSpecialSymbol => "string.special.symbol",
                SyntaxTag => "tag",
                SyntaxTextLiteral => "text.literal",
                SyntaxTitle => "title",
                SyntaxType => "type",
                SyntaxVariable => "variable",
                SyntaxVariableSpecial => "variable.special",
                SyntaxVariant => "variant",
            }
        )
    }
}

impl ZedSyntaxToken {
    pub fn to_vscode(&self) -> &'static str {
        use ZedSyntaxToken::*;

        match self {
            SyntaxAttribute => "entity.other.attribute-name",
            SyntaxBoolean => "constant.language",
            SyntaxComment => "comment",
            SyntaxCommentDoc => "comment.block.documentation",
            SyntaxConstant => "constant.character",
            SyntaxConstructor => "entity.name.function.definition.special.constructor",
            SyntaxEmbedded => "embedded",
            SyntaxEmphasis => "emphasis",
            SyntaxEmphasisStrong => "emphasis.strong",
            SyntaxEnum => "support.type.enum",
            SyntaxFunction => "entity.name.function",
            SyntaxHint => "hint",
            SyntaxKeyword => "keyword",
            SyntaxLabel => "label",
            SyntaxLinkText => "link_text",
            SyntaxLinkUri => "link_uri",
            SyntaxNumber => "number",
            SyntaxOperator => "operator",
            SyntaxPredictive => "predictive",
            SyntaxPreproc => "preproc",
            SyntaxPrimary => "primary",
            SyntaxProperty => "variable.object.property", //"variable.other.field"
            SyntaxPunctuation => "punctuation",
            SyntaxPunctuationBracket => "punctuation.bracket",
            SyntaxPunctuationDelimiter => "punctuation.delimiter",
            SyntaxPunctuationListMarker => "punctuation.list_marker",
            SyntaxPunctuationSpecial => "punctuation.special",
            SyntaxString => "string",
            SyntaxStringEscape => "string.escape",
            SyntaxStringRegex => "string.regex",
            SyntaxStringSpecial => "string.special",
            SyntaxStringSpecialSymbol => "string.special.symbol",
            SyntaxTag => "tag",
            SyntaxTextLiteral => "text.literal",
            SyntaxTitle => "title",
            SyntaxType => "entity.name.type",
            SyntaxVariable => "variable.language",
            SyntaxVariableSpecial => "variable.special",
            SyntaxVariant => "variant",
        }
    }
}
