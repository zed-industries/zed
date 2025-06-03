use anyhow::Context as _;
use collections::HashMap;
use std::borrow::Cow;
use std::ffi::OsStr;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::NamedTempFile;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(
        "SHELL environment variable is not assigned so we can't source login environment variables"
    )]
    CannotResolveShellPath(std::env::VarError),

    #[error(transparent)]
    CannotCreateTempfile(std::io::Error),

    #[error(transparent)]
    SpawnFailed(std::io::Error),

    #[error(transparent)]
    CannotReadTempfile(std::io::Error),

    #[error(
        "login shell exited with {}. stdout: {:?}, stderr: {:?}",
        .0.status,
        String::from_utf8_lossy(&.0.stdout),
        String::from_utf8_lossy(&.0.stderr),
    )]
    ShellExitedWithError(std::process::Output),

    #[error(transparent)]
    ParseFailed(anyhow::Error),
}

/// Capture all environment variables from the login shell.
pub fn capture(change_dir: Option<impl AsRef<Path>>) -> Result<HashMap<String, String>> {
    let shell_path = std::env::var("SHELL")
        .map(PathBuf::from)
        .map_err(Error::CannotResolveShellPath)?;
    let shell_name = shell_path.file_name().and_then(OsStr::to_str);

    let mut command_string = String::new();

    // What we're doing here is to spawn a shell and then `cd` into
    // the project directory to get the env in there as if the user
    // `cd`'d into it. We do that because tools like direnv, asdf, ...
    // hook into `cd` and only set up the env after that.
    if let Some(dir) = change_dir {
        let dir_str = dir.as_ref().to_string_lossy();
        command_string.push_str(&format!("cd '{dir_str}';"));
    }

    // In certain shells we need to execute additional_command in order to
    // trigger the behavior of direnv, etc.
    command_string.push_str(match shell_name {
        Some("fish") => "emit fish_prompt;",
        _ => "",
    });

    let mut env_output_file = NamedTempFile::new().map_err(Error::CannotCreateTempfile)?;
    command_string.push_str(&format!(
        "sh -c 'export -p' > '{}';",
        env_output_file.path().to_string_lossy(),
    ));

    let mut command = Command::new(&shell_path);

    // For csh/tcsh, the login shell option is set by passing `-` as
    // the 0th argument instead of using `-l`.
    if let Some("tcsh" | "csh") = shell_name {
        #[cfg(unix)]
        std::os::unix::process::CommandExt::arg0(&mut command, "-");
    } else {
        command.arg("-l");
    }

    command.args(["-i", "-c", &command_string]);

    let process_output = super::set_pre_exec_to_start_new_session(&mut command)
        .output()
        .map_err(Error::SpawnFailed)?;

    if !process_output.status.success() {
        return Err(Error::ShellExitedWithError(process_output));
    }

    let mut env_output = String::new();
    env_output_file
        .read_to_string(&mut env_output)
        .map_err(Error::CannotReadTempfile)?;

    parse(&env_output)
        .filter_map(|entry| match entry {
            Ok((name, value)) => Some(Ok((name.into(), value?.into()))),
            Err(err) => Some(Err(err)),
        })
        .collect::<Result<HashMap<String, String>>>()
}

/// Parse the result of calling `sh -c 'export -p'`.
///
/// https://www.man7.org/linux/man-pages/man1/export.1p.html
fn parse(mut input: &str) -> impl Iterator<Item = Result<(Cow<'_, str>, Option<Cow<'_, str>>)>> {
    std::iter::from_fn(move || {
        if input.is_empty() {
            return None;
        }
        match parse_declaration(input) {
            Ok((entry, rest)) => {
                input = rest;
                Some(Ok(entry))
            }
            Err(err) => Some(Err(err)),
        }
    })
}

fn parse_declaration(input: &str) -> Result<((Cow<'_, str>, Option<Cow<'_, str>>), &str)> {
    let rest = input
        .strip_prefix("export ")
        .context("expected 'export ' prefix")
        .map_err(Error::ParseFailed)?;

    if let Some((name, rest)) = parse_name_and_terminator(rest, '\n') {
        Ok(((name, None), rest))
    } else {
        let (name, rest) = parse_name_and_terminator(rest, '=')
            .context("invalid name")
            .map_err(Error::ParseFailed)?;
        let (value, rest) = parse_literal_and_terminator(rest, '\n')
            .context("invalid value")
            .map_err(Error::ParseFailed)?;
        Ok(((name, Some(value)), rest))
    }
}

