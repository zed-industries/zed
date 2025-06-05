#![cfg_attr(not(unix), allow(unused))]

use anyhow::{Context as _, Result};
use std::borrow::Cow;

/// Capture all environment variables from the login shell.
#[cfg(unix)]
pub fn capture(directory: &std::path::Path) -> Result<collections::HashMap<String, String>> {
    use std::os::unix::process::CommandExt;

    let shell_path = std::env::var("SHELL").map(std::path::PathBuf::from)?;
    let shell_name = shell_path.file_name().and_then(std::ffi::OsStr::to_str);

    let mut command = std::process::Command::new(&shell_path);
    let mut command_string = String::new();

    // What we're doing here is to spawn a shell and then `cd` into
    // the project directory to get the env in there as if the user
    // `cd`'d into it. We do that because tools like direnv, asdf, ...
    // hook into `cd` and only set up the env after that.
    command_string.push_str(&format!("cd '{}';", directory.display()));

    // In certain shells we need to execute additional_command in order to
    // trigger the behavior of direnv, etc.
    command_string.push_str(match shell_name {
        Some("fish") => "emit fish_prompt;",
        _ => "",
    });

    command_string.push_str("sh -c 'export -p' >&3;");
    unsafe {
        command.pre_exec(|| {
            libc::dup2(1, 3);
            libc::dup2(2, 1);
            Ok(())
        });
    }

    // For csh/tcsh, the login shell option is set by passing `-` as
    // the 0th argument instead of using `-l`.
    if let Some("tcsh" | "csh") = shell_name {
        command.arg0("-");
    } else {
        command.arg("-l");
    }

    command.args(["-i", "-c", &command_string]);

    let output = super::set_pre_exec_to_start_new_session(&mut command).output()?;
    anyhow::ensure!(
        output.status.success(),
        "login shell exited with {}: {:?}",
        output.status,
        String::from_utf8_lossy(&output.stderr),
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    parse(&stdout)
        .filter_map(|entry| match entry {
            Ok((name, value)) => Some(Ok((name.into(), value?.into()))),
            Err(err) => Some(Err(err)),
        })
        .collect::<Result<_>>()
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
        .context("expected 'export ' prefix")?;

    if let Some((name, rest)) = parse_name_and_terminator(rest, '\n') {
        Ok(((name, None), rest))
    } else {
        let (name, rest) = parse_name_and_terminator(rest, '=').context("invalid name")?;
        let (value, rest) = parse_literal_and_terminator(rest, '\n').context("invalid value")?;
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
