//! Request/response helpers for the nREPL ops that aren't streamed into
//! an [`OutputView`](crate::OutputView): one-shot evals whose value we
//! need as a string (`;; =>` comments, `in-ns`), and the cider-nrepl
//! tooling ops (`complete`, `info`, stacktrace analysis).
//!
//! cider-nrepl is optional. Every helper here degrades gracefully when the
//! server doesn't support an op: nREPL answers unknown ops with
//! `:status ["unknown-op" "done"]`, which we surface as `Ok(None)` /
//! an empty list rather than an error.

use anyhow::{Result, bail};
use futures::StreamExt as _;

use crate::client::{NreplClient, RequestStream};
use crate::{Value, dict};

/// Drains a request stream, returning every reply message.
pub async fn collect(mut stream: RequestStream) -> Vec<Value> {
    let mut messages = Vec::new();
    while let Some(message) = stream.next().await {
        messages.push(message);
    }
    messages
}

/// Status entries of a reply message, e.g. `["eval-error", "done"]`.
pub fn statuses(message: &Value) -> Vec<&str> {
    message
        .get("status")
        .and_then(Value::as_list)
        .map(|items| items.iter().filter_map(Value::as_str).collect())
        .unwrap_or_default()
}

fn has_status(messages: &[Value], status: &str) -> bool {
    messages
        .iter()
        .any(|message| statuses(message).contains(&status))
}

fn str_field(message: &Value, key: &str) -> Option<String> {
    message.get(key).and_then(Value::as_str).map(str::to_string)
}

/// Result of a one-shot evaluation.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct EvalOutcome {
    /// The last `:value` the server reported, if any.
    pub value: Option<String>,
    /// Concatenated `:out` text.
    pub out: String,
    /// Concatenated `:err` text.
    pub err: String,
    /// The namespace the server reported after evaluating.
    pub ns: Option<String>,
    /// `:ex` class of the exception, when evaluation failed.
    pub exception: Option<String>,
}

impl EvalOutcome {
    pub fn from_messages(messages: &[Value]) -> Self {
        let mut outcome = Self::default();
        for message in messages {
            if let Some(value) = str_field(message, "value") {
                outcome.value = Some(value);
            }
            if let Some(out) = message.get("out").and_then(Value::as_str) {
                outcome.out.push_str(out);
            }
            if let Some(err) = message.get("err").and_then(Value::as_str) {
                outcome.err.push_str(err);
            }
            if let Some(ns) = str_field(message, "ns") {
                outcome.ns = Some(ns);
            }
            if let Some(ex) = str_field(message, "ex").or_else(|| str_field(message, "root-ex")) {
                outcome.exception = Some(ex);
            }
        }
        outcome
    }
}

pub fn eval_request(code: &str, ns: Option<&str>, session: &str) -> Value {
    let mut entries = vec![
        ("op", Value::str("eval")),
        ("code", Value::str(code)),
        ("session", Value::str(session)),
    ];
    if let Some(ns) = ns {
        entries.push(("ns", Value::str(ns)));
    }
    dict(entries)
}

/// Evaluates `code` and waits for the complete reply.
pub async fn eval(
    client: &NreplClient,
    session: &str,
    ns: Option<&str>,
    code: &str,
) -> Result<EvalOutcome> {
    let messages = collect(client.send(eval_request(code, ns, session))?).await;
    if messages.is_empty() {
        bail!("connection closed before the evaluation finished");
    }
    if has_status(&messages, "namespace-not-found") {
        bail!("namespace {} not found", ns.unwrap_or("user"));
    }
    Ok(EvalOutcome::from_messages(&messages))
}

/// One candidate returned by cider-nrepl's `complete` op.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompletionCandidate {
    pub candidate: String,
    /// `function`, `macro`, `var`, `namespace`, `keyword`, `class`, ...
    pub kind: Option<String>,
    pub ns: Option<String>,
    pub doc: Option<String>,
}

/// Runs cider-nrepl's `complete` op. Returns an empty list when the
/// server doesn't support completion.
pub async fn complete(
    client: &NreplClient,
    session: &str,
    ns: &str,
    prefix: &str,
) -> Result<Vec<CompletionCandidate>> {
    let request = dict([
        ("op", Value::str("complete")),
        ("prefix", Value::str(prefix)),
        ("ns", Value::str(ns)),
        ("session", Value::str(session)),
        ("extra-metadata", Value::List(vec![Value::str("doc")])),
    ]);
    let messages = collect(client.send(request)?).await;
    Ok(parse_completions(&messages))
}

fn parse_completions(messages: &[Value]) -> Vec<CompletionCandidate> {
    messages
        .iter()
        .filter_map(|message| message.get("completions").and_then(Value::as_list))
        .flatten()
        .filter_map(|item| {
            Some(CompletionCandidate {
                candidate: str_field(item, "candidate")?,
                kind: str_field(item, "type"),
                ns: str_field(item, "ns"),
                doc: str_field(item, "doc"),
            })
        })
        .collect()
}

