use gpui2::{FontWeight, Hsla};
use indexmap::IndexMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SyntaxColorName {
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

pub struct SyntaxStyles(pub IndexMap<SyntaxColorName, SyntaxStyle>);
