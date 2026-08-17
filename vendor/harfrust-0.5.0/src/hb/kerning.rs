use super::aat::layout::DELETED_GLYPH;
use alloc::boxed::Box;
use read_fonts::{
    tables::{
        aat,
        kern::{Subtable, Subtable0, Subtable2, Subtable3, SubtableKind},
    },
    types::{GlyphId, GlyphId16},
};

use super::aat::layout_common::{AatApplyContext, ClassCache, START_OF_TEXT};
use super::aat::layout_kerx_table::SimpleKerning;
use super::buffer::*;
use super::ot_layout::TableIndex;
use super::ot_layout_common::lookup_flags;
use super::ot_layout_gpos_table::attach_type;
use super::ot_layout_gsubgpos::{skipping_iterator_t, OT::hb_ot_apply_context_t};
use super::ot_shape_plan::hb_ot_shape_plan_t;
use super::{hb_font_t, hb_mask_t};
use crate::U32Set;

pub(crate) fn get_class(machine: &aat::StateTable, glyph_id: GlyphId, cache: &ClassCache) -> u8 {
    if let Some(klass) = cache.get(glyph_id.to_u32()) {
        return klass as u8;
    }
    let klass = machine
        .class(GlyphId16::new(glyph_id.to_u32() as u16))
        .unwrap_or(aat::class::OUT_OF_BOUNDS);
    cache.set(glyph_id.to_u32(), klass as u32);
    klass
}

pub fn hb_ot_layout_kern(
    plan: &hb_ot_shape_plan_t,
    face: &hb_font_t,
    buffer: &mut hb_buffer_t,
) -> Option<()> {
    let mut c = AatApplyContext::new(plan, face, buffer);

    let (kern, subtable_caches) = c.face.aat_tables.kern.as_ref()?;

    let mut subtable_idx = 0;

    let mut seen_cross_stream = false;
    for subtable in kern.subtables() {
        let Ok(subtable) = subtable else { continue };

        let subtable_cache = subtable_caches.get(subtable_idx);
        let Some(subtable_cache) = subtable_cache.as_ref() else {
            break;
        };
        subtable_idx += 1;

        if subtable.is_variable() {
            continue;
        }

        if c.buffer.direction.is_horizontal() != subtable.is_horizontal() {
            continue;
        }

        c.first_set = Some(&subtable_cache.first_set);
        c.second_set = Some(&subtable_cache.second_set);
        c.machine_class_cache = Some(&subtable_cache.class_cache);
        c.start_end_safe_to_break = subtable_cache.start_end_safe_to_break;

        if !c.buffer_intersects_machine() {
            continue;
        }

        let reverse = c.buffer.direction.is_backward();
        let is_cross_stream = subtable.is_cross_stream();

        if !seen_cross_stream && is_cross_stream {
            seen_cross_stream = true;

            // Attach all glyphs into a chain.
            for pos in &mut c.buffer.pos {
                pos.set_attach_type(attach_type::CURSIVE);
                pos.set_attach_chain(if c.buffer.direction.is_forward() {
                    -1
                } else {
                    1
                });
                // We intentionally don't set BufferScratchFlags::HAS_GPOS_ATTACHMENT,
                // since there needs to be a non-zero attachment for post-positioning to
                // be needed.
            }
        }

        let Ok(kind) = subtable.kind() else {
            continue;
        };

        if reverse != c.buffer_is_reversed {
            c.reverse_buffer();
        }

        match kind {
            SubtableKind::Format0(format0) if plan.requested_kerning => {
                apply_simple_kerning(&mut c, &format0, is_cross_stream);
            }
            SubtableKind::Format1(format1) => {
                apply_state_machine_kerning(&mut c, &format1, is_cross_stream);
            }
            SubtableKind::Format2(format2) if plan.requested_kerning => {
                apply_simple_kerning(&mut c, &format2, is_cross_stream);
            }
            SubtableKind::Format3(format3) if plan.requested_kerning => {
                apply_simple_kerning(&mut c, &format3, is_cross_stream);
            }
            _ => {}
        }
    }
    if c.buffer_is_reversed {
        c.reverse_buffer();
    }
    Some(())
}