/// Symbol information returned by cider-nrepl's `info` op.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SymbolInfo {
    pub name: String,
    pub ns: Option<String>,
    pub arglists: Option<String>,
    pub doc: Option<String>,
    /// `file:/abs/path.clj` or `jar:file:...!/path.clj`.
    pub file: Option<String>,
    pub line: Option<u32>,
    pub column: Option<u32>,
}

impl SymbolInfo {
    /// Human-readable documentation, CIDER `cider-doc` style.
    pub fn render(&self) -> String {
        let mut text = match &self.ns {
            Some(ns) => format!("{ns}/{}", self.name),
            None => self.name.clone(),
        };
        if let Some(arglists) = &self.arglists {
            for arglist in arglists.lines() {
                text.push_str("\n  ");
                text.push_str(arglist.trim());
            }
        }
        if let Some(doc) = &self.doc {
            text.push_str("\n\n");
            text.push_str(doc.trim());
        }
        text
    }
}

/// Runs cider-nrepl's `info` op (falling back to nREPL's built-in
/// `lookup`). Returns `None` when the symbol can't be resolved.
pub async fn info(
    client: &NreplClient,
    session: &str,
    ns: &str,
    symbol: &str,
) -> Result<Option<SymbolInfo>> {
    for op in ["info", "lookup"] {
        let request = dict([
            ("op", Value::str(op)),
            ("sym", Value::str(symbol)),
            ("ns", Value::str(ns)),
            ("session", Value::str(session)),
        ]);
        let messages = collect(client.send(request)?).await;
        if has_status(&messages, "unknown-op") {
            continue;
        }
        return Ok(parse_info(&messages));
    }
    bail!("the nREPL server supports neither `info` nor `lookup`; add cider-nrepl middleware")
}

fn parse_info(messages: &[Value]) -> Option<SymbolInfo> {
    messages.iter().find_map(|message| {
        if statuses(message).contains(&"no-info") {
            return None;
        }
        // `lookup` nests the data under `:info`; `info` puts it at the top.
        let data = message
            .get("info")
            .filter(|info| info.as_dict().is_some_and(|map| !map.is_empty()))
            .unwrap_or(message);
        let name = str_field(data, "name")?;
        Some(SymbolInfo {
            name,
            ns: str_field(data, "ns"),
            arglists: str_field(data, "arglists-str").or_else(|| str_field(data, "arglists")),
            doc: str_field(data, "doc"),
            file: str_field(data, "file"),
            line: data.get("line").and_then(Value::as_int).map(|n| n as u32),
            column: data.get("column").and_then(Value::as_int).map(|n| n as u32),
        })
    })
}

/// One frame of an analyzed stacktrace.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StackFrame {
    pub name: String,
    pub file: Option<String>,
    /// `file:/abs/path.clj` when the frame lives in a local source file.
    pub file_url: Option<String>,
    pub line: Option<u32>,
    /// True when cider flags the frame as belonging to the user's project.
    pub project: bool,
}

/// One cause in an analyzed exception chain.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExceptionCause {
    pub class: String,
    pub message: String,
    pub frames: Vec<StackFrame>,
}

/// Fetches the last exception's analyzed stacktrace from cider-nrepl.
/// Tries the current `analyze-last-stacktrace` op first, then the
/// deprecated `stacktrace` op. Returns an empty list without cider.
pub async fn last_stacktrace(client: &NreplClient, session: &str) -> Result<Vec<ExceptionCause>> {
    for op in ["analyze-last-stacktrace", "stacktrace"] {
        let request = dict([("op", Value::str(op)), ("session", Value::str(session))]);
        let messages = collect(client.send(request)?).await;
        if has_status(&messages, "unknown-op") {
            continue;
        }
        return Ok(parse_stacktrace(&messages));
    }
    Ok(Vec::new())
}

fn parse_stacktrace(messages: &[Value]) -> Vec<ExceptionCause> {
    messages
        .iter()
        .filter_map(|message| {
            let class = str_field(message, "class")?;
            let frames = message
                .get("stacktrace")
                .and_then(Value::as_list)
                .unwrap_or_default()
                .iter()
                .filter_map(|frame| {
                    let flags: Vec<&str> = frame
                        .get("flags")
                        .and_then(Value::as_list)
                        .map(|flags| flags.iter().filter_map(Value::as_str).collect())
                        .unwrap_or_default();
                    if flags.contains(&"dup") {
                        return None;
                    }
                    let name = str_field(frame, "name")?;
                    let name = if flags.contains(&"clj") {
                        demunge_frame_name(&name)
                    } else {
                        name
                    };
                    Some(StackFrame {
                        name,
                        file: str_field(frame, "file"),
                        file_url: str_field(frame, "file-url"),
                        line: frame.get("line").and_then(Value::as_int).map(|n| n as u32),
                        project: flags.contains(&"project"),
                    })
                })
                .collect();
            Some(ExceptionCause {
                class,
                message: str_field(message, "message").unwrap_or_default(),
                frames,
            })
        })
        .collect()
}

