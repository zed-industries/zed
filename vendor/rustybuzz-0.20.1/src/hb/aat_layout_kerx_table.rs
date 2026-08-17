use core::convert::TryFrom;

use ttf_parser::{ankr, apple_layout, kerx, FromData, GlyphId};

use super::buffer::*;
use super::hb_font_t;
use super::ot_layout::TableIndex;
use super::ot_layout_common::lookup_flags;
use super::ot_layout_gpos_table::attach_type;
use super::ot_layout_gsubgpos::{skipping_iterator_t, OT::hb_ot_apply_context_t};
use super::ot_shape_plan::hb_ot_shape_plan_t;

// TODO: Use set_digest, similarly to how it's used in harfbuzz.

trait ExtendedStateTableExt<T: FromData + Copy> {
    fn class(&self, glyph_id: GlyphId) -> Option<u16>;
    fn entry(&self, state: u16, class: u16) -> Option<apple_layout::GenericStateEntry<T>>;
}

impl ExtendedStateTableExt<kerx::EntryData> for kerx::Subtable1<'_> {
    fn class(&self, glyph_id: GlyphId) -> Option<u16> {
        self.state_table.class(glyph_id)
    }

    fn entry(
        &self,
        state: u16,
        class: u16,
    ) -> Option<apple_layout::GenericStateEntry<kerx::EntryData>> {
        self.state_table.entry(state, class)
    }
}

impl ExtendedStateTableExt<kerx::EntryData> for kerx::Subtable4<'_> {
    fn class(&self, glyph_id: GlyphId) -> Option<u16> {
        self.state_table.class(glyph_id)
    }

    fn entry(
        &self,
        state: u16,
        class: u16,
    ) -> Option<apple_layout::GenericStateEntry<kerx::EntryData>> {
        self.state_table.entry(state, class)
    }
}

pub(crate) fn apply(
    plan: &hb_ot_shape_plan_t,
    face: &hb_font_t,
    buffer: &mut hb_buffer_t,
) -> Option<()> {
    buffer.unsafe_to_concat(None, None);

    let mut seen_cross_stream = false;
    for subtable in face.tables().kerx?.subtables {
        if subtable.variable {
            continue;
        }

        if buffer.direction.is_horizontal() != subtable.horizontal {
            continue;
        }

        let reverse = buffer.direction.is_backward();

        if !seen_cross_stream && subtable.has_cross_stream {
            seen_cross_stream = true;

            // Attach all glyphs into a chain.
            for pos in &mut buffer.pos {
                pos.set_attach_type(attach_type::CURSIVE);
                pos.set_attach_chain(if buffer.direction.is_forward() { -1 } else { 1 });
                // We intentionally don't set BufferScratchFlags::HAS_GPOS_ATTACHMENT,
                // since there needs to be a non-zero attachment for post-positioning to
                // be needed.
            }
        }

        if reverse {
            buffer.reverse();
        }

        match subtable.format {
            kerx::Format::Format0(_) => {
                if !plan.requested_kerning {
                    continue;
                }

                apply_simple_kerning(&subtable, plan, face, buffer);
            }
            kerx::Format::Format1(ref sub) => {
                let mut driver = Driver1 {
                    stack: [0; 8],
                    depth: 0,
                };

                apply_state_machine_kerning(&subtable, sub, &mut driver, plan, buffer);
            }
            kerx::Format::Format2(_) => {
                if !plan.requested_kerning {
                    continue;
                }

                buffer.unsafe_to_concat(None, None);

                apply_simple_kerning(&subtable, plan, face, buffer);
            }
            kerx::Format::Format4(ref sub) => {
                let mut driver = Driver4 {
                    mark_set: false,
                    mark: 0,
                    ankr_table: face.tables().ankr.clone(),
                };

                apply_state_machine_kerning(&subtable, sub, &mut driver, plan, buffer);
            }
            kerx::Format::Format6(_) => {
                if !plan.requested_kerning {
                    continue;
                }

                apply_simple_kerning(&subtable, plan, face, buffer);
            }
        }

        if reverse {
            buffer.reverse();
        }
    }

    Some(())
}

