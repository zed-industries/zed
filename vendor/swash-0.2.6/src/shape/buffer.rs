use super::cluster::{Glyph, GlyphInfo};
use super::feature::*;
use crate::text::{
    cluster::{Char, CharCluster, ClusterInfo, ShapeClass, SourceRange, MAX_CLUSTER_SIZE},
    JoiningType,
};

use alloc::vec::Vec;
use core::ops::Range;

// Glyph flags.
pub const SUBSTITUTED: u16 = 1;
pub const LIGATED: u16 = 2;
pub const COMPONENT: u16 = 4;
pub const MARK_ATTACH: u16 = 8;
pub const CURSIVE_ATTACH: u16 = 16;
pub const IGNORABLE: u16 = 64;

/// Per glyph shaping data.
#[derive(Copy, Clone, Default, Debug)]
pub struct GlyphData {
    pub id: u16,
    pub flags: u16,
    pub class: u8,
    pub char_class: ShapeClass,
    pub mark_type: u8,
    pub joining_type: u8,
    pub mask: u8,
    pub skip: bool,
    pub component: u8,
    pub cluster: u32,
    pub data: u32,
}

impl GlyphData {
    pub fn is_component(&self) -> bool {
        self.flags & COMPONENT != 0
    }
}

/// Per glyph shaping position data.
#[derive(Copy, Clone, Default, Debug)]
pub struct PositionData {
    pub base: u8,
    pub flags: u16,
    pub x: f32,
    pub y: f32,
    pub advance: f32,
}

impl Glyph {
    pub(super) fn new(g: &GlyphData, p: &PositionData) -> Self {
        Self {
            id: g.id,
            info: GlyphInfo(p.flags),
            x: p.x,
            y: p.y,
            advance: p.advance,
            data: g.data,
        }
    }
}

#[derive(Clone, Default)]
pub struct Buffer {
    pub glyphs: Vec<GlyphData>,
    pub positions: Vec<PositionData>,
    pub infos: Vec<(ClusterInfo, bool, u32)>,
    pub ranges: Vec<SourceRange>,
    pub shaped_glyphs: Vec<Glyph>,
    pub is_rtl: bool,
    pub dotted_circle: Option<u16>,
    pub has_cursive: bool,
    pub has_marks: bool,
    pub reversed: bool,
    pub next_cluster: u32,
    pub skip_state: SkipState,
    pub sub_args: Vec<u16>,
    pub pos_args: Vec<u16>,
}

#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub struct SkipState {
    pub flags: u8,
    pub mask: u8,
    pub mark_check: u8,
    pub mark_class: u8,
    pub mark_set: u32,
}

