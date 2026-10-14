use std::{collections::BTreeMap, ops::Range};

use anyhow::{Result, anyhow};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct ByteSpan {
    identity: u64,
    length: usize,
}

struct SavedSpan {
    offset: usize,
    length: usize,
}

pub(super) struct ChangeTracking {
    spans: Vec<ByteSpan>,
    saved: BTreeMap<u64, SavedSpan>,
    next_identity: u64,
    pub inserted: usize,
    pub deletions: BTreeMap<usize, usize>,
}

impl ChangeTracking {
    pub fn new(length: usize) -> Self {
        let mut this = Self {
            spans: if length == 0 {
                Vec::new()
            } else {
                vec![ByteSpan {
                    identity: 0,
                    length,
                }]
            },
            saved: BTreeMap::new(),
            next_identity: length as u64,
            inserted: 0,
            deletions: BTreeMap::new(),
        };
        this.mark_saved();
        this
    }

    pub fn slice(&self, range: Range<usize>) -> Vec<ByteSpan> {
        let mut spans = Vec::new();
        let mut offset = 0;
        for span in &self.spans {
            let start = range.start.max(offset);
            let end = range.end.min(offset + span.length);
            if start < end {
                spans.push(ByteSpan {
                    identity: span.identity + (start - offset) as u64,
                    length: end - start,
                });
            }
            offset += span.length;
            if offset >= range.end {
                break;
            }
        }
        spans
    }

    pub fn replacement(&mut self, range: Range<usize>, length: usize) -> Result<Vec<ByteSpan>> {
        let retained = range.len().min(length);
        let mut spans = self.slice(range.start..range.start + retained);
        let added = length - retained;
        if added > 0 {
            let identity = self.next_identity;
            self.next_identity = identity
                .checked_add(added as u64)
                .ok_or_else(|| anyhow!("Too many inserted bytes"))?;
            spans.push(ByteSpan {
                identity,
                length: added,
            });
        }
        Ok(spans)
    }

    pub fn splice(&mut self, range: Range<usize>, replacement: &[ByteSpan]) {
        let length = self.spans.iter().map(|span| span.length).sum();
        let mut spans = self.slice(0..range.start);
        spans.extend_from_slice(replacement);
        spans.extend(self.slice(range.end..length));
        self.spans.clear();
        for span in spans {
            if let Some(last) = self.spans.last_mut()
                && last.identity + last.length as u64 == span.identity
            {
                last.length += span.length;
            } else {
                self.spans.push(span);
            }
        }
        self.update_changes();
    }

    pub fn mark_saved(&mut self) {
        self.saved.clear();
        let mut offset = 0;
        for span in &self.spans {
            self.saved.insert(
                span.identity,
                SavedSpan {
                    offset,
                    length: span.length,
                },
            );
            offset += span.length;
        }
        self.update_changes();
    }

    pub fn saved_offset(&self, mut offset: usize) -> Option<usize> {
        for span in &self.spans {
            if offset < span.length {
                let identity = span.identity + offset as u64;
                return self.saved_offset_for(identity);
            }
            offset -= span.length;
        }
        None
    }

    pub fn saved_offset_for(&self, identity: u64) -> Option<usize> {
        let (&start, saved) = self.saved.range(..=identity).next_back()?;
        let relative = (identity - start) as usize;
        (relative < saved.length).then_some(saved.offset + relative)
    }

    pub fn identity_at(&self, mut offset: usize) -> Option<u64> {
        for span in &self.spans {
            if offset < span.length {
                return Some(span.identity + offset as u64);
            }
            offset -= span.length;
        }
        None
    }

    pub fn offset_for(&self, identity: u64) -> Option<usize> {
        let mut offset = 0;
        for span in &self.spans {
            if identity >= span.identity && identity - span.identity < span.length as u64 {
                return Some(offset + (identity - span.identity) as usize);
            }
            offset += span.length;
        }
        None
    }

    fn update_changes(&mut self) {
        self.deletions.clear();
        let mut current_offset = 0;
        let mut saved_end = 0;
        let mut retained = 0;
        for span in &self.spans {
            let end = span.identity + span.length as u64;
            let start = self
                .saved
                .range(..=span.identity)
                .next_back()
                .map_or(span.identity, |(&start, _)| start);
            for (&identity, saved) in self.saved.range(start..end) {
                let intersection_start = span.identity.max(identity);
                let intersection_end = end.min(identity + saved.length as u64);
                if intersection_start >= intersection_end {
                    continue;
                }
                let offset = saved.offset + (intersection_start - identity) as usize;
                let length = (intersection_end - intersection_start) as usize;
                if offset > saved_end {
                    self.deletions.insert(
                        current_offset + (intersection_start - span.identity) as usize,
                        offset - saved_end,
                    );
                }
                saved_end = offset + length;
                retained += length;
            }
            current_offset += span.length;
        }
        let saved_length: usize = self.saved.values().map(|span| span.length).sum();
        if saved_end < saved_length {
            self.deletions
                .insert(current_offset, saved_length - saved_end);
        }
        self.inserted = current_offset - retained;
    }
}
