//! Hiding secrets in a command line before it is shown.
//!
//! A command line is visible to anyone looking at the screen, and tokens get
//! passed on them: `--token=…`, `NPM_TOKEN=…`, a registry URL with a password
//! in it. What looks like one is replaced with `***`. Leaving a harmless
//! value hidden costs less than showing a secret, so the names matched are
//! generous.

const HIDDEN: &str = "***";

/// Whether an option, variable, header or query parameter name names a
/// secret: `--token`, `--api-key`, `NPM_TOKEN`, `password`, `Authorization`.
fn is_secret_name(name: &str) -> bool {
    const CONTAINS: &[&str] = &[
        "token",
        "password",
        "passwd",
        "secret",
        "apikey",
        "accesskey",
        "privatekey",
        "credential",
        "authorization",
        "cookie",
        "sessionid",
    ];
    const WHOLE: &[&str] = &["auth", "pass", "sig", "signature"];
    let name: String = name
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .collect::<String>()
        .to_ascii_lowercase();
    !name.is_empty()
        && (CONTAINS.iter().any(|part| name.contains(part)) || WHOLE.contains(&name.as_str()))
}

/// `args` with the values of anything that looks like a secret replaced.
pub fn redact(args: &[String]) -> Vec<String> {
    let mut redacted = Vec::with_capacity(args.len());
    let mut hide_next = false;
    for arg in args {
        // The value after `--token`, unless that was a flag on its own.
        if std::mem::take(&mut hide_next) && !arg.starts_with('-') {
            redacted.push(HIDDEN.to_string());
            continue;
        }
        redacted.push(redact_arg(arg, &mut hide_next));
    }
    redacted
}

fn redact_arg(arg: &str, hide_next: &mut bool) -> String {
    // `--token=…`, `NPM_TOKEN=…`
    if let Some((name, _)) = arg.split_once('=')
        && !name.contains("://")
        && is_secret_name(name)
    {
        return format!("{name}={HIDDEN}");
    }
    // A header passed whole: `Authorization: Bearer …`
    if let Some((name, _)) = arg.split_once(": ")
        && is_secret_name(name)
    {
        return format!("{name}: {HIDDEN}");
    }
    // `--token …`: the value is the next argument.
    if arg.starts_with('-') && is_secret_name(arg) {
        *hide_next = true;
        return arg.to_string();
    }
    redact_url(arg)
}

/// A URL in `arg` without the password (or a token used as the user name)
/// in front of its host, and without the values of secret query
/// parameters: `https://user:***@host/x?token=***`.
fn redact_url(arg: &str) -> String {
    let Some(scheme_end) = arg.find("://") else {
        return arg.to_string();
    };
    let start = scheme_end + 3;
    let rest = &arg[start..];
    let authority_end = rest.find(['/', '?', '#']).unwrap_or(rest.len());
    let authority = &rest[..authority_end];

    let mut redacted = arg[..start].to_string();
    match authority.rsplit_once('@') {
        Some((user_info, host)) => match user_info.split_once(':') {
            Some((user, _)) => redacted.push_str(&format!("{user}:{HIDDEN}@{host}")),
            // `https://ghp_…@github.com`: a lone user name is often a token.
            None => redacted.push_str(&format!("{HIDDEN}@{host}")),
        },
        None => redacted.push_str(authority),
    }

    let tail = &rest[authority_end..];
    match tail.split_once('?') {
        Some((path, query)) => {
            redacted.push_str(path);
            redacted.push('?');
            let params: Vec<String> = query
                .split('&')
                .map(|param| match param.split_once('=') {
                    Some((name, _)) if is_secret_name(name) => format!("{name}={HIDDEN}"),
                    _ => param.to_string(),
                })
                .collect();
            redacted.push_str(&params.join("&"));
        }
        None => redacted.push_str(tail),
    }
    redacted
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shown(parts: &[&str]) -> String {
        let args: Vec<String> = parts.iter().map(|part| part.to_string()).collect();
        redact(&args).join(" ")
    }

    #[test]
    fn secret_values_are_hidden() {
        let cases: &[(&[&str], &str)] = &[
            (&["node", "x.js", "--token=abc"], "node x.js --token=***"),
            (
                &["node", "x.js", "--api-key", "abc", "--port", "3000"],
                "node x.js --api-key *** --port 3000",
            ),
            (
                &["node", "x.js", "NPM_TOKEN=abc"],
                "node x.js NPM_TOKEN=***",
            ),
            (
                &["node", "x.js", "--client_secret=abc"],
                "node x.js --client_secret=***",
            ),
            (
                &["curl", "-H", "Authorization: Bearer abc"],
                "curl -H Authorization: ***",
            ),
            (
                &["python", "x.py", "--password", "hunter2"],
                "python x.py --password ***",
            ),
            (
                &["npm", "--registry=https://me:pw@npm.example.com/"],
                "npm --registry=https://me:***@npm.example.com/",
            ),
            (
                &["git", "clone", "https://ghp_abc@github.com/a/b.git"],
                "git clone https://***@github.com/a/b.git",
            ),
            (
                &[
                    "node",
                    "x.js",
                    "https://api.example.com/v1?user=me&access_token=abc",
                ],
                "node x.js https://api.example.com/v1?user=me&access_token=***",
            ),
        ];
        for (parts, expected) in cases {
            assert_eq!(shown(parts), *expected, "{parts:?}");
        }
    }

    #[test]
    fn ordinary_arguments_are_left_alone() {
        let cases: &[&[&str]] = &[
            &[
                "node",
                "--max-old-space-size=4096",
                "server.js",
                "--port",
                "3000",
            ],
            &["node", "/app/node_modules/next/dist/bin/next", "dev"],
            &["python", "-m", "http.server", "8000"],
            &["node", "x.js", "https://example.com/a?page=2"],
            // A flag on its own doesn't hide the flag after it.
            &["node", "x.js", "--auth", "--verbose"],
        ];
        for parts in cases {
            assert_eq!(shown(parts), parts.join(" "), "{parts:?}");
        }
    }
}
