//! Which npm package an `import` or `require` in the code refers to.

/// Node.js's own modules: imported by name, but not npm packages.
const NODE_BUILTINS: &[&str] = &[
    "assert",
    "async_hooks",
    "buffer",
    "child_process",
    "cluster",
    "console",
    "constants",
    "crypto",
    "dgram",
    "diagnostics_channel",
    "dns",
    "domain",
    "events",
    "fs",
    "http",
    "http2",
    "https",
    "inspector",
    "module",
    "net",
    "os",
    "path",
    "perf_hooks",
    "process",
    "punycode",
    "querystring",
    "readline",
    "repl",
    "stream",
    "string_decoder",
    "sys",
    "timers",
    "tls",
    "trace_events",
    "tty",
    "url",
    "util",
    "v8",
    "vm",
    "wasi",
    "worker_threads",
    "zlib",
];

/// The npm package `specifier` (an import's module string) names:
/// `lodash/debounce` names `lodash`, `@mui/material/Button` names
/// `@mui/material`. `None` for a relative or absolute path, a Node.js built-in,
/// or anything that isn't a package name.
pub fn package_from_specifier(specifier: &str) -> Option<String> {
    let specifier = specifier.trim();
    if specifier.is_empty()
        || specifier.starts_with('.')
        || specifier.starts_with('/')
        || specifier.starts_with("node:")
    {
        return None;
    }
    let mut parts = specifier.split('/');
    let first = parts.next()?;
    let name = if let Some(scope) = first.strip_prefix('@') {
        let package = parts.next()?;
        if scope.is_empty() || package.is_empty() {
            return None;
        }
        format!("{first}/{package}")
    } else {
        first.to_string()
    };
    if NODE_BUILTINS.contains(&name.as_str()) {
        return None;
    }
    let plain = |part: &str| {
        !part.is_empty()
            && part
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.' | '~'))
    };
    name.trim_start_matches('@')
        .split('/')
        .all(plain)
        .then_some(name)
}

/// The package named by the quoted string around byte `column` of `line`:
/// the `'lodash/debounce'` in `import debounce from 'lodash/debounce'`.
/// `None` when the cursor isn't inside a quoted string.
pub fn package_at(line: &str, column: usize) -> Option<String> {
    let before = line.get(..column.min(line.len()))?;
    let start = before.rfind(['\'', '"', '`'])?;
    let quote = line[start..].chars().next()?;
    let end = start + 1 + line[start + 1..].find(quote)?;
    if column > end {
        return None;
    }
    package_from_specifier(&line[start + 1..end])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn specifiers_name_their_package() {
        let cases = [
            ("lodash", Some("lodash")),
            ("lodash/debounce", Some("lodash")),
            ("@mui/material", Some("@mui/material")),
            ("@mui/material/Button", Some("@mui/material")),
            ("./utils", None),
            ("../x", None),
            ("/abs/path", None),
            ("node:fs", None),
            ("fs", None),
            ("path", None),
            ("@scope", None),
            ("has space", None),
            ("", None),
        ];
        for (specifier, expected) in cases {
            assert_eq!(
                package_from_specifier(specifier).as_deref(),
                expected,
                "{specifier}"
            );
        }
    }

    #[test]
    fn the_quoted_string_under_the_cursor_is_read() {
        let line = "import debounce from 'lodash/debounce';";
        let inside = line.find("debounce'").unwrap();
        assert_eq!(package_at(line, inside).as_deref(), Some("lodash"));
        // On the import's own name, outside the quotes: nothing.
        assert_eq!(package_at(line, 9), None);

        let line = r#"const x = require("@scope/pkg/sub");"#;
        let inside = line.find("pkg").unwrap();
        assert_eq!(package_at(line, inside).as_deref(), Some("@scope/pkg"));
    }
}