impl Buffer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.glyphs.len()
    }

    pub fn push(&mut self, cluster: &CharCluster) -> Range<usize> {
        let start = self.glyphs.len();
        let chars = cluster.mapped_chars();
        if cluster.info().is_broken() {
            if let Some(id) = self.dotted_circle {
                let first = &chars[0];
                self.push_char(&Char {
                    ch: '\u{25cc}',
                    shape_class: ShapeClass::Base,
                    joining_type: JoiningType::U,
                    ignorable: false,
                    contributes_to_shaping: true,
                    glyph_id: id,
                    offset: first.offset,
                    data: first.data,
                });
            }
        }
        for ch in chars {
            self.push_char(ch);
        }
        self.next_cluster += 1;
        self.push_cluster(cluster);
        start..self.glyphs.len()
    }

    pub fn push_order(&mut self, cluster: &CharCluster, order: &[usize]) -> Range<usize> {
        let start = self.glyphs.len();
        let chars = cluster.mapped_chars();
        if cluster.info().is_broken() {
            if let Some(id) = self.dotted_circle {
                let first = &chars[order[0]];
                self.push_char(&Char {
                    ch: '\u{25cc}',
                    shape_class: ShapeClass::Base,
                    joining_type: JoiningType::U,
                    ignorable: false,
                    contributes_to_shaping: true,
                    glyph_id: id,
                    offset: first.offset,
                    data: first.data,
                });
            }
        }
        for ch in order[..chars.len()].iter().map(|i| &chars[*i]) {
            self.push_char(ch);
        }
        self.next_cluster += 1;
        self.push_cluster(cluster);
        start..self.glyphs.len()
    }

    pub fn _push_hangul(&mut self, cluster: &CharCluster) -> Range<usize> {
        let start = self.glyphs.len();
        let chars = cluster.mapped_chars();
        if cluster.info().is_broken() {
            if let Some(id) = self.dotted_circle {
                let first = &chars[0];
                self.push_char(&Char {
                    ch: '\u{25cc}',
                    shape_class: ShapeClass::Base,
                    joining_type: JoiningType::U,
                    ignorable: false,
                    contributes_to_shaping: true,
                    glyph_id: id,
                    offset: first.offset,
                    data: first.data,
                });
            }
        }
        for ch in chars {
            self._push_hangul_char(ch);
        }
        self.next_cluster += 1;
        self.push_cluster(cluster);
        start..self.glyphs.len()
    }

    #[inline(always)]
    fn push_char(&mut self, ch: &Char) {
        let cluster = self.next_cluster;
        self.glyphs.push(GlyphData {
            id: ch.glyph_id,
            flags: (ch.ignorable as u16) << 6,
            class: 0,
            char_class: ch.shape_class,
            joining_type: ch.joining_type as u8,
            mark_type: 0,
            mask: 0,
            skip: false,
            component: !0,
            cluster,
            data: ch.data,
        });
    }

    fn _push_hangul_char(&mut self, ch: &Char) {
        let cluster = self.next_cluster;
        let c = ch.ch as u32;
        let mask = if (0x1100..=0x115F).contains(&c) || (0xA960..=0xA97C).contains(&c) {
            1
        } else if (0x1160..=0x11A7).contains(&c) || (0xD7B0..=0xD7C6).contains(&c) {
            2
        } else if (0x11A8..=0x11FF).contains(&c) || (0xD7CB..=0xD7FB).contains(&c) {
            4
        } else {
            1 | 2 | 4
        };
        self.glyphs.push(GlyphData {
            id: ch.glyph_id,
            flags: (ch.ignorable as u16) << 6,
            class: 0,
            char_class: ch.shape_class,
            joining_type: ch.joining_type as u8,
            mark_type: 0,
            mask,
            skip: false,
            component: !0,
            cluster,
            data: ch.data,
        });
    }

    fn push_cluster(&mut self, cluster: &CharCluster) {
        self.infos
            .push((cluster.info(), false, cluster.user_data()));
        self.ranges.push(cluster.range());
    }

    pub fn clear(&mut self) {
        self.glyphs.clear();
        self.positions.clear();
        self.infos.clear();
        self.ranges.clear();
        self.is_rtl = false;
        self.reversed = false;
        self.has_cursive = false;
        self.has_marks = false;
        self.dotted_circle = None;
        self.next_cluster = 0;
        self.skip_state = SkipState::default();
    }

    pub fn ensure_order(&mut self, reversed: bool) {
        if reversed != self.reversed {
            self.glyphs.reverse();
            if !self.positions.is_empty() {
                self.positions.reverse();
            }
            self.reversed = reversed;
        }
    }

    pub fn clear_flags(&mut self, flags: u16, range: Option<Range<usize>>) {
        if let Some(range) = range {
            for g in &mut self.glyphs[range] {
                g.flags &= !flags;
            }
        } else {
            for g in &mut self.glyphs {
                g.flags &= !flags;
            }
        }
    }

    pub fn setup_positions(&mut self, was_morx: bool) {
        if was_morx {
            self.glyphs
                .retain(|g| g.flags & COMPONENT == 0 && g.id != 0xFFFF);
        } else {
            self.glyphs.retain(|g| g.flags & COMPONENT == 0);
        }
        self.positions.clear();
        self.positions
            .resize(self.glyphs.len(), PositionData::default());
    }

    pub fn substitute(&mut self, index: usize, id: u16) {
        let g = &mut self.glyphs[index];
        // if TRACE {
        //     println!("!subst[{}] {} -> {}", index, g.id, id);
        // }
        g.id = id;
        g.flags |= SUBSTITUTED;
    }

    pub fn substitute_ligature(&mut self, index: usize, id: u16, components: &[usize]) {
        // if TRACE {
        //     print!("!subst[{}] {}", index, self.glyphs[index].id);
        //     for c in components {
        //         print!(" {}", self.glyphs[*c].id)
        //     }
        //     println!(" -> {}", id);
        // }
        if components.is_empty() {
            return;
        }
        let g = &mut self.glyphs[index];
        g.id = id;
        g.flags |= SUBSTITUTED | LIGATED;
        let cluster = g.cluster;
        let mut last_index = index;
        for (i, &index) in components.iter().enumerate() {
            let g = &mut self.glyphs[index];
            self.infos[g.cluster as usize].1 = true;
            g.id = 0xFFFF;
            g.flags |= COMPONENT;
            g.class = 5;
            g.cluster = cluster;
            g.skip = true;
            if (index - last_index) > 1 {
                let component = i as u8;
                for g in &mut self.glyphs[last_index + 1..index] {
                    if g.mark_type != 0 || g.class == 3 {
                        g.component = component;
                        g.cluster = cluster;
                    }
                }
            }
            last_index = index;
        }
        if (last_index + 1) < self.glyphs.len() {
            let last_component = components.len() as u8;
            for g in &mut self.glyphs[last_index + 1..] {
                if g.mark_type != 0 || g.class == 3 {
                    g.component = last_component;
                    g.cluster = cluster;
                } else {
                    break;
                }
            }
        }
    }

    pub fn substitute_multiple(&mut self, index: usize, ids: &[u16]) {
        let count = ids.len();
        if count == 0 {
            self.glyphs.remove(index);
            return;
        } else if count == 1 {
            self.substitute(index, ids[0]);
            return;
        }
        // if TRACE {
        //     println!("!subst[{}] {} -> {:?}", index, self.glyphs[index].id, ids);
        // }
        let g = self.glyphs[index];
        self.glyphs
            .splice(index..index + 1, SubstIter { ids, g, cur: 0 });
    }

    pub fn multiply(&mut self, index: usize, count: usize) {
        let g = self
            .glyphs
            .get(index)
            .copied()
            .unwrap_or_else(GlyphData::default);
        self.glyphs.splice(index..index, (0..count).map(|_| g));
    }

    pub fn position(&mut self, index: usize, x: f32, y: f32, xadvance: f32, _yadvance: f32) {
        let p = &mut self.positions[index];
        p.x += x;
        p.y += y;
        p.advance += xadvance;
    }

    pub fn position_cursive(&mut self, index: usize, next: usize, x: f32, y: f32) {
        let p = &mut self.positions[index];
        self.has_cursive = true;
        p.flags = CURSIVE_ATTACH;
        if true {
            //self.dir.is_horizontal() {
            p.y = y;
        //p.advance -= x;
        } else {
            p.x = x;
        }
        p.base = (next - index) as u8;
    }

    pub fn position_mark(&mut self, index: usize, base: usize, dx: f32, dy: f32) {
        let p = &mut self.positions[index];
        self.has_marks = true;
        p.flags = MARK_ATTACH;
        p.base = (index - base) as u8;
        p.x = dx;
        p.y = dy;
    }

    pub fn set_join_masks(&mut self) {
        let mut prev: Option<usize> = None;
        let mut state = 0;
        let glyphs = &mut self.glyphs;
        let len = glyphs.len();
        // Transparent joining type.
        const JOIN_T: u8 = 6;
        for i in 0..len {
            let ty = glyphs[i].joining_type;
            if ty == JOIN_T {
                continue;
            }
            let entry = JOIN_STATES[state][ty as usize];
            if let Some(j) = prev {
                if entry.0 != NONE_MASK {
                    glyphs[j].mask = entry.0;
                }
            }
            glyphs[i].mask = entry.1;
            prev = Some(i);
            state = entry.2 as usize;
        }
    }
}

