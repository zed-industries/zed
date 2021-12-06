use crate::{buffer, Buffer, Chunk};
use collections::HashMap;
use gpui::{AppContext, Entity, ModelContext, ModelHandle};
use parking_lot::Mutex;
use smallvec::{smallvec, SmallVec};
use std::{cmp, iter, mem, ops::Range};
use sum_tree::{Bias, Cursor, SumTree};
use text::TextSummary;
use theme::SyntaxTheme;

const NEWLINES: &'static [u8] = &[b'\n'; u8::MAX as usize];

pub trait ToOffset {
    fn to_offset<'a>(&self, content: &Snapshot) -> usize;
}

pub type FragmentId = Location;

#[derive(Default)]
pub struct FragmentList {
    snapshot: Mutex<Snapshot>,
    buffers: HashMap<usize, BufferState>,
}

struct BufferState {
    buffer: ModelHandle<Buffer>,
    subscription: text::Subscription,
    fragments: Vec<FragmentId>,
}

#[derive(Clone, Default)]
pub struct Snapshot {
    entries: SumTree<Entry>,
}

pub struct FragmentProperties<'a, T> {
    buffer: &'a ModelHandle<Buffer>,
    range: Range<T>,
    header_height: u8,
}

#[derive(Clone)]
struct Entry {
    id: FragmentId,
    buffer: buffer::Snapshot,
    buffer_range: Range<usize>,
    text_summary: TextSummary,
    header_height: u8,
}

#[derive(Clone, Debug, Default)]
struct EntrySummary {
    fragment_id: FragmentId,
    text: TextSummary,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Location(SmallVec<[usize; 4]>);

pub struct Chunks<'a> {
    range: Range<usize>,
    cursor: Cursor<'a, Entry, usize>,
    header_height: u8,
    entry_chunks: Option<buffer::Chunks<'a>>,
    theme: Option<&'a SyntaxTheme>,
}

impl FragmentList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn snapshot(&self, cx: &AppContext) -> Snapshot {
        self.sync(cx);
        self.snapshot.lock().clone()
    }

    pub fn push<O>(
        &mut self,
        props: FragmentProperties<O>,
        cx: &mut ModelContext<Self>,
    ) -> FragmentId
    where
        O: text::ToOffset,
    {
        self.sync(cx);

        let buffer = props.buffer.read(cx);
        let buffer_range = props.range.start.to_offset(buffer)..props.range.end.to_offset(buffer);
        let mut text_summary =
            buffer.text_summary_for_range::<TextSummary, _>(buffer_range.clone());
        if props.header_height > 0 {
            text_summary.first_line_chars = 0;
            text_summary.lines.row += props.header_height as u32;
            text_summary.lines_utf16.row += props.header_height as u32;
            text_summary.bytes += props.header_height as usize;
        }

        let mut snapshot = self.snapshot.lock();
        let prev_id = snapshot.entries.last().map(|e| &e.id);
        let id = FragmentId::between(prev_id.unwrap_or(&FragmentId::min()), &FragmentId::max());
        snapshot.entries.push(
            Entry {
                id: id.clone(),
                buffer: props.buffer.read(cx).snapshot(),
                buffer_range,
                text_summary,
                header_height: props.header_height,
            },
            &(),
        );

        self.buffers
            .entry(props.buffer.id())
            .or_insert_with(|| {
                let subscription = props.buffer.update(cx, |buffer, _| buffer.subscribe());
                BufferState {
                    buffer: props.buffer.clone(),
                    subscription,
                    fragments: Default::default(),
                }
            })
            .fragments
            .push(id.clone());

        id
    }

    fn sync(&self, cx: &AppContext) {
        let mut snapshot = self.snapshot.lock();
        let mut patches = Vec::new();
        let mut fragments_to_edit = Vec::new();
        for buffer_state in self.buffers.values() {
            let patch = buffer_state.subscription.consume();
            if !patch.is_empty() {
                let patch_ix = patches.len();
                patches.push(patch);
                fragments_to_edit.extend(
                    buffer_state
                        .fragments
                        .iter()
                        .map(|fragment_id| (&buffer_state.buffer, fragment_id, patch_ix)),
                )
            }
        }
        fragments_to_edit.sort_unstable_by_key(|(_, fragment_id, _)| *fragment_id);

        let old_fragments = mem::take(&mut snapshot.entries);
        let mut cursor = old_fragments.cursor::<FragmentId>();
        for (buffer, fragment_id, patch_ix) in fragments_to_edit {
            snapshot
                .entries
                .push_tree(cursor.slice(fragment_id, Bias::Left, &()), &());

            let fragment = cursor.item().unwrap();
            let mut new_range = fragment.buffer_range.clone();
            for edit in patches[patch_ix].edits() {
                let edit_start = edit.new.start;
                let edit_end = edit.new.start + edit.old_len();
                if edit_start > new_range.end {
                    break;
                } else if edit_end < new_range.start {
                    let delta = edit.new_len() as isize - edit.old_len() as isize;
                    new_range.start = (new_range.start as isize + delta) as usize;
                    new_range.end = (new_range.end as isize + delta) as usize;
                } else {
                    let mut new_range_len = new_range.len();
                    new_range_len -=
                        cmp::min(new_range.end, edit_end) - cmp::max(new_range.start, edit_start);
                    if edit_start < new_range.start {
                        new_range.start = edit.new.end;
                    } else {
                        new_range_len += edit.new_len();
                    }

                    new_range.end = new_range.start + new_range_len;
                }
            }

            let buffer = buffer.read(cx);
            let mut text_summary: TextSummary = buffer.text_summary_for_range(new_range.clone());
            if fragment.header_height > 0 {
                text_summary.first_line_chars = 0;
                text_summary.lines.row += fragment.header_height as u32;
                text_summary.lines_utf16.row += fragment.header_height as u32;
                text_summary.bytes += fragment.header_height as usize;
            }
            snapshot.entries.push(
                Entry {
                    id: fragment.id.clone(),
                    buffer: buffer.snapshot(),
                    buffer_range: new_range,
                    text_summary,
                    header_height: fragment.header_height,
                },
                &(),
            );

            cursor.next(&());
        }
        snapshot.entries.push_tree(cursor.suffix(&()), &());
    }
}

