use anyhow::{Result, anyhow};
use collections::HashMap;
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq)]
pub struct ParsedUserCommand<'a> {
    pub name: &'a str,
    pub raw_arguments: &'a str,
}

pub fn try_parse_user_command(line: &str) -> Option<ParsedUserCommand<'_>> {
    let line = line.trim_start();
    if !line.starts_with('/') {
        return None;
    }

    let after_slash = &line[1..];
    let (name, raw_arguments) = if let Some(space_idx) = after_slash.find(char::is_whitespace) {
        let name = &after_slash[..space_idx];
        let rest = &after_slash[space_idx..].trim_start();
        (name, *rest)
    } else {
        (after_slash, "")
    };

    if name.is_empty() {
        return None;
    }

    Some(ParsedUserCommand {
        name,
        raw_arguments,
    })
}

pub fn parse_arguments(input: &str) -> Result<Vec<Cow<'_, str>>> {
    let mut arguments = Vec::new();
    let mut chars = input.char_indices().peekable();

    while let Some((start_idx, c)) = chars.next() {
        if c.is_whitespace() {
            continue;
        }

        if c == '"' {
            let mut result = String::new();
            let mut closed = false;

            while let Some((_, ch)) = chars.next() {
                if ch == '\\' {
                    if let Some((_, next_ch)) = chars.next() {
                        match next_ch {
                            '"' => result.push('"'),
                            '\\' => result.push('\\'),
                            'n' => result.push('\n'),
                            other => {
                                return Err(anyhow!("Unknown escape sequence: \\{}", other));
                            }
                        }
                    } else {
                        return Err(anyhow!("Unexpected end of input after backslash"));
                    }
                } else if ch == '"' {
                    closed = true;
                    break;
                } else {
                    result.push(ch);
                }
            }

            if !closed {
                return Err(anyhow!("Unclosed quote in command arguments"));
            }

            arguments.push(Cow::Owned(result));
        } else {
            let mut end_idx = start_idx + c.len_utf8();
            while let Some(&(idx, ch)) = chars.peek() {
                if ch.is_whitespace() {
                    break;
                }
                if ch == '"' {
                    return Err(anyhow!("Quote in middle of unquoted argument"));
                }
                end_idx = idx + ch.len_utf8();
                chars.next();
            }

            arguments.push(Cow::Borrowed(&input[start_idx..end_idx]));
        }
    }

    Ok(arguments)
}

pub fn count_placeholders(template: &str) -> usize {
    let mut max_placeholder = 0;
    let mut chars = template.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' {
            chars.next();
            continue;
        }
        if c == '$' {
            let mut num_str = String::new();
            while let Some(&next_c) = chars.peek() {
                if next_c.is_ascii_digit() {
                    num_str.push(next_c);
                    chars.next();
                } else {
                    break;
                }
            }
            if !num_str.is_empty() {
                if let Ok(n) = num_str.parse::<usize>() {
                    max_placeholder = max_placeholder.max(n);
                }
            }
        }
    }

    max_placeholder
}