fn apply_simple_kerning(
    subtable: &kerx::Subtable,
    plan: &hb_ot_shape_plan_t,
    face: &hb_font_t,
    buffer: &mut hb_buffer_t,
) {
    let mut ctx = hb_ot_apply_context_t::new(TableIndex::GPOS, face, buffer);
    ctx.set_lookup_mask(plan.kern_mask);
    ctx.lookup_props = u32::from(lookup_flags::IGNORE_FLAGS);

    let horizontal = ctx.buffer.direction.is_horizontal();

    let mut i = 0;
    while i < ctx.buffer.len {
        if (ctx.buffer.info[i].mask & plan.kern_mask) == 0 {
            i += 1;
            continue;
        }

        let mut iter = skipping_iterator_t::new(&ctx, i, false);

        let mut unsafe_to = 0;
        if !iter.next(Some(&mut unsafe_to)) {
            ctx.buffer.unsafe_to_concat(Some(i), Some(unsafe_to));
            i += 1;
            continue;
        }

        let j = iter.index();

        let info = &ctx.buffer.info;
        let kern = subtable
            .glyphs_kerning(info[i].as_glyph(), info[j].as_glyph())
            .unwrap_or(0);
        let kern = i32::from(kern);

        let pos = &mut ctx.buffer.pos;
        if kern != 0 {
            if horizontal {
                if subtable.has_cross_stream {
                    pos[j].y_offset = kern;
                    ctx.buffer.scratch_flags |= HB_BUFFER_SCRATCH_FLAG_HAS_GPOS_ATTACHMENT;
                } else {
                    let kern1 = kern >> 1;
                    let kern2 = kern - kern1;
                    pos[i].x_advance += kern1;
                    pos[j].x_advance += kern2;
                    pos[j].x_offset += kern2;
                }
            } else {
                if subtable.has_cross_stream {
                    pos[j].x_offset = kern;
                    ctx.buffer.scratch_flags |= HB_BUFFER_SCRATCH_FLAG_HAS_GPOS_ATTACHMENT;
                } else {
                    let kern1 = kern >> 1;
                    let kern2 = kern - kern1;
                    pos[i].y_advance += kern1;
                    pos[j].y_advance += kern2;
                    pos[j].y_offset += kern2;
                }
            }

            ctx.buffer.unsafe_to_break(Some(i), Some(j + 1))
        }

        i = j;
    }
}

const START_OF_TEXT: u16 = 0;

trait KerxEntryDataExt {
    fn action_index(self) -> u16;
    fn is_actionable(&self) -> bool;
}

impl KerxEntryDataExt for apple_layout::GenericStateEntry<kerx::EntryData> {
    fn action_index(self) -> u16 {
        self.extra.action_index
    }
    fn is_actionable(&self) -> bool {
        self.extra.action_index != 0xFFFF
    }
}

fn apply_state_machine_kerning<T, E>(
    subtable: &kerx::Subtable,
    state_table: &T,
    driver: &mut dyn StateTableDriver<T, E>,
    plan: &hb_ot_shape_plan_t,
    buffer: &mut hb_buffer_t,
) where
    T: ExtendedStateTableExt<E>,
    E: FromData + Copy,
    apple_layout::GenericStateEntry<E>: KerxEntryDataExt,
{
    let mut state = START_OF_TEXT;
    buffer.idx = 0;
    loop {
        let class = if buffer.idx < buffer.len {
            state_table
                .class(buffer.info[buffer.idx].as_glyph())
                .unwrap_or(1)
        } else {
            u16::from(apple_layout::class::END_OF_TEXT)
        };

        let entry: apple_layout::GenericStateEntry<E> = match state_table.entry(state, class) {
            Some(v) => v,
            None => break,
        };

        // Unsafe-to-break before this if not in state 0, as things might
        // go differently if we start from state 0 here.
        if state != START_OF_TEXT && buffer.backtrack_len() != 0 && buffer.idx < buffer.len {
            // If there's no value and we're just epsilon-transitioning to state 0, safe to break.
            if entry.is_actionable() || entry.new_state != START_OF_TEXT || entry.has_advance()
            {
                buffer.unsafe_to_break_from_outbuffer(
                    Some(buffer.backtrack_len() - 1),
                    Some(buffer.idx + 1),
                );
            }
        }

        // Unsafe-to-break if end-of-text would kick in here.
        if buffer.idx + 2 <= buffer.len {
            let end_entry: Option<apple_layout::GenericStateEntry<E>> =
                state_table.entry(state, u16::from(apple_layout::class::END_OF_TEXT));
            let end_entry = match end_entry {
                Some(v) => v,
                None => break,
            };

            if end_entry.is_actionable() {
                buffer.unsafe_to_break(Some(buffer.idx), Some(buffer.idx + 2));
            }
        }

        let _ = driver.transition(
            state_table,
            entry,
            subtable.has_cross_stream,
            subtable.tuple_count,
            plan,
            buffer,
        );

        state = entry.new_state;

        if buffer.idx >= buffer.len {
            break;
        }

        if entry.has_advance() || buffer.max_ops <= 0 {
            buffer.next_glyph();
        }
        buffer.max_ops -= 1;
    }
}