impl Entity for FragmentList {
    type Event = ();
}

impl Snapshot {
    pub fn text(&self) -> String {
        self.chunks(0..self.len(), None)
            .map(|chunk| chunk.text)
            .collect()
    }

    pub fn len(&self) -> usize {
        self.entries.summary().text.bytes
    }

    pub fn chunks<'a, T: ToOffset>(
        &'a self,
        range: Range<T>,
        theme: Option<&'a SyntaxTheme>,
    ) -> Chunks<'a> {
        let range = range.start.to_offset(self)..range.end.to_offset(self);
        let mut cursor = self.entries.cursor::<usize>();
        cursor.seek(&range.start, Bias::Right, &());

        let entry_chunks = cursor.item().map(|entry| {
            let buffer_start = entry.buffer_range.start + (range.start - cursor.start());
            let buffer_end = cmp::min(
                entry.buffer_range.end,
                entry.buffer_range.start + (range.end - cursor.start()),
            );
            entry.buffer.chunks(buffer_start..buffer_end, theme)
        });
        let header_height = cursor.item().map_or(0, |entry| entry.header_height);

        Chunks {
            range,
            cursor,
            header_height,
            entry_chunks,
            theme,
        }
    }
}

impl sum_tree::Item for Entry {
    type Summary = EntrySummary;

    fn summary(&self) -> Self::Summary {
        EntrySummary {
            fragment_id: self.id.clone(),
            text: self.text_summary.clone(),
        }
    }
}

impl sum_tree::Summary for EntrySummary {
    type Context = ();

    fn add_summary(&mut self, summary: &Self, _: &()) {
        debug_assert!(summary.fragment_id > self.fragment_id);
        self.fragment_id = summary.fragment_id.clone();
        self.text.add_summary(&summary.text, &());
    }
}

impl<'a> sum_tree::Dimension<'a, EntrySummary> for usize {
    fn add_summary(&mut self, summary: &'a EntrySummary, _: &()) {
        *self += summary.text.bytes;
    }
}