fn machine_kern<F>(
    face: &hb_font_t,
    buffer: &mut hb_buffer_t,
    kern_mask: hb_mask_t,
    cross_stream: bool,
    get_kerning: F,
) where
    F: Fn(u32, u32) -> i32,
{
    buffer.unsafe_to_concat(None, None);
    let mut ctx = hb_ot_apply_context_t::new(TableIndex::GPOS, face, buffer);
    ctx.set_lookup_mask(kern_mask);
    ctx.lookup_props = u32::from(lookup_flags::IGNORE_MARKS);
    ctx.update_matchers();

    let horizontal = ctx.buffer.direction.is_horizontal();

    let mut i = 0;
    let mut iter = skipping_iterator_t::new(&mut ctx, false);
    while i < iter.buffer.len {
        if (iter.buffer.info[i].mask & kern_mask) == 0 {
            i += 1;
            continue;
        }

        iter.reset_fast(i);

        let mut unsafe_to = 0;
        if !iter.next(Some(&mut unsafe_to)) {
            i += 1;
            continue;
        }

        let j = iter.index();

        let info = &iter.buffer.info;
        let kern = get_kerning(info[i].glyph_id, info[j].glyph_id);

        let pos = &mut iter.buffer.pos;
        if kern != 0 {
            if horizontal {
                if cross_stream {
                    pos[j].y_offset = kern;
                    iter.buffer.scratch_flags |= HB_BUFFER_SCRATCH_FLAG_HAS_GPOS_ATTACHMENT;
                } else {
                    let kern1 = kern >> 1;
                    let kern2 = kern - kern1;
                    pos[i].x_advance += kern1;
                    pos[j].x_advance += kern2;
                    pos[j].x_offset += kern2;
                }
            } else {
                if cross_stream {
                    pos[j].x_offset = kern;
                    iter.buffer.scratch_flags |= HB_BUFFER_SCRATCH_FLAG_HAS_GPOS_ATTACHMENT;
                } else {
                    let kern1 = kern >> 1;
                    let kern2 = kern - kern1;
                    pos[i].y_advance += kern1;
                    pos[j].y_advance += kern2;
                    pos[j].y_offset += kern2;
                }
            }

            iter.buffer.unsafe_to_break(Some(i), Some(j + 1));
        }

        i = j;
    }
}

fn apply_simple_kerning<T: SimpleKerning>(
    c: &mut AatApplyContext,
    subtable: &T,
    is_cross_stream: bool,
) {
    let first_set = c.first_set.as_ref().unwrap();
    let second_set = c.second_set.as_ref().unwrap();

    machine_kern(
        c.face,
        c.buffer,
        c.plan.kern_mask,
        is_cross_stream,
        |left, right| {
            if !first_set.contains(left) || !second_set.contains(right) {
                0
            } else {
                subtable
                    .simple_kerning(left.into(), right.into())
                    .unwrap_or(0)
            }
        },
    );
}

struct StateMachineDriver {
    stack: [usize; 8],
    depth: usize,
}

pub trait CollectGlyphs {
    /// For each valid index, read the value of type `T`.
    /// If `filter(&value)` returns true, insert the index into `set`.
    fn collect_glyphs_filtered<F>(&self, _set: &mut U32Set, _num_glyphs: u32, _filter: F)
    where
        F: Fn(u8) -> bool;
}

impl CollectGlyphs for aat::ClassSubtable<'_> {
    fn collect_glyphs_filtered<F>(&self, set: &mut U32Set, _num_glyphs: u32, filter: F)
    where
        F: Fn(u8) -> bool,
    {
        let first_glyph = self.first_glyph() as u32;
        let class_array = self.class_array();
        for (i, class) in class_array.iter().enumerate() {
            let gid = first_glyph + i as u32;
            if filter(*class) {
                set.insert(gid);
            }
        }
    }
}

