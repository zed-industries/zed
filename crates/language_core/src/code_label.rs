use crate::highlight_map::HighlightId;
use std::ops::Range;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum SymbolKind {
    File,
    Module,
    Namespace,
    Package,
    Class,
    Method,
    Property,
    Field,
    Constructor,
    Enum,
    Interface,
    Function,
    Variable,
    Constant,
    String,
    Number,
    Boolean,
    Array,
    Object,
    Key,
    Null,
    EnumMember,
    Struct,
    Event,
    Operator,
    TypeParameter,
}

macro_rules! proto_mapping {
    ($($kind:ident = $value:literal),* $(,)?) => {
        pub fn to_proto(self) -> i32 {
            match self {
                $(Self::$kind => $value,)*
            }
        }

        pub fn from_proto(value: i32) -> Self {
            match value {
                $($value => Self::$kind,)*
                _ => Self::Null,
            }
        }
    };
}

impl SymbolKind {
    proto_mapping! {
        File = 1,
        Module = 2,
        Namespace = 3,
        Package = 4,
        Class = 5,
        Method = 6,
        Property = 7,
        Field = 8,
        Constructor = 9,
        Enum = 10,
        Interface = 11,
        Function = 12,
        Variable = 13,
        Constant = 14,
        String = 15,
        Number = 16,
        Boolean = 17,
        Array = 18,
        Object = 19,
        Key = 20,
        Null = 21,
        EnumMember = 22,
        Struct = 23,
        Event = 24,
        Operator = 25,
        TypeParameter = 26,
    }
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub container_name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CodeLabel {
    /// The text to display.
    pub text: String,
    /// Syntax highlighting runs.
    pub runs: Vec<(Range<usize>, HighlightId)>,
    /// The portion of the text that should be used in fuzzy filtering.
    pub filter_range: Range<usize>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CodeLabelBuilder {
    /// The text to display.
    text: String,
    /// Syntax highlighting runs.
    runs: Vec<(Range<usize>, HighlightId)>,
    /// The portion of the text that should be used in fuzzy filtering.
    filter_range: Range<usize>,
}

impl CodeLabel {
    pub fn plain(text: String, filter_text: Option<&str>) -> Self {
        Self::filtered(text.clone(), text.len(), filter_text, Vec::new())
    }

    pub fn filtered(
        text: String,
        label_len: usize,
        filter_text: Option<&str>,
        runs: Vec<(Range<usize>, HighlightId)>,
    ) -> Self {
        assert!(label_len <= text.len());
        let filter_range = filter_text
            .and_then(|filter| text.find(filter).map(|index| index..index + filter.len()))
            .unwrap_or(0..label_len);
        Self::new(text, filter_range, runs)
    }

    pub fn new(
        text: String,
        filter_range: Range<usize>,
        runs: Vec<(Range<usize>, HighlightId)>,
    ) -> Self {
        assert!(
            text.get(filter_range.clone()).is_some(),
            "invalid filter range"
        );
        runs.iter().for_each(|(range, _)| {
            assert!(
                text.get(range.clone()).is_some(),
                "invalid run range with inputs. Requested range {range:?} in text '{text}'",
            );
        });
        Self {
            runs,
            filter_range,
            text,
        }
    }

    pub fn text(&self) -> &str {
        self.text.as_str()
    }

    pub fn filter_text(&self) -> &str {
        &self.text[self.filter_range.clone()]
    }
}

impl From<String> for CodeLabel {
    fn from(value: String) -> Self {
        Self::plain(value, None)
    }
}

impl From<&str> for CodeLabel {
    fn from(value: &str) -> Self {
        Self::plain(value.to_string(), None)
    }
}

impl CodeLabelBuilder {
    pub fn respan_filter_range(&mut self, filter_text: Option<&str>) {
        self.filter_range = filter_text
            .and_then(|filter| {
                self.text
                    .find(filter)
                    .map(|index| index..index + filter.len())
            })
            .unwrap_or(0..self.text.len());
    }

    pub fn push_str(&mut self, text: &str, highlight: Option<HighlightId>) {
        let start_index = self.text.len();
        self.text.push_str(text);
        if let Some(highlight) = highlight {
            let end_index = self.text.len();
            self.runs.push((start_index..end_index, highlight));
        }
    }

    pub fn build(mut self) -> CodeLabel {
        if self.filter_range.end == 0 {
            self.respan_filter_range(None);
        }
        CodeLabel {
            text: self.text,
            runs: self.runs,
            filter_range: self.filter_range,
        }
    }
}
