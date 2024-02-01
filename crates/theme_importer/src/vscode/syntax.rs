use indexmap::IndexMap;
use serde::Deserialize;

use crate::syntax::ZedSyntaxToken;

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

impl ZedSyntaxToken {
    pub fn find_best_vscode_token_color_match<'a>(
        &self,
        token_colors: &'a [VsCodeTokenColor],
    ) -> Option<&'a VsCodeTokenColor> {
        let mut ranked_matches = IndexMap::new();

        for (ix, token_color) in token_colors.iter().enumerate() {
            if token_color.settings.foreground.is_none() {
                continue;
            }

            let Some(rank) = self.rank_vscode_match(token_color) else {
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

    fn rank_vscode_match(&self, token_color: &VsCodeTokenColor) -> Option<u32> {
        let candidate_scopes = match token_color.scope.as_ref()? {
            VsCodeTokenScope::One(scope) => vec![scope],
            VsCodeTokenScope::Many(scopes) => scopes.iter().collect(),
        }
        .iter()
        .map(|scope| scope.as_str())
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

    pub fn to_vscode(&self) -> Vec<&'static str> {
        match self {
            ZedSyntaxToken::Attribute => vec!["entity.other.attribute-name"],
            ZedSyntaxToken::Boolean => vec!["constant.language"],
            ZedSyntaxToken::Comment => vec!["comment"],
            ZedSyntaxToken::CommentDoc => vec!["comment.block.documentation"],
            ZedSyntaxToken::Constant => vec!["constant", "constant.language", "constant.character"],
            ZedSyntaxToken::Constructor => {
                vec![
                    "entity.name.tag",
                    "entity.name.function.definition.special.constructor",
                ]
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
                "entity.function",
                "entity.name.function",
                "variable.function",
            ],
            ZedSyntaxToken::Hint => vec![],
            ZedSyntaxToken::Keyword => vec![
                "keyword",
                "keyword.other.fn.rust",
                "keyword.control",
                "keyword.control.fun",
                "keyword.control.class",
                "punctuation.accessor",
                "entity.name.tag",
            ],
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
            ZedSyntaxToken::Predictive => vec![],
            ZedSyntaxToken::Preproc => vec![
                "preproc",
                "meta.preprocessor",
                "punctuation.definition.preprocessor",
            ],
            ZedSyntaxToken::Primary => vec![],
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
            ZedSyntaxToken::Type => vec![
                "entity.name.type",
                "entity.name.type.primitive",
                "entity.name.type.numeric",
                "keyword.type",
                "support.type",
                "support.type.primitive",
                "support.class",
            ],
            ZedSyntaxToken::Variable => vec![
                "variable",
                "variable.language",
                "variable.member",
                "variable.parameter",
                "variable.parameter.function-call",
            ],
            ZedSyntaxToken::VariableSpecial => vec![
                "variable.special",
                "variable.member",
                "variable.annotation",
                "variable.language",
            ],
            ZedSyntaxToken::Variant => vec!["variant"],
        }
    }
}
