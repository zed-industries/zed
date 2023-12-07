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
    pub fn to_vscode(&self) -> Vec<&'static str> {
        match self {
            ZedSyntaxToken::Attribute => vec!["entity.other.attribute-name"],
            ZedSyntaxToken::Boolean => vec!["constant.language"],
            ZedSyntaxToken::Comment => vec!["comment"],
            ZedSyntaxToken::CommentDoc => vec!["comment.block.documentation"],
            ZedSyntaxToken::Constant => vec!["constant.character"],
            ZedSyntaxToken::Constructor => {
                vec!["entity.name.function.definition.special.constructor"]
            }
            ZedSyntaxToken::Embedded => vec!["meta.embedded"],
            ZedSyntaxToken::Emphasis => vec!["markup.italic"],
            ZedSyntaxToken::EmphasisStrong => vec![
                "markup.bold",
                "markup.italic markup.bold",
                "markup.bold markup.italic",
            ],
            ZedSyntaxToken::Enum => vec!["support.type.enum"],
            ZedSyntaxToken::Function => vec![
                "entity.name.function",
                "variable.function",
                "support.function",
            ],
            ZedSyntaxToken::Keyword => vec!["keyword"],
            ZedSyntaxToken::Label => vec![
                "label",
                "entity.name",
                "entity.name.import",
                "entity.name.package",
            ],
            ZedSyntaxToken::LinkText => vec!["markup.underline.link", "string.other.link"],
            ZedSyntaxToken::LinkUri => vec!["markup.underline.link", "string.other.link"],
            ZedSyntaxToken::Number => vec!["constant.numeric", "number"],
            ZedSyntaxToken::Operator => vec!["operator", "keyword.operator"],
            ZedSyntaxToken::Preproc => vec!["preproc"],
            ZedSyntaxToken::Property => vec![
                "variable.member",
                "support.type.property-name",
                "variable.object.property",
                "variable.other.field",
            ],
            ZedSyntaxToken::Punctuation => vec![
                "punctuation",
                "punctuation.section",
                "punctuation.accessor",
                "punctuation.separator",
                "punctuation.terminator",
                "punctuation.definition.tag",
            ],
            ZedSyntaxToken::PunctuationBracket => vec![
                "punctuation.bracket",
                "punctuation.definition.tag.begin",
                "punctuation.definition.tag.end",
            ],
            ZedSyntaxToken::PunctuationDelimiter => vec![
                "punctuation.delimiter",
                "punctuation.separator",
                "punctuation.terminator",
            ],
            ZedSyntaxToken::PunctuationListMarker => {
                vec!["markup.list punctuation.definition.list.begin"]
            }
            ZedSyntaxToken::PunctuationSpecial => vec!["punctuation.special"],
            ZedSyntaxToken::String => vec!["string"],
            ZedSyntaxToken::StringEscape => {
                vec!["string.escape", "constant.character", "constant.other"]
            }
            ZedSyntaxToken::StringRegex => vec!["string.regex"],
            ZedSyntaxToken::StringSpecial => vec!["string.special", "constant.other.symbol"],
            ZedSyntaxToken::StringSpecialSymbol => {
                vec!["string.special.symbol", "constant.other.symbol"]
            }
            ZedSyntaxToken::Tag => vec!["tag", "entity.name.tag", "meta.tag.sgml"],
            ZedSyntaxToken::TextLiteral => vec!["text.literal", "string"],
            ZedSyntaxToken::Title => vec!["title", "entity.name"],
            ZedSyntaxToken::Type => vec!["entity.name.type", "support.type", "support.class"],
            ZedSyntaxToken::Variable => vec![
                "variable",
                "variable.language",
                "variable.member",
                "variable.parameter.function-call",
            ],
            ZedSyntaxToken::VariableSpecial => vec![
                "variable.special",
                "variable.member",
                "variable.annotation",
                "variable.language",
            ],
            ZedSyntaxToken::Variant => vec!["variant"],
            _ => vec![],
        }
    }
}