/// Turns a compiled Clojure frame name (`my.app$f/invokeStatic`,
/// `my.app$eval123/invoke`) into the var it came from (`my.app/f`).
fn demunge_frame_name(name: &str) -> String {
    let class = name.split('/').next().unwrap_or(name);
    let Some((ns, rest)) = class.split_once('$') else {
        return name.to_string();
    };
    let function = rest.split('$').next().unwrap_or(rest);
    let function = function
        .replace("_QMARK_", "?")
        .replace("_BANG_", "!")
        .replace("_STAR_", "*")
        .replace("_GT_", ">")
        .replace("_LT_", "<")
        .replace("_EQ_", "=")
        .replace("_PLUS_", "+")
        .replace('_', "-");
    format!("{}/{function}", ns.replace('_', "-"))
}

/// Converts a `file:` URL (as reported by cider) to a local path.
/// Returns `None` for `jar:` URLs and anything else non-local.
pub fn local_path_from_file_url(url: &str) -> Option<std::path::PathBuf> {
    let path = url
        .strip_prefix("file://")
        .or_else(|| url.strip_prefix("file:"))?;
    if path.is_empty() {
        return None;
    }
    Some(std::path::PathBuf::from(percent_decode(path)))
}

fn percent_decode(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%'
            && index + 2 < bytes.len()
            && let Ok(byte) = u8::from_str_radix(&input[index + 1..index + 3], 16)
        {
            out.push(byte);
            index += 3;
            continue;
        }
        out.push(bytes[index]);
        index += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn status(items: &[&str]) -> Value {
        Value::List(items.iter().map(|s| Value::str(*s)).collect())
    }

    #[test]
    fn eval_outcome_collects_streams() {
        let messages = vec![
            dict([("out", Value::str("hi\n"))]),
            dict([("value", Value::str("42")), ("ns", Value::str("user"))]),
            dict([("status", status(&["done"]))]),
        ];
        let outcome = EvalOutcome::from_messages(&messages);
        assert_eq!(outcome.value.as_deref(), Some("42"));
        assert_eq!(outcome.out, "hi\n");
        assert_eq!(outcome.ns.as_deref(), Some("user"));
    }

    #[test]
    fn parses_completions() {
        let messages = vec![dict([(
            "completions",
            Value::List(vec![dict([
                ("candidate", Value::str("map")),
                ("type", Value::str("function")),
                ("ns", Value::str("clojure.core")),
            ])]),
        )])];
        let parsed = parse_completions(&messages);
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].candidate, "map");
        assert_eq!(parsed[0].kind.as_deref(), Some("function"));
    }

    #[test]
    fn parses_info_and_lookup_shapes() {
        let top_level = vec![dict([
            ("name", Value::str("map")),
            ("ns", Value::str("clojure.core")),
            ("arglists-str", Value::str("([f] [f coll])")),
            ("file", Value::str("file:/tmp/core.clj")),
            ("line", Value::Int(2700)),
        ])];
        let info = parse_info(&top_level).unwrap();
        assert_eq!(info.line, Some(2700));
        assert!(info.render().starts_with("clojure.core/map"));

        let nested = vec![dict([(
            "info",
            dict([
                ("name", Value::str("inc")),
                ("ns", Value::str("clojure.core")),
            ]),
        )])];
        assert_eq!(parse_info(&nested).unwrap().name, "inc");

        let missing = vec![dict([("status", status(&["no-info", "done"]))])];
        assert!(parse_info(&missing).is_none());
    }

    #[test]
    fn parses_stacktrace_skipping_dups() {
        let messages = vec![dict([
            ("class", Value::str("java.lang.ArithmeticException")),
            ("message", Value::str("Divide by zero")),
            (
                "stacktrace",
                Value::List(vec![
                    dict([
                        ("name", Value::str("my.app/f")),
                        ("file-url", Value::str("file:/src/my/app.clj")),
                        ("line", Value::Int(3)),
                        ("flags", Value::List(vec![Value::str("project")])),
                    ]),
                    dict([
                        ("name", Value::str("dup")),
                        ("flags", Value::List(vec![Value::str("dup")])),
                    ]),
                ]),
            ),
        ])];
        let causes = parse_stacktrace(&messages);
        assert_eq!(causes.len(), 1);
        assert_eq!(causes[0].frames.len(), 1);
        assert!(causes[0].frames[0].project);
    }

    #[test]
    fn demunges_clojure_frames() {
        assert_eq!(demunge_frame_name("my.app$f/invokeStatic"), "my.app/f");
        assert_eq!(
            demunge_frame_name("my_lib.core$valid_QMARK_/invoke"),
            "my-lib.core/valid?"
        );
        assert_eq!(
            demunge_frame_name("clojure.lang.Numbers/divide"),
            "clojure.lang.Numbers/divide"
        );
    }

    #[test]
    fn file_urls_to_paths() {
        assert_eq!(
            local_path_from_file_url("file:/a/b%20c.clj"),
            Some(std::path::PathBuf::from("/a/b c.clj"))
        );
        assert_eq!(
            local_path_from_file_url("file:///a/b.clj"),
            Some(std::path::PathBuf::from("/a/b.clj"))
        );
        assert_eq!(local_path_from_file_url("jar:file:/x.jar!/a.clj"), None);
    }
}
