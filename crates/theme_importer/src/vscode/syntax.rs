use indexmap::IndexMap;
use serde::Deserialize;
use strum::EnumIter;

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
pub enum VsCodeTokenScope {
    One(String),
    Many(Vec<String>),
}

#[derive(Debug, Deserialize)]
pub struct VsCodeTokenColor {
    pub name: Option<String>,
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
pub enum CodeOrbitSyntaxToken {
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

impl std::fmt::Display for CodeOrbitSyntaxToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CodeOrbitSyntaxToken::Attribute => "attribute",
                CodeOrbitSyntaxToken::Boolean => "boolean",
                CodeOrbitSyntaxToken::Comment => "comment",
                CodeOrbitSyntaxToken::CommentDoc => "comment.doc",
                CodeOrbitSyntaxToken::Constant => "constant",
                CodeOrbitSyntaxToken::Constructor => "constructor",
                CodeOrbitSyntaxToken::Embedded => "embedded",
                CodeOrbitSyntaxToken::Emphasis => "emphasis",
                CodeOrbitSyntaxToken::EmphasisStrong => "emphasis.strong",
                CodeOrbitSyntaxToken::Enum => "enum",
                CodeOrbitSyntaxToken::Function => "function",
                CodeOrbitSyntaxToken::Hint => "hint",
                CodeOrbitSyntaxToken::Keyword => "keyword",
                CodeOrbitSyntaxToken::Label => "label",
                CodeOrbitSyntaxToken::LinkText => "link_text",
                CodeOrbitSyntaxToken::LinkUri => "link_uri",
                CodeOrbitSyntaxToken::Number => "number",
                CodeOrbitSyntaxToken::Operator => "operator",
                CodeOrbitSyntaxToken::Predictive => "predictive",
                CodeOrbitSyntaxToken::Preproc => "preproc",
                CodeOrbitSyntaxToken::Primary => "primary",
                CodeOrbitSyntaxToken::Property => "property",
                CodeOrbitSyntaxToken::Punctuation => "punctuation",
                CodeOrbitSyntaxToken::PunctuationBracket => "punctuation.bracket",
                CodeOrbitSyntaxToken::PunctuationDelimiter => "punctuation.delimiter",
                CodeOrbitSyntaxToken::PunctuationListMarker => "punctuation.list_marker",
                CodeOrbitSyntaxToken::PunctuationSpecial => "punctuation.special",
                CodeOrbitSyntaxToken::String => "string",
                CodeOrbitSyntaxToken::StringEscape => "string.escape",
                CodeOrbitSyntaxToken::StringRegex => "string.regex",
                CodeOrbitSyntaxToken::StringSpecial => "string.special",
                CodeOrbitSyntaxToken::StringSpecialSymbol => "string.special.symbol",
                CodeOrbitSyntaxToken::Tag => "tag",
                CodeOrbitSyntaxToken::TextLiteral => "text.literal",
                CodeOrbitSyntaxToken::Title => "title",
                CodeOrbitSyntaxToken::Type => "type",
                CodeOrbitSyntaxToken::Variable => "variable",
                CodeOrbitSyntaxToken::VariableSpecial => "variable.special",
                CodeOrbitSyntaxToken::Variant => "variant",
            }
        )
    }
}

