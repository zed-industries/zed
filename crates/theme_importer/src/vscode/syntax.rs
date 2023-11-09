// Create ThemeSyntaxRefinement
// Map tokenColors style to HighlightStyle (fontStyle, foreground, background)
// Take in the scopes from the tokenColors and try to match each to our HighlightStyles

use anyhow::Result;
use indexmap::IndexMap;
use serde::Deserialize;
use theme::UserHighlightStyle;

use crate::util::Traverse;
use crate::vscode::try_parse_color;

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

impl VsCodeTokenColor {
    pub fn highlight_styles(&self) -> Result<IndexMap<String, UserHighlightStyle>> {
        let mut highlight_styles = IndexMap::new();

        let scope = match self.scope {
            Some(VsCodeTokenScope::One(ref scope)) => vec![scope.clone()],
            Some(VsCodeTokenScope::Many(ref scopes)) => scopes.clone(),
            None => return Ok(IndexMap::new()),
        };

        for scope in &scope {
            let Some(syntax_token) = Self::to_zed_token(&scope) else {
                continue;
            };

            let highlight_style = UserHighlightStyle {
                color: self
                    .settings
                    .foreground
                    .as_ref()
                    .traverse(|color| try_parse_color(&color))?,
            };

            if highlight_style.is_empty() {
                continue;
            }

            highlight_styles.insert(syntax_token, highlight_style);
        }

        Ok(highlight_styles)
    }

    fn to_zed_token(scope: &str) -> Option<String> {
        match scope {
            "attribute" => Some("attribute".to_string()),
            "boolean" => Some("boolean".to_string()),
            "comment" => Some("comment".to_string()),
            "comment.doc" => Some("comment.doc".to_string()),
            "punctuation"
            | "punctuation.accessor"
            | "punctuation.definition.array.begin.json"
            | "punctuation.definition.array.end.json"
            | "punctuation.definition.dictionary.begin.json"
            | "punctuation.definition.dictionary.end.json"
            | "punctuation.definition.markdown"
            | "punctuation.definition.tag"
            | "punctuation.definition.tag.begin"
            | "punctuation.definition.tag.end"
            | "punctuation.definition.template-expression"
            | "punctuation.definition.variable"
            | "punctuation.section"
            | "punctuation.section.embedded"
            | "punctuation.section.embedded.begin"
            | "punctuation.section.embedded.end"
            | "punctuation.separator"
            | "punctuation.separator.array.json"
            | "punctuation.separator.dictionary.key-value.json"
            | "punctuation.separator.dictionary.pair.json" => Some("punctuation".to_string()),

            // ---
            "constant" | "character" | "language" | "language.python" | "numeric" | "other"
            | "other.symbol" => Some("something".to_string()),

            "entity"
            | "name"
            | "name.class"
            | "name.filename.find-in-files"
            | "name.function"
            | "name.function.python"
            | "name.import"
            | "name.package"
            | "name.tag"
            | "name.type"
            | "name.type.class.python"
            | "other.attribute-name"
            | "other.inherited-class" => Some("something".to_string()),

            "markup" | "bold" | "changed" | "deleted" | "heading" | "heading.setext"
            | "inline.raw" | "italic" | "list" | "quote" | "raw" | "raw.inline" | "strike"
            | "table" | "underline.link" => Some("something".to_string()),

            "source" => Some("something".to_string()),
            "storage" => Some("something".to_string()),
            "string" => Some("something".to_string()),
            "support" => Some("something".to_string()),
            "text" => Some("something".to_string()),
            "token" => Some("something".to_string()),
            "variable" => Some("something".to_string()),
            _ => None,
        }
    }
}

