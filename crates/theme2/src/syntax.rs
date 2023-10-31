use gpui2::{FontWeight, Hsla, SharedString};
use indexmap::IndexMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SyntaxStyleName {
    Comment,
    CommentDoc,
    Primary,
    Predictive,
    Hint,
    Emphasis,
    EmphasisStrong,
    Title,
    LinkUri,
    LinkText,
    TextLiteral,
    Punctuation,
    PunctuationBracket,
    PunctuationDelimiter,
    PunctuationSpecial,
    PunctuationListMarker,
    String,
    StringSpecial,
    StringSpecialSymbol,
    StringEscape,
    StringRegex,
    Constructor,
    Variant,
    Type,
    TypeBuiltin,
    Variable,
    VariableSpecial,
    Label,
    Tag,
    Attribute,
    Property,
    Constant,
    Keyword,
    Enum,
    Operator,
    Number,
    Boolean,
    ConstantBuiltin,
    Function,
    FunctionBuiltin,
    FunctionDefinition,
    FunctionSpecialDefinition,
    FunctionMethod,
    FunctionMethodBuiltin,
    Preproc,
    Embedded,
    Custom(SharedString),
}

impl std::str::FromStr for SyntaxStyleName {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "attribute" => Self::Attribute,
            "boolean" => Self::Boolean,
            "comment" => Self::Comment,
            "comment.doc" => Self::CommentDoc,
            "constant" => Self::Constant,
            "constructor" => Self::Constructor,
            "embedded" => Self::Embedded,
            "emphasis" => Self::Emphasis,
            "emphasis.strong" => Self::EmphasisStrong,
            "enum" => Self::Enum,
            "function" => Self::Function,
            "function.builtin" => Self::FunctionBuiltin,
            "function.definition" => Self::FunctionDefinition,
            "function.special_definition" => Self::FunctionSpecialDefinition,
            "function.method" => Self::FunctionMethod,
            "function.method_builtin" => Self::FunctionMethodBuiltin,
            "hint" => Self::Hint,
            "keyword" => Self::Keyword,
            "label" => Self::Label,
            "link_text" => Self::LinkText,
            "link_uri" => Self::LinkUri,
            "number" => Self::Number,
            "operator" => Self::Operator,
            "predictive" => Self::Predictive,
            "preproc" => Self::Preproc,
            "primary" => Self::Primary,
            "property" => Self::Property,
            "punctuation" => Self::Punctuation,
            "punctuation.bracket" => Self::PunctuationBracket,
            "punctuation.delimiter" => Self::PunctuationDelimiter,
            "punctuation.list_marker" => Self::PunctuationListMarker,
            "punctuation.special" => Self::PunctuationSpecial,
            "string" => Self::String,
            "string.escape" => Self::StringEscape,
            "string.regex" => Self::StringRegex,
            "string.special" => Self::StringSpecial,
            "string.special.symbol" => Self::StringSpecialSymbol,
            "tag" => Self::Tag,
            "text.literal" => Self::TextLiteral,
            "title" => Self::Title,
            "type" => Self::Type,
            "type.builtin" => Self::TypeBuiltin,
            "variable" => Self::Variable,
            "variable.special" => Self::VariableSpecial,
            "constant.builtin" => Self::ConstantBuiltin,
            "variant" => Self::Variant,
            name => Self::Custom(name.to_string().into()),
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SyntaxStyle {
    pub color: Hsla,
    pub weight: FontWeight,
    pub underline: bool,
    pub italic: bool,
    // Nate: In the future I'd like to enable using background highlights for syntax highlighting
    // pub highlight: Hsla,
}

impl SyntaxStyle {
    pub fn builder() -> SyntaxStyleBuilder {
        SyntaxStyleBuilder::new()
    }
}

impl Default for SyntaxStyle {
    fn default() -> Self {
        Self {
            color: gpui2::black(),
            weight: FontWeight::default(),
            italic: false,
            underline: false,
        }
    }
}

pub struct SyntaxStyleBuilder {
    pub color: Hsla,
    pub weight: FontWeight,
    pub underline: bool,
    pub italic: bool,
}

impl SyntaxStyleBuilder {
    pub fn new() -> Self {
        SyntaxStyleBuilder {
            color: gpui2::black(),
            weight: FontWeight::default(),
            underline: false,
            italic: false,
        }
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = color;
        self
    }

    pub fn weight(mut self, weight: FontWeight) -> Self {
        self.weight = weight;
        self
    }

    pub fn underline(mut self, underline: bool) -> Self {
        self.underline = underline;
        self
    }

    pub fn italic(mut self, italic: bool) -> Self {
        self.italic = italic;
        self
    }

    pub fn build(self) -> SyntaxStyle {
        SyntaxStyle {
            color: self.color,
            weight: self.weight,
            underline: self.underline,
            italic: self.italic,
        }
    }
}

pub struct SyntaxStyles(pub IndexMap<SyntaxStyleName, SyntaxStyle>);

impl SyntaxStyles {
    // TOOD: Get this working with `#[cfg(test)]`. Why isn't it?
    pub fn new_test(colors: impl IntoIterator<Item = (&'static str, Hsla)>) -> Self {
        Self(IndexMap::from_iter(colors.into_iter().map(
            |(name, color)| {
                (
                    name.parse().unwrap(),
                    SyntaxStyle::builder().color(color).build(),
                )
            },
        )))
    }

    pub fn get(&self, name: &str) -> SyntaxStyle {
        self.0
            .get(&name.parse::<SyntaxStyleName>().unwrap())
            .cloned()
            .unwrap_or_default()
    }

    pub fn color(&self, name: &str) -> Hsla {
        self.get(name).color
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_syntax_style_name() {
        let name = "comment".parse::<SyntaxStyleName>().unwrap();
        assert_eq!(name, SyntaxStyleName::Comment);
    }

    #[test]
    fn create_custom_syntax_style_name() {
        let name = "custom".parse::<SyntaxStyleName>().unwrap();
        assert_eq!(name, SyntaxStyleName::Custom("custom".into()));
    }
}
