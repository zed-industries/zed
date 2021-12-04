use std::{
    cmp,
    ops::{Deref, Range},
};
use sum_tree::{Bias, Cursor, SumTree};
use text::TextSummary;
use theme::SyntaxTheme;
use util::post_inc;

use crate::{buffer, Buffer, Chunk};
use gpui::{Entity, ModelContext, ModelHandle};

const NEWLINES: &'static [u8] = &[b'\n'; u8::MAX as usize];

pub trait ToOffset {
    fn to_offset<'a>(&self, content: &Snapshot) -> usize;
}

pub type FragmentId = usize;

#[derive(Default)]
pub struct FragmentList {
    snapshot: Snapshot,
    next_fragment_id: FragmentId,
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
    buffer: buffer::Snapshot,
    buffer_id: usize,
    buffer_range: Range<usize>,
    text_summary: TextSummary,
    header_height: u8,
}

#[derive(Clone, Debug, Default)]
struct EntrySummary {
    min_buffer_id: usize,
    max_buffer_id: usize,
    text: TextSummary,
}

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

    pub fn push<'a, O: text::ToOffset>(
        &mut self,
        props: FragmentProperties<'a, O>,
        cx: &mut ModelContext<Self>,
    ) -> FragmentId {
        let id = post_inc(&mut self.next_fragment_id);

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

        self.snapshot.entries.push(
            Entry {
                buffer: props.buffer.read(cx).snapshot(),
                buffer_id: props.buffer.id(),
                buffer_range,
                text_summary,
                header_height: props.header_height,
            },
            &(),
        );

        id
    }
}

impl Deref for FragmentList {
    type Target = Snapshot;

    fn deref(&self) -> &Self::Target {
        &self.snapshot
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
            min_buffer_id: self.buffer_id,
            max_buffer_id: self.buffer_id,
            text: self.text_summary.clone(),
        }
    }
}

impl sum_tree::Summary for EntrySummary {
    type Context = ();

    fn add_summary(&mut self, summary: &Self, _: &()) {
        self.min_buffer_id = cmp::min(self.min_buffer_id, summary.min_buffer_id);
        self.max_buffer_id = cmp::max(self.max_buffer_id, summary.max_buffer_id);
        self.text.add_summary(&summary.text, &());
    }
}

impl<'a> sum_tree::Dimension<'a, EntrySummary> for usize {
    fn add_summary(&mut self, summary: &'a EntrySummary, _: &()) {
        *self += summary.text.bytes
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

#[cfg(test)]
mod tests {
    use super::{FragmentList, FragmentProperties};
    use crate::Buffer;
    use gpui::MutableAppContext;
    use text::Point;
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
            list.read(cx).text(),
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
            list.read(cx).text(),
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
}
