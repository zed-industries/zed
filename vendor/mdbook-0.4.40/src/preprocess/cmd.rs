use super::{Preprocessor, PreprocessorContext};
use crate::book::Book;
use crate::errors::*;
use log::{debug, trace, warn};
use shlex::Shlex;
use std::io::{self, Read, Write};
use std::process::{Child, Command, Stdio};

/// A custom preprocessor which will shell out to a 3rd-party program.
///
/// # Preprocessing Protocol
///
/// When the `supports_renderer()` method is executed, `CmdPreprocessor` will
/// execute the shell command `$cmd supports $renderer`. If the renderer is
/// supported, custom preprocessors should exit with a exit code of `0`,
/// any other exit code be considered as unsupported.
///
/// The `run()` method is implemented by passing a `(PreprocessorContext, Book)`
/// tuple to the spawned command (`$cmd`) as JSON via `stdin`. Preprocessors
/// should then "return" a processed book by printing it to `stdout` as JSON.
/// For convenience, the `CmdPreprocessor::parse_input()` function can be used
/// to parse the input provided by `mdbook`.
///
/// Exiting with a non-zero exit code while preprocessing is considered an
/// error. `stderr` is passed directly through to the user, so it can be used
/// for logging or emitting warnings if desired.
///
/// # Examples
///
/// An example preprocessor is available in this project's `examples/`
/// directory.
#[derive(Debug, Clone, PartialEq)]
pub struct CmdPreprocessor {
    name: String,
    cmd: String,
}

impl CmdPreprocessor {
    /// Create a new `CmdPreprocessor`.
    pub fn new(name: String, cmd: String) -> CmdPreprocessor {
        CmdPreprocessor { name, cmd }
    }

    /// A convenience function custom preprocessors can use to parse the input
    /// written to `stdin` by a `CmdRenderer`.
    pub fn parse_input<R: Read>(reader: R) -> Result<(PreprocessorContext, Book)> {
        serde_json::from_reader(reader).with_context(|| "Unable to parse the input")
    }

    fn write_input_to_child(&self, child: &mut Child, book: &Book, ctx: &PreprocessorContext) {
        let stdin = child.stdin.take().expect("Child has stdin");

        if let Err(e) = self.write_input(stdin, book, ctx) {
            // Looks like the backend hung up before we could finish
            // sending it the render context. Log the error and keep going
            warn!("Error writing the RenderContext to the backend, {}", e);
        }
    }

    fn write_input<W: Write>(
        &self,
        writer: W,
        book: &Book,
        ctx: &PreprocessorContext,
    ) -> Result<()> {
        serde_json::to_writer(writer, &(ctx, book)).map_err(Into::into)
    }

    /// The command this `Preprocessor` will invoke.
    pub fn cmd(&self) -> &str {
        &self.cmd
    }

    fn command(&self) -> Result<Command> {
        let mut words = Shlex::new(&self.cmd);
        let executable = match words.next() {
            Some(e) => e,
            None => bail!("Command string was empty"),
        };

        let mut cmd = Command::new(executable);

        for arg in words {
            cmd.arg(arg);
        }

        Ok(cmd)
    }
}

impl Preprocessor for CmdPreprocessor {
    fn name(&self) -> &str {
        &self.name
    }

    fn run(&self, ctx: &PreprocessorContext, book: Book) -> Result<Book> {
        let mut cmd = self.command()?;

        let mut child = cmd
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .with_context(|| {
                format!(
                    "Unable to start the \"{}\" preprocessor. Is it installed?",
                    self.name()
                )
            })?;

        self.write_input_to_child(&mut child, &book, ctx);

        let output = child.wait_with_output().with_context(|| {
            format!(
                "Error waiting for the \"{}\" preprocessor to complete",
                self.name
            )
        })?;

        trace!("{} exited with output: {:?}", self.cmd, output);
        ensure!(
            output.status.success(),
            format!(
                "The \"{}\" preprocessor exited unsuccessfully with {} status",
                self.name, output.status
            )
        );

        serde_json::from_slice(&output.stdout).with_context(|| {
            format!(
                "Unable to parse the preprocessed book from \"{}\" processor",
                self.name
            )
        })
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        debug!(
            "Checking if the \"{}\" preprocessor supports \"{}\"",
            self.name(),
            renderer
        );

        let mut cmd = match self.command() {
            Ok(c) => c,
            Err(e) => {
                warn!(
                    "Unable to create the command for the \"{}\" preprocessor, {}",
                    self.name(),
                    e
                );
                return false;
            }
        };

        let outcome = cmd
            .arg("supports")
            .arg(renderer)
            .stdin(Stdio::null())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .map(|status| status.code() == Some(0));

        if let Err(ref e) = outcome {
            if e.kind() == io::ErrorKind::NotFound {
                warn!(
                    "The command wasn't found, is the \"{}\" preprocessor installed?",
                    self.name
                );
                warn!("\tCommand: {}", self.cmd);
            }
        }

        outcome.unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MDBook;
    use std::path::Path;

    fn guide() -> MDBook {
        let example = Path::new(env!("CARGO_MANIFEST_DIR")).join("guide");
        MDBook::load(example).unwrap()
    }

    #[test]
    fn round_trip_write_and_parse_input() {
        let cmd = CmdPreprocessor::new("test".to_string(), "test".to_string());
        let md = guide();
        let ctx = PreprocessorContext::new(
            md.root.clone(),
            md.config.clone(),
            "some-renderer".to_string(),
        );

        let mut buffer = Vec::new();
        cmd.write_input(&mut buffer, &md.book, &ctx).unwrap();

        let (got_ctx, got_book) = CmdPreprocessor::parse_input(buffer.as_slice()).unwrap();

        assert_eq!(got_book, md.book);
        assert_eq!(got_ctx, ctx);
    }
}
