use std::ops::Range;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use collections::{HashMap, HashSet};
use editor::{MultiBufferOffset, MultiBufferSnapshot};
use ui::SharedString;

use crate::motion::{Motion, is_character_match};

const BASE_LABEL_CHARS: &[char] = &[
    'f', 'j', 'd', 'k', 's', 'l', 'a', 'g', 'h', 'r', 'u', 'e', 'i', 'o', 'w', 'm', 'n', 'c', 'v',
    'x', 'z', 'p', 'q', 'y', 't', 'b',
];

pub(crate) const BEAM_JUMP_PENDING_COMMIT_TIMEOUT: Duration = Duration::from_millis(700);

static NEXT_BEAM_JUMP_SESSION_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum BeamJumpDirection {
    Forward,
    Backward,
}

#[derive(Clone, Debug)]
pub(crate) struct BeamJumpMatch {
    pub(crate) start: MultiBufferOffset,
    pub(crate) end: MultiBufferOffset,
}

#[derive(Clone, Debug)]
pub(crate) struct BeamJumpLabels {
    pub(crate) label_len: usize,
    pub(crate) units: Vec<char>,
    pub(crate) label_buffer: String,
    pub(crate) label_key_set: HashSet<char>,
    pub(crate) label_by_start: HashMap<MultiBufferOffset, SharedString>,
    pub(crate) start_by_label: HashMap<String, MultiBufferOffset>,
    pub(crate) label_pool: Option<Vec<String>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct BeamJumpPendingCommit {
    pub(crate) id: u64,
}

#[derive(Clone, Debug)]
pub(crate) struct BeamJumpState {
    pub(crate) smartcase: bool,
    pub(crate) cursor_offset: MultiBufferOffset,
    pub(crate) view_start: MultiBufferOffset,
    pub(crate) view_end: MultiBufferOffset,
    pub(crate) previous_last_find: Option<Motion>,
    pub(crate) session_id: u64,

    pub(crate) pattern: String,
    pub(crate) pattern_len: usize,

    pub(crate) matches: Vec<BeamJumpMatch>,
    pub(crate) labels: Option<BeamJumpLabels>,

    pub(crate) pending_commit: Option<BeamJumpPendingCommit>,
    pending_commit_next_id: u64,
}

#[derive(Clone, Debug)]
pub(crate) struct BeamJumpJump {
    pub(crate) direction: BeamJumpDirection,
    pub(crate) pattern: String,
    pub(crate) smartcase: bool,
    pub(crate) count: usize,
    pub(crate) search_range: Option<Range<MultiBufferOffset>>,
}

#[derive(Clone, Debug)]
pub(crate) enum BeamJumpAction {
    Continue,
    Cancel,
    Jump(BeamJumpJump),
}

impl BeamJumpState {
    pub(crate) fn new(
        smartcase: bool,
        cursor_offset: MultiBufferOffset,
        view_start: MultiBufferOffset,
        view_end: MultiBufferOffset,
        previous_last_find: Option<Motion>,
    ) -> Self {
        Self {
            smartcase,
            cursor_offset,
            view_start,
            view_end,
            previous_last_find,
            session_id: NEXT_BEAM_JUMP_SESSION_ID.fetch_add(1, Ordering::Relaxed),
            pattern: String::new(),
            pattern_len: 0,
            matches: Vec::new(),
            labels: None,
            pending_commit: None,
            pending_commit_next_id: 0,
        }
    }

