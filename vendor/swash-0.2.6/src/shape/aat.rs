use super::buffer::*;
use super::internal::aat::*;

pub fn apply_morx(
    data: &[u8],
    morx: u32,
    buffer: &mut Buffer,
    selectors: &[(u16, u16)],
) -> Option<()> {
    use morx::*;
    let max_ops = buffer.glyphs.len() * 16;
    for chain in chains(data, morx) {
        let mut ops = 0;
        let mut flags = chain.default_flags();
        if !selectors.is_empty() {
            for feature in chain.features() {
                let key = (feature.selector, feature.setting_selector);
                if selectors.binary_search(&key).is_ok() {
                    flags = flags & feature.disable_flags | feature.enable_flags;
                }
            }
        }
        for (_i, subtable) in chain.subtables().enumerate() {
            if subtable.flags() & flags == 0 {
                // if TRACE {
                //     println!("    <SKIP chain subtable {}>", i);
                // }
                continue;
            } else {
                // if TRACE {
                //     println!("    <chain subtable {} order: {:?}>", i, subtable.order());
                // }
            }
            let reverse = subtable.should_reverse(buffer.is_rtl);
            buffer.ensure_order(reverse);
            let kind = match subtable.kind() {
                Some(kind) => kind,
                _ => continue,
            };
            match kind {
                SubtableKind::Rearrangement(t) => {
                    //println!(".. rearrangement");
                    let mut i = 0;
                    let mut state = RearrangementState::new();
                    while i < buffer.glyphs.len() && ops < max_ops {
                        let g = buffer.glyphs[i].id;
                        match t.next(&mut state, i, g, false, |r| {
                            // if TRACE {
                            //     println!("Rearrange!");
                            // }
                            r.apply(&mut buffer.glyphs);
                            Some(())
                        }) {
                            Some(advance) => i += advance,
                            None => break,
                        }
                        ops += 1;
                    }
                    // Apply END_OF_TEXT state
                    t.next(&mut state, i, 0, true, |r| {
                        r.apply(&mut buffer.glyphs);
                        Some(())
                    });
                }
                SubtableKind::Contextual(t) => {
                    //println!(".. contextual");
                    let mut state = ContextualState::new();
                    for i in 0..buffer.glyphs.len() {
                        let g = buffer.glyphs[i].id;
                        t.next(&mut state, i, g, false, |i, g| {
                            buffer.substitute(i, g);
                            Some(())
                        });
                    }
                    // Apply END_OF_TEXT state
                    if let Some(last_id) = buffer.glyphs.last().map(|g| g.id) {
                        t.next(
                            &mut state,
                            buffer.glyphs.len() - 1,
                            last_id,
                            true,
                            |i, g| {
                                buffer.substitute(i, g);
                                Some(())
                            },
                        );
                    }
                }
                SubtableKind::NonContextual(t) => {
                    //println!(".. non-contextual");
                    for (_i, g) in buffer.glyphs.iter_mut().enumerate() {
                        if let Some(s) = t.substitute(g.id) {
                            // if TRACE {
                            //     println!("NonContextual[{}] {} -> {}", i, g.id, s);
                            // }
                            g.id = s;
                        }
                    }
                }
                SubtableKind::Ligature(t) => {
                    //println!(".. ligature");
                    let mut i = 0;
                    let mut state = LigatureState::new();
                    while i < buffer.glyphs.len() && ops < max_ops {
                        let g = buffer.glyphs[i].id;
                        let f = |i, g, comps: &[usize]| {
                            buffer.substitute_ligature(i, g, comps);
                            Some(())
                        };
                        if t.next(&mut state, i, g, false, f).is_none() {
                            break;
                        }
                        i += 1;
                        ops += 1;
                    }
                    // Apply END_OF_TEXT state
                    t.next(
                        &mut state,
                        buffer.glyphs.len().saturating_sub(1),
                        0,
                        true,
                        |i, g, comps| {
                            buffer.substitute_ligature(i, g, comps);
                            Some(())
                        },
                    );
                }
                SubtableKind::Insertion(t) => {
                    //println!(".. insertion");
                    let mut i = 0;
                    let mut state = InsertionState::new();
                    while i < buffer.glyphs.len() && ops < max_ops {
                        let g = buffer.glyphs[i].id;
                        match t.next(&mut state, i, g, false, |i, array| {
                            // if TRACE {
                            //     let rep = array.iter().collect::<Vec<_>>();
                            //     println!("Insert[{}] {:?}", i, &rep);
                            // }
                            buffer.multiply(i, array.len());
                            let start = i;
                            let end = start + array.len();
                            for (g, s) in buffer.glyphs[start..end].iter_mut().zip(array.iter()) {
                                g.id = s;
                                g.flags = 0;
                            }
                            Some(())
                        }) {
                            Some(advance) => i += advance,
                            None => break,
                        }
                        ops += 1;
                    }
                    // Apply END_OF_TEXT state
                    t.next(
                        &mut state,
                        buffer.glyphs.len().saturating_sub(1),
                        0,
                        true,
                        |i, array| {
                            buffer.multiply(i, array.len());
                            let start = i;
                            let end = start + array.len();
                            for (g, s) in buffer.glyphs[start..end].iter_mut().zip(array.iter()) {
                                g.id = s;
                                g.flags = 0;
                            }
                            Some(())
                        },
                    );
                }
            }
        }
    }
    buffer.ensure_order(false);
    Some(())
}