trait StateTableDriver<Table, E: FromData> {
    fn transition(
        &mut self,
        aat: &Table,
        entry: apple_layout::GenericStateEntry<E>,
        has_cross_stream: bool,
        tuple_count: u32,
        plan: &hb_ot_shape_plan_t,
        buffer: &mut hb_buffer_t,
    ) -> Option<()>;
}

struct Driver1 {
    stack: [usize; 8],
    depth: usize,
}

impl StateTableDriver<kerx::Subtable1<'_>, kerx::EntryData> for Driver1 {
    fn transition(
        &mut self,
        aat: &kerx::Subtable1,
        entry: apple_layout::GenericStateEntry<kerx::EntryData>,
        has_cross_stream: bool,
        tuple_count: u32,
        plan: &hb_ot_shape_plan_t,
        buffer: &mut hb_buffer_t,
    ) -> Option<()> {
        if entry.has_reset() {
            self.depth = 0;
        }

        if entry.has_push() {
            if self.depth < self.stack.len() {
                self.stack[self.depth] = buffer.idx;
                self.depth += 1;
            } else {
                self.depth = 0; // Probably not what CoreText does, but better?
            }
        }

        if entry.is_actionable() && self.depth != 0 {
            let tuple_count = u16::try_from(tuple_count.max(1)).ok()?;

            let mut action_index = entry.action_index();

            // From Apple 'kern' spec:
            // "Each pops one glyph from the kerning stack and applies the kerning value to it.
            // The end of the list is marked by an odd value...
            let mut last = false;
            while !last && self.depth != 0 {
                self.depth -= 1;
                let idx = self.stack[self.depth];
                let mut v = aat.glyphs_kerning(action_index)? as i32;
                action_index = action_index.checked_add(tuple_count)?;
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
                    if has_cross_stream {
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
                    } else if glyph_mask & plan.kern_mask != 0 {
                        pos.x_advance += v;
                        pos.x_offset += v;
                    }
                } else {
                    if has_cross_stream {
                        // CoreText doesn't do crossStream kerning in vertical. We do.
                        if v == -0x8000 {
                            pos.set_attach_type(0);
                            pos.set_attach_chain(0);
                            pos.x_offset = 0;
                        } else if pos.attach_type() != 0 {
                            pos.x_offset += v;
                            has_gpos_attachment = true;
                        }
                    } else if glyph_mask & plan.kern_mask != 0 {
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

        Some(())
    }
}

struct Driver4<'a> {
    mark_set: bool,
    mark: usize,
    ankr_table: Option<ankr::Table<'a>>,
}

impl StateTableDriver<kerx::Subtable4<'_>, kerx::EntryData> for Driver4<'_> {
    fn transition(
        &mut self,
        aat: &kerx::Subtable4,
        entry: apple_layout::GenericStateEntry<kerx::EntryData>,
        _has_cross_stream: bool,
        _tuple_count: u32,
        _opt: &hb_ot_shape_plan_t,
        buffer: &mut hb_buffer_t,
    ) -> Option<()> {
        if self.mark_set && entry.is_actionable() && buffer.idx < buffer.len {
            if let Some(ref ankr_table) = self.ankr_table {
                let point = aat.anchor_points.get(entry.action_index())?;

                let mark_idx = buffer.info[self.mark].as_glyph();
                let mark_anchor = ankr_table
                    .points(mark_idx)
                    .and_then(|list| list.get(u32::from(point.0)))
                    .unwrap_or_default();

                let curr_idx = buffer.cur(0).as_glyph();
                let curr_anchor = ankr_table
                    .points(curr_idx)
                    .and_then(|list| list.get(u32::from(point.1)))
                    .unwrap_or_default();

                let pos = buffer.cur_pos_mut();
                pos.x_offset = i32::from(mark_anchor.x - curr_anchor.x);
                pos.y_offset = i32::from(mark_anchor.y - curr_anchor.y);
            }

            buffer.cur_pos_mut().set_attach_type(attach_type::MARK);
            let idx = buffer.idx;
            buffer
                .cur_pos_mut()
                .set_attach_chain(self.mark as i16 - idx as i16);
            buffer.scratch_flags |= HB_BUFFER_SCRATCH_FLAG_HAS_GPOS_ATTACHMENT;
        }

        if entry.has_mark() {
            self.mark_set = true;
            self.mark = buffer.idx;
        }

        Some(())
    }
}