impl<'a> sum_tree::Dimension<'a, EntrySummary> for FragmentId {
    fn add_summary(&mut self, summary: &'a EntrySummary, _: &()) {
        debug_assert!(summary.fragment_id > *self);
        *self = summary.fragment_id.clone();
    }
}

impl<'a> Iterator for Chunks<'a> {
    type Item = Chunk<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.header_height > 0 {
            let chunk = Chunk {
                text: unsafe {
                    std::str::from_utf8_unchecked(&NEWLINES[..self.header_height as usize])
                },
                ..Default::default()
            };
            self.header_height = 0;
            return Some(chunk);
        }

        if let Some(entry_chunks) = self.entry_chunks.as_mut() {
            if let Some(chunk) = entry_chunks.next() {
                return Some(chunk);
            } else {
                self.entry_chunks.take();
            }
        }

        self.cursor.next(&());
        let entry = self.cursor.item()?;

        let buffer_end = cmp::min(
            entry.buffer_range.end,
            entry.buffer_range.start + (self.range.end - self.cursor.start()),
        );

        self.header_height = entry.header_height;
        self.entry_chunks = Some(
            entry
                .buffer
                .chunks(entry.buffer_range.start..buffer_end, self.theme),
        );

        Some(Chunk {
            text: "\n",
            ..Default::default()
        })
    }
}

impl ToOffset for usize {
    fn to_offset<'a>(&self, _: &Snapshot) -> usize {
        *self
    }
}

impl Default for Location {
    fn default() -> Self {
        Self::min()
    }
}

impl Location {
    pub fn min() -> Self {
        Self(smallvec![usize::MIN])
    }

    pub fn max() -> Self {
        Self(smallvec![usize::MAX])
    }