fn collect_initial_glyphs(machine: &aat::StateTable, glyphs: &mut U32Set, num_glyphs: u32) {
    let mut classes = U32Set::default();

    let class_table = machine.header.class_table().ok();
    let Some(class_table) = class_table else {
        return;
    };

    let n_classes = machine.header.state_size();
    for i in 0..n_classes {
        if let Ok(entry) = machine.entry(START_OF_TEXT, i as u8) {
            if entry.new_state == START_OF_TEXT
                && !entry.is_action_initiable()
                && !entry.is_actionable()
            {
                continue;
            }
            classes.insert(i as u32);
        }
    }

    // And glyphs in those classes.

    let filter = |class: u8| classes.contains(class as u32);

    if filter(aat::class::DELETED_GLYPH) {
        glyphs.insert(DELETED_GLYPH);
    }

    class_table.collect_glyphs_filtered(glyphs, num_glyphs, filter);
}

fn collect_start_end_safe_to_break(machine: &aat::StateTable) -> u64 {
    let mut result = 0u64;
    for state in 0..64 {
        let bit = if let Ok(entry) = machine.entry(state, aat::class::END_OF_TEXT) {
            !entry.is_actionable()
        } else {
            true
        };
        if bit {
            result |= 1 << state;
        }
    }
    result
}

fn apply_state_machine_kerning(
    c: &mut AatApplyContext,
    subtable: &aat::StateTable,
    is_cross_stream: bool,
) {
    let mut driver = StateMachineDriver {
        stack: [0; 8],
        depth: 0,
    };

    let mut state = START_OF_TEXT;
    c.buffer.idx = 0;
    loop {
        let class = if c.buffer.idx < c.buffer.len {
            get_class(
                subtable,
                c.buffer.cur(0).as_glyph(),
                c.machine_class_cache.unwrap(),
            )
        } else {
            aat::class::END_OF_TEXT
        };

        let Ok(entry) = subtable.entry(state, class) else {
            break;
        };

        let next_state = entry.new_state;

        // Conditions under which it's guaranteed safe-to-break before current glyph:
        //
        // 1. There was no action in this transition; and
        //
        // 2. If we break before current glyph, the results will be the same. That
        //    is guaranteed if:
        //
        //    2a. We were already in start-of-text state; or
        //
        //    2b. We are epsilon-transitioning to start-of-text state; or
        //
        //    2c. Starting from start-of-text state seeing current glyph:
        //
        //        2c'. There won't be any actions; and
        //
        //        2c". We would end up in the same state that we were going to end up
        //             in now, including whether epsilon-transitioning.
        //
        //    and
        //
        // 3. If we break before current glyph, there won't be any end-of-text action
        //    after previous glyph.
        //
        // This triples the transitions we need to look up, but is worth returning
        // granular unsafe-to-break results. See eg.:
        //
        //   https://github.com/harfbuzz/harfbuzz/issues/2860

        let is_safe_to_break =
            // 1
            !entry.is_actionable() &&

            // 2
            (
                state == START_OF_TEXT
                || (!entry.has_advance() && next_state == START_OF_TEXT)
                ||
                {
                    // 2c
                    if let Ok(wouldbe_entry) = subtable.entry(START_OF_TEXT, class) {
                        // 2c'
                        !wouldbe_entry.is_actionable() &&

                        // 2c"
                        (
                            next_state == wouldbe_entry.new_state &&
                            entry.has_advance() == wouldbe_entry.has_advance()
                        )
                    } else {
                        false
                    }
                }
            ) &&

            // 3
            (
                if state < 64 {
                    (c.start_end_safe_to_break & (1 << state)) != 0
                } else {
                    if let Ok(end_entry) = subtable.entry(state, aat::class::END_OF_TEXT) {
                        !end_entry.is_actionable()
                    } else {
                        false
                    }
                }
            )
        ;

        if !is_safe_to_break && c.buffer.backtrack_len() > 0 && c.buffer.idx < c.buffer.len {
            c.buffer.unsafe_to_break_from_outbuffer(
                Some(c.buffer.backtrack_len() - 1),
                Some(c.buffer.idx + 1),
            );
        }

        state_machine_transition(c, subtable, &entry, is_cross_stream, &mut driver);

        state = next_state;

        if c.buffer.idx >= c.buffer.len {
            break;
        }

        c.buffer.max_ops -= 1;
        if entry.has_advance() || c.buffer.max_ops <= 0 {
            c.buffer.next_glyph();
        }
    }
}

