//! Stop a turn that is repeating itself.
//!
//! Two shapes show up on local models. The same tool call comes back with the
//! same arguments, or one completion repeats a passage of reasoning and never
//! calls a tool. Neither is visible to the model server.

/// Stop on the third model completion in a row that repeats a tool call.
/// Two repeats still run: a command that fails once is often run again on
/// purpose. Copies of that call inside one completion count once, so a
/// response that asks for the same tool several times in parallel still runs.
pub const IDENTICAL_TOOL_CALL_LIMIT: usize = 3;

/// A passage has to be this long before a repeat counts. Short phrases
/// ("let me check") show up in ordinary reasoning.
pub const THINKING_REPEAT_CHARS: usize = 80;

/// How many times that passage must occur, including the copy just streamed.
pub const THINKING_REPEAT_COUNT: usize = 3;

pub const TOOL_LOOP_NOTICE: &str = "Stopped: the same tool call was repeated. Say how to proceed.";

pub const THINKING_LOOP_NOTICE: &str =
    "Stopped: the reasoning repeated the same passage. Say how to proceed.";

pub fn tool_fingerprint(name: &str, input_json: &str) -> String {
    format!("{name}\n{input_json}")
}

/// `rounds` is oldest-first. Each round is one model completion and already
/// includes the call about to run. Empty rounds are skipped. A round that
/// does not contain `fingerprint` breaks the streak.
pub fn repeated_tool_rounds<Round, Call>(rounds: &[Round], fingerprint: &str) -> bool
where
    Round: AsRef<[Call]>,
    Call: AsRef<str>,
{
    let mut streak = 0;
    for round in rounds.iter().rev() {
        let calls = round.as_ref();
        if calls.is_empty() {
            continue;
        }
        if calls.iter().any(|call| call.as_ref() == fingerprint) {
            streak += 1;
            if streak >= IDENTICAL_TOOL_CALL_LIMIT {
                return true;
            }
        } else {
            break;
        }
    }
    false
}

pub fn thinking_repeats(text: &str) -> bool {
    let Some(tail) = suffix_chars(text, THINKING_REPEAT_CHARS) else {
        return false;
    };
    text.matches(tail).count() >= THINKING_REPEAT_COUNT
}

fn suffix_chars(text: &str, chars: usize) -> Option<&str> {
    let mut seen = 0;
    for (index, _) in text.char_indices().rev() {
        seen += 1;
        if seen == chars {
            return Some(&text[index..]);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_identical_tool_calls_still_run() {
        let fingerprint = tool_fingerprint("terminal", r#"{"command":"ls"}"#);
        let rounds = vec![vec![fingerprint.clone()], vec![fingerprint.clone()]];
        assert!(!repeated_tool_rounds(&rounds, &fingerprint));
    }

    #[test]
    fn third_identical_tool_call_stops() {
        let fingerprint = tool_fingerprint("read_file", r#"{"path":"a.rs"}"#);
        let rounds = vec![
            vec![fingerprint.clone()],
            vec![fingerprint.clone()],
            vec![fingerprint.clone()],
        ];
        assert!(repeated_tool_rounds(&rounds, &fingerprint));
    }

    #[test]
    fn parallel_copies_in_one_completion_still_run() {
        let fingerprint = tool_fingerprint("read_file", r#"{"path":"a.rs"}"#);
        let rounds = vec![vec![
            fingerprint.clone(),
            fingerprint.clone(),
            fingerprint.clone(),
        ]];
        assert!(!repeated_tool_rounds(&rounds, &fingerprint));
    }

    #[test]
    fn a_different_call_breaks_the_streak() {
        let read = tool_fingerprint("read_file", r#"{"path":"a.rs"}"#);
        let grep = tool_fingerprint("grep", r#"{"regex":"fn"}"#);
        let rounds = vec![vec![read.clone()], vec![grep], vec![read.clone()]];
        assert!(!repeated_tool_rounds(&rounds, &read));
    }

    #[test]
    fn thinking_repeat_needs_three_long_copies() {
        let passage = "x".repeat(THINKING_REPEAT_CHARS);
        assert!(!thinking_repeats(&passage));
        assert!(!thinking_repeats(&passage.repeat(2)));
        assert!(thinking_repeats(&passage.repeat(3)));
    }

    #[test]
    fn short_repeated_phrases_do_not_count() {
        let phrase = "Let me check.\n".repeat(8);
        assert!(!thinking_repeats(&phrase));
    }

    #[test]
    fn multibyte_suffix_is_on_a_char_boundary() {
        let passage = "é".repeat(THINKING_REPEAT_CHARS);
        assert!(thinking_repeats(&passage.repeat(3)));
    }
}
