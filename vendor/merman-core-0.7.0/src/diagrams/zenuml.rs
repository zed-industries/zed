use crate::{Error, ParseMetadata, Result};
use serde_json::Value;

/// Parses a ZenUML diagram into a Mermaid-like semantic model.
///
/// Upstream Mermaid integrates ZenUML via the `mermaid-zenuml` external diagram package, which
/// uses `@zenuml/core` in the browser. `merman` is headless and pure Rust, so for now we implement
/// a conservative compatibility mode: a small ZenUML subset is translated into Mermaid
/// `sequenceDiagram` syntax and then parsed by the existing sequence parser.
///
/// This is intended to support basic `Actor->Actor: message` diagrams for headless integrations.
pub fn parse_zenuml(code: &str, meta: &ParseMetadata) -> Result<Value> {
    let sequence_code = translate_zenuml_to_sequence(code, meta)?;
    crate::diagrams::sequence::parse_sequence(&sequence_code, meta)
}

pub fn parse_zenuml_model_for_render(
    code: &str,
    meta: &ParseMetadata,
) -> Result<crate::diagrams::sequence::SequenceDiagramRenderModel> {
    let sequence_code = translate_zenuml_to_sequence(code, meta)?;
    crate::diagrams::sequence::parse_sequence_model_for_render(&sequence_code, meta)
}