impl CodeOrbitSyntaxToken {
    pub fn find_best_token_color_match<'a>(
        &self,
        token_colors: &'a [VsCodeTokenColor],
    ) -> Option<&'a VsCodeTokenColor> {
        let mut ranked_matches = IndexMap::new();

        for (ix, token_color) in token_colors.iter().enumerate() {
            if token_color.settings.foreground.is_none() {
                continue;
            }

            let Some(rank) = self.rank_match(token_color) else {
                continue;
            };

            if rank > 0 {
                ranked_matches.insert(ix, rank);
            }
        }

        ranked_matches
            .into_iter()
            .max_by_key(|(_, rank)| *rank)
            .map(|(ix, _)| &token_colors[ix])
    }

    fn rank_match(&self, token_color: &VsCodeTokenColor) -> Option<u32> {
        let candidate_scopes = match token_color.scope.as_ref()? {
            VsCodeTokenScope::One(scope) => vec![scope],
            VsCodeTokenScope::Many(scopes) => scopes.iter().collect(),
        }
        .iter()
        .flat_map(|scope| scope.split(',').map(|s| s.trim()))
        .collect::<Vec<_>>();

        let scopes_to_match = self.to_vscode();
        let number_of_scopes_to_match = scopes_to_match.len();

        let mut matches = 0;

        for (ix, scope) in scopes_to_match.into_iter().enumerate() {
            // Assign each entry a weight that is inversely proportional to its
            // position in the list.
            //
            // Entries towards the front are weighted higher than those towards the end.
            let weight = (number_of_scopes_to_match - ix) as u32;

            if candidate_scopes.contains(&scope) {
                matches += 1 + weight;
            }
        }

        Some(matches)
    }

    pub fn fallbacks(&self) -> &[Self] {
        match self {
            CodeOrbitSyntaxToken::CommentDoc => &[CodeOrbitSyntaxToken::Comment],
            CodeOrbitSyntaxToken::Number => &[CodeOrbitSyntaxToken::Constant],
            CodeOrbitSyntaxToken::VariableSpecial => &[CodeOrbitSyntaxToken::Variable],
            CodeOrbitSyntaxToken::PunctuationBracket
            | CodeOrbitSyntaxToken::PunctuationDelimiter
            | CodeOrbitSyntaxToken::PunctuationListMarker
            | CodeOrbitSyntaxToken::PunctuationSpecial => &[CodeOrbitSyntaxToken::Punctuation],
            CodeOrbitSyntaxToken::StringEscape
            | CodeOrbitSyntaxToken::StringRegex
            | CodeOrbitSyntaxToken::StringSpecial
            | CodeOrbitSyntaxToken::StringSpecialSymbol => &[CodeOrbitSyntaxToken::String],
            _ => &[],
        }
    }

    fn to_vscode(self) -> Vec<&'static str> {
        match self {
            CodeOrbitSyntaxToken::Attribute => vec!["entity.other.attribute-name"],
            CodeOrbitSyntaxToken::Boolean => vec!["constant.language"],
            CodeOrbitSyntaxToken::Comment => vec!["comment"],
            CodeOrbitSyntaxToken::CommentDoc => vec!["comment.block.documentation"],
            CodeOrbitSyntaxToken::Constant => vec!["constant", "constant.language", "constant.character"],
            CodeOrbitSyntaxToken::Constructor => {
                vec![
                    "entity.name.tag",
                    "entity.name.function.definition.special.constructor",
                ]
            }
            CodeOrbitSyntaxToken::Embedded => vec!["meta.embedded"],
            CodeOrbitSyntaxToken::Emphasis => vec!["markup.italic"],
            CodeOrbitSyntaxToken::EmphasisStrong => vec![
                "markup.bold",
                "markup.italic markup.bold",
                "markup.bold markup.italic",
            ],
            CodeOrbitSyntaxToken::Enum => vec!["support.type.enum"],
            CodeOrbitSyntaxToken::Function => vec![
                "entity.function",
                "entity.name.function",
                "variable.function",
            ],
            CodeOrbitSyntaxToken::Hint => vec![],
            CodeOrbitSyntaxToken::Keyword => vec![
                "keyword",
                "keyword.other.fn.rust",
                "keyword.control",
                "keyword.control.fun",
                "keyword.control.class",
                "punctuation.accessor",
                "entity.name.tag",
            ],
            CodeOrbitSyntaxToken::Label => vec![
                "label",
                "entity.name",
                "entity.name.import",
                "entity.name.package",
            ],
            CodeOrbitSyntaxToken::LinkText => vec!["markup.underline.link", "string.other.link"],
            CodeOrbitSyntaxToken::LinkUri => vec!["markup.underline.link", "string.other.link"],
            CodeOrbitSyntaxToken::Number => vec!["constant.numeric", "number"],
            CodeOrbitSyntaxToken::Operator => vec!["operator", "keyword.operator"],
            CodeOrbitSyntaxToken::Predictive => vec![],
            CodeOrbitSyntaxToken::Preproc => vec![
                "preproc",
                "meta.preprocessor",
                "punctuation.definition.preprocessor",
            ],
            CodeOrbitSyntaxToken::Primary => vec![],
            CodeOrbitSyntaxToken::Property => vec![
                "variable.member",
                "support.type.property-name",
                "variable.object.property",
                "variable.other.field",
            ],
            CodeOrbitSyntaxToken::Punctuation => vec![
                "punctuation",
                "punctuation.section",
                "punctuation.accessor",
                "punctuation.separator",
                "punctuation.definition.tag",
            ],
            CodeOrbitSyntaxToken::PunctuationBracket => vec![
                "punctuation.bracket",
                "punctuation.definition.tag.begin",
                "punctuation.definition.tag.end",
            ],
            CodeOrbitSyntaxToken::PunctuationDelimiter => vec![
                "punctuation.delimiter",
                "punctuation.separator",
                "punctuation.terminator",
            ],
            CodeOrbitSyntaxToken::PunctuationListMarker => {
                vec!["markup.list punctuation.definition.list.begin"]
            }
            CodeOrbitSyntaxToken::PunctuationSpecial => vec!["punctuation.special"],
            CodeOrbitSyntaxToken::String => vec!["string"],
            CodeOrbitSyntaxToken::StringEscape => {
                vec!["string.escape", "constant.character", "constant.other"]
            }
            CodeOrbitSyntaxToken::StringRegex => vec!["string.regex"],
            CodeOrbitSyntaxToken::StringSpecial => vec!["string.special", "constant.other.symbol"],
            CodeOrbitSyntaxToken::StringSpecialSymbol => {
                vec!["string.special.symbol", "constant.other.symbol"]
            }
            CodeOrbitSyntaxToken::Tag => vec!["tag", "entity.name.tag", "meta.tag.sgml"],
            CodeOrbitSyntaxToken::TextLiteral => vec!["text.literal", "string"],
            CodeOrbitSyntaxToken::Title => vec!["title", "entity.name"],
            CodeOrbitSyntaxToken::Type => vec![
                "entity.name.type",
                "entity.name.type.primitive",
                "entity.name.type.numeric",
                "keyword.type",
                "support.type",
                "support.type.primitive",
                "support.class",
            ],
            CodeOrbitSyntaxToken::Variable => vec![
                "variable",
                "variable.language",
                "variable.member",
                "variable.parameter",
                "variable.parameter.function-call",
            ],
            CodeOrbitSyntaxToken::VariableSpecial => vec![
                "variable.special",
                "variable.member",
                "variable.annotation",
                "variable.language",
            ],
            CodeOrbitSyntaxToken::Variant => vec!["variant"],
        }
    }
}
