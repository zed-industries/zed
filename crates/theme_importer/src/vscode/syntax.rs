use serde::Deserialize;
use strum::EnumIter;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum VsCodeTokenScope {
    One(String),
    Many(Vec<String>),
}

impl VsCodeTokenScope {
    pub fn multimatch(&self, matches: &[&'static str]) -> bool {
        match self {
            VsCodeTokenScope::One(scope) => matches.iter().any(|&s| s == scope),
            VsCodeTokenScope::Many(scopes) => {
                matches.iter().any(|s| scopes.contains(&s.to_string()))
            }
        }
    }
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
    pub fn to_vscode(&self) -> Vec<&'static str> {
        use ZedSyntaxToken::*;

        match self {
            SyntaxAttribute => vec!["entity.other.attribute-name"],
            SyntaxBoolean => vec!["constant.language"],
            SyntaxComment => vec!["comment"],
            SyntaxCommentDoc => vec!["comment.block.documentation"],
            SyntaxConstant => vec!["constant.character"],
            SyntaxConstructor => vec!["entity.name.function.definition.special.constructor"],
            SyntaxEmbedded => vec!["meta.embedded"],
            SyntaxEmphasis => vec!["markup.italic"],
            SyntaxEmphasisStrong => vec![
                "markup.bold",
                "markup.italic markup.bold",
                "markup.bold markup.italic",
            ],
            SyntaxEnum => vec!["support.type.enum"],
            SyntaxFunction => vec![
                "entity.name.function",
                "variable.function",
                "support.function",
            ],
            SyntaxKeyword => vec!["keyword"],
            SyntaxLabel => vec![
                "label",
                "entity.name",
                "entity.name.import",
                "entity.name.package",
            ],
            SyntaxLinkText => vec!["markup.underline.link", "string.other.link"],
            SyntaxLinkUri => vec!["markup.underline.link", "string.other.link"],
            SyntaxNumber => vec!["constant.numeric", "number"],
            SyntaxOperator => vec!["operator", "keyword.operator"],
            SyntaxPreproc => vec!["preproc"],
            SyntaxProperty => vec![
                "variable.member",
                "support.type.property-name",
                "variable.object.property",
                "variable.other.field",
            ],
            SyntaxPunctuation => vec![
                "punctuation",
                "punctuation.section",
                "punctuation.accessor",
                "punctuation.separator",
                "punctuation.terminator",
                "punctuation.definition.tag",
            ],
            SyntaxPunctuationBracket => vec![
                "punctuation.bracket",
                "punctuation.definition.tag.begin",
                "punctuation.definition.tag.end",
            ],
            SyntaxPunctuationDelimiter => vec![
                "punctuation.delimiter",
                "punctuation.separator",
                "punctuation.terminator",
            ],
            SyntaxPunctuationListMarker => vec!["markup.list punctuation.definition.list.begin"],
            SyntaxPunctuationSpecial => vec!["punctuation.special"],
            SyntaxString => vec!["string"],
            SyntaxStringEscape => vec!["string.escape", "constant.character", "constant.other"],
            SyntaxStringRegex => vec!["string.regex"],
            SyntaxStringSpecial => vec!["string.special", "constant.other.symbol"],
            SyntaxStringSpecialSymbol => vec!["string.special.symbol", "constant.other.symbol"],
            SyntaxTag => vec!["tag", "entity.name.tag", "meta.tag.sgml"],
            SyntaxTextLiteral => vec!["text.literal", "string"],
            SyntaxTitle => vec!["title", "entity.name"],
            SyntaxType => vec!["entity.name.type", "support.type", "support.class"],
            SyntaxVariable => vec![
                "variable",
                "variable.language",
                "variable.member",
                "variable.parameter.function-call",
            ],
            SyntaxVariableSpecial => vec![
                "variable.special",
                "variable.member",
                "variable.annotation",
                "variable.language",
            ],
            SyntaxVariant => vec!["variant"],
            _ => vec![],
        }
    }
}