    pub(crate) fn on_typed_char(
        &mut self,
        ch: char,
        buffer: &MultiBufferSnapshot,
    ) -> BeamJumpAction {
        let was_pattern_extension = self.pattern_len >= 2;

        if self.pattern_len >= 2 && self.matches.len() > 1 {
            if let Some(labels) = &mut self.labels {
                if labels.label_key_set.contains(&ch) {
                    labels.label_buffer.push(ch);
                    if labels.label_buffer.len() >= labels.label_len {
                        let label = std::mem::take(&mut labels.label_buffer);
                        if let Some(&start) = labels.start_by_label.get(&label)
                            && let Some((direction, count)) =
                                self.direction_and_count_for_start(start)
                        {
                            return BeamJumpAction::Jump(BeamJumpJump {
                                direction,
                                pattern: self.pattern.clone(),
                                smartcase: self.smartcase,
                                count,
                                search_range: Some(self.view_start..self.view_end),
                            });
                        }

                        return BeamJumpAction::Cancel;
                    }

                    return BeamJumpAction::Continue;
                }

                labels.label_buffer.clear();
            }
        }

        self.push_pattern_char(ch, buffer);

        debug_assert!(
            self.matches
                .iter()
                .all(|m| m.start > self.cursor_offset || m.end <= self.cursor_offset)
        );

        if self.pattern_len < 2 {
            return BeamJumpAction::Continue;
        }

        if was_pattern_extension && self.matches.is_empty() {
            if let Some(jump) = self.auto_global_jump(buffer) {
                return BeamJumpAction::Jump(jump);
            }

            self.clear_pattern();
            return BeamJumpAction::Cancel;
        }

        match self.matches.len() {
            0 => {
                self.pending_commit = None;
                self.labels = None;
                BeamJumpAction::Continue
            }
            1 => {
                self.labels = None;

                let id = self.pending_commit_next_id;
                self.pending_commit_next_id = self.pending_commit_next_id.wrapping_add(1);
                self.pending_commit = Some(BeamJumpPendingCommit { id });

                BeamJumpAction::Continue
            }
            _ => {
                self.pending_commit = None;

                if self.labels.is_none() {
                    self.labels = Some(self.assign_labels(buffer));
                }

                BeamJumpAction::Continue
            }
        }
    }

    fn push_pattern_char(&mut self, ch: char, buffer: &MultiBufferSnapshot) {
        self.pattern.push(ch);
        self.pattern_len += 1;

        if self.pattern_len == 1 {
            self.matches = self.scan_first_char(ch, buffer);
            return;
        }

        self.extend_matches(ch, buffer);

        if self.labels.is_some() {
            self.retain_labels_for_matches();
        }
    }

    fn clear_pattern(&mut self) {
        self.pattern.clear();
        self.pattern_len = 0;
        self.matches.clear();
        self.labels = None;
        self.pending_commit = None;
    }

    fn auto_global_jump(&self, buffer: &MultiBufferSnapshot) -> Option<BeamJumpJump> {
        let pattern: Vec<char> = self.pattern.chars().collect();
        if pattern.is_empty() {
            return None;
        }

        if self.has_global_match_after_cursor(buffer, &pattern) {
            return Some(BeamJumpJump {
                direction: BeamJumpDirection::Forward,
                pattern: self.pattern.clone(),
                smartcase: self.smartcase,
                count: 1,
                search_range: None,
            });
        }

        let count_before_cursor = self.count_global_matches_before_cursor(buffer, &pattern);
        if count_before_cursor == 0 {
            return None;
        }

        Some(BeamJumpJump {
            direction: BeamJumpDirection::Backward,
            pattern: self.pattern.clone(),
            smartcase: self.smartcase,
            count: count_before_cursor,
            search_range: None,
        })
    }

    fn has_global_match_after_cursor(
        &self,
        buffer: &MultiBufferSnapshot,
        pattern: &[char],
    ) -> bool {
        let buffer_end = buffer.len();
        let cursor_offset = std::cmp::min(self.cursor_offset, buffer_end);
        if cursor_offset >= buffer_end {
            return false;
        }

        let Some(cursor_char) = buffer.chars_at(cursor_offset).next() else {
            return false;
        };

        let mut offset = cursor_offset;
        offset += cursor_char.len_utf8();
        while offset < buffer_end {
            if match_pattern_at(buffer, offset, pattern, self.smartcase).is_some() {
                return true;
            }

            let Some(ch) = buffer.chars_at(offset).next() else {
                break;
            };
            offset += ch.len_utf8();
        }

        false
    }

    fn count_global_matches_before_cursor(
        &self,
        buffer: &MultiBufferSnapshot,
        pattern: &[char],
    ) -> usize {
        let buffer_end = buffer.len();
        let cursor_offset = std::cmp::min(self.cursor_offset, buffer_end);

        let mut count = 0;
        let mut offset = MultiBufferOffset(0);
        while offset < cursor_offset {
            if let Some(match_end) = match_pattern_at(buffer, offset, pattern, self.smartcase) {
                if match_end <= cursor_offset {
                    count += 1;
                }
            }

            let Some(ch) = buffer.chars_at(offset).next() else {
                break;
            };
            offset += ch.len_utf8();
        }

        count
    }

    fn scan_first_char(&self, target: char, buffer: &MultiBufferSnapshot) -> Vec<BeamJumpMatch> {
        let mut matches = Vec::new();
        let mut offset = self.view_start;
        while offset < self.view_end {
            let Some(ch) = buffer.chars_at(offset).next() else {
                break;
            };

            let mut next = offset;
            next += ch.len_utf8();

            if next > self.view_end {
                break;
            }

            if offset != self.cursor_offset
                && !(offset < self.cursor_offset && self.cursor_offset < next)
                && is_character_match(target, ch, self.smartcase)
            {
                matches.push(BeamJumpMatch {
                    start: offset,
                    end: next,
                });
            }

            offset = next;
        }

        matches
    }

