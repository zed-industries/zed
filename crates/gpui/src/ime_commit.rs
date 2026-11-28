//! Deciding what to do with a printable character that an input source commits as raw text.
//!
//! Some input sources, when the active input mode is not composing, answer a plain key press by
//! committing the character through `insertText:` instead of composing it. Sogou's English mode
//! does this for every letter. That is fine while the key is meant to be typed, but it breaks
//! multi-stroke key bindings: the input source swallows the first stroke of `j k`, so the chord
//! can never match. See <https://github.com/zed-industries/zed/issues/53630>.
//!
//! The fix must not touch composition: an input source that starts composing reports characters
//! through `setMarkedText:` and that path is left alone. It must also never lose a character, and
//! it must tolerate commits that arrive late — Sogou can commit a character a few hundred
//! milliseconds after the key event returns, when it finishes a composition.
//!
//! This module is the whole decision, kept free of platform plumbing so the timing rules can be
//! tested directly: the caller installs the keystroke it handed to the input source, reports each
//! commit, and acts on the verdict.

use std::time::{Duration, Instant};

use crate::Keystroke;

/// How long a raw-text commit is still attributed to the printable key handed to the input source.
///
/// Sogou has been observed committing ~700ms after the key event returned, so this is comfortably
/// above that, and bounded so a stale entry can never claim an unrelated character.
pub const IME_COMMIT_TIMEOUT: Duration = Duration::from_secs(2);

/// What the caller should do with a commit reported to [`ImeCommitState::commit`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImeCommitVerdict {
    /// Drop the commit and dispatch the keystroke to key-binding matching. The caller must
    /// re-commit the character itself if no binding consumes the key.
    Divert,
    /// Drop the commit: it is a repeat of a key press already diverted, so the character is
    /// already accounted for.
    Duplicate,
    /// Type the text as usual.
    Passthrough,
}

/// A printable key handed to the input source whose raw-text commit may arrive later.
#[derive(Clone, Debug)]
struct PendingCommit {
    keystroke: Keystroke,
    begins_binding: bool,
    deadline: Instant,
    /// Set once this key's commit was dropped. A second commit for the same key press is then a
    /// [`ImeCommitVerdict::Duplicate`] rather than a fresh diversion. A new printable key press
    /// replaces the entry, so this never suppresses a separate keystroke.
    diverted: bool,
}

/// Tracks the printable key currently handed to the input source and decides what to do with the
/// raw-text commits it produces.
#[derive(Debug, Default)]
pub struct ImeCommitState {
    pending: Option<PendingCommit>,
    /// Set when a commit was dropped while the key event was still on the stack, i.e. from inside
    /// the input context's callback. The caller dispatches it once the callback returns.
    suppressed: Option<Keystroke>,
    /// True between [`Self::begin_key_event`] and [`Self::end_key_event`].
    in_key_event: bool,
}

impl ImeCommitState {
    /// Records the printable keystroke about to be handed to the input source.
    ///
    /// `begins_binding` is the application's answer to "would this key start or extend a
    /// multi-stroke binding in the focused context?". Entries are only installed for printable
    /// keys; composition is reported through marked text and must never reach this decision.
    pub fn begin_key_event(
        &mut self,
        keystroke: Keystroke,
        begins_binding: bool,
        now: Instant,
    ) -> bool {
        self.pending = Some(PendingCommit {
            keystroke,
            begins_binding,
            deadline: now + IME_COMMIT_TIMEOUT,
            diverted: false,
        });
        self.suppressed = None;
        self.in_key_event = true;
        begins_binding
    }

    /// Ends the key event, returning the keystroke whose commit was dropped from inside the input
    /// context's callback. The pending entry survives, so a late commit for the same key is still
    /// recognised (and, having already been diverted, dropped as a duplicate).
    pub fn end_key_event(&mut self) -> Option<Keystroke> {
        self.in_key_event = false;
        self.suppressed.take()
    }

    /// Forgets the pending entry: the input source is composing, declined the key, or the key was
    /// committed as text, so there is nothing left to divert.
    ///
    /// The diverted dispatch is *not* discarded: `setMarkedText`/`unmarkText` can follow the
    /// `insertText` for the same key, and the keystroke that was already dropped from the commit
    /// still has to reach key-binding matching.
    pub fn clear_pending(&mut self) {
        self.pending = None;
    }

    /// Forgets everything, including a diverted dispatch that has not been handed back yet.
    pub fn clear(&mut self) {
        self.pending = None;
        self.suppressed = None;
    }

    /// Whether a commit is currently being reported from inside the input context's callback.
    ///
    /// The caller must not re-enter the application from there; it dispatches the diverted
    /// keystroke once the callback returns.
    pub fn is_in_key_event(&self) -> bool {
        self.in_key_event
    }

    /// The keystroke whose raw-text commit is pending, if any.
    ///
    /// Only meaningful after [`Self::commit`] returned [`ImeCommitVerdict::Divert`], which is when
    /// the caller needs to know which key to dispatch.
    pub fn pending_keystroke(&self) -> Option<&Keystroke> {
        self.pending.as_ref().map(|pending| &pending.keystroke)
    }