struct SubstIter<'a> {
    ids: &'a [u16],
    g: GlyphData,
    cur: usize,
}

impl<'a> Iterator for SubstIter<'a> {
    type Item = GlyphData;

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.ids.len() - self.cur;
        (remaining, Some(remaining))
    }

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur >= self.ids.len() {
            return None;
        }
        let g = GlyphData {
            id: self.ids[self.cur],
            flags: SUBSTITUTED,
            ..self.g
        };
        self.cur += 1;
        Some(g)
    }
}

pub fn reorder_myanmar(chars: &[Char], order: &mut Vec<usize>) {
    use ShapeClass::*;
    let mut ignored = [false; MAX_CLUSTER_SIZE];
    let mut base = None;
    let mut kinzi: Option<Range<usize>> = None;
    let mut medial_ra = None;
    let mut vpre: Option<Range<usize>> = None;
    let mut vblw: Option<usize> = None;
    let mut anus: Option<Range<usize>> = None;
    let mut i = 0;
    let mut last_vblw = false;
    let len = chars.len();
    if len == 0 {
        return;
    }
    if order.len() < len {
        order.resize(chars.len(), 0);
    }
    if chars[0].shape_class == Kinzi {
        kinzi = Some(0..3);
        ignored[0] = true;
        ignored[1] = true;
        ignored[2] = true;
        i = 3;
    }
    while i < len {
        let ch = chars[i];
        let k = ch.shape_class;
        if last_vblw && k == Anusvara {
            anus = match anus {
                Some(r) => Some(r.start..i - r.start + 1),
                None => Some(i..i + 1),
            };
            ignored[i] = true;
            i += 1;
            continue;
        }
        last_vblw = false;
        if k == VBlw {
            if vblw.is_none() {
                vblw = Some(i);
            }
            last_vblw = true;
        }
        if k == Base && base.is_none() {
            base = Some(i);
            ignored[i] = true;
        } else if k == MedialRa {
            medial_ra = Some(i);
            ignored[i] = true;
        } else if k == VPre {
            vpre = match vpre {
                Some(r) => Some(r.start..i - r.start + 1),
                None => Some(i..i + 1),
            };
            ignored[i] = true;
        }
        i += 1;
    }
    i = 0;
    if let Some(r) = vpre {
        for j in r {
            order[i] = j;
            i += 1;
        }
    }
    if let Some(j) = medial_ra {
        order[i] = j;
        i += 1;
    }
    if let Some(j) = base {
        order[i] = j;
        i += 1;
    }
    if let Some(r) = kinzi {
        for j in r {
            order[i] = j;
            i += 1;
        }
    }
    let mut j = 0;
    while j < len {
        if ignored[j] {
            j += 1;
            continue;
        }
        if Some(j) == vblw && anus.is_some() {
            for k in anus.take().unwrap() {
                order[i] = k;
                i += 1;
            }
        }
        order[i] = j;
        i += 1;
        j += 1;
    }
}

