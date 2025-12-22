use serde::{Deserialize, Serialize};
use std::{borrow::Cow, mem, path::Path, sync::Arc};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExampleSpec {
    #[serde(default)]
    pub name: String,
    pub repository_url: String,
    pub revision: String,
    #[serde(default)]
    pub uncommitted_diff: String,
    pub cursor_path: Arc<Path>,
    pub cursor_position: String,
    pub edit_history: String,
    pub expected_patch: String,
}

const UNCOMMITTED_DIFF_HEADING: &str = "Uncommitted Diff";
const EDIT_HISTORY_HEADING: &str = "Edit History";
const CURSOR_POSITION_HEADING: &str = "Cursor Position";
const EXPECTED_PATCH_HEADING: &str = "Expected Patch";
const EXPECTED_CONTEXT_HEADING: &str = "Expected Context";

#[derive(Serialize, Deserialize)]
struct FrontMatter<'a> {
    repository_url: Cow<'a, str>,
    revision: Cow<'a, str>,
}

impl ExampleSpec {
    /// Format this example spec as markdown.
    pub fn to_markdown(&self) -> String {
        use std::fmt::Write as _;

        let front_matter = FrontMatter {
            repository_url: Cow::Borrowed(&self.repository_url),
            revision: Cow::Borrowed(&self.revision),
        };
        let front_matter_toml =
            toml::to_string_pretty(&front_matter).unwrap_or_else(|_| String::new());

        let mut markdown = String::new();

        _ = writeln!(markdown, "+++");
        markdown.push_str(&front_matter_toml);
        if !markdown.ends_with('\n') {
            markdown.push('\n');
        }
        _ = writeln!(markdown, "+++");
        markdown.push('\n');

        _ = writeln!(markdown, "# {}", self.name);
        markdown.push('\n');

        if !self.uncommitted_diff.is_empty() {
            _ = writeln!(markdown, "## {}", UNCOMMITTED_DIFF_HEADING);
            _ = writeln!(markdown);
            _ = writeln!(markdown, "```diff");
            markdown.push_str(&self.uncommitted_diff);
            if !markdown.ends_with('\n') {
                markdown.push('\n');
            }
            _ = writeln!(markdown, "```");
            markdown.push('\n');
        }

        _ = writeln!(markdown, "## {}", EDIT_HISTORY_HEADING);
        _ = writeln!(markdown);

        if self.edit_history.is_empty() {
            _ = writeln!(markdown, "(No edit history)");
            _ = writeln!(markdown);
        } else {
            _ = writeln!(markdown, "```diff");
            markdown.push_str(&self.edit_history);
            if !markdown.ends_with('\n') {
                markdown.push('\n');
            }
            _ = writeln!(markdown, "```");
            markdown.push('\n');
        }

        _ = writeln!(markdown, "## {}", CURSOR_POSITION_HEADING);
        _ = writeln!(markdown);
        _ = writeln!(markdown, "```{}", self.cursor_path.to_string_lossy());
        markdown.push_str(&self.cursor_position);
        if !markdown.ends_with('\n') {
            markdown.push('\n');
        }
        _ = writeln!(markdown, "```");
        markdown.push('\n');

        _ = writeln!(markdown, "## {}", EXPECTED_PATCH_HEADING);
        markdown.push('\n');
        _ = writeln!(markdown, "```diff");
        markdown.push_str(&self.expected_patch);
        if !markdown.ends_with('\n') {
            markdown.push('\n');
        }
        _ = writeln!(markdown, "```");
        markdown.push('\n');

        markdown
    }

    /// Parse an example spec from markdown.
    pub fn from_markdown(name: String, mut input: &str) -> anyhow::Result<Self> {
        use pulldown_cmark::{CodeBlockKind, CowStr, Event, HeadingLevel, Parser, Tag, TagEnd};

        let mut spec = ExampleSpec {
            name,
            repository_url: String::new(),
            revision: String::new(),
            uncommitted_diff: String::new(),
            cursor_path: Path::new("").into(),
            cursor_position: String::new(),
            edit_history: String::new(),
            expected_patch: String::new(),
        };

        if let Some(rest) = input.strip_prefix("+++\n")
            && let Some((front_matter, rest)) = rest.split_once("+++\n")
        {
            if let Ok(data) = toml::from_str::<FrontMatter<'_>>(front_matter) {
                spec.repository_url = data.repository_url.into_owned();
                spec.revision = data.revision.into_owned();
            }
            input = rest.trim_start();
        }

        let parser = Parser::new(input);
        let mut text = String::new();
        let mut block_info: CowStr = "".into();

        #[derive(PartialEq)]
        enum Section {
            Start,
            UncommittedDiff,
            EditHistory,
            CursorPosition,
            ExpectedExcerpts,
            ExpectedPatch,
            Other,
        }

        let mut current_section = Section::Start;

        for event in parser {
            match event {
                Event::Text(line) => {
                    text.push_str(&line);
                }
                Event::End(TagEnd::Heading(HeadingLevel::H2)) => {
                    let title = mem::take(&mut text);
                    current_section = if title.eq_ignore_ascii_case(UNCOMMITTED_DIFF_HEADING) {
                        Section::UncommittedDiff
                    } else if title.eq_ignore_ascii_case(EDIT_HISTORY_HEADING) {
                        Section::EditHistory
                    } else if title.eq_ignore_ascii_case(CURSOR_POSITION_HEADING) {
                        Section::CursorPosition
                    } else if title.eq_ignore_ascii_case(EXPECTED_PATCH_HEADING) {
                        Section::ExpectedPatch
                    } else if title.eq_ignore_ascii_case(EXPECTED_CONTEXT_HEADING) {
                        Section::ExpectedExcerpts
                    } else {
                        Section::Other
                    };
                }
                Event::End(TagEnd::Heading(HeadingLevel::H3)) => {
                    mem::take(&mut text);
                }
                Event::End(TagEnd::Heading(HeadingLevel::H4)) => {
                    mem::take(&mut text);
                }
                Event::End(TagEnd::Heading(level)) => {
                    anyhow::bail!("Unexpected heading level: {level}");
                }
                Event::Start(Tag::CodeBlock(kind)) => {
                    match kind {
                        CodeBlockKind::Fenced(info) => {
                            block_info = info;
                        }
                        CodeBlockKind::Indented => {
                            anyhow::bail!("Unexpected indented codeblock");
                        }
                    };
                }
                Event::Start(_) => {
                    text.clear();
                    block_info = "".into();
                }
                Event::End(TagEnd::CodeBlock) => {
                    let block_info = block_info.trim();
                    match current_section {
                        Section::UncommittedDiff => {
                            spec.uncommitted_diff = mem::take(&mut text);
                        }
                        Section::EditHistory => {
                            spec.edit_history.push_str(&mem::take(&mut text));
                        }
                        Section::CursorPosition => {
                            spec.cursor_path = Path::new(block_info).into();
                            spec.cursor_position = mem::take(&mut text);
                        }
                        Section::ExpectedExcerpts => {
                            mem::take(&mut text);
                        }
                        Section::ExpectedPatch => {
                            spec.expected_patch = mem::take(&mut text);
                        }
                        Section::Start | Section::Other => {}
                    }
                }
                _ => {}
            }
        }

        if spec.cursor_path.as_ref() == Path::new("") || spec.cursor_position.is_empty() {
            anyhow::bail!("Missing cursor position codeblock");
        }

        Ok(spec)
    }
}
