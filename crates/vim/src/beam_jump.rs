use std::ops::Range;

use collections::{HashMap, HashSet};
use editor::{MultiBufferOffset, MultiBufferSnapshot};
use ui::SharedString;

use crate::motion::{Motion, is_character_match};

const BASE_LABEL_CHARS: &[char] = &[
    'f', 'j', 'd', 'k', 's', 'l', 'a', 'g', 'h', 'r', 'u', 'e', 'i', 'o', 'w', 'm', 'n', 'c', 'v',
    'x', 'z', 'p', 'q', 'y', 't', 'b',
];

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

#[derive(Clone, Debug)]
pub(crate) struct BeamJumpState {
    pub(crate) direction: BeamJumpDirection,
    pub(crate) smartcase: bool,
    pub(crate) search_start: MultiBufferOffset,
    pub(crate) search_end: MultiBufferOffset,
    pub(crate) previous_last_find: Option<Motion>,

    pub(crate) pattern: String,
    pub(crate) pattern_len: usize,

    pub(crate) matches: Vec<BeamJumpMatch>,
    pub(crate) labels: Option<BeamJumpLabels>,
}

#[derive(Clone, Debug)]
pub(crate) struct BeamJumpJump {
    pub(crate) direction: BeamJumpDirection,
    pub(crate) pattern: String,
    pub(crate) smartcase: bool,
    pub(crate) count: usize,
    pub(crate) search_range: Range<MultiBufferOffset>,
}

#[derive(Clone, Debug)]
pub(crate) enum BeamJumpAction {
    Continue,
    Cancel,
    PassThrough,
    Jump(BeamJumpJump),
}

impl BeamJumpState {
    pub(crate) fn new(
        direction: BeamJumpDirection,
        smartcase: bool,
        search_start: MultiBufferOffset,
        search_end: MultiBufferOffset,
        previous_last_find: Option<Motion>,
    ) -> Self {
        Self {
            direction,
            smartcase,
            search_start,
            search_end,
            previous_last_find,
            pattern: String::new(),
            pattern_len: 0,
            matches: Vec::new(),
            labels: None,
        }
    }

    pub(crate) fn on_typed_char(
        &mut self,
        ch: char,
        buffer: &MultiBufferSnapshot,
    ) -> BeamJumpAction {
        if self.pattern_len >= 2 && self.matches.len() > 1 {
            if let Some(labels) = &mut self.labels {
                if labels.label_key_set.contains(&ch) {
                    labels.label_buffer.push(ch);
                    if labels.label_buffer.len() >= labels.label_len {
                        let label = std::mem::take(&mut labels.label_buffer);
                        if let Some(&start) = labels.start_by_label.get(&label)
                            && let Some(count) = self.count_for_start(start)
                        {
                            return BeamJumpAction::Jump(BeamJumpJump {
                                direction: self.direction,
                                pattern: self.pattern.clone(),
                                smartcase: self.smartcase,
                                count,
                                search_range: self.search_start..self.search_end,
                            });
                        }

                        return BeamJumpAction::Cancel;
                    }

                    return BeamJumpAction::Continue;
                }

                labels.label_buffer.clear();
                if !self.can_extend_pattern_with(ch, buffer) {
                    return BeamJumpAction::PassThrough;
                }
            }
        }

        self.push_pattern_char(ch, buffer);

        if self.pattern_len < 2 {
            return BeamJumpAction::Continue;
        }

        match self.matches.len() {
            0 | 1 => BeamJumpAction::Jump(BeamJumpJump {
                direction: self.direction,
                pattern: self.pattern.clone(),
                smartcase: self.smartcase,
                count: 1,
                search_range: self.search_start..self.search_end,
            }),
            _ => {
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

    fn can_extend_pattern_with(&self, ch: char, buffer: &MultiBufferSnapshot) -> bool {
        self.matches.iter().any(|m| {
            if m.end >= self.search_end {
                return false;
            }

            let Some(next_ch) = buffer.chars_at(m.end).next() else {
                return false;
            };

            is_character_match(ch, next_ch, self.smartcase)
        })
    }

    fn scan_first_char(&self, target: char, buffer: &MultiBufferSnapshot) -> Vec<BeamJumpMatch> {
        let mut matches = Vec::new();
        let mut offset = self.search_start;
        while offset < self.search_end {
            let Some(ch) = buffer.chars_at(offset).next() else {
                break;
            };

            let mut next = offset;
            next += ch.len_utf8();

            if next > self.search_end {
                break;
            }

            if is_character_match(target, ch, self.smartcase) {
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
            if m.end >= self.search_end {
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

            if new_end > self.search_end {
                return false;
            }

            m.end = new_end;
            true
        });
    }

    fn retain_labels_for_matches(&mut self) {
        let matches = &self.matches;
        let direction = self.direction;

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

        Self::backfill_labels_for_matches(matches, direction, labels);

        labels.label_key_set = labels
            .label_by_start
            .values()
            .flat_map(|label| label.chars())
            .collect();
    }

    fn backfill_labels_for_matches(
        matches: &[BeamJumpMatch],
        direction: BeamJumpDirection,
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

        let matches_in_order: Box<dyn Iterator<Item = &BeamJumpMatch>> = match direction {
            BeamJumpDirection::Forward => Box::new(matches.iter()),
            BeamJumpDirection::Backward => Box::new(matches.iter().rev()),
        };

        let mut label_index = 0;
        for m in matches_in_order {
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
            if m.end >= self.search_end {
                continue;
            }

            let classifier = buffer.char_classifier_at(m.end);
            let mut offset = m.end;
            for ch in buffer.chars_at(offset) {
                let mut next = offset;
                next += ch.len_utf8();
                if next > self.search_end {
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

        let matches_in_order: Box<dyn Iterator<Item = &BeamJumpMatch>> = match self.direction {
            BeamJumpDirection::Forward => Box::new(self.matches.iter()),
            BeamJumpDirection::Backward => Box::new(self.matches.iter().rev()),
        };

        for (m, label) in matches_in_order.take(label_count).zip(labels) {
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

    fn count_for_start(&self, start: MultiBufferOffset) -> Option<usize> {
        let pos = self.matches.iter().position(|m| m.start == start)?;
        match self.direction {
            BeamJumpDirection::Forward => Some(pos + 1),
            BeamJumpDirection::Backward => Some(self.matches.len().saturating_sub(pos)),
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