    pub fn between(lhs: &Self, rhs: &Self) -> Self {
        let lhs = lhs.0.iter().copied().chain(iter::repeat(usize::MIN));
        let rhs = rhs.0.iter().copied().chain(iter::repeat(usize::MAX));
        let mut location = SmallVec::new();
        for (lhs, rhs) in lhs.zip(rhs) {
            let mid = lhs + (rhs.saturating_sub(lhs)) / 2;
            location.push(mid);
            if mid > lhs {
                break;
            }
        }
        Self(location)
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;
    use crate::Buffer;
    use gpui::MutableAppContext;
    use rand::prelude::*;
    use text::{Point, RandomCharIter};
    use util::test::sample_text;

    #[gpui::test]
    fn test_fragment_buffer(cx: &mut MutableAppContext) {
        let buffer_1 = cx.add_model(|cx| Buffer::new(0, sample_text(6, 6, 'a'), cx));
        let buffer_2 = cx.add_model(|cx| Buffer::new(0, sample_text(6, 6, 'g'), cx));

        let list = cx.add_model(|cx| {
            let mut list = FragmentList::new();

            list.push(
                FragmentProperties {
                    buffer: &buffer_1,
                    range: Point::new(1, 2)..Point::new(2, 5),
                    header_height: 2,
                },
                cx,
            );
            list.push(
                FragmentProperties {
                    buffer: &buffer_1,
                    range: Point::new(3, 3)..Point::new(4, 4),
                    header_height: 1,
                },
                cx,
            );
            list.push(
                FragmentProperties {
                    buffer: &buffer_2,
                    range: Point::new(3, 1)..Point::new(3, 3),
                    header_height: 3,
                },
                cx,
            );
            list
        });

        assert_eq!(
            list.read(cx).snapshot(cx).text(),
            concat!(
                "\n",      // Preserve newlines
                "\n",      //
                "bbbb\n",  //
                "ccccc\n", //
                "\n",      //
                "ddd\n",   //
                "eeee\n",  //
                "\n",      //
                "\n",      //
                "\n",      //
                "jj"       //
            )
        );

        buffer_1.update(cx, |buffer, cx| {
            buffer.edit(
                [
                    Point::new(0, 0)..Point::new(0, 0),
                    Point::new(2, 1)..Point::new(2, 2),
                ],
                "\n",
                cx,
            );
        });

        assert_eq!(
            list.read(cx).snapshot(cx).text(),
            concat!(
                "\n",     // Preserve newlines
                "\n",     //
                "bbbb\n", //
                "c\n",    //
                "ccc\n",  //
                "\n",     //
                "ddd\n",  //
                "eeee\n", //
                "\n",     //
                "\n",     //
                "\n",     //
                "jj"      //
            )
        );
    }

    #[gpui::test(iterations = 100)]
    fn test_random(cx: &mut MutableAppContext, mut rng: StdRng) {
        let operations = env::var("OPERATIONS")
            .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
            .unwrap_or(10);

        let mut buffers: Vec<ModelHandle<Buffer>> = Vec::new();
        let list = cx.add_model(|_| FragmentList::new());
        let mut fragment_ids = Vec::new();
        let mut expected_fragments = Vec::new();

        for _ in 0..operations {
            match rng.gen_range(0..100) {
                0..=19 if !buffers.is_empty() => {
                    let buffer = buffers.choose(&mut rng).unwrap();
                    buffer.update(cx, |buf, cx| buf.randomly_edit(&mut rng, 5, cx));
                }
                _ => {
                    let buffer_handle = if buffers.is_empty() || rng.gen_bool(0.4) {
                        let base_text = RandomCharIter::new(&mut rng).take(10).collect::<String>();
                        buffers.push(cx.add_model(|cx| Buffer::new(0, base_text, cx)));
                        buffers.last().unwrap()
                    } else {
                        buffers.choose(&mut rng).unwrap()
                    };

                    let buffer = buffer_handle.read(cx);
                    let end_ix = buffer.clip_offset(rng.gen_range(0..=buffer.len()), Bias::Right);
                    let start_ix = buffer.clip_offset(rng.gen_range(0..=end_ix), Bias::Left);
                    let header_height = rng.gen_range(0..=5);
                    let anchor_range = buffer.anchor_before(start_ix)..buffer.anchor_after(end_ix);
                    log::info!(
                        "Pushing fragment for buffer {}: {:?}[{:?}] = {:?}",
                        buffer_handle.id(),
                        buffer.text(),
                        start_ix..end_ix,
                        &buffer.text()[start_ix..end_ix]
                    );

                    let fragment_id = list.update(cx, |list, cx| {
                        list.push(
                            FragmentProperties {
                                buffer: &buffer_handle,
                                range: start_ix..end_ix,
                                header_height,
                            },
                            cx,
                        )
                    });
                    fragment_ids.push(fragment_id);
                    expected_fragments.push((buffer_handle.clone(), anchor_range, header_height));
                }
            }

            let snapshot = list.read(cx).snapshot(cx);
            let mut expected_text = String::new();
            for (buffer, range, header_height) in &expected_fragments {
                let buffer = buffer.read(cx);
                if !expected_text.is_empty() {
                    expected_text.push('\n');
                }

                for _ in 0..*header_height {
                    expected_text.push('\n');
                }
                expected_text.extend(buffer.text_for_range(range.clone()));
            }
            assert_eq!(snapshot.text(), expected_text);
        }
    }

    #[gpui::test(iterations = 100)]
    fn test_location(mut rng: StdRng) {
        let mut lhs = Default::default();
        let mut rhs = Default::default();
        while lhs == rhs {
            lhs = Location(
                (0..rng.gen_range(1..=5))
                    .map(|_| rng.gen_range(0..=100))
                    .collect(),
            );
            rhs = Location(
                (0..rng.gen_range(1..=5))
                    .map(|_| rng.gen_range(0..=100))
                    .collect(),
            );
        }

        if lhs > rhs {
            mem::swap(&mut lhs, &mut rhs);
        }

        let middle = Location::between(&lhs, &rhs);
        assert!(middle > lhs);
        assert!(middle < rhs);
        for ix in 0..middle.0.len() - 1 {
            assert!(
                middle.0[ix] == *lhs.0.get(ix).unwrap_or(&0)
                    || middle.0[ix] == *rhs.0.get(ix).unwrap_or(&0)
            );
        }
    }
}