fn translate_zenuml_to_sequence(code: &str, meta: &ParseMetadata) -> Result<String> {
    let mut out: Vec<String> = vec!["sequenceDiagram".to_string()];

    let mut saw_header = false;
    let mut pending_comments: Vec<String> = Vec::new();
    let mut pending_return_annotator: bool = false;

    #[derive(Debug, Clone)]
    enum BlockKind {
        Loop,
        Opt,
        Par { branch_started: bool },
        IfAlt,
        TryAlt,
        SyncCall { actor: String },
    }

    fn starts_with_word_ci(haystack: &str, word: &str) -> bool {
        haystack
            .get(0..word.len())
            .is_some_and(|p| p.eq_ignore_ascii_case(word))
            && haystack
                .get(word.len()..word.len() + 1)
                .is_none_or(|c| c.chars().all(|ch| ch.is_ascii_whitespace() || ch == '('))
    }

    fn strip_trailing_open_brace(line: &str) -> Option<&str> {
        let trimmed = line.trim_end();
        trimmed.strip_suffix('{').map(str::trim_end)
    }

    fn translate_participant_decl(line: &str) -> Option<String> {
        // Participant order control:
        //   Bob
        //   Alice
        //
        // Annotators:
        //   @Actor Alice
        //   @Database Bob
        //
        // Aliases:
        //   A as Alice
        //   J as John
        let l = line.trim();
        if l.is_empty() {
            return None;
        }

        if let Some(rest) = l.strip_prefix('@') {
            let (kind, name) = rest.split_once(' ')?;
            let kind = kind.trim();
            let name = name.trim();
            if name.is_empty() {
                return None;
            }
            // Mermaid `sequenceDiagram` supports a limited set of participant kinds in our
            // headless parser today. Keep this translation conservative so fixtures can be
            // snapshot-gated deterministically.
            let kw = if kind.eq_ignore_ascii_case("actor") {
                "actor"
            } else {
                // `@Database`, `@Boundary`, etc. are represented as standard participants.
                "participant"
            };
            return Some(format!("{kw} {name}"));
        }

        if let Some((id, label)) = l.split_once(" as ") {
            let id = id.trim();
            let label = label.trim();
            if id.is_empty() || label.is_empty() {
                return None;
            }
            // ZenUML uses `A as Alice` where `A` is used in messages and `Alice` is the label.
            // Mermaid sequence supports `participant Alice as A` (label first, alias second).
            return Some(format!("participant {label} as {id}"));
        }

        // Bare participant/actor declaration (single token).
        if l.chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '.')
        {
            return Some(format!("participant {l}"));
        }

        None
    }

    fn translate_assignment(line: &str) -> Option<(String, String, String)> {
        // Minimal supported syntax (ZenUML docs "Reply message"):
        //   a = A.SyncMessage()
        //   SomeType a = A.SyncMessage()
        //
        // Returns (var, actor, call_text).
        let (lhs, rhs) = line.split_once('=')?;
        let lhs = lhs.trim();
        let rhs = rhs.trim();
        if lhs.is_empty() || rhs.is_empty() {
            return None;
        }

        let var = lhs.split_whitespace().last()?.trim();
        if var.is_empty() {
            return None;
        }

        let (actor, call) = rhs.split_once('.')?;
        let actor = actor.trim();
        let call = call.trim();
        if actor.is_empty() || call.is_empty() {
            return None;
        }

        Some((var.to_string(), actor.to_string(), call.to_string()))
    }

    fn translate_message_line(line: &str) -> Option<String> {
        // Minimal supported syntax:
        //   Alice->Bob: Hello
        //   Bob-->Alice: Reply
        //
        // Map to Mermaid sequence syntax:
        //   Alice->>Bob: Hello
        //   Bob-->>Alice: Reply
        let (lhs, label) = if let Some((a, b)) = line.split_once(':') {
            (a.trim(), Some(b.trim()))
        } else {
            (line.trim(), None)
        };

        let (from, arrow, to) = if let Some((a, b)) = lhs.split_once("-->") {
            (a.trim(), "-->", b.trim())
        } else if let Some((a, b)) = lhs.split_once("->") {
            (a.trim(), "->", b.trim())
        } else {
            return None;
        };

        if from.is_empty() || to.is_empty() {
            return None;
        }

        let seq_arrow = match arrow {
            "-->" => "-->>",
            "->" => "->>",
            _ => return None,
        };

        let mut out = String::new();
        out.push_str(from);
        out.push_str(seq_arrow);
        out.push_str(to);
        if let Some(lbl) = label {
            if !lbl.is_empty() {
                out.push_str(": ");
                out.push_str(lbl);
            }
        }
        Some(out)
    }

    fn flush_pending_comments_as_notes(
        pending: &mut Vec<String>,
        out: &mut Vec<String>,
        from: &str,
        to: &str,
    ) {
        if pending.is_empty() {
            return;
        }
        for c in pending.drain(..) {
            let text = c.trim();
            if text.is_empty() {
                continue;
            }
            // ZenUML comments are rendered above messages/fragments. Approximate this behavior
            // by emitting a Mermaid sequence note spanning the message participants.
            out.push(format!("Note over {from},{to}: {text}"));
        }
    }

    let mut stack: Vec<BlockKind> = Vec::new();

    fn par_maybe_and(stack: &mut [BlockKind], out: &mut Vec<String>) {
        let Some(BlockKind::Par { branch_started }) = stack.last_mut() else {
            return;
        };
        if *branch_started {
            out.push("and".to_string());
        } else {
            *branch_started = true;
        }
    }

    fn close_brace(rest: &str, stack: &mut Vec<BlockKind>, out: &mut Vec<String>) {
        let Some(top) = stack.last() else {
            return;
        };

        // For `if { ... } else { ... }` and `try { ... } catch { ... }`, the brace before the next
        // branch must *not* close the translated Mermaid fragment.
        match top {
            BlockKind::IfAlt => {
                if rest.starts_with("else") {
                    return;
                }
            }
            BlockKind::TryAlt => {
                if rest.starts_with("catch") || rest.starts_with("finally") {
                    return;
                }
            }
            BlockKind::SyncCall { .. } => {}
            _ => {}
        }

        let closed = stack.pop();
        match closed {
            Some(BlockKind::SyncCall { actor }) => {
                out.push(format!("deactivate {actor}"));
            }
            Some(_) => {
                out.push("end".to_string());
            }
            None => {}
        }
    }

    for raw in code.lines() {
        let mut line = raw.trim();
        if line.is_empty() {
            continue;
        }

        if !saw_header && line.to_ascii_lowercase().starts_with("zenuml") {
            saw_header = true;
            continue;
        }

        // ZenUML renders `// ...` comments above the following messages/fragments.
        // - a comment on a participant will not be rendered
        // - a comment on a message should be rendered
        if let Some(c) = line.strip_prefix("//") {
            pending_comments.push(c.trim().to_string());
            continue;
        }

        // ZenUML reply annotators:
        //   @return
        //   @reply
        //
        // These affect the next message. We approximate this by forcing the next message to use
        // a Mermaid-style "return" arrow (`-->>`) regardless of the original arrow.
        if line.eq_ignore_ascii_case("@return") || line.eq_ignore_ascii_case("@reply") {
            pending_return_annotator = true;
            continue;
        }

        // Handle leading close braces, including `} else {` and `} catch {` forms.
        loop {
            let trimmed = line.trim_start();
            if !trimmed.starts_with('}') {
                line = trimmed;
                break;
            }
            let rest = trimmed[1..].trim_start();
            close_brace(rest, &mut stack, &mut out);
            line = rest;
            if line.is_empty() {
                break;
            }
        }
        if line.is_empty() {
            continue;
        }

        // Pass through common metadata directives as-is when possible.
        if line.to_ascii_lowercase().starts_with("title ") {
            out.push(line.to_string());
            pending_comments.clear();
            continue;
        }
        if line.to_ascii_lowercase().starts_with("acctitle ") {
            out.push(line.to_string());
            pending_comments.clear();
            continue;
        }
        if line.to_ascii_lowercase().starts_with("accdescr ") {
            out.push(line.to_string());
            pending_comments.clear();
            continue;
        }

        // Branch continuations for `if` and `try` structures.
        if let Some(prefix) = strip_trailing_open_brace(line) {
            let p = prefix.trim();
            if starts_with_word_ci(p, "else if") {
                let Some((_, cond)) = p.split_once('(') else {
                    return Err(Error::DiagramParse {
                        diagram_type: meta.diagram_type.clone(),
                        message: format!("unsupported zenuml statement: {line}"),
                    });
                };
                let Some((cond, _)) = cond.rsplit_once(')') else {
                    return Err(Error::DiagramParse {
                        diagram_type: meta.diagram_type.clone(),
                        message: format!("unsupported zenuml statement: {line}"),
                    });
                };
                let label = format!("if({})", cond.trim());
                out.push(format!("else {label}"));
                pending_comments.clear();
                continue;
            }
            if starts_with_word_ci(p, "else") {
                out.push("else".to_string());
                pending_comments.clear();
                continue;
            }
            if starts_with_word_ci(p, "catch") {
                out.push("else catch".to_string());
                pending_comments.clear();
                continue;
            }
            if starts_with_word_ci(p, "finally") {
                out.push("else finally".to_string());
                pending_comments.clear();
                continue;
            }
        }

        // Block openings.
        if let Some(prefix) = strip_trailing_open_brace(line) {
            let p = prefix.trim();

            if starts_with_word_ci(p, "while") {
                par_maybe_and(&mut stack, &mut out);
                out.push(format!("loop {p}"));
                stack.push(BlockKind::Loop);
                pending_comments.clear();
                continue;
            }
            if starts_with_word_ci(p, "for")
                || starts_with_word_ci(p, "foreach")
                || starts_with_word_ci(p, "forEach")
                || starts_with_word_ci(p, "loop")
            {
                par_maybe_and(&mut stack, &mut out);
                out.push(format!("loop {p}"));
                stack.push(BlockKind::Loop);
                pending_comments.clear();
                continue;
            }
            if starts_with_word_ci(p, "opt") {
                par_maybe_and(&mut stack, &mut out);
                let label = p.strip_prefix("opt").unwrap_or("").trim();
                if label.is_empty() {
                    out.push("opt".to_string());
                } else {
                    out.push(format!("opt {label}"));
                }
                stack.push(BlockKind::Opt);
                pending_comments.clear();
                continue;
            }
            if starts_with_word_ci(p, "par") {
                par_maybe_and(&mut stack, &mut out);
                let label = p.strip_prefix("par").unwrap_or("").trim();
                if label.is_empty() {
                    out.push("par".to_string());
                } else {
                    out.push(format!("par {label}"));
                }
                stack.push(BlockKind::Par {
                    branch_started: false,
                });
                pending_comments.clear();
                continue;
            }
            if starts_with_word_ci(p, "if") {
                par_maybe_and(&mut stack, &mut out);
                let Some((_, cond)) = p.split_once('(') else {
                    return Err(Error::DiagramParse {
                        diagram_type: meta.diagram_type.clone(),
                        message: format!("unsupported zenuml statement: {line}"),
                    });
                };
                let Some((cond, _)) = cond.rsplit_once(')') else {
                    return Err(Error::DiagramParse {
                        diagram_type: meta.diagram_type.clone(),
                        message: format!("unsupported zenuml statement: {line}"),
                    });
                };
                out.push(format!("alt if({})", cond.trim()));
                stack.push(BlockKind::IfAlt);
                pending_comments.clear();
                continue;
            }
            if starts_with_word_ci(p, "try") {
                par_maybe_and(&mut stack, &mut out);
                out.push("alt try".to_string());
                stack.push(BlockKind::TryAlt);
                pending_comments.clear();
                continue;
            }

            // Sync message / method-call blocks:
            //   A.SyncMessage(with, parameters) { ... }
            //
            // Translate to a self-message plus explicit activation scope.
            if let Some((actor, method)) = p.split_once('.') {
                let actor = actor.trim();
                let method = method.trim();
                if !actor.is_empty() && !method.is_empty() {
                    par_maybe_and(&mut stack, &mut out);
                    flush_pending_comments_as_notes(&mut pending_comments, &mut out, actor, actor);
                    out.push(format!("{actor}->>{actor}: {method}"));
                    out.push(format!("activate {actor}"));
                    stack.push(BlockKind::SyncCall {
                        actor: actor.to_string(),
                    });
                    continue;
                }
            }
        }

        // Creation messages:
        //   new A1
        //   new A2(with, parameters)
        if let Some(rest) = line.strip_prefix("new ") {
            let rest = rest.trim();
            if rest.is_empty() {
                return Err(Error::DiagramParse {
                    diagram_type: meta.diagram_type.clone(),
                    message: format!("unsupported zenuml statement: {line}"),
                });
            }

            // Extract a stable id for Mermaid sequence: the leading identifier token.
            let chars = rest.chars();
            let mut id = String::new();
            for ch in chars {
                if ch.is_ascii_alphanumeric() || ch == '_' || ch == '.' {
                    id.push(ch);
                } else {
                    break;
                }
            }
            if id.is_empty() {
                return Err(Error::DiagramParse {
                    diagram_type: meta.diagram_type.clone(),
                    message: format!("unsupported zenuml statement: {line}"),
                });
            }

            par_maybe_and(&mut stack, &mut out);
            pending_comments.clear();

            // If the creation has arguments, keep the full text as the label (description).
            if rest != id {
                out.push(format!("create participant {id} as {rest}"));
            } else {
                out.push(format!("create participant {id}"));
            }
            continue;
        }

        // Participants.
        if let Some(decl) = translate_participant_decl(line) {
            par_maybe_and(&mut stack, &mut out);
            out.push(decl);
            // ZenUML comment on a participant is not rendered.
            pending_comments.clear();
            continue;
        }

        // Reply assignments must be handled before generic `Actor.Method(...)` parsing, because
        // an assignment line contains a `.` and would otherwise be misinterpreted as a sync call.
        if let Some((var, actor, call)) = translate_assignment(line) {
            par_maybe_and(&mut stack, &mut out);
            flush_pending_comments_as_notes(&mut pending_comments, &mut out, &actor, &actor);
            out.push(format!("{actor}->>{actor}: {call} => {var}"));
            pending_return_annotator = false;
            continue;
        }

        // Sync messages without blocks:
        //   A.SyncMessage
        //   A.SyncMessage(with, parameters)
        if let Some((actor, method)) = line.split_once('.') {
            let actor = actor.trim();
            let method = method.trim();
            if !actor.is_empty() && !method.is_empty() {
                par_maybe_and(&mut stack, &mut out);
                flush_pending_comments_as_notes(&mut pending_comments, &mut out, actor, actor);
                out.push(format!("{actor}->>{actor}: {method}"));
                continue;
            }
        }

        // Return statements inside sync call blocks.
        if let Some(rest) = line.strip_prefix("return ") {
            let Some(actor) = stack.last().and_then(|b| match b {
                BlockKind::SyncCall { actor } => Some(actor.clone()),
                _ => None,
            }) else {
                return Err(Error::DiagramParse {
                    diagram_type: meta.diagram_type.clone(),
                    message: format!("unsupported zenuml statement: {line}"),
                });
            };
            par_maybe_and(&mut stack, &mut out);
            flush_pending_comments_as_notes(&mut pending_comments, &mut out, &actor, &actor);
            out.push(format!("{actor}-->>{actor}: {}", rest.trim()));
            pending_return_annotator = false;
            continue;
        }

        if let Some(mut seq_line) = translate_message_line(line) {
            par_maybe_and(&mut stack, &mut out);
            let (lhs, _) = if let Some((a, b)) = line.split_once(':') {
                (a.trim(), Some(b.trim()))
            } else {
                (line.trim(), None)
            };
            let (from, to) = if let Some((a, b)) = lhs.split_once("-->") {
                (a.trim(), b.trim())
            } else if let Some((a, b)) = lhs.split_once("->") {
                (a.trim(), b.trim())
            } else {
                ("", "")
            };
            flush_pending_comments_as_notes(&mut pending_comments, &mut out, from, to);
            if pending_return_annotator {
                // Convert `->>` to `-->>` for return/reply.
                seq_line = seq_line.replace("->>", "-->>");
                pending_return_annotator = false;
            }
            out.push(seq_line);
            continue;
        }

        return Err(Error::DiagramParse {
            diagram_type: meta.diagram_type.clone(),
            message: format!("unsupported zenuml statement: {line}"),
        });
    }

    Ok(out.join("\n"))
}