#[inline(always)]
fn state_machine_transition(
    c: &mut AatApplyContext,
    subtable: &aat::StateTable,
    entry: &aat::StateEntry,
    is_cross_stream: bool,
    driver: &mut StateMachineDriver,
) {
    let buffer = &mut c.buffer;
    let kern_mask = c.plan.kern_mask;

    if entry.has_push() {
        if driver.depth < driver.stack.len() {
            driver.stack[driver.depth] = buffer.idx;
            driver.depth += 1;
        } else {
            driver.depth = 0; // Probably not what CoreText does, but better?
        }
    }

    if entry.has_offset() && driver.depth != 0 {
        let mut value_offset = entry.value_offset();
        let Ok(mut value) = subtable.read_value::<i16>(value_offset as usize) else {
            driver.depth = 0;
            return;
        };

        // From Apple 'kern' spec:
        // "Each pops one glyph from the kerning stack and applies the kerning value to it.
        // The end of the list is marked by an odd value...
        let mut last = false;
        while !last && driver.depth != 0 {
            driver.depth -= 1;
            let idx = driver.stack[driver.depth];
            let mut v = value as i32;
            value_offset = value_offset.wrapping_add(2);
            value = subtable
                .read_value::<i16>(value_offset as usize)
                .unwrap_or(0);
            if idx >= buffer.len {
                continue;
            }

            // "The end of the list is marked by an odd value..."
            last = v & 1 != 0;
            v &= !1;

            // Testing shows that CoreText only applies kern (cross-stream or not)
            // if none has been applied by previous subtables. That is, it does
            // NOT seem to accumulate as otherwise implied by specs.

            let mut has_gpos_attachment = false;
            let glyph_mask = buffer.info[idx].mask;
            let pos = &mut buffer.pos[idx];

            if buffer.direction.is_horizontal() {
                if is_cross_stream {
                    // The following flag is undocumented in the spec, but described
                    // in the 'kern' table example.
                    if v == -0x8000 {
                        pos.set_attach_type(0);
                        pos.set_attach_chain(0);
                        pos.y_offset = 0;
                    } else if pos.attach_type() != 0 {
                        pos.y_offset += v;
                        has_gpos_attachment = true;
                    }
                } else if glyph_mask & kern_mask != 0 {
                    pos.x_advance += v;
                    pos.x_offset += v;
                }
            } else {
                if is_cross_stream {
                    // CoreText doesn't do crossStream kerning in vertical. We do.
                    if v == -0x8000 {
                        pos.set_attach_type(0);
                        pos.set_attach_chain(0);
                        pos.x_offset = 0;
                    } else if pos.attach_type() != 0 {
                        pos.x_offset += v;
                        has_gpos_attachment = true;
                    }
                } else if glyph_mask & kern_mask != 0 {
                    if pos.y_offset == 0 {
                        pos.y_advance += v;
                        pos.y_offset += v;
                    }
                }
            }

            if has_gpos_attachment {
                buffer.scratch_flags |= HB_BUFFER_SCRATCH_FLAG_HAS_GPOS_ATTACHMENT;
            }
        }
    }
}

trait KernStateEntryExt {
    fn flags(&self) -> u16;