    /// Whether a commit arriving now would be for the key handed to the input source.
    pub fn has_pending_entry(&self, now: Instant) -> bool {
        self.pending
            .as_ref()
            .is_some_and(|pending| pending.deadline >= now)
    }

    /// Classifies a raw-text commit of `text`.
    ///
    /// `replacement_range` is the range the input source asked to replace; only a plain insertion
    /// is ever diverted, because replacing existing text is not "typing this key".
    pub fn commit(
        &mut self,
        text: &str,
        replacement_range_is_none: bool,
        now: Instant,
    ) -> ImeCommitVerdict {
        if self
            .pending
            .as_ref()
            .is_some_and(|pending| pending.deadline < now)
        {
            self.pending = None;
        }

        let Some(pending) = self.pending.as_mut() else {
            return ImeCommitVerdict::Passthrough;
        };

        let matches_key = pending.begins_binding
            && replacement_range_is_none
            && text.chars().count() == 1
            && pending.keystroke.key_char.as_deref() == Some(text);

        if !matches_key {
            // The input source committed this key as text after all, so nothing is pending for it.
            self.pending = None;
            return ImeCommitVerdict::Passthrough;
        }

        if pending.diverted {
            return ImeCommitVerdict::Duplicate;
        }

        pending.diverted = true;
        if self.in_key_event {
            self.suppressed = Some(pending.keystroke.clone());
        }
        ImeCommitVerdict::Divert
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn keystroke(key: &str) -> Keystroke {
        Keystroke {
            key: key.to_string(),
            key_char: Some(key.to_string()),
            ..Default::default()
        }
    }

    #[test]
    fn plain_text_is_never_diverted() {
        let mut state = ImeCommitState::default();
        let now = Instant::now();
        assert_eq!(state.commit("a", true, now), ImeCommitVerdict::Passthrough);
    }

    #[test]
    fn sync_commit_of_a_chord_prefix_is_diverted_and_handed_back_to_the_key_event() {
        let mut state = ImeCommitState::default();
        let now = Instant::now();
        assert!(state.begin_key_event(keystroke("j"), true, now));
        assert_eq!(state.commit("j", true, now), ImeCommitVerdict::Divert);
        assert_eq!(state.end_key_event(), Some(keystroke("j")));
    }

    #[test]
    fn a_single_stroke_binding_is_not_diverted() {
        let mut state = ImeCommitState::default();
        let now = Instant::now();
        assert!(!state.begin_key_event(keystroke("j"), false, now));
        assert_eq!(state.commit("j", true, now), ImeCommitVerdict::Passthrough);
        assert_eq!(state.end_key_event(), None);
        assert!(!state.has_pending_entry(now));
    }

    #[test]
    fn a_late_commit_of_a_chord_prefix_is_diverted_after_the_key_event_returned() {
        let mut state = ImeCommitState::default();
        let now = Instant::now();
        state.begin_key_event(keystroke("j"), true, now);
        assert_eq!(state.end_key_event(), None);
        let late = now + Duration::from_millis(712);
        assert_eq!(state.commit("j", true, late), ImeCommitVerdict::Divert);
    }

    #[test]
    fn a_repeat_commit_of_the_same_key_press_is_dropped() {
        let mut state = ImeCommitState::default();
        let now = Instant::now();
        state.begin_key_event(keystroke("j"), true, now);
        assert_eq!(state.commit("j", true, now), ImeCommitVerdict::Divert);
        assert_eq!(state.end_key_event(), Some(keystroke("j")));
        assert_eq!(
            state.commit("j", true, now + Duration::from_millis(20)),
            ImeCommitVerdict::Duplicate
        );
    }

    #[test]
    fn a_new_key_press_is_not_suppressed_by_the_previous_one() {
        let mut state = ImeCommitState::default();
        let now = Instant::now();
        state.begin_key_event(keystroke("j"), true, now);
        assert_eq!(state.commit("j", true, now), ImeCommitVerdict::Divert);
        state.end_key_event();

        // The user presses `j` again while the first one is still pending at the binding level.
        state.begin_key_event(keystroke("j"), true, now + Duration::from_millis(90));
        assert_eq!(
            state.commit("j", true, now + Duration::from_millis(90)),
            ImeCommitVerdict::Divert
        );
        assert_eq!(state.end_key_event(), Some(keystroke("j")));
    }

    #[test]
    fn a_commit_too_late_to_belong_to_the_key_is_typed() {
        let mut state = ImeCommitState::default();
        let now = Instant::now();
        state.begin_key_event(keystroke("j"), true, now);
        state.end_key_event();
        let stale = now + IME_COMMIT_TIMEOUT + Duration::from_millis(1);
        assert_eq!(
            state.commit("j", true, stale),
            ImeCommitVerdict::Passthrough
        );
    }

    #[test]
    fn a_commit_that_replaces_text_is_typed() {
        let mut state = ImeCommitState::default();
        let now = Instant::now();
        state.begin_key_event(keystroke("j"), true, now);
        assert_eq!(state.commit("j", false, now), ImeCommitVerdict::Passthrough);
    }

    #[test]
    fn a_multi_character_commit_is_typed() {
        let mut state = ImeCommitState::default();
        let now = Instant::now();
        state.begin_key_event(keystroke("j"), true, now);
        assert_eq!(state.commit("ji", true, now), ImeCommitVerdict::Passthrough);
    }

    #[test]
    fn a_commit_that_was_already_typed_by_the_key_event_is_not_diverted() {
        // The input source declined the key (`doCommandBySelector:`) and it reached the bindings
        // as a plain key press; if a commit still follows, it belongs to nothing we diverted.
        let mut state = ImeCommitState::default();
        let now = Instant::now();
        state.begin_key_event(keystroke("j"), true, now);
        state.clear();
        assert_eq!(state.commit("j", true, now), ImeCommitVerdict::Passthrough);
    }

    #[test]
    fn composition_clears_the_pending_entry() {
        let mut state = ImeCommitState::default();
        let now = Instant::now();
        state.begin_key_event(keystroke("j"), true, now);
        state.end_key_event();
        // `setMarkedText` arrived: the key started a composition instead of committing.
        state.clear_pending();
        assert_eq!(state.commit("j", true, now), ImeCommitVerdict::Passthrough);
    }

    #[test]
    fn clearing_the_pending_entry_keeps_a_diverted_dispatch() {
        // Sogou can send `insertText:` and then `setMarkedText:` for the same key press. The
        // diverted keystroke has not been handed back yet at that point, so `clear_pending` must
        // not drop it: doing so would swallow the first stroke of `j k`.
        let mut state = ImeCommitState::default();
        let now = Instant::now();
        state.begin_key_event(keystroke("j"), true, now);
        assert_eq!(state.commit("j", true, now), ImeCommitVerdict::Divert);
        state.clear_pending();
        assert!(!state.has_pending_entry(now));
        assert_eq!(state.end_key_event(), Some(keystroke("j")));
    }

    #[test]
    fn clearing_everything_drops_the_diverted_dispatch() {
        let mut state = ImeCommitState::default();
        let now = Instant::now();
        state.begin_key_event(keystroke("j"), true, now);
        assert_eq!(state.commit("j", true, now), ImeCommitVerdict::Divert);
        state.clear();
        assert_eq!(state.end_key_event(), None);
    }

    #[test]
    fn a_key_that_does_not_match_the_commit_is_typed_and_consumes_the_entry() {
        let mut state = ImeCommitState::default();
        let now = Instant::now();
        state.begin_key_event(keystroke("j"), true, now);
        assert_eq!(state.commit("k", true, now), ImeCommitVerdict::Passthrough);
        assert!(!state.has_pending_entry(now));
    }

    #[test]
    fn chinese_composition_reports_through_marked_text_and_is_never_diverted() {
        // Replays the event sequence captured from a real Sogou session in Chinese (pinyin) mode:
        // the first letter starts a composition (`setMarkedText`) instead of committing text, so
        // the pending entry is dropped before any commit can be attributed to it. Even when the
        // composition is finally committed, the text is multi-character and typed as usual.
        let mut state = ImeCommitState::default();
        let now = Instant::now();
        state.begin_key_event(keystroke("n"), true, now);
        state.clear_pending(); // setMarkedText("n")
        assert_eq!(state.commit("n", true, now), ImeCommitVerdict::Passthrough);

        state.begin_key_event(keystroke("g"), true, now);
        state.clear_pending(); // setMarkedText("g")
        state.begin_key_event(keystroke("q"), false, now);
        state.clear_pending(); // setMarkedText("g'q")
        assert_eq!(state.commit("gq", true, now), ImeCommitVerdict::Passthrough);
        assert_eq!(state.end_key_event(), None);
    }

    #[test]
    fn a_key_that_starts_a_composition_is_typed_when_the_composition_commits_it() {
        // `j` begins a chord, but the input source composes it instead of committing it alone;
        // the composition's own commit must still reach the document.
        let mut state = ImeCommitState::default();
        let now = Instant::now();
        assert!(state.begin_key_event(keystroke("j"), true, now));
        state.clear_pending(); // setMarkedText("ji")
        assert_eq!(state.commit("ji", true, now), ImeCommitVerdict::Passthrough);
        assert_eq!(state.end_key_event(), None);
    }

    #[test]
    fn a_composition_that_commits_the_typed_letter_types_it() {
        // Some input sources compose and then commit exactly the letter that was pressed (a
        // single-candidate composition). That is text entry, not a chord, and must not be dropped.
        let mut state = ImeCommitState::default();
        let now = Instant::now();
        state.begin_key_event(keystroke("j"), true, now);
        state.clear_pending(); // setMarkedText("j")
        assert_eq!(state.commit("j", true, now), ImeCommitVerdict::Passthrough);
    }
}