    fn extend_matches(&mut self, target: char, buffer: &MultiBufferSnapshot) {
        self.matches.retain_mut(|m| {
            if m.end >= self.view_end {
                return false;
            }

            let Some(next_ch) = buffer.chars_at(m.end).next() else {
                return false;
            };

            if !is_character_match(target, next_ch, self.smartcase) {
                return false;
            }

            let mut new_end = m.end;
            new_end += next_ch.len_utf8();

            if new_end > self.view_end {
                return false;
            }

            if m.start < self.cursor_offset && self.cursor_offset < new_end {
                return false;
            }

            m.end = new_end;
            true
        });
    }

    fn retain_labels_for_matches(&mut self) {
        let matches = &self.matches;

        let Some(labels) = &mut self.labels else {
            return;
        };

        let starts: HashSet<MultiBufferOffset> = matches.iter().map(|m| m.start).collect();
        labels
            .label_by_start
            .retain(|start, _| starts.contains(start));
        labels
            .start_by_label
            .retain(|_, start| starts.contains(start));

        Self::backfill_labels_for_matches(matches, self.cursor_offset, labels);

        labels.label_key_set = labels
            .label_by_start
            .values()
            .flat_map(|label| label.chars())
            .collect();
    }

    fn backfill_labels_for_matches(
        matches: &[BeamJumpMatch],
        cursor_offset: MultiBufferOffset,
        labels: &mut BeamJumpLabels,
    ) {
        let unit_count = labels.units.len();
        let capacity = match labels.label_len {
            1 => unit_count,
            2 => unit_count.saturating_mul(unit_count),
            _ => 0,
        };
        let desired_label_count = std::cmp::min(matches.len(), capacity);
        if desired_label_count <= labels.label_by_start.len() {
            return;
        }

        let label_pool = labels
            .label_pool
            .take()
            .unwrap_or_else(|| generate_labels(&labels.units, labels.label_len, capacity));

        let mut label_index = 0;
        for m in ProximityMatchIter::new(matches, cursor_offset) {
            if labels.label_by_start.contains_key(&m.start) {
                continue;
            }

            if labels.label_by_start.len() >= desired_label_count {
                break;
            }

            let mut next_label = None;
            while label_index < label_pool.len() {
                let candidate = &label_pool[label_index];
                label_index += 1;
                if labels.start_by_label.contains_key(candidate.as_str()) {
                    continue;
                }
                next_label = Some(candidate.clone());
                break;
            }

            let Some(label) = next_label else {
                break;
            };

            labels.start_by_label.insert(label.clone(), m.start);
            labels
                .label_by_start
                .insert(m.start, SharedString::from(label));
        }

        labels.label_pool = Some(label_pool);
    }

    fn collect_extension_chars(&self, buffer: &MultiBufferSnapshot) -> HashSet<char> {
        let mut extension_chars = HashSet::default();

        for m in &self.matches {
            if m.end >= self.view_end {
                continue;
            }

            let classifier = buffer.char_classifier_at(m.end);
            let mut offset = m.end;
            for ch in buffer.chars_at(offset) {
                let mut next = offset;
                next += ch.len_utf8();
                if next > self.view_end {
                    break;
                }

                if !classifier.is_word(ch) {
                    break;
                }

                let normalized = if self.smartcase {
                    ch.to_ascii_lowercase()
                } else {
                    ch
                };
                if BASE_LABEL_CHARS.contains(&normalized) {
                    extension_chars.insert(normalized);
                    if extension_chars.len() == BASE_LABEL_CHARS.len() {
                        return extension_chars;
                    }
                }

                offset = next;
            }
        }

        extension_chars
    }