    fn is_action_initiable(&self) -> bool {
        self.flags() & 0x8000 != 0
    }

    fn is_actionable(&self) -> bool {
        self.flags() & 0x3FFF != 0
    }

    fn has_offset(&self) -> bool {
        self.flags() & 0x3FFF != 0
    }

    fn value_offset(&self) -> u16 {
        self.flags() & 0x3FFF
    }

    fn has_advance(&self) -> bool {
        self.flags() & 0x4000 == 0
    }

    fn has_push(&self) -> bool {
        self.flags() & 0x8000 != 0
    }
}

impl<T> KernStateEntryExt for aat::StateEntry<T> {
    fn flags(&self) -> u16 {
        self.flags
    }
}

impl SimpleKerning for Subtable0<'_> {
    fn simple_kerning(&self, left: GlyphId, right: GlyphId) -> Option<i32> {
        self.kerning(left, right)
    }
    fn collect_glyphs(&self, first_set: &mut U32Set, second_set: &mut U32Set, _num_glyphs: u32) {
        for &pair in self.pairs() {
            first_set.insert(pair.left.get().to_u32());
            second_set.insert(pair.right.get().to_u32());
        }
    }
}

impl SimpleKerning for Subtable2<'_> {
    fn simple_kerning(&self, left: GlyphId, right: GlyphId) -> Option<i32> {
        self.kerning(left, right)
    }
    fn collect_glyphs(&self, first_set: &mut U32Set, second_set: &mut U32Set, _num_glyphs: u32) {
        let left_classes = &self.left_offset_table;
        let right_classes = &self.right_offset_table;

        let first_glyph = left_classes.first_glyph().to_u32();
        let last_glyphs = first_glyph + left_classes.n_glyphs().saturating_sub(1) as u32;
        first_set.insert_range(first_glyph..=last_glyphs);

        let first_glyph = right_classes.first_glyph().to_u32();
        let last_glyphs = first_glyph + right_classes.n_glyphs().saturating_sub(1) as u32;
        second_set.insert_range(first_glyph..=last_glyphs);
    }
}

impl SimpleKerning for Subtable3<'_> {
    fn simple_kerning(&self, left: GlyphId, right: GlyphId) -> Option<i32> {
        self.kerning(left, right)
    }
    fn collect_glyphs(&self, first_set: &mut U32Set, second_set: &mut U32Set, _num_glyphs: u32) {
        first_set.insert_range(0..=self.glyph_count().saturating_sub(1) as u32);
        second_set.insert_range(0..=self.glyph_count().saturating_sub(1) as u32);
    }
}

pub(crate) struct KernSubtableCache {
    start_end_safe_to_break: u64,
    first_set: U32Set,
    second_set: U32Set,
    class_cache: Box<ClassCache>,
}

impl KernSubtableCache {
    pub(crate) fn new(subtable: &Subtable, num_glyphs: u32) -> Self {
        let mut start_end_safe_to_break = 0u64;
        let mut first_set = U32Set::default();
        let mut second_set = U32Set::default();
        if let Ok(kind) = subtable.kind() {
            match &kind {
                SubtableKind::Format0(format0) => {
                    format0.collect_glyphs(&mut first_set, &mut second_set, num_glyphs);
                }
                SubtableKind::Format1(format1) => {
                    start_end_safe_to_break = collect_start_end_safe_to_break(format1);
                    collect_initial_glyphs(format1, &mut first_set, num_glyphs);
                }
                SubtableKind::Format2(format2) => {
                    format2.collect_glyphs(&mut first_set, &mut second_set, num_glyphs);
                }
                SubtableKind::Format3(format3) => {
                    format3.collect_glyphs(&mut first_set, &mut second_set, num_glyphs);
                }
            }
        };
        KernSubtableCache {
            start_end_safe_to_break,
            first_set,
            second_set,
            class_cache: Box::new(ClassCache::new()),
        }
    }
}