#[cfg(test)]
mod tests {
    use crate::{Engine, ParseOptions, RenderSemanticModel};

    #[test]
    fn zenuml_basic_translates_to_sequence_model() {
        let engine = Engine::new();
        let input = "zenuml\n  Alice->Bob: Hello\n  Bob-->Alice: Reply\n";
        let parsed =
            futures::executor::block_on(engine.parse_diagram(input, ParseOptions::lenient()))
                .unwrap()
                .unwrap();
        assert_eq!(parsed.meta.diagram_type, "zenuml");
        assert!(parsed.model.get("messages").is_some());
    }

    #[test]
    fn zenuml_participants_and_fragments_translate_to_sequence_model() {
        let engine = Engine::new();
        let input = r#"zenuml
title Demo
Bob
Alice
Alice->Bob: Hi Bob
while(true) {
  Bob->Alice: Hi Alice
}
if(is_sick) {
  Bob->Alice: Not so good :(
} else {
  Bob->Alice: Feeling fresh
}
opt {
  Bob->Alice: Thanks
}
par {
  Alice->Bob: Hello guys!
  Alice->John: Hello guys!
}
"#;
        let parsed =
            futures::executor::block_on(engine.parse_diagram(input, ParseOptions::lenient()))
                .unwrap()
                .unwrap();
        assert_eq!(parsed.meta.diagram_type, "zenuml");
        assert!(parsed.model.get("messages").is_some());
    }

    #[test]
    fn zenuml_reply_message_forms_translate() {
        let engine = Engine::new();
        let input = r#"zenuml
SomeType a = A.SyncMessage()
a = A.SyncMessage()
A.SyncMessage() {
  return result
}
@return
A->B: ok
"#;
        let parsed =
            futures::executor::block_on(engine.parse_diagram(input, ParseOptions::lenient()))
                .unwrap()
                .unwrap();
        assert_eq!(parsed.meta.diagram_type, "zenuml");
        assert!(parsed.model.get("messages").is_some());
    }

    #[test]
    fn zenuml_render_model_uses_sequence_typed_variant_without_changing_json_parse() {
        let engine = Engine::new();
        let input = r#"zenuml
title Login Flow
Alice->Bob: Login
Bob-->Alice: Ack
"#;

        let parsed = engine
            .parse_diagram_for_render_model_sync(input, ParseOptions::strict())
            .unwrap()
            .unwrap();

        assert_eq!(parsed.meta.diagram_type, "zenuml");
        match parsed.model {
            RenderSemanticModel::Sequence(model) => {
                assert_eq!(model.title.as_deref(), Some("Login Flow"));
                assert_eq!(model.messages.len(), 2);
                assert_eq!(model.messages[0].from.as_deref(), Some("Alice"));
                assert_eq!(model.messages[0].to.as_deref(), Some("Bob"));
                assert_eq!(model.messages[0].message_text(), "Login");
            }
            other => {
                panic!("zenuml render parse should return sequence typed model, got {other:?}")
            }
        }

        let parsed_json = engine
            .parse_diagram_sync(input, ParseOptions::strict())
            .unwrap()
            .unwrap();
        assert_eq!(parsed_json.meta.diagram_type, "zenuml");
        assert!(parsed_json.model.get("messages").is_some());
        assert_eq!(parsed_json.model["title"], serde_json::json!("Login Flow"));
    }
}
