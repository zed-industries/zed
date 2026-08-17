//! An example that shows how to implement a simple custom file database.
//! The database uses 32-bit file-ids, which could be useful for optimizing
//! memory usage.
//!
//! To run this example, execute the following command from the top level of
//! this repository:
//!
//! ```sh
//! cargo run --example custom_files
//! ```

use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::term::{self, Config};
use core::ops::Range;

#[cfg(not(feature = "termcolor"))]
fn main() {
    panic!("this example requires termcolor feature");
}

#[cfg(feature = "termcolor")]
fn main() -> anyhow::Result<()> {
    use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};

    let mut files = files::Files::new();

    let file_id0 = files.add("0.greeting", "hello world!").unwrap();
    let file_id1 = files.add("1.greeting", "bye world").unwrap();

    let messages = vec![
        Message::UnwantedGreetings {
            greetings: vec![(file_id0, 0..5), (file_id1, 0..3)],
        },
        Message::OverTheTopExclamations {
            exclamations: vec![(file_id0, 11..12)],
        },
    ];

    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = Config::default();
    for message in &messages {
        term::emit_to_write_style(
            &mut writer.lock(),
            &config,
            &files,
            &message.to_diagnostic(),
        )?;
    }

    Ok(())
}

/// A module containing the file implementation
mod files {
    use codespan_reporting::files;
    use core::ops::Range;

    /// A file that is backed by an `Arc<String>`.
    #[derive(Debug, Clone)]
    struct File {
        /// The name of the file.
        name: String,
        /// The source code of the file.
        source: String,
        /// The starting byte indices in the source code.
        line_starts: Vec<usize>,
    }

    impl File {
        fn line_start(&self, line_index: usize) -> Result<usize, files::Error> {
            use core::cmp::Ordering;

            match line_index.cmp(&self.line_starts.len()) {
                Ordering::Less => Ok(*self
                    .line_starts
                    .get(line_index)
                    .expect("failed despite previous check")),
                Ordering::Equal => Ok(self.source.len()),
                Ordering::Greater => Err(files::Error::LineTooLarge {
                    given: line_index,
                    max: self.line_starts.len() - 1,
                }),
            }
        }
    }

    /// An opaque file identifier.
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct FileId(u32);

    #[derive(Debug, Clone)]
    pub struct Files {
        files: Vec<File>,
    }

    impl Files {
        /// Create a new files database.
        pub fn new() -> Files {
            Files { files: Vec::new() }
        }

        /// Add a file to the database, returning the handle that can be used to
        /// refer to it again.
        pub fn add(
            &mut self,
            name: impl Into<String>,
            source: impl Into<String>,
        ) -> Option<FileId> {
            use core::convert::TryFrom;

            let file_id = FileId(u32::try_from(self.files.len()).ok()?);
            let name = name.into();
            let source = source.into();
            let line_starts = files::line_starts(&source).collect();

            self.files.push(File {
                name,
                line_starts,
                source,
            });

            Some(file_id)
        }

        /// Get the file corresponding to the given id.
        fn get(&self, file_id: FileId) -> Result<&File, files::Error> {
            self.files
                .get(file_id.0 as usize)
                .ok_or(files::Error::FileMissing)
        }
    }

    impl<'files> files::Files<'files> for Files {
        type FileId = FileId;
        type Name = &'files str;
        type Source = &'files str;

        fn name(&self, file_id: FileId) -> Result<&str, files::Error> {
            Ok(self.get(file_id)?.name.as_ref())
        }

        fn source(&self, file_id: FileId) -> Result<&str, files::Error> {
            Ok(&self.get(file_id)?.source)
        }

        fn line_index(&self, file_id: FileId, byte_index: usize) -> Result<usize, files::Error> {
            self.get(file_id)?
                .line_starts
                .binary_search(&byte_index)
                .or_else(|next_line| Ok(next_line - 1))
        }

        fn line_range(
            &self,
            file_id: FileId,
            line_index: usize,
        ) -> Result<Range<usize>, files::Error> {
            let file = self.get(file_id)?;
            let line_start = file.line_start(line_index)?;
            let next_line_start = file.line_start(line_index + 1)?;

            Ok(line_start..next_line_start)
        }
    }
}

/// A Diagnostic message.
enum Message {
    UnwantedGreetings {
        greetings: Vec<(files::FileId, Range<usize>)>,
    },
    OverTheTopExclamations {
        exclamations: Vec<(files::FileId, Range<usize>)>,
    },
}

impl Message {
    fn to_diagnostic(&self) -> Diagnostic<files::FileId> {
        match self {
            Message::UnwantedGreetings { greetings } => Diagnostic::error()
                .with_message("greetings are not allowed")
                .with_labels(
                    greetings
                        .iter()
                        .map(|(file_id, range)| {
                            Label::primary(*file_id, range.clone()).with_message("a greeting")
                        })
                        .collect(),
                )
                .with_notes(vec![
                    "found greetings!".to_owned(),
                    "pleas no greetings :(".to_owned(),
                ]),
            Message::OverTheTopExclamations { exclamations } => Diagnostic::error()
                .with_message("over-the-top exclamations")
                .with_labels(
                    exclamations
                        .iter()
                        .map(|(file_id, range)| {
                            Label::primary(*file_id, range.clone()).with_message("an exclamation")
                        })
                        .collect(),
                )
                .with_notes(vec!["ridiculous!".to_owned()]),
        }
    }
}
