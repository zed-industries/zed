use std::borrow::Cow;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use strum::IntoEnumIterator;

#[derive(
    Clone, Copy, Debug, Eq, PartialEq, strum::EnumIter, strum::EnumString, strum::IntoStaticStr,
)]
pub enum QueryFile {
    #[strum(serialize = "highlights.scm")]
    Highlights,
    #[strum(serialize = "brackets.scm")]
    Brackets,
    #[strum(serialize = "outline.scm")]
    Outline,
    #[strum(serialize = "indents.scm")]
    Indents,
    #[strum(serialize = "injections.scm")]
    Injections,
    #[strum(serialize = "overrides.scm")]
    Overrides,
    #[strum(serialize = "redactions.scm")]
    Redactions,
    #[strum(serialize = "runnables.scm")]
    Runnables,
    #[strum(serialize = "debugger.scm")]
    Debugger,
    #[strum(serialize = "textobjects.scm")]
    TextObjects,
}

impl QueryFile {
    pub fn file_name(self) -> &'static str {
        self.into()
    }
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
    pub struct QueryFiles: u16 {
        const HIGHLIGHTS = 1 << 0;
        const BRACKETS = 1 << 1;
        const OUTLINE = 1 << 2;
        const INDENTS = 1 << 3;
        const INJECTIONS = 1 << 4;
        const OVERRIDES = 1 << 5;
        const REDACTIONS = 1 << 6;
        const RUNNABLES = 1 << 7;
        const DEBUGGER = 1 << 8;
        const TEXT_OBJECTS = 1 << 9;
    }
}

impl QueryFiles {
    pub fn query_files(self) -> impl Iterator<Item = QueryFile> {
        QueryFile::iter().filter(move |query_file| self.contains((*query_file).into()))
    }
}

impl From<QueryFile> for QueryFiles {
    fn from(query_file: QueryFile) -> Self {
        match query_file {
            QueryFile::Highlights => Self::HIGHLIGHTS,
            QueryFile::Brackets => Self::BRACKETS,
            QueryFile::Outline => Self::OUTLINE,
            QueryFile::Indents => Self::INDENTS,
            QueryFile::Injections => Self::INJECTIONS,
            QueryFile::Overrides => Self::OVERRIDES,
            QueryFile::Redactions => Self::REDACTIONS,
            QueryFile::Runnables => Self::RUNNABLES,
            QueryFile::Debugger => Self::DEBUGGER,
            QueryFile::TextObjects => Self::TEXT_OBJECTS,
        }
    }
}

impl FromIterator<QueryFile> for QueryFiles {
    fn from_iter<T: IntoIterator<Item = QueryFile>>(query_files: T) -> Self {
        query_files
            .into_iter()
            .fold(Self::empty(), |files, query_file| files | query_file.into())
    }
}

impl Serialize for QueryFiles {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u16(self.bits())
    }
}

impl<'de> Deserialize<'de> for QueryFiles {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::from_bits_retain(u16::deserialize(deserializer)?))
    }
}

#[derive(Debug)]
pub struct QueryFileContents {
    pub query_file: QueryFile,
    pub contents: Cow<'static, str>,
}

impl QueryFileContents {
    pub fn new(query_file: QueryFile, contents: Cow<'static, str>) -> Self {
        Self {
            query_file,
            contents,
        }
    }
}

/// Tree-sitter language queries for a given language.
#[derive(Debug, Default)]
pub struct LanguageQueries {
    pub highlights: Option<Cow<'static, str>>,
    pub brackets: Option<Cow<'static, str>>,
    pub indents: Option<Cow<'static, str>>,
    pub outline: Option<Cow<'static, str>>,
    pub injections: Option<Cow<'static, str>>,
    pub overrides: Option<Cow<'static, str>>,
    pub redactions: Option<Cow<'static, str>>,
    pub runnables: Option<Cow<'static, str>>,
    pub text_objects: Option<Cow<'static, str>>,
    pub debugger: Option<Cow<'static, str>>,
}

impl LanguageQueries {
    pub fn from_files(files: impl IntoIterator<Item = QueryFileContents>) -> Self {
        let mut queries = Self::default();
        for QueryFileContents {
            query_file,
            contents,
        } in files
        {
            let field = match query_file {
                QueryFile::Highlights => &mut queries.highlights,
                QueryFile::Brackets => &mut queries.brackets,
                QueryFile::Outline => &mut queries.outline,
                QueryFile::Indents => &mut queries.indents,
                QueryFile::Injections => &mut queries.injections,
                QueryFile::Overrides => &mut queries.overrides,
                QueryFile::Redactions => &mut queries.redactions,
                QueryFile::Runnables => &mut queries.runnables,
                QueryFile::Debugger => &mut queries.debugger,
                QueryFile::TextObjects => &mut queries.text_objects,
            };
            *field = Some(contents);
        }
        queries
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_files_have_unique_names_and_fields() {
        let queries = LanguageQueries::from_files(QueryFile::iter().map(|query_file| {
            QueryFileContents::new(query_file, Cow::Borrowed(query_file.into()))
        }));

        assert_eq!(queries.highlights.as_deref(), Some("highlights.scm"));
        assert_eq!(queries.brackets.as_deref(), Some("brackets.scm"));
        assert_eq!(queries.outline.as_deref(), Some("outline.scm"));
        assert_eq!(queries.indents.as_deref(), Some("indents.scm"));
        assert_eq!(queries.injections.as_deref(), Some("injections.scm"));
        assert_eq!(queries.overrides.as_deref(), Some("overrides.scm"));
        assert_eq!(queries.redactions.as_deref(), Some("redactions.scm"));
        assert_eq!(queries.runnables.as_deref(), Some("runnables.scm"));
        assert_eq!(queries.debugger.as_deref(), Some("debugger.scm"));
        assert_eq!(queries.text_objects.as_deref(), Some("textobjects.scm"));

        for query_file in QueryFile::iter() {
            let file_name: &'static str = query_file.into();
            assert_eq!(file_name.parse(), Ok(query_file));
        }
        assert_eq!(
            "highlights_extra.scm".parse::<QueryFile>(),
            Err(strum::ParseError::VariantNotFound)
        );
    }

    #[test]
    fn query_files_bitflags_map_to_query_files() {
        let all_query_files = QueryFile::iter().collect::<QueryFiles>();
        assert_eq!(all_query_files, QueryFiles::all());
        assert_eq!(
            all_query_files.query_files().collect::<Vec<_>>(),
            QueryFile::iter().collect::<Vec<_>>()
        );

        let selected = QueryFiles::from(QueryFile::Highlights) | QueryFile::Outline.into();
        assert_eq!(
            selected.query_files().collect::<Vec<_>>(),
            vec![QueryFile::Highlights, QueryFile::Outline]
        );
    }

    #[test]
    fn query_files_serialize_as_bits() {
        let query_files = QueryFiles::HIGHLIGHTS | QueryFiles::TEXT_OBJECTS;
        let serialized = serde_json::to_string(&query_files).unwrap();
        assert_eq!(serialized, "513");
        assert_eq!(
            serde_json::from_str::<QueryFiles>(&serialized).unwrap(),
            query_files
        );

        let unknown_bit = 1 << 15;
        let query_files = serde_json::from_str::<QueryFiles>(&unknown_bit.to_string()).unwrap();
        assert_eq!(query_files.bits(), unknown_bit);
    }
}