#[allow(clippy::needless_range_loop)]
pub fn reorder_complex(glyphs: &mut [GlyphData], buf: &mut Vec<GlyphData>, order: &mut Vec<usize>) {
    use ShapeClass::*;
    let mut first_base = None;
    let mut last_base = None;
    let mut last_halant = None;
    let mut reph = None;
    let mut pref = None;
    let mut vpre: Option<Range<usize>> = None;
    let mut vmpre: Option<Range<usize>> = None;
    let mut ignored = [false; 64];
    let len = glyphs.len();
    if buf.len() < glyphs.len() {
        buf.resize(len, GlyphData::default());
    }
    let buf = &mut buf[..len];
    if order.len() < len {
        order.resize(len, 0);
    }
    let order = &mut order[..len];
    for (i, g) in glyphs.iter().enumerate() {
        if g.is_component() {
            continue;
        }
        match g.char_class {
            Base => {
                if first_base.is_none() {
                    first_base = Some(i);
                    ignored[i] = true;
                }
                if last_halant.is_none() {
                    last_base = Some(i);
                }
            }
            Halant => {
                last_halant = Some(i);
            }
            Reph => {
                if reph.is_none() {
                    reph = Some(i);
                    ignored[i] = true;
                }
            }
            Pref => {
                if pref.is_none() {
                    pref = Some(i);
                    ignored[i] = true;
                }
            }
            VPre => {
                vpre = match vpre {
                    Some(r) => Some(r.start..i - r.start + 1),
                    None => Some(i..i + 1),
                };
                ignored[i] = true;
            }
            VMPre => {
                vmpre = match vmpre {
                    Some(r) => Some(r.start..i - r.start + 1),
                    None => Some(i..i + 1),
                };
                ignored[i] = true;
            }
            _ => {}
        }
    }
    let mut j = 0;
    // No explicit virama; insert vmpre, vpre, pref
    if last_halant.is_none() {
        if let Some(r) = vmpre.clone() {
            for i in r {
                order[j] = i;
                j += 1;
            }
        }
        if let Some(r) = vpre.clone() {
            for i in r {
                order[j] = i;
                j += 1;
            }
        }
        if let Some(i) = pref {
            order[j] = i;
            j += 1;
        }
    }
    // Insert the base...
    if let Some(i) = first_base {
        order[j] = i;
        j += 1;
    }
    if last_base.is_none() {
        // ... and the reph
        if let Some(i) = reph {
            order[j] = i;
            j += 1;
        }
    }
    // Now the rest
    let len = glyphs.len();
    for i in 0..len {
        if ignored[i] {
            continue;
        }
        // println!(" -> i = {}, j = {}", i, j);
        // println!("order: {:?}", order);
        // println!("ignored: {:?}", &ignored[0..order.len()]);
        order[j] = i;
        j += 1;
        // Insert reph after final base
        if Some(i) == last_base {
            if let Some(i) = reph {
                order[j] = i;
                j += 1;
            }
        }
        // Move vmpre, vpre and pref after the final virama
        if Some(i) == last_halant {
            if let Some(r) = vmpre.clone() {
                for i in r {
                    order[j] = i;
                    j += 1;
                }
            }
            if let Some(r) = vpre.clone() {
                for i in r {
                    order[j] = i;
                    j += 1;
                }
            }
            if let Some(i) = pref {
                order[j] = i;
                j += 1;
            }
        }
    }
    // Reorder glyphs
    buf.copy_from_slice(glyphs);
    for (i, j) in order.iter().enumerate() {
        glyphs[i] = buf[*j];
    }
    // println!("order: {:?}", order);
    // let new = glyphs
    //     .iter()
    //     .map(|g| (g.class, char_kind(g.subclass)))
    //     .collect::<Vec<_>>();
    // println!("new: {:?}", &new[..]);
}