fn parse_name_and_terminator(input: &str, terminator: char) -> Option<(Cow<'_, str>, &str)> {
    let (name, rest) = parse_literal_and_terminator(input, terminator)?;
    (!name.is_empty() && !name.contains('=')).then_some((name, rest))
}

fn parse_literal_and_terminator(input: &str, terminator: char) -> Option<(Cow<'_, str>, &str)> {
    if let Some((literal, rest)) = parse_literal_single_quoted(input) {
        let rest = rest.strip_prefix(terminator)?;
        Some((Cow::Borrowed(literal), rest))
    } else if let Some((literal, rest)) = parse_literal_double_quoted(input) {
        let rest = rest.strip_prefix(terminator)?;
        Some((Cow::Owned(literal), rest))
    } else {
        let (literal, rest) = input.split_once(terminator)?;
        (!literal.contains(|c: char| c.is_ascii_whitespace()))
            .then_some((Cow::Borrowed(literal), rest))
    }
}

/// https://www.gnu.org/software/bash/manual/html_node/Single-Quotes.html
fn parse_literal_single_quoted(input: &str) -> Option<(&str, &str)> {
    input.strip_prefix('\'')?.split_once('\'')
}

/// https://www.gnu.org/software/bash/manual/html_node/Double-Quotes.html
fn parse_literal_double_quoted(input: &str) -> Option<(String, &str)> {
    let rest = input.strip_prefix('"')?;

    let mut char_indices = rest.char_indices();
    let mut escaping = false;
    let (literal, rest) = loop {
        let (index, char) = char_indices.next()?;
        if char == '"' && !escaping {
            break (&rest[..index], &rest[index + 1..]);
        } else {
            escaping = !escaping && char == '\\';
        }
    };

    let literal = literal
        .replace("\\$", "$")
        .replace("\\`", "`")
        .replace("\\\"", "\"")
        .replace("\\\n", "")
        .replace("\\\\", "\\");

    Some((literal, rest))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = indoc::indoc! {r#"
        export foo
        export 'foo'
        export "foo"
        export foo=
        export 'foo'=
        export "foo"=
        export foo=bar
        export foo='bar'
        export foo="bar"
        export foo='b
        a
        z'
        export foo="b
        a
        z"
        export foo='b\
        a\
        z'
        export foo="b\
        a\
        z"
        export foo='\`Hello\`
        \"wo\
        rld\"\n!\\
        !'
        export foo="\`Hello\`
        \"wo\
        rld\"\n!\\
        !"
        "#};

        let expected_values = [
            None,
            None,
            None,
            Some(""),
            Some(""),
            Some(""),
            Some("bar"),
            Some("bar"),
            Some("bar"),
            Some("b\na\nz"),
            Some("b\na\nz"),
            Some("b\\\na\\\nz"),
            Some("baz"),
            Some(indoc::indoc! {r#"
            \`Hello\`
            \"wo\
            rld\"\n!\\
            !"#}),
            Some(indoc::indoc! {r#"
            `Hello`
            "world"\n!\!"#}),
        ];
        let expected = expected_values
            .into_iter()
            .map(|value| ("foo".into(), value.map(Into::into)))
            .collect::<Vec<_>>();

        let actual = parse(input).collect::<Result<Vec<_>>>().unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_declaration() {
        let ((name, value), rest) = parse_declaration("export foo\nrest").unwrap();
        assert_eq!(name, "foo");
        assert_eq!(value, None);
        assert_eq!(rest, "rest");

        let ((name, value), rest) = parse_declaration("export foo=bar\nrest").unwrap();
        assert_eq!(name, "foo");
        assert_eq!(value.as_deref(), Some("bar"));
        assert_eq!(rest, "rest");
    }

    #[test]
    fn test_parse_literal_single_quoted() {
        let input = indoc::indoc! {r#"
        '\`Hello\`
        \"wo\
        rld\"\n!\\
        !'
        rest"#};

        let expected = indoc::indoc! {r#"
        \`Hello\`
        \"wo\
        rld\"\n!\\
        !"#};

        let (actual, rest) = parse_literal_single_quoted(input).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(rest, "\nrest");
    }

    #[test]
    fn test_parse_literal_double_quoted() {
        let input = indoc::indoc! {r#"
        "\`Hello\`
        \"wo\
        rld\"\n!\\
        !"
        rest"#};

        let expected = indoc::indoc! {r#"
        `Hello`
        "world"\n!\!"#};

        let (actual, rest) = parse_literal_double_quoted(input).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(rest, "\nrest");
    }
}