pub fn apply_kerx(
    data: &[u8],
    kerx: u32,
    ankr: u32,
    buffer: &mut Buffer,
    disable_kern: bool,
) -> Option<()> {
    use kerx::*;
    for (_i, subtable) in subtables(data, kerx, ankr).enumerate() {
        // if TRACE {
        //     println!("    <kerx subtable {}>", i);
        // }
        let reverse = subtable.should_reverse(buffer.is_rtl);
        buffer.ensure_order(reverse);
        let kind = match subtable.kind() {
            Some(kind) => kind,
            _ => continue,
        };
        if subtable.is_vertical() || subtable.is_cross_stream() {
            continue;
        }
        match kind {
            SubtableKind::Format0(t) => {
                if disable_kern {
                    continue;
                }
                let len = buffer.len();
                let mut left_index = if let Some((index, _)) = buffer
                    .glyphs
                    .iter()
                    .enumerate()
                    .find(|(_, g)| g.joining_type != 6)
                {
                    index
                } else {
                    continue;
                };
                let mut left = buffer.glyphs[left_index].id;
                for i in left_index + 1..len {
                    if buffer.glyphs[i].joining_type == 6 {
                        continue;
                    }
                    let right = buffer.glyphs[i].id;
                    if let Some(kerning) = t.get(left, right) {
                        if kerning != 0 {
                            // if TRACE {
                            //     println!("KERN [{} & {}] {}", left_index, i, kerning);
                            // }
                            buffer.positions[left_index].advance += kerning as f32;
                        }
                    }
                    left_index = i;
                    left = right;
                }
            }
            SubtableKind::Format1(t) => {
                if disable_kern {
                    continue;
                }
                let mut i = 0;
                let len = buffer.glyphs.len();
                let mut state = ContextualState::new();
                while i < len {
                    match t.next(&mut state, i, buffer.glyphs[i].id, |i, kerning| {
                        buffer.positions[i].advance += kerning as f32;
                        Some(())
                    }) {
                        Some(advance) => i += advance,
                        None => break,
                    }
                }
            }
            SubtableKind::Format2(t) => {
                if disable_kern {
                    continue;
                }
                //println!("142/116 = {:?}", t.get(142, 116));
                let len = buffer.len();
                let mut left_index = if let Some((index, _)) = buffer
                    .glyphs
                    .iter()
                    .enumerate()
                    .find(|(_, g)| g.joining_type != 6)
                {
                    index
                } else {
                    continue;
                };
                let mut left = buffer.glyphs[left_index].id;
                for i in left_index + 1..len {
                    if buffer.glyphs[i].joining_type == 6 {
                        continue;
                    }
                    let right = buffer.glyphs[i].id;
                    if let Some(kerning) = t.get(left, right) {
                        if kerning != 0 {
                            // if TRACE {
                            //     println!("KERN [{} & {}] {}", left_index, i, kerning);
                            // }
                            buffer.positions[left_index].advance += kerning as f32;
                        }
                    }
                    left_index = i;
                    left = right;
                }
            }
            SubtableKind::Format4(t) => {
                let mut i = 0;
                let len = buffer.glyphs.len();
                let mut state = Format4State::new();
                while i < len {
                    match t.next(&mut state, i, buffer.glyphs[i].id, |i, base, x, y| {
                        buffer.position_mark(i, base, x, y);
                        Some(())
                    }) {
                        Some(advance) => i += advance,
                        None => break,
                    }
                }
            }
        }
    }
    buffer.ensure_order(false);
    Some(())
}