// "comment" => ""
// "constant.character" => ""
// "constant.language" => ""
// "constant.language.python" => ""
// "constant.numeric" => ""
// "constant.numeric.line-number.find-in-files - match" => ""
// "constant.numeric.line-number.match" => ""
// "constant.other" => ""
// "constant.other.symbol" => ""
// "entity.name" => ""
// "entity.name.class" => ""
// "entity.name.filename.find-in-files" => ""
// "entity.name.function" => ""
// "entity.name.function.python" => ""
// "entity.name.import" => ""
// "entity.name.package" => ""
// "entity.name.tag" => ""
// "entity.name.type" => ""
// "entity.name.type.class.python" => ""
// "entity.other.attribute-name" => ""
// "entity.other.inherited-class" => ""
// "invalid" => ""
// "keyword" => ""
// "keyword.control.from" => ""
// "keyword.control.import" => ""
// "keyword.operator" => ""
// "keyword.other.new" => ""
// "markup.bold markup.italic" => ""
// "markup.bold" => ""
// "markup.changed" => ""
// "markup.deleted" => ""
// "markup.heading entity.name" => ""
// "markup.heading" => ""
// "markup.heading.setext" => ""
// "markup.inline.raw" => ""
// "markup.inserted" => ""
// "markup.inserted" => ""
// "markup.italic markup.bold" => ""
// "markup.italic" => ""
// "markup.list punctuation.definition.list.begin" => ""
// "markup.list" => ""
// "markup.quote" => ""
// "markup.raw" => ""
// "markup.raw.inline" => ""
// "markup.strike" => ""
// "markup.table" => ""
// "markup.underline.link" => ""
// "message.error" => ""
// "meta.decorator punctuation.decorator" => ""
// "meta.decorator variable.other" => ""
// "meta.diff" => ""
// "meta.diff.header" => ""
// "meta.embedded" => ""
// "meta.function-call" => ""
// "meta.function-call.generic" => ""
// "meta.import" => ""
// "meta.parameter" => ""
// "meta.preprocessor" => ""
// "meta.separator" => ""
// "meta.tag.sgml" => ""
// "punctuation.accessor" => ""
// "punctuation.definition.array.begin.json" => ""
// "punctuation.definition.array.end.json" => ""
// "punctuation.definition.dictionary.begin.json" => ""
// "punctuation.definition.dictionary.end.json" => ""
// "punctuation.definition.markdown" => ""
// "punctuation.definition.tag" => ""
// "punctuation.definition.tag.begin" => ""
// "punctuation.definition.tag.end" => ""
// "punctuation.definition.template-expression" => ""
// "punctuation.definition.variable" => ""
// "punctuation.section" => ""
// "punctuation.section.embedded" => ""
// "punctuation.section.embedded.begin" => ""
// "punctuation.section.embedded.end" => ""
// "punctuation.separator" => ""
// "punctuation.separator.array.json" => ""
// "punctuation.separator.dictionary.key-value.json" => ""
// "punctuation.separator.dictionary.pair.json" => ""
// "punctuation.terminator" => ""
// "source.c storage.type" => ""
// "source.css entity.name.tag" => ""
// "source.css support.type" => ""
// "source.go storage.type" => ""
// "source.groovy.embedded" => ""
// "source.haskell storage.type" => ""
// "source.java storage.type" => ""
// "source.java storage.type.primitive" => ""
// "source.less entity.name.tag" => ""
// "source.less support.type" => ""
// "source.python" => ""
// "source.ruby variable.other.readwrite" => ""
// "source.sass entity.name.tag" => ""
// "source.sass support.type" => ""
// "source.scss entity.name.tag" => ""
// "source.scss support.type" => ""
// "source.stylus entity.name.tag" => ""
// "source.stylus support.type" => ""
// "source.ts" => ""
// "storage" => ""
// "storage.modifier" => ""
// "storage.modifier.async" => ""
// "storage.modifier.tsx" => ""
// "storage.type.annotation" => ""
// "storage.type.function" => ""
// "string" => ""
// "string.other.link" => ""
// "string.regexp" => ""
// "support.class" => ""
// "support.class.component" => ""
// "support.constant" => ""
// "support.function" => ""
// "support.function.construct" => ""
// "support.function.go" => ""
// "support.macro" => ""
// "support.other.variable" => ""
// "support.type" => ""
// "support.type.exception" => ""
// "support.type.primitive" => ""
// "support.type.property-name" => ""
// "support.type.python" => ""
// "text.html.markdown markup.inline.raw" => ""
// "text.html.markdown meta.dummy.line-break" => ""
// "token.debug-token" => ""
// "token.error-token" => ""
// "token.info-token" => ""
// "token.warn-token" => ""
// "variable" => ""
// "variable.annotation" => ""
// "variable.function" => ""
// "variable.language" => ""
// "variable.member" => ""
// "variable.object.property" => ""
// "variable.other" => ""
// "variable.parameter" => ""
// "variable.parameter.function-call" => ""