    fn assign_labels(&self, buffer: &MultiBufferSnapshot) -> BeamJumpLabels {
        let extension_chars = self.collect_extension_chars(buffer);

        // Exclude chars that appear later in any matched word; ';' and ',' are reserved for navigation.
        let safe_units: Vec<char> = BASE_LABEL_CHARS
            .iter()
            .copied()
            .filter(|ch| *ch != ';' && *ch != ',' && !extension_chars.contains(ch))
            .collect();

        // If no safe units remain, capacity drops to 0 and labels stay hidden.
        let label_len = select_label_len(self.matches.len(), safe_units.len());
        let capacity = match label_len {
            1 => safe_units.len(),
            2 => safe_units.len().saturating_mul(safe_units.len()),
            _ => 0,
        };
        let label_count = std::cmp::min(self.matches.len(), capacity);
        let labels = generate_labels(&safe_units, label_len, label_count);

        let mut label_by_start = HashMap::default();
        let mut start_by_label = HashMap::default();
        let mut label_key_set = HashSet::default();

        for (m, label) in ProximityMatchIter::new(&self.matches, self.cursor_offset)
            .take(label_count)
            .zip(labels)
        {
            label_key_set.extend(label.chars());

            start_by_label.insert(label.clone(), m.start);
            label_by_start.insert(m.start, SharedString::from(label));
        }

        BeamJumpLabels {
            label_len,
            units: safe_units,
            label_buffer: String::new(),
            label_key_set,
            label_by_start,
            start_by_label,
            label_pool: None,
        }
    }

    fn direction_and_count_for_start(
        &self,
        start: MultiBufferOffset,
    ) -> Option<(BeamJumpDirection, usize)> {
        let pos = self
            .matches
            .binary_search_by_key(&start, |m| m.start)
            .ok()?;

        match start.cmp(&self.cursor_offset) {
            std::cmp::Ordering::Less => {
                let before_cursor_end_ix = self
                    .matches
                    .partition_point(|m| m.start < self.cursor_offset);
                Some((
                    BeamJumpDirection::Backward,
                    before_cursor_end_ix.saturating_sub(pos),
                ))
            }
            std::cmp::Ordering::Greater => {
                let after_cursor_start_ix = self
                    .matches
                    .partition_point(|m| m.start <= self.cursor_offset);
                Some((BeamJumpDirection::Forward, pos - after_cursor_start_ix + 1))
            }
            std::cmp::Ordering::Equal => None,
        }
    }
}

fn match_pattern_at(
    buffer: &MultiBufferSnapshot,
    start: MultiBufferOffset,
    pattern: &[char],
    smartcase: bool,
) -> Option<MultiBufferOffset> {
    let mut offset = start;
    for &target in pattern {
        let Some(ch) = buffer.chars_at(offset).next() else {
            return None;
        };

        if !is_character_match(target, ch, smartcase) {
            return None;
        }

        offset += ch.len_utf8();
    }

    Some(offset)
}

#[derive(Clone, Copy)]
struct ProximityMatchIter<'a> {
    matches: &'a [BeamJumpMatch],
    cursor_offset: MultiBufferOffset,
    left: isize,
    right: usize,
}

impl<'a> ProximityMatchIter<'a> {
    fn new(matches: &'a [BeamJumpMatch], cursor_offset: MultiBufferOffset) -> Self {
        let right = matches.partition_point(|m| m.start < cursor_offset);
        let left = right as isize - 1;
        Self {
            matches,
            cursor_offset,
            left,
            right,
        }
    }
}

impl<'a> Iterator for ProximityMatchIter<'a> {
    type Item = &'a BeamJumpMatch;

    fn next(&mut self) -> Option<Self::Item> {
        if self.left < 0 && self.right >= self.matches.len() {
            return None;
        }

        if self.left < 0 {
            let m = &self.matches[self.right];
            self.right += 1;
            return Some(m);
        }

        if self.right >= self.matches.len() {
            let m = &self.matches[self.left as usize];
            self.left -= 1;
            return Some(m);
        }

        let left_match = &self.matches[self.left as usize];
        let right_match = &self.matches[self.right];

        let left_dist = self.cursor_offset.0.abs_diff(left_match.start.0);
        let right_dist = self.cursor_offset.0.abs_diff(right_match.start.0);

        if left_dist <= right_dist {
            self.left -= 1;
            Some(left_match)
        } else {
            self.right += 1;
            Some(right_match)
        }
    }
}

fn select_label_len(match_count: usize, unit_count: usize) -> usize {
    if match_count <= unit_count { 1 } else { 2 }
}

fn generate_labels(units: &[char], label_len: usize, count: usize) -> Vec<String> {
    let mut labels = Vec::with_capacity(count);

    match label_len {
        1 => {
            for &ch in units.iter().take(count) {
                labels.push(ch.to_string());
            }
        }
        2 => {
            'outer: for &a in units {
                for &b in units {
                    labels.push(format!("{}{}", a, b));
                    if labels.len() == count {
                        break 'outer;
                    }
                }
            }
        }
        _ => {}
    }

    labels
}