pub fn apply_kern(data: &[u8], kern: u32, buffer: &mut Buffer) -> Option<()> {
    use kern::*;
    for (_i, subtable) in subtables(data, kern).enumerate() {
        let kind = match subtable.kind() {
            Some(kind) => kind,
            _ => continue,
        };
        if !subtable.is_horizontal() {
            continue;
        }
        buffer.ensure_order(buffer.is_rtl);
        let cross_stream = subtable.cross_stream();
        match kind {
            SubtableKind::Format0(t) => {
                buffer.ensure_order(false);
                let len = buffer.len();
                let mut left_index = if let Some((index, _)) = buffer
                    .glyphs
                    .iter()
                    .enumerate()
                    .find(|(_, g)| g.joining_type != 6)
                {
                    index
                } else {
                    continue;
                };
                let mut left = buffer.glyphs[left_index].id;
                for i in left_index + 1..len {
                    if buffer.glyphs[i].joining_type == 6 {
                        continue;
                    }
                    let right = buffer.glyphs[i].id;
                    if let Some(kerning) = t.get(left, right) {
                        if kerning != 0 {
                            // if TRACE {
                            //     println!("KERN [{} & {}] {}", left_index, i, kerning);
                            // }
                            buffer.positions[left_index].advance += kerning as f32;
                        }
                    }
                    left_index = i;
                    left = right;
                }
            }
            SubtableKind::Format1(t) => {
                let mut i = 0;
                let len = buffer.glyphs.len();
                let mut state = Format1State::new();
                while i < len {
                    match t.next(&mut state, i, buffer.glyphs[i].id, |i, kerning| {
                        let g = &buffer.glyphs[i];
                        if g.joining_type == 6 {
                            if cross_stream {
                                let pos = &mut buffer.positions[i];
                                if pos.y == 0. {
                                    pos.y = kerning as f32;
                                }
                            } else if let Some(base) = find_base(buffer, buffer.is_rtl, i) {
                                let diff = if base >= i { base - i } else { i - base };
                                if diff < 255 {
                                    let pos = &mut buffer.positions[i];
                                    if pos.base == 0 {
                                        pos.flags |= MARK_ATTACH;
                                        pos.base = diff as u8;
                                        pos.x = kerning as f32;
                                        buffer.has_marks = true;
                                    }
                                }
                            }
                        }
                        Some(())
                    }) {
                        Some(advance) => i += advance,
                        None => break,
                    }
                }
            }
        }
    }
    buffer.ensure_order(false);
    Some(())
}

fn find_base(buffer: &Buffer, reverse: bool, index: usize) -> Option<usize> {
    use crate::text::cluster::ShapeClass;
    let cluster = buffer.glyphs[index].cluster;
    if reverse {
        for i in index + 1..buffer.len() {
            let g = &buffer.glyphs[i];
            if g.cluster != cluster {
                return None;
            }
            if g.char_class == ShapeClass::Base {
                return Some(i);
            }
        }
    } else if index > 0 {
        for i in (0..index).rev() {
            let g = &buffer.glyphs[i];
            if g.cluster != cluster {
                return None;
            }
            if g.char_class == ShapeClass::Base {
                return Some(i);
            }
        }
    }
    None
}