// Matches the Arabic joining table from Harfbuzz.
#[rustfmt::skip]
const JOIN_STATES: [[(u8, u8, u8); 6]; 7] = [
    //   U,                            L,                       R,                        D,                    ALAPH,                 DALATH_RISH
    // State 0: prev was U, not willing to join.
    [ (NONE_MASK,NONE_MASK,0), (NONE_MASK,ISOL_MASK,2), (NONE_MASK,ISOL_MASK,1), (NONE_MASK,ISOL_MASK,2), (NONE_MASK,ISOL_MASK,1), (NONE_MASK,ISOL_MASK,6), ],
    // State 1: prev was R or ISOL_MASK/ALAPH, not willing to join.
    [ (NONE_MASK,NONE_MASK,0), (NONE_MASK,ISOL_MASK,2), (NONE_MASK,ISOL_MASK,1), (NONE_MASK,ISOL_MASK,2), (NONE_MASK,FIN2_MASK,5), (NONE_MASK,ISOL_MASK,6), ],
    // State 2: prev was D/L in ISOL_MASK form, willing to join.
    [ (NONE_MASK,NONE_MASK,0), (NONE_MASK,ISOL_MASK,2), (INIT_MASK,FINA_MASK,1), (INIT_MASK,FINA_MASK,3), (INIT_MASK,FINA_MASK,4), (INIT_MASK,FINA_MASK,6), ],
    // State 3: prev was D in FINA_MASK form, willing to join. */
    [ (NONE_MASK,NONE_MASK,0), (NONE_MASK,ISOL_MASK,2), (MEDI_MASK,FINA_MASK,1), (MEDI_MASK,FINA_MASK,3), (MEDI_MASK,FINA_MASK,4), (MEDI_MASK,FINA_MASK,6), ],
    // State 4: prev was FINA_MASK ALAPH, not willing to join. */
    [ (NONE_MASK,NONE_MASK,0), (NONE_MASK,ISOL_MASK,2), (MED2_MASK,ISOL_MASK,1), (MED2_MASK,ISOL_MASK,2), (MED2_MASK,FIN2_MASK,5), (MED2_MASK,ISOL_MASK,6), ],
    // State 5: prev was FIN2_MASK/FIN3_MASK ALAPH, not willing to join. */
    [ (NONE_MASK,NONE_MASK,0), (NONE_MASK,ISOL_MASK,2), (ISOL_MASK,ISOL_MASK,1), (ISOL_MASK,ISOL_MASK,2), (ISOL_MASK,FIN2_MASK,5), (ISOL_MASK,ISOL_MASK,6), ],
    // State 6: prev was DALATH/RISH, not willing to join. */
    [ (NONE_MASK,NONE_MASK,0), (NONE_MASK,ISOL_MASK,2), (NONE_MASK,ISOL_MASK,1), (NONE_MASK,ISOL_MASK,2), (NONE_MASK,FIN3_MASK,5), (NONE_MASK,ISOL_MASK,6), ]
];
