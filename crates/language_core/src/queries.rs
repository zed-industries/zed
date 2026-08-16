use std::borrow::Cow;

#[derive(Clone, Copy, Debug, Eq, PartialEq, strum::EnumIter)]
pub enum QueryFile {
    Highlights,
    Brackets,
    Outline,
    Indents,
    Injections,
    Overrides,
    Redactions,
    Runnables,
    Debugger,
    TextObjects,
}

impl QueryFile {
    pub const fn file_name(self) -> &'static str {
        match self {
            Self::Highlights => "highlights.scm",
            Self::Brackets => "brackets.scm",
            Self::Outline => "outline.scm",
            Self::Indents => "indents.scm",
            Self::Injections => "injections.scm",
            Self::Overrides => "overrides.scm",
            Self::Redactions => "redactions.scm",
            Self::Runnables => "runnables.scm",
            Self::Debugger => "debugger.scm",
            Self::TextObjects => "textobjects.scm",
        }
    }

    pub fn from_file_name(file_name: &str) -> Option<Self> {
        match file_name {
            "highlights.scm" => Some(Self::Highlights),
            "brackets.scm" => Some(Self::Brackets),
            "outline.scm" => Some(Self::Outline),
            "indents.scm" => Some(Self::Indents),
            "injections.scm" => Some(Self::Injections),
            "overrides.scm" => Some(Self::Overrides),
            "redactions.scm" => Some(Self::Redactions),
            "runnables.scm" => Some(Self::Runnables),
            "debugger.scm" => Some(Self::Debugger),
            "textobjects.scm" => Some(Self::TextObjects),
            _ => None,
        }
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
    use strum::IntoEnumIterator;

    use super::*;

    #[test]
    fn query_files_have_unique_names_and_fields() {
        let queries = LanguageQueries::from_files(QueryFile::iter().map(|query_file| {
            QueryFileContents::new(query_file, Cow::Borrowed(query_file.file_name()))
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
            assert_eq!(
                QueryFile::from_file_name(query_file.file_name()),
                Some(query_file)
            );
        }
        assert_eq!(QueryFile::from_file_name("highlights_extra.scm"), None);
    }
}