pub fn validate_arguments(
    command_name: &str,
    template: &str,
    arguments: &[Cow<'_, str>],
) -> Result<()> {
    if template.is_empty() {
        return Err(anyhow!("Template cannot be empty"));
    }

    let required_count = count_placeholders(template);

    if required_count == 0 && !arguments.is_empty() {
        return Err(anyhow!(
            "The /{} command accepts no arguments, but {} {} provided",
            command_name,
            arguments.len(),
            if arguments.len() == 1 { "was" } else { "were" }
        ));
    }

    if arguments.len() < required_count {
        return Err(anyhow!(
            "The /{} command requires {} {}, but only {} {} provided",
            command_name,
            required_count,
            if required_count == 1 {
                "argument"
            } else {
                "arguments"
            },
            arguments.len(),
            if arguments.len() == 1 { "was" } else { "were" }
        ));
    }

    if arguments.len() > required_count {
        return Err(anyhow!(
            "The /{} command accepts {} {}, but {} {} provided",
            command_name,
            required_count,
            if required_count == 1 {
                "argument"
            } else {
                "arguments"
            },
            arguments.len(),
            if arguments.len() == 1 { "was" } else { "were" }
        ));
    }

    Ok(())
}

pub fn expand_template(template: &str, arguments: &[Cow<'_, str>]) -> Result<String> {
    let mut result = String::with_capacity(template.len());
    let mut chars = template.char_indices().peekable();

    while let Some((_, c)) = chars.next() {
        if c == '\\' {
            if let Some((_, next_c)) = chars.next() {
                match next_c {
                    '$' => result.push('$'),
                    '"' => result.push('"'),
                    '\\' => result.push('\\'),
                    'n' => result.push('\n'),
                    other => {
                        return Err(anyhow!("Unknown escape sequence: \\{}", other));
                    }
                }
            }
        } else if c == '$' {
            let mut num_str = String::new();
            while let Some(&(_, next_c)) = chars.peek() {
                if next_c.is_ascii_digit() {
                    num_str.push(next_c);
                    chars.next();
                } else {
                    break;
                }
            }
            if !num_str.is_empty() {
                let n: usize = num_str.parse()?;
                if n == 0 {
                    return Err(anyhow!(
                        "Placeholder $0 is invalid; placeholders start at $1"
                    ));
                }
                if let Some(arg) = arguments.get(n - 1) {
                    result.push_str(arg);
                } else {
                    return Err(anyhow!("Missing argument for placeholder ${}", n));
                }
            } else {
                result.push('$');
            }
        } else {
            result.push(c);
        }
    }

    Ok(result)
}

pub fn expand_user_slash_command(
    command_name: &str,
    template: &str,
    arguments: &[Cow<'_, str>],
) -> Result<String> {
    validate_arguments(command_name, template, arguments)?;
    expand_template(template, arguments)
}

pub fn try_expand_user_slash_command(
    line: &str,
    slash_commands: &HashMap<String, String>,
) -> Result<Option<String>> {
    let Some(parsed) = try_parse_user_command(line) else {
        return Ok(None);
    };

    let Some(template) = slash_commands.get(parsed.name) else {
        return Ok(None);
    };

    let arguments = parse_arguments(parsed.raw_arguments)?;
    let expanded = expand_user_slash_command(parsed.name, template, &arguments)?;
    Ok(Some(expanded))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_parse_user_command() {
        assert_eq!(
            try_parse_user_command("/review"),
            Some(ParsedUserCommand {
                name: "review",
                raw_arguments: ""
            })
        );

        assert_eq!(
            try_parse_user_command("/review arg1 arg2"),
            Some(ParsedUserCommand {
                name: "review",
                raw_arguments: "arg1 arg2"
            })
        );

        assert_eq!(
            try_parse_user_command("/cmd \"multi word\" simple"),
            Some(ParsedUserCommand {
                name: "cmd",
                raw_arguments: "\"multi word\" simple"
            })
        );

        assert_eq!(try_parse_user_command("not a command"), None);
        assert_eq!(try_parse_user_command(""), None);
        assert_eq!(try_parse_user_command("/"), None);
    }

    #[test]
    fn test_parse_arguments_simple_unquoted() {
        let args = parse_arguments("foo bar").unwrap();
        assert_eq!(args, vec!["foo", "bar"]);
    }

    #[test]
    fn test_parse_arguments_quoted() {
        let args = parse_arguments("\"foo bar\"").unwrap();
        assert_eq!(args, vec!["foo bar"]);
    }

    #[test]
    fn test_parse_arguments_mixed() {
        let args = parse_arguments("\"foo bar\" baz \"qux\"").unwrap();
        assert_eq!(args, vec!["foo bar", "baz", "qux"]);
    }

    #[test]
    fn test_parse_arguments_escaped_quotes() {
        let args = parse_arguments("\"foo \\\"bar\\\" baz\"").unwrap();
        assert_eq!(args, vec!["foo \"bar\" baz"]);
    }

    #[test]
    fn test_parse_arguments_escaped_backslash() {
        let args = parse_arguments("\"foo\\\\bar\"").unwrap();
        assert_eq!(args, vec!["foo\\bar"]);
    }

    #[test]
    fn test_parse_arguments_empty_quoted() {
        let args = parse_arguments("\"\"").unwrap();
        assert_eq!(args, vec![""]);
    }

    #[test]
    fn test_parse_arguments_unclosed_quote_error() {
        let result = parse_arguments("\"foo");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unclosed quote"));
    }

    #[test]
    fn test_parse_arguments_quote_in_middle_error() {
        let result = parse_arguments("foo\"bar");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Quote in middle"));
    }

    #[test]
    fn test_parse_arguments_unknown_escape_error() {
        let result = parse_arguments("\"\\x\"");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unknown escape"));
    }

    #[test]
    fn test_parse_arguments_newline_escape() {
        let args = parse_arguments("\"line1\\nline2\"").unwrap();
        assert_eq!(args, vec!["line1\nline2"]);
    }

    #[test]
    fn test_count_placeholders() {
        assert_eq!(count_placeholders("Hello $1"), 1);
        assert_eq!(count_placeholders("$1 and $2"), 2);
        assert_eq!(count_placeholders("$1 $1"), 1);
        assert_eq!(count_placeholders("$2 then $1"), 2);
        assert_eq!(count_placeholders("no placeholders"), 0);
        assert_eq!(count_placeholders("\\$1 escaped"), 0);
        assert_eq!(count_placeholders("$10 big number"), 10);
    }

    #[test]
    fn test_expand_template_basic() {
        let args = vec![Cow::Borrowed("world")];
        let result = expand_template("Hello $1", &args).unwrap();
        assert_eq!(result, "Hello world");
    }

    #[test]
    fn test_expand_template_multiple_placeholders() {
        let args = vec![Cow::Borrowed("a"), Cow::Borrowed("b")];
        let result = expand_template("$1 and $2", &args).unwrap();
        assert_eq!(result, "a and b");
    }

    #[test]
    fn test_expand_template_repeated_placeholder() {
        let args = vec![Cow::Borrowed("x")];
        let result = expand_template("$1 $1", &args).unwrap();
        assert_eq!(result, "x x");
    }

    #[test]
    fn test_expand_template_out_of_order() {
        let args = vec![Cow::Borrowed("a"), Cow::Borrowed("b")];
        let result = expand_template("$2 then $1", &args).unwrap();
        assert_eq!(result, "b then a");
    }

    #[test]
    fn test_expand_template_newline_escape() {
        let args: Vec<Cow<'_, str>> = vec![];
        let result = expand_template("line1\\nline2", &args).unwrap();
        assert_eq!(result, "line1\nline2");
    }

    #[test]
    fn test_expand_template_dollar_escape() {
        let args: Vec<Cow<'_, str>> = vec![];
        let result = expand_template("cost is \\$1", &args).unwrap();
        assert_eq!(result, "cost is $1");
    }

    #[test]
    fn test_expand_template_quote_escape() {
        let args: Vec<Cow<'_, str>> = vec![];
        let result = expand_template("say \\\"hi\\\"", &args).unwrap();
        assert_eq!(result, "say \"hi\"");
    }

    #[test]
    fn test_expand_template_backslash_escape() {
        let args: Vec<Cow<'_, str>> = vec![];
        let result = expand_template("path\\\\file", &args).unwrap();
        assert_eq!(result, "path\\file");
    }

    #[test]
    fn test_validate_arguments_exact_match() {
        let args = vec![Cow::Borrowed("a"), Cow::Borrowed("b")];
        let result = validate_arguments("test", "$1 $2", &args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_arguments_missing_args() {
        let args = vec![Cow::Borrowed("a")];
        let result = validate_arguments("foo", "$1 $2", &args);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("/foo"));
        assert!(err.contains("requires 2 arguments"));
        assert!(err.contains("only 1 was provided"));
    }

    #[test]
    fn test_validate_arguments_extra_args() {
        let args = vec![Cow::Borrowed("a"), Cow::Borrowed("b")];
        let result = validate_arguments("foo", "$1", &args);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("/foo"));
        assert!(err.contains("accepts 1 argument"));
        assert!(err.contains("2 were provided"));
    }

    #[test]
    fn test_validate_arguments_no_placeholders_no_args() {
        let args: Vec<Cow<'_, str>> = vec![];
        let result = validate_arguments("test", "no placeholders here", &args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_arguments_no_placeholders_has_args() {
        let args = vec![Cow::Borrowed("unexpected")];
        let result = validate_arguments("test", "no placeholders here", &args);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("accepts no arguments")
        );
    }

    #[test]
    fn test_validate_arguments_empty_template() {
        let args: Vec<Cow<'_, str>> = vec![];
        let result = validate_arguments("test", "", &args);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));
    }

    #[test]
    fn test_expand_user_slash_command() {
        let result =
            expand_user_slash_command("review", "Please review: $1", &[Cow::Borrowed("security")])
                .unwrap();
        assert_eq!(result, "Please review: security");
    }

    #[test]
    fn test_try_expand_user_slash_command() {
        let mut commands = HashMap::default();
        commands.insert("review".to_string(), "Please review: $1".to_string());
        commands.insert("explain".to_string(), "Explain at $1 level: $2".to_string());

        let result = try_expand_user_slash_command("/review security", &commands).unwrap();
        assert_eq!(result, Some("Please review: security".to_string()));

        let result =
            try_expand_user_slash_command("/explain \"beginner\" \"this code\"", &commands)
                .unwrap();
        assert_eq!(
            result,
            Some("Explain at beginner level: this code".to_string())
        );

        let result = try_expand_user_slash_command("/unknown arg", &commands).unwrap();
        assert_eq!(result, None);

        let result = try_expand_user_slash_command("not a command", &commands).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn test_try_expand_user_slash_command_with_missing_args() {
        let mut commands = HashMap::default();
        commands.insert("review".to_string(), "Please review: $1".to_string());

        let result = try_expand_user_slash_command("/review", &commands);
        assert!(result.is_err());
    }
}
