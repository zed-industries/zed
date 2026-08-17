//! Apple advanced typography tables.

use super::{raw_tag, Array, Bytes, FromBeData, RawTag};

pub const MORX: RawTag = raw_tag(b"morx");
pub const LTAG: RawTag = raw_tag(b"ltag");
pub const KERX: RawTag = raw_tag(b"kerx");
pub const ANKR: RawTag = raw_tag(b"ankr");
pub const KERN: RawTag = raw_tag(b"kern");

/// Maximum number of times we allow consecutive `DONT_ADVANCE` states.
const MAX_CYCLES: u16 = 16;

/// Gets a value from a lookup table.
///
/// <https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6Tables.html>
pub fn lookup<T: FromBeData>(data: &Bytes, offset: usize, id: u16) -> Option<T> {
    let element_size = T::SIZE;
    let fmt = data.read::<u16>(offset)?;
    match fmt {
        0 => {
            return data.read::<T>(offset + 2 + id as usize * element_size);
        }
        2 => {
            let reclen = (4 + element_size).max(6);
            let nrecs = data.read::<u16>(offset + 4)? as usize;
            let base = offset + 12;
            let mut l = 0;
            let mut h = nrecs;
            while l < h {
                let i = (l + h) / 2;
                let rec = base + i * reclen;
                let last = data.read::<u16>(rec)?;
                if id > last {
                    l = i + 1;
                } else if id < data.read::<u16>(rec + 2)? {
                    h = i;
                } else {
                    return data.read::<T>(rec + 4);
                }
            }
        }
        4 => {
            let reclen = 6;
            let nrecs = data.read::<u16>(offset + 4)? as usize;
            let base = offset + 12;
            let mut l = 0;
            let mut h = nrecs;
            while l < h {
                let i = (l + h) / 2;
                let rec = base + i * reclen;
                let first = data.read::<u16>(rec + 2)?;
                let last = data.read::<u16>(rec)?;
                if id > last {
                    l = i + 1;
                } else if id < first {
                    h = i;
                } else {
                    let index = (id - first) as usize;
                    let value_offset =
                        data.read::<u16>(rec + 4)? as usize + offset + index * element_size;
                    return data.read::<T>(value_offset);
                }
            }
        }
        6 => {
            let reclen = (2 + element_size).max(4);
            let nrecs = data.read::<u16>(offset + 4)? as usize;
            let base = offset + 12;
            let mut l = 0;
            let mut h = nrecs;
            while l < h {
                use core::cmp::Ordering::*;
                let i = (l + h) / 2;
                let rec = base + i * reclen;
                let glyph = data.read::<u16>(rec)?;
                match id.cmp(&glyph) {
                    Greater => l = i + 1,
                    Less => h = i,
                    Equal => return data.read::<T>(rec + 2),
                }
            }
        }
        8 => {
            let first = data.read::<u16>(offset + 2)?;
            let count = data.read::<u16>(offset + 4)?;
            if id < first {
                return None;
            }
            let index = id - first;
            if index >= count {
                return None;
            }
            return data.read::<T>(offset + 6 + index as usize * element_size);
        }
        10 => {
            let first = data.read::<u16>(offset + 4)?;
            let count = data.read::<u16>(offset + 6)?;
            if id < first {
                return None;
            }
            let index = id - first;
            if index >= count {
                return None;
            }
            return data.read::<T>(offset + 8 + index as usize * element_size);
        }
        _ => {}
    }
    None
}

/// Extended state table.
///
/// <https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6Tables.html>
#[derive(Copy, Clone)]
pub struct ExtendedStateTable<'a> {
    data: Bytes<'a>,
    classes: u32,
    class_table: u32,
    state_array: u32,
    entry_table: u32,
}

impl<'a> ExtendedStateTable<'a> {
    /// Creates a new extended state table for the specified data.
    pub fn new(data: &Bytes<'a>) -> Option<Self> {
        Some(Self {
            data: *data,
            classes: data.read::<u32>(0)?,
            class_table: data.read::<u32>(4)?,
            state_array: data.read::<u32>(8)?,
            entry_table: data.read::<u32>(12)?,
        })
    }

    /// Returns the class for the specified glyph id.
    pub fn class(&self, glyph_id: u16) -> u16 {
        if glyph_id == 0xFFFF {
            return 2;
        }
        lookup::<u16>(&self.data, self.class_table as usize, glyph_id).unwrap_or(1)
    }

    /// Returns the entry for the specified state and class.
    pub fn entry<T: FromBeData>(&self, state: u16, class: u16) -> Option<Entry<T>> {
        let mut offset = self.state_array as usize;
        offset += state as usize * self.classes as usize * 2 + class as usize * 2;
        let index = self.data.read::<u16>(offset)? as usize;
        let entry_offset = index * (4 + T::SIZE);
        self.data
            .read::<Entry<T>>(self.entry_table as usize + entry_offset)
    }
}

#[derive(Copy, Clone)]
pub struct StateTable<'a> {
    data: Bytes<'a>,
    class_count: u16,
    state_array: u32,
    states: &'a [u8],
    entry_table: u32,
    first_glyph: u16,
    classes: &'a [u8],
}

impl<'a> StateTable<'a> {
    /// Creates a new extended state table for the specified data.
    pub fn new(data: &Bytes<'a>) -> Option<Self> {
        let class_count = data.read_u16(0)?;
        let class_offset = data.read_u16(2)? as usize;
        let state_array = data.read_u16(4)? as u32;
        let entry_table = data.read_u16(6)? as u32;
        let first_glyph = data.read_u16(class_offset)?;
        let glyph_count = data.read_u16(class_offset + 2)? as usize;
        let classes = data.read_bytes(class_offset + 4, glyph_count)?;
        let states = data.data().get(state_array as usize..)?;
        Some(Self {
            data: *data,
            class_count,
            state_array,
            states,
            entry_table,
            first_glyph,
            classes,
        })
    }

    /// Returns the class for the specified glyph id.
    pub fn class(&self, glyph_id: u16) -> u16 {
        if glyph_id == 0xFFFF {
            return 2;
        }
        if let Some(index) = glyph_id.checked_sub(self.first_glyph) {
            self.classes.get(index as usize).copied().unwrap_or(1) as u16
        } else {
            1
        }
    }

    /// Returns the entry for the specified state and class.
    pub fn entry<T: FromBeData>(&self, state: u16, class: u16) -> Option<Entry<T>> {
        let index = state as usize * self.class_count as usize + class as usize;
        let entry_index = *self.states.get(index)? as usize;
        let entry_offset = entry_index * (4 + T::SIZE);
        let mut entry = self
            .data
            .read::<Entry<T>>(self.entry_table as usize + entry_offset)?;
        let new_state =
            (entry.new_state as u32).checked_sub(self.state_array)? / self.class_count as u32;
        entry.new_state = new_state as u16;
        Some(entry)
    }
}

/// Entry in a state table.
#[derive(Copy, Clone)]
pub struct Entry<T> {
    pub new_state: u16,
    pub flags: u16,
    pub data: T,
}

impl<T: FromBeData> FromBeData for Entry<T> {
    const SIZE: usize = 4 + T::SIZE;

    unsafe fn from_be_data_unchecked(buf: &[u8], offset: usize) -> Self {
        let new_state = u16::from_be_data_unchecked(buf, offset);
        let flags = u16::from_be_data_unchecked(buf, offset + 2);
        let data = T::from_be_data_unchecked(buf, offset + 4);
        Self {
            new_state,
            flags,
            data,
        }
    }
}

/// Extended glyph metamorphosis table.
pub mod morx {
    use super::*;

    /// Returns an iterator over the chains in a `morx` table at the
    /// specified offset.
    pub fn chains<'a>(data: &'a [u8], offset: u32) -> Chains<'a> {
        let data = Bytes::with_offset(data, offset as usize).unwrap_or_else(|| Bytes::new(&[]));
        let len = data.read_u32(4).unwrap_or(0);
        Chains {
            data,
            offset: 8,
            len,
            cur: 0,
        }
    }

    /// Iterator over the chains in a metamorphosis table.
    #[derive(Copy, Clone)]
    pub struct Chains<'a> {
        data: Bytes<'a>,
        offset: usize,
        len: u32,
        cur: u32,
    }

    impl<'a> Iterator for Chains<'a> {
        type Item = Chain<'a>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.cur >= self.len {
                return None;
            }
            self.cur += 1;
            let offset = self.offset;
            let len = self.data.read_u32(offset + 4)? as usize;
            self.offset += len;
            let default_flags = self.data.read_u32(offset)?;
            let feature_count = self.data.read_u32(offset + 8)?;
            let subtable_count = self.data.read_u32(offset + 12)?;
            Some(Chain {
                data: self.data,
                offset,
                default_flags,
                feature_count,
                subtable_count,
            })
        }
    }

    /// Chain of subtables in a metamorphosis table.
    #[derive(Copy, Clone)]
    pub struct Chain<'a> {
        data: Bytes<'a>,
        offset: usize,
        default_flags: u32,
        feature_count: u32,
        subtable_count: u32,
    }

    impl<'a> Chain<'a> {
        /// Returns the default flags bitmask for the chain.
        pub fn default_flags(&self) -> u32 {
            self.default_flags
        }

        /// Returns an iterator over the features in the chain.
        pub fn features(&self) -> Features<'a> {
            Features {
                inner: *self,
                cur: 0,
            }
        }

        /// Returns an iterator over the subtables in the chain.
        pub fn subtables(&self) -> Subtables<'a> {
            let offset = self.offset + 16 + 12 * self.feature_count as usize;
            Subtables {
                inner: *self,
                offset,
                len: self.subtable_count,
                cur: 0,
            }
        }
    }

    /// Iterator over the features in a chain.
    #[derive(Copy, Clone)]
    pub struct Features<'a> {
        inner: Chain<'a>,
        cur: u32,
    }

    impl<'a> Iterator for Features<'a> {
        type Item = Feature;

        fn next(&mut self) -> Option<Self::Item> {
            if self.cur >= self.inner.feature_count {
                return None;
            }
            let index = self.cur;
            self.cur += 1;
            let offset = self.inner.offset + 16 + index as usize * 12;
            let b = &self.inner.data;
            Some(Feature {
                selector: b.read::<u16>(offset)?,
                setting_selector: b.read::<u16>(offset + 2)?,
                enable_flags: b.read::<u32>(offset + 4)?,
                disable_flags: b.read::<u32>(offset + 8)?,
            })
        }
    }

    /// Feature descriptor for a chain.
    #[derive(Copy, Clone)]
    pub struct Feature {
        /// The feature selector.
        pub selector: u16,
        /// The feature setting selector.
        pub setting_selector: u16,
        /// Flags to apply if the feature is enabled.
        pub enable_flags: u32,
        /// Flags to apply if the feature is disabled.
        pub disable_flags: u32,
    }

    /// Iterator over the subtables in a chain.
    #[derive(Copy, Clone)]
    pub struct Subtables<'a> {
        inner: Chain<'a>,
        offset: usize,
        len: u32,
        cur: u32,
    }

    impl<'a> Iterator for Subtables<'a> {
        type Item = Subtable<'a>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.cur >= self.len {
                return None;
            }
            self.cur += 1;
            let offset = self.offset;
            let b = self.inner.data;
            self.offset += b.read_u32(offset)? as usize;
            let coverage = b.read_u32(offset + 4)?;
            let kind = coverage & 0xFF;
            let flags = b.read_u32(offset + 8)?;
            Some(Subtable {
                data: b,
                offset,
                kind,
                coverage,
                flags,
            })
        }
    }

    /// Defines the expected order of the glyph stream for a subtable.
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum Order {
        Layout,
        ReverseLayout,
        Logical,
        ReverseLogical,
    }

    /// Subtable in a chain.
    #[derive(Copy, Clone)]
    pub struct Subtable<'a> {
        data: Bytes<'a>,
        offset: usize,
        kind: u32,
        coverage: u32,
        flags: u32,
    }

    impl<'a> Subtable<'a> {
        /// Returns the raw coverage of the subtable.
        pub fn coverage(&self) -> u32 {
            self.coverage
        }

        /// Returns the feature flags of the subtable.
        pub fn flags(&self) -> u32 {
            self.flags
        }

        /// Returns the expected order of the glyph stream for the subtable.
        pub fn order(&self) -> Order {
            let order_bits = (self.coverage >> 28) & 0b101;
            match order_bits {
                0b000 => Order::Layout,
                0b100 => Order::ReverseLayout,
                0b001 => Order::Logical,
                0b101 => Order::ReverseLogical,
                _ => Order::Layout,
            }
        }

        /// Returns true if processing should be reversed based on the
        /// requirements of the subtable and the `is_rtl` parameter which
        /// specifies the current order of the glyph buffer.
        pub fn should_reverse(&self, is_rtl: bool) -> bool {
            let order_bits = (self.coverage >> 28) & 0b101;
            match order_bits {
                0b000 => is_rtl,
                0b100 => !is_rtl,
                0b001 => false,
                0b101 => true,
                _ => false,
            }
        }

        /// Returns the kind of the subtable.
        pub fn kind(&self) -> Option<SubtableKind<'a>> {
            let data = Bytes::with_offset(self.data.data(), self.offset + 12)?;
            Some(match self.kind {
                0 => SubtableKind::Rearrangement(Rearrangement::new(data)?),
                1 => SubtableKind::Contextual(Contextual::new(data)?),
                2 => SubtableKind::Ligature(Ligature::new(data)?),
                4 => SubtableKind::NonContextual(NonContextual::new(data)),
                5 => SubtableKind::Insertion(Insertion::new(data)?),
                _ => return None,
            })
        }
    }

    /// Typed subtable in a chain.
    #[derive(Copy, Clone)]
    pub enum SubtableKind<'a> {
        Rearrangement(Rearrangement<'a>),
        Contextual(Contextual<'a>),
        Ligature(Ligature<'a>),
        NonContextual(NonContextual<'a>),
        Insertion(Insertion<'a>),
    }

    /// Rearrangement subtable.
    #[derive(Copy, Clone)]
    pub struct Rearrangement<'a> {
        state_table: ExtendedStateTable<'a>,
    }

    impl<'a> Rearrangement<'a> {
        fn new(data: Bytes<'a>) -> Option<Self> {
            Some(Self {
                state_table: ExtendedStateTable::new(&data)?,
            })
        }

        /// Processes the next glyph. Returns the number of glyphs to advance by
        /// for the next iteration.
        pub fn next(
            &self,
            state: &mut RearrangementState,
            index: usize,
            glyph_id: u16,
            end_of_text: bool,
            mut f: impl FnMut(&Rearrange) -> Option<()>,
        ) -> Option<usize> {
            const MARK_FIRST: u16 = 0x8000;
            const DONT_ADVANCE: u16 = 0x4000;
            const MARK_LAST: u16 = 0x2000;
            let class = if end_of_text {
                0
            } else {
                self.state_table.class(glyph_id)
            };
            let entry = self.state_table.entry::<()>(state.state, class)?;
            state.state = entry.new_state;
            if entry.flags & MARK_FIRST != 0 {
                state.first = index;
            }
            if entry.flags & MARK_LAST != 0 {
                state.last = index;
            }
            let verb = entry.flags & 0xF;
            let start = state.first;
            let end = state.last;
            if verb != 0 && start <= end {
                let m = REARRANGEMENT_MAP[verb as usize & 0xF];
                let l = 2.min(m >> 4) as usize;
                let r = 2.min(m & 0x0F) as usize;
                let reverse_l = 3 == (m >> 4);
                let reverse_r = 3 == (m & 0x0F);
                let rearrange = Rearrange {
                    l,
                    r,
                    reverse_l,
                    reverse_r,
                    start,
                    end,
                };
                f(&rearrange)?;
            }
            let mut advance = entry.flags & DONT_ADVANCE == 0;
            if advance {
                state.cycles = 0;
            } else if state.cycles > MAX_CYCLES {
                state.cycles = 0;
                advance = true;
            } else {
                state.cycles += 1;
            }
            Some(advance as usize)
        }
    }

    const REARRANGEMENT_MAP: [u8; 16] = [
        0x00, // 0  no change
        0x10, // 1  Ax => xA
        0x01, // 2  xD => Dx
        0x11, // 3  AxD => DxA
        0x20, // 4  ABx => xAB
        0x30, // 5  ABx => xBA
        0x02, // 6  xCD => CDx
        0x03, // 7  xCD => DCx
        0x12, // 8  AxCD => CDxA
        0x13, // 9  AxCD => DCxA
        0x21, // 10 ABxD => DxAB
        0x31, // 11 ABxD => DxBA
        0x22, // 12 ABxCD => CDxAB
        0x32, // 13 ABxCD => CDxBA
        0x23, // 14 ABxCD => DCxAB
        0x33, // 15 ABxCD => DCxBA
    ];

    /// State for rearrangement subtable processing.
    #[derive(Copy, Clone, Default)]
    pub struct RearrangementState {
        state: u16,
        first: usize,
        last: usize,
        cycles: u16,
    }

    impl RearrangementState {
        /// Creates a new rearrangement state.
        pub fn new() -> Self {
            Self::default()
        }
    }

    /// Defines a rearrangement that can be applied to a buffer.
    #[derive(Copy, Clone)]
    pub struct Rearrange {
        l: usize,
        r: usize,
        reverse_l: bool,
        reverse_r: bool,
        start: usize,
        end: usize,
    }

    impl Rearrange {
        /// Applies this rearrangement to the specified buffer.
        pub fn apply<T: Copy + Default>(&self, buffer: &mut [T]) {
            let l = self.l;
            let r = self.r;
            let reverse_l = self.reverse_l;
            let reverse_r = self.reverse_r;
            let start = self.start;
            let end = (self.end + 1).min(buffer.len());
            let mut tmp = [T::default(); 4];
            if end - start >= l + r {
                tmp[..l].copy_from_slice(&buffer[start..(start + l)]);
                tmp[2..(2 + r)].copy_from_slice(&buffer[(end - r)..end]);
                if l != r {
                    buffer.copy_within((start + l)..(end - r), start + r);
                }
                buffer[start..(r + start)].copy_from_slice(&tmp[2..(r + 2)]);
                buffer[(end - l)..end].copy_from_slice(&tmp[..l]);
                if reverse_l {
                    buffer.swap(end - 1, end - 2);
                }
                if reverse_r {
                    buffer.swap(start, start + 1);
                }
            }
        }
    }

    /// Contextual subtable.
    #[derive(Copy, Clone)]
    pub struct Contextual<'a> {
        data: Bytes<'a>,
        state_table: ExtendedStateTable<'a>,
        table: u32,
    }

    impl<'a> Contextual<'a> {
        fn new(data: Bytes<'a>) -> Option<Self> {
            let table = data.read_u32(16)?;
            Some(Self {
                data,
                state_table: ExtendedStateTable::new(&data)?,
                table,
            })
        }

        /// Processes the next glyph.
        pub fn next(
            &self,
            state: &mut ContextualState,
            index: usize,
            glyph_id: u16,
            end_of_text: bool,
            mut f: impl FnMut(usize, u16) -> Option<()>,
        ) -> Option<()> {
            const SET_MARK: u16 = 0x8000;
            const DONT_ADVANCE: u16 = 0x4000;
            if end_of_text && !state.mark_set {
                return Some(());
            }
            let mut last_glyph_id = glyph_id;
            let mut current_glyph_id = glyph_id;
            let mut class = if end_of_text {
                0
            } else {
                self.state_table.class(glyph_id)
            };
            if index == 0 && !end_of_text {
                state.mark_index = 0;
                state.mark_id = glyph_id;
            }
            let mut cycles = 0;
            loop {
                let entry = self
                    .state_table
                    .entry::<ContextualData>(state.state, class)?;
                state.state = entry.new_state;
                if entry.data.mark_index != 0xFFFF {
                    if let Some(g) = self.lookup(entry.data.mark_index, state.mark_id) {
                        f(state.mark_index, g)?;
                        if state.mark_index == index {
                            last_glyph_id = g;
                            current_glyph_id = g;
                        }
                    }
                }
                if entry.data.current_index != 0xFFFF {
                    if let Some(g) = self.lookup(entry.data.current_index, last_glyph_id) {
                        f(index, g)?;
                        current_glyph_id = g;
                    }
                }
                if entry.flags & SET_MARK != 0 {
                    state.mark_set = true;
                    state.mark_index = index;
                    state.mark_id = current_glyph_id;
                }
                if entry.flags & DONT_ADVANCE == 0 || cycles > MAX_CYCLES {
                    break;
                }
                cycles += 1;
                class = self.state_table.class(current_glyph_id);
                last_glyph_id = current_glyph_id;
            }
            Some(())
        }

        fn lookup(&self, table_index: u16, glyph_id: u16) -> Option<u16> {
            let offset = self
                .data
                .read_u32(self.table as usize + table_index as usize * 4)?
                as usize
                + self.table as usize;
            lookup(&self.data, offset, glyph_id)
        }
    }

    /// State for contextual subtable processing.
    #[derive(Copy, Clone, Default)]
    pub struct ContextualState {
        state: u16,
        mark_set: bool,
        mark_index: usize,
        mark_id: u16,
    }

    impl ContextualState {
        /// Creates a new contextual state.
        pub fn new() -> Self {
            Self::default()
        }
    }

    #[derive(Copy, Clone)]
    struct ContextualData {
        mark_index: u16,
        current_index: u16,
    }

    impl FromBeData for ContextualData {
        unsafe fn from_be_data_unchecked(buf: &[u8], offset: usize) -> Self {
            let mark_index = u16::from_be_data_unchecked(buf, offset);
            let current_index = u16::from_be_data_unchecked(buf, offset + 2);
            Self {
                mark_index,
                current_index,
            }
        }
    }

    /// Ligature subtable.
    #[derive(Copy, Clone)]
    pub struct Ligature<'a> {
        data: Bytes<'a>,
        state_table: ExtendedStateTable<'a>,
        action: u32,
        component: u32,
        ligature: u32,
    }

    impl<'a> Ligature<'a> {
        fn new(data: Bytes<'a>) -> Option<Self> {
            let state_table = ExtendedStateTable::new(&data)?;
            let action = data.read::<u32>(16)?;
            let component = data.read::<u32>(20)?;
            let ligature = data.read::<u32>(24)?;
            Some(Self {
                data,
                state_table,
                action,
                component,
                ligature,
            })
        }

        /// Processes the next glyph.
        pub fn next(
            &self,
            state: &mut LigatureState,
            index: usize,
            glyph_id: u16,
            end_of_text: bool,
            mut f: impl FnMut(usize, u16, &[usize]) -> Option<()>,
        ) -> Option<()> {
            const SET_COMPONENT: u16 = 0x8000;
            const DONT_ADVANCE: u16 = 0x4000;
            const PERFORM_ACTION: u16 = 0x2000;
            const LAST: u32 = 0x80000000;
            const STORE: u32 = 0x40000000;
            let class = if end_of_text {
                0
            } else {
                self.state_table.class(glyph_id)
            };
            let mut cycles = 0;
            loop {
                let entry = self.state_table.entry::<u16>(state.state, class)?;
                state.state = entry.new_state;
                if entry.flags & SET_COMPONENT != 0 {
                    state.push(index, glyph_id)?;
                }
                if entry.flags & PERFORM_ACTION != 0 {
                    let mut action_index = entry.data;
                    let mut ligature_index = 0;
                    let end_pos = state.pos;
                    let mut pos = end_pos;
                    while pos > 0 {
                        pos -= 1;
                        let glyph_index = state.indices[pos];
                        let action = self.action(action_index)?;
                        action_index += 1;
                        let mut offset = action & 0x3FFFFFFF;
                        if offset & 0x20000000 != 0 {
                            offset |= 0xC0000000;
                        }
                        let offset = offset as i32;
                        let component_index = state.glyphs[pos] as i32 + offset;
                        let component = self.component(component_index as u32)?;
                        ligature_index += component as u32;
                        if action & (LAST | STORE) != 0 {
                            let ligature = self.ligature(ligature_index)?;
                            f(glyph_index, ligature, state.indices.get(pos + 1..end_pos)?)?;
                            state.glyphs[pos] = ligature;
                            pos += 1;
                        }
                        if action & LAST != 0 || cycles > (MAX_CYCLES * 2) {
                            break;
                        }
                        cycles += 1;
                    }
                    state.pos = pos;
                }
                if entry.flags & DONT_ADVANCE == 0 || cycles > MAX_CYCLES {
                    break;
                }
                cycles += 1;
            }
            Some(())
        }

        fn action(&self, index: u16) -> Option<u32> {
            self.data
                .read::<u32>(self.action as usize + index as usize * 4)
        }

        fn component(&self, index: u32) -> Option<u16> {
            self.data
                .read::<u16>(self.component as usize + index as usize * 2)
        }

        fn ligature(&self, index: u32) -> Option<u16> {
            self.data
                .read::<u16>(self.ligature as usize + index as usize * 2)
        }
    }

    /// State for processing a ligature subtable.
    #[derive(Copy, Clone)]
    pub struct LigatureState {
        state: u16,
        indices: [usize; 32],
        glyphs: [u16; 32],
        pos: usize,
    }

    impl LigatureState {
        /// Creates a new ligature state.
        pub fn new() -> Self {
            Self {
                state: 0,
                indices: [0; 32],
                glyphs: [0; 32],
                pos: 0,
            }
        }

        fn push(&mut self, index: usize, glyph_id: u16) -> Option<()> {
            *self.indices.get_mut(self.pos)? = index;
            *self.glyphs.get_mut(self.pos)? = glyph_id;
            self.pos += 1;
            Some(())
        }
    }

    /// Non-contextual subtable.
    #[derive(Copy, Clone)]
    pub struct NonContextual<'a> {
        data: Bytes<'a>,
    }

    impl<'a> NonContextual<'a> {
        fn new(data: Bytes<'a>) -> Self {
            Self { data }
        }

        /// Returns a substitution for the specified glyph id.
        pub fn substitute(&self, glyph_id: u16) -> Option<u16> {
            lookup(&self.data, 0, glyph_id)
        }
    }

    /// Insertion subtable.
    #[derive(Copy, Clone)]
    pub struct Insertion<'a> {
        data: Bytes<'a>,
        state_table: ExtendedStateTable<'a>,
        action: usize,
    }

    impl<'a> Insertion<'a> {
        fn new(data: Bytes<'a>) -> Option<Self> {
            let state_table = ExtendedStateTable::new(&data)?;
            let action = data.read_u32(16)? as usize;
            Some(Self {
                data,
                state_table,
                action,
            })
        }

        /// Processes the next glyph. Returns the number of glyphs to advance by
        /// for the next iteration.
        pub fn next(
            &self,
            state: &mut InsertionState,
            index: usize,
            glyph_id: u16,
            end_of_text: bool,
            mut f: impl FnMut(usize, Array<'a, u16>) -> Option<()>,
        ) -> Option<usize> {
            const SET_MARK: u16 = 0x8000;
            const DONT_ADVANCE: u16 = 0x4000;
            const _CURRENT_IS_KASHIDA_LIKE: u16 = 0x2000;
            const _MARKED_IS_KASHIDA_LIKE: u16 = 0x1000;
            const CURRENT_INSERT_BEFORE: u16 = 0x800;
            const MARKED_INSERT_BEFORE: u16 = 0x400;
            let class = if end_of_text {
                0
            } else {
                self.state_table.class(glyph_id)
            };
            let entry = self
                .state_table
                .entry::<InsertionData>(state.state, class)?;
            state.state = entry.new_state;
            let mut working_index = index;
            let mut mark_inserted = 0;
            if entry.data.mark_index != 0xFFFF {
                let before = entry.flags & MARKED_INSERT_BEFORE != 0;
                let base = if before { state.mark } else { state.mark + 1 };
                let glyphs = self.marked_glyphs(entry.flags, entry.data.mark_index)?;
                mark_inserted = glyphs.len();
                working_index += mark_inserted;
                f(base, glyphs)?;
            }
            if entry.flags & SET_MARK != 0 {
                state.mark = index;
            }
            let mut current_inserted = 0;
            if entry.data.current_index != 0xFFFF {
                let before = entry.flags & CURRENT_INSERT_BEFORE != 0;
                let base = if before {
                    working_index
                } else {
                    working_index + 1
                };
                let glyphs = self.current_glyphs(entry.flags, entry.data.current_index)?;
                current_inserted = glyphs.len();
                f(base, glyphs)?;
            }
            let mut advance = entry.flags & DONT_ADVANCE == 0;
            if advance {
                state.cycles = 0;
            } else if state.cycles > MAX_CYCLES {
                state.cycles = 0;
                advance = true;
            } else {
                state.cycles += 1;
            }
            if advance {
                Some(mark_inserted + current_inserted + 1)
            } else {
                Some(mark_inserted)
            }
        }

        fn marked_glyphs(&self, flags: u16, index: u16) -> Option<Array<'a, u16>> {
            const MARKED_INSERT_COUNT: u16 = 0x1F;
            let len = (flags & MARKED_INSERT_COUNT) as usize;
            let offset = self.action + index as usize * 2;
            self.data.read_array::<u16>(offset, len)
        }

        fn current_glyphs(&self, flags: u16, index: u16) -> Option<Array<'a, u16>> {
            const CURRENT_INSERT_COUNT: u16 = 0x3E0;
            let len = (flags & CURRENT_INSERT_COUNT) as usize >> 5;
            let offset = self.action + index as usize * 2;
            self.data.read_array::<u16>(offset, len)
        }
    }

    #[derive(Copy, Clone)]
    struct InsertionData {
        current_index: u16,
        mark_index: u16,
    }

    impl FromBeData for InsertionData {
        unsafe fn from_be_data_unchecked(buf: &[u8], offset: usize) -> Self {
            let current_index = u16::from_be_data_unchecked(buf, offset);
            let mark_index = u16::from_be_data_unchecked(buf, offset + 2);
            Self {
                current_index,
                mark_index,
            }
        }
    }

    /// State for processing an insertion subtable.
    #[derive(Copy, Clone, Default)]
    pub struct InsertionState {
        state: u16,
        mark: usize,
        cycles: u16,
    }

    impl InsertionState {
        /// Creates a new insertion state.
        pub fn new() -> Self {
            Self::default()
        }
    }

    /// Returns the `ltag` index for the specified language.
    pub fn language_index(data: &[u8], ltag: u32, language: &str) -> Option<u32> {
        let name = language.as_bytes();
        let b = Bytes::with_offset(data, ltag as usize)?;
        let count = b.read_u32(8)?;
        let mut s = b.stream_at(12)?;
        for i in 0..count {
            let offset = s.read_u16()? as usize;
            let len = s.read_u16()? as usize;
            let bytes = b.read_bytes(offset, len)?;
            if bytes == name {
                return Some(i);
            }
        }
        None
    }

    /// Returns the corresponding AAT feature and on/off selectors for the
    /// specified OpenType feature tag.
    pub fn feature_from_tag(tag: RawTag) -> Option<(u16, [u16; 2])> {
        Some(match AT_TO_AAT.binary_search_by(|e| e.0.cmp(&tag)) {
            Ok(index) => {
                let (_, feature, on, off) = AT_TO_AAT[index];
                (feature as u16, [on as u16, off as u16])
            }
            _ => return None,
        })
    }

    const AT_TO_AAT: [(u32, u8, u8, u8); 77] = [
        (raw_tag(b"afrc"), 11, 1, 0),
        (raw_tag(b"c2pc"), 38, 2, 0),
        (raw_tag(b"c2sc"), 38, 1, 0),
        (raw_tag(b"calt"), 36, 0, 1),
        (raw_tag(b"case"), 33, 0, 1),
        (raw_tag(b"clig"), 1, 18, 19),
        (raw_tag(b"cpsp"), 33, 2, 3),
        (raw_tag(b"cswh"), 36, 4, 5),
        (raw_tag(b"dlig"), 1, 4, 5),
        (raw_tag(b"expt"), 20, 10, 16),
        (raw_tag(b"frac"), 11, 2, 0),
        (raw_tag(b"fwid"), 22, 1, 7),
        (raw_tag(b"halt"), 22, 6, 7),
        (raw_tag(b"hist"), 1, 20, 21),
        (raw_tag(b"hkna"), 34, 0, 1),
        (raw_tag(b"hlig"), 1, 20, 21),
        (raw_tag(b"hngl"), 23, 1, 0),
        (raw_tag(b"hojo"), 20, 12, 16),
        (raw_tag(b"hwid"), 22, 2, 7),
        (raw_tag(b"ital"), 32, 2, 3),
        (raw_tag(b"jp04"), 20, 11, 16),
        (raw_tag(b"jp78"), 20, 2, 16),
        (raw_tag(b"jp83"), 20, 3, 16),
        (raw_tag(b"jp90"), 20, 4, 16),
        (raw_tag(b"liga"), 1, 2, 3),
        (raw_tag(b"lnum"), 21, 1, 2),
        (raw_tag(b"mgrk"), 15, 10, 11),
        (raw_tag(b"nlck"), 20, 13, 16),
        (raw_tag(b"onum"), 21, 0, 2),
        (raw_tag(b"ordn"), 10, 3, 0),
        (raw_tag(b"palt"), 22, 5, 7),
        (raw_tag(b"pcap"), 37, 2, 0),
        (raw_tag(b"pkna"), 22, 0, 7),
        (raw_tag(b"pnum"), 6, 1, 4),
        (raw_tag(b"pwid"), 22, 0, 7),
        (raw_tag(b"qwid"), 22, 4, 7),
        (raw_tag(b"rlig"), 1, 0, 1),
        (raw_tag(b"ruby"), 28, 2, 3),
        (raw_tag(b"sinf"), 10, 4, 0),
        (raw_tag(b"smcp"), 37, 1, 0),
        (raw_tag(b"smpl"), 20, 1, 16),
        (raw_tag(b"ss01"), 35, 2, 3),
        (raw_tag(b"ss02"), 35, 4, 5),
        (raw_tag(b"ss03"), 35, 6, 7),
        (raw_tag(b"ss04"), 35, 8, 9),
        (raw_tag(b"ss05"), 35, 10, 11),
        (raw_tag(b"ss06"), 35, 12, 13),
        (raw_tag(b"ss07"), 35, 14, 15),
        (raw_tag(b"ss08"), 35, 16, 17),
        (raw_tag(b"ss09"), 35, 18, 19),
        (raw_tag(b"ss10"), 35, 20, 21),
        (raw_tag(b"ss11"), 35, 22, 23),
        (raw_tag(b"ss12"), 35, 24, 25),
        (raw_tag(b"ss13"), 35, 26, 27),
        (raw_tag(b"ss14"), 35, 28, 29),
        (raw_tag(b"ss15"), 35, 30, 31),
        (raw_tag(b"ss16"), 35, 32, 33),
        (raw_tag(b"ss17"), 35, 34, 35),
        (raw_tag(b"ss18"), 35, 36, 37),
        (raw_tag(b"ss19"), 35, 38, 39),
        (raw_tag(b"ss20"), 35, 40, 41),
        (raw_tag(b"subs"), 10, 2, 0),
        (raw_tag(b"sups"), 10, 1, 0),
        (raw_tag(b"swsh"), 36, 2, 3),
        (raw_tag(b"titl"), 19, 4, 0),
        (raw_tag(b"tnam"), 20, 14, 16),
        (raw_tag(b"tnum"), 6, 0, 4),
        (raw_tag(b"trad"), 20, 0, 16),
        (raw_tag(b"twid"), 22, 3, 7),
        (raw_tag(b"unic"), 3, 14, 15),
        (raw_tag(b"valt"), 22, 5, 7),
        (raw_tag(b"vert"), 4, 0, 1),
        (raw_tag(b"vhal"), 22, 6, 7),
        (raw_tag(b"vkna"), 34, 2, 3),
        (raw_tag(b"vpal"), 22, 5, 7),
        (raw_tag(b"vrt2"), 4, 0, 1),
        (raw_tag(b"zero"), 14, 4, 5),
    ];
}

/// Extended kerning table.
pub mod kerx {
    use super::*;

    /// Returns an iterator over the subtables for the extended kerning
    /// table.
    pub fn subtables<'a>(data: &'a [u8], kerx: u32, ankr: u32) -> Subtables<'a> {
        let b = if kerx == 0 {
            Bytes::new(&[])
        } else {
            Bytes::with_offset(data, kerx as usize).unwrap_or_else(|| Bytes::new(&[]))
        };
        let ankr = if ankr == 0 {
            &[]
        } else {
            data.get(ankr as usize..).unwrap_or(&[])
        };
        let version = b.read_u16(0).unwrap_or(0);
        let len = b.read_u32(4).unwrap_or(0);
        Subtables {
            data: b,
            version,
            offset: 8,
            len,
            cur: 0,
            ankr,
        }
    }

    /// Iterator over the subtables of an extended kerning table.
    #[derive(Copy, Clone)]
    pub struct Subtables<'a> {
        data: Bytes<'a>,
        version: u16,
        offset: usize,
        len: u32,
        cur: u32,
        ankr: &'a [u8],
    }

    impl<'a> Iterator for Subtables<'a> {
        type Item = Subtable<'a>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.cur >= self.len {
                return None;
            }
            self.cur += 1;
            let offset = self.offset;
            let subtable = Subtable::new(&self.data, offset, self.version, self.ankr)?;
            self.offset = self.offset.checked_add(subtable.size as usize)?;
            Some(subtable)
        }
    }

    /// Extended kerning subtable.
    #[derive(Copy, Clone)]
    #[allow(dead_code)]
    pub struct Subtable<'a> {
        data: Bytes<'a>,
        version: u16,
        size: u32,
        coverage: u32,
        tuple_count: u32,
        ankr: &'a [u8],
    }

    impl<'a> Subtable<'a> {
        fn new(data: &Bytes<'a>, offset: usize, version: u16, ankr: &'a [u8]) -> Option<Self> {
            let data = Bytes::with_offset(data.data(), offset)?;
            let size = data.read_u32(0)?;
            let coverage = data.read_u32(4)?;
            let tuple_count = if version >= 4 { data.read_u32(8)? } else { 0 };
            Some(Self {
                data,
                version,
                size,
                coverage,
                tuple_count,
                ankr,
            })
        }

        pub fn is_vertical(&self) -> bool {
            self.coverage & 0x80000000 != 0
        }

        pub fn is_cross_stream(&self) -> bool {
            self.coverage & 0x40000000 != 0
        }

        pub fn has_variations(&self) -> bool {
            self.coverage & 0x20000000 != 0
        }

        pub fn should_reverse(&self, is_rtl: bool) -> bool {
            if self.coverage & 0x10000000 != 0 {
                !is_rtl
            } else {
                is_rtl
            }
        }

        pub fn kind(&self) -> Option<SubtableKind<'a>> {
            let format = self.coverage & 0xFF;
            Some(match format {
                0 => SubtableKind::Format0(Format0::new(self.data)?),
                1 => SubtableKind::Format1(Format1::new(self.data, self.tuple_count)?),
                2 => SubtableKind::Format2(Format2::new(self.data)?),
                4 => SubtableKind::Format4(Format4::new(self.data, self.ankr)?),
                _ => return None,
            })
        }
    }

    /// Kind of an extended kerning subtable.
    #[derive(Copy, Clone)]
    pub enum SubtableKind<'a> {
        Format0(Format0<'a>),
        Format1(Format1<'a>),
        Format2(Format2<'a>),
        Format4(Format4<'a>),
    }

    /// Order pair kerning subtable.
    #[derive(Copy, Clone)]
    pub struct Format0<'a> {
        data: Bytes<'a>,
        count: usize,
    }

    impl<'a> Format0<'a> {
        fn new(data: Bytes<'a>) -> Option<Self> {
            let count = data.read_u32(12)? as usize;
            Some(Self { data, count })
        }

        /// Returns the kerning adjustment for the specified pair of glyphs.
        pub fn get(&self, left: u16, right: u16) -> Option<i16> {
            let key = (left as u32) << 16 | right as u32;
            let base = 28;
            let reclen = 6;
            let b = &self.data;
            let mut l = 0;
            let mut h = self.count;
            while l < h {
                use core::cmp::Ordering::*;
                let i = (l + h) / 2;
                let pair = b.read::<u32>(base + i * reclen)?;
                match key.cmp(&pair) {
                    Greater => l = i + 1,
                    Less => h = i,
                    Equal => return b.read_i16(base + i * reclen + 4),
                }
            }
            None
        }
    }

    /// Contextual kerning subtable.
    #[derive(Copy, Clone)]
    #[allow(dead_code)]
    pub struct Format1<'a> {
        data: Bytes<'a>,
        state_table: ExtendedStateTable<'a>,
        value: usize,
        tuple_count: u32,
        tuple_size: usize,
    }

    impl<'a> Format1<'a> {
        fn new(data: Bytes<'a>, tuple_count: u32) -> Option<Self> {
            let data = Bytes::with_offset(data.data(), 12)?;
            let state_table = ExtendedStateTable::new(&data)?;
            let value = data.read_u32(16)? as usize;
            let tuple_size = if tuple_count == 0 {
                2
            } else {
                tuple_count as usize * 2
            };
            Some(Self {
                data,
                state_table,
                value,
                tuple_count,
                tuple_size,
            })
        }

        pub fn next(
            &self,
            state: &mut ContextualState,
            index: usize,
            glyph_id: u16,
            mut f: impl FnMut(usize, i16) -> Option<()>,
        ) -> Option<usize> {
            const PUSH: u16 = 0x8000;
            const DONT_ADVANCE: u16 = 0x4000;
            const RESET: u16 = 0x2000;
            let class = self.state_table.class(glyph_id);
            let entry = self.state_table.entry::<u16>(state.state, class)?;
            state.state = entry.new_state;
            if entry.flags & PUSH != 0 {
                if state.pos == state.stack.len() {
                    return None;
                }
                state.stack[state.pos] = index;
                state.pos += 1;
            }
            if entry.data != 0xFFFF {
                let mut offset = self
                    .value
                    .checked_add(entry.data as usize)?
                    .checked_mul(self.tuple_size)?;
                while state.pos > 0 {
                    let value = self.data.read_i16(offset)?;
                    if value as usize == 0xFFFF {
                        break;
                    }
                    let pos = state.pos - 1;
                    state.pos = pos;
                    f(state.stack[pos], value)?;
                    offset = offset.checked_add(self.tuple_size)?;
                }
            }
            if entry.flags & RESET != 0 {
                state.pos = 0;
            }
            let mut advance = entry.flags & DONT_ADVANCE == 0;
            if advance {
                state.cycles = 0;
            } else if state.cycles > MAX_CYCLES {
                state.cycles = 0;
                advance = true;
            } else {
                state.cycles += 1;
            }
            Some(advance as usize)
        }
    }

    /// State for a contextual kerning subtable.
    #[derive(Copy, Clone, Default)]
    pub struct ContextualState {
        state: u16,
        stack: [usize; 8],
        pos: usize,
        cycles: u16,
    }

    impl ContextualState {
        /// Creates a new contextual state.
        pub fn new() -> Self {
            Self::default()
        }
    }

    /// Two dimensional array kerning subtable.
    #[derive(Copy, Clone)]
    pub struct Format2<'a> {
        data: Bytes<'a>,
        l_table: usize,
        r_table: usize,
        array: usize,
    }

    impl<'a> Format2<'a> {
        fn new(data: Bytes<'a>) -> Option<Self> {
            let l_table = data.read_u32(16)? as usize;
            let r_table = data.read_u32(20)? as usize;
            let array = data.read_u32(24)? as usize;
            Some(Self {
                data,
                l_table,
                r_table,
                array,
            })
        }

        /// Returns the kerning adjustment for the specified pair of glyphs.
        pub fn get(&self, left: u16, right: u16) -> Option<i16> {
            let b = &self.data;
            let row = lookup::<u16>(b, self.l_table, left)? as usize;
            let column = lookup::<u16>(b, self.r_table, right)? as usize;
            b.read_i16(self.array + row + column)
        }
    }

    /// Control/anchor point subtable.
    #[derive(Copy, Clone)]
    #[allow(dead_code)]
    pub struct Format4<'a> {
        data: Bytes<'a>,
        state_table: ExtendedStateTable<'a>,
        action_type: u32,
        control_table: usize,
        ankr: &'a [u8],
    }

    impl<'a> Format4<'a> {
        fn new(data: Bytes<'a>, ankr: &'a [u8]) -> Option<Self> {
            let data = Bytes::with_offset(data.data(), 12)?;
            let state_table = ExtendedStateTable::new(&data)?;
            let flags = data.read_u32(16)?;
            let action_type = (flags & 0xC0000000) >> 30;
            let control_table = (flags & 0x00FFFFFF) as usize;
            Some(Self {
                data,
                state_table,
                action_type,
                control_table,
                ankr,
            })
        }

        pub fn next(
            &self,
            state: &mut Format4State,
            index: usize,
            glyph_id: u16,
            mut f: impl FnMut(usize, usize, f32, f32) -> Option<()>,
        ) -> Option<usize> {
            const MARK: u16 = 0x8000;
            const DONT_ADVANCE: u16 = 0x4000;
            let class = self.state_table.class(glyph_id);
            let entry = self.state_table.entry::<u16>(state.state, class)?;
            state.state = entry.new_state;
            if entry.flags & MARK != 0 {
                state.mark = index;
                state.mark_id = glyph_id;
            }
            if entry.data != 0xFFFF {
                let offset = self.control_table.checked_add(entry.data as usize * 2)?;
                match self.action_type {
                    0 => {}
                    1 => {
                        let mark_index = self.data.read_u16(offset)?;
                        let cur_index = self.data.read_u16(offset + 2)?;
                        if let Some((x, y)) =
                            self.anchor_offset(mark_index, state.mark_id, cur_index, glyph_id)
                        {
                            let diff = index - state.mark;
                            if diff < 255 {
                                f(index, diff, x, y);
                            }
                        }
                    }
                    2 => {}
                    _ => {}
                }
            }
            let mut advance = entry.flags & DONT_ADVANCE == 0;
            if advance {
                state.cycles = 0;
            } else if state.cycles > MAX_CYCLES {
                state.cycles = 0;
                advance = true;
            } else {
                state.cycles += 1;
            }
            Some(advance as usize)
        }

        fn anchor_offset(
            &self,
            mark_index: u16,
            mark_id: u16,
            cur_index: u16,
            cur_id: u16,
        ) -> Option<(f32, f32)> {
            let mark_point = anchor_points(self.ankr, mark_id)?.get(mark_index as u32)?;
            let cur_point = anchor_points(self.ankr, cur_id)?.get(cur_index as u32)?;
            Some((
                mark_point.0 as f32 - cur_point.0 as f32,
                mark_point.1 as f32 - cur_point.1 as f32,
            ))
        }
    }

    /// State for a format4 kerning subtable.
    #[derive(Copy, Clone, Default)]
    pub struct Format4State {
        state: u16,
        mark: usize,
        mark_id: u16,
        cycles: u16,
    }

    impl Format4State {
        pub fn new() -> Self {
            Self::default()
        }
    }

    /// Returns the set of anchor points for the specified glyph.
    pub fn anchor_points<'a>(data: &'a [u8], glyph_id: u16) -> Option<AnchorPoints<'a>> {
        if data.is_empty() {
            return None;
        }
        let b = Bytes::new(data);
        let lookup_offset = b.read_u32(4)? as usize;
        let base = b.read_u32(8)? as usize;
        let offset = lookup::<u16>(&b, lookup_offset, glyph_id)? as usize + base;
        Some(AnchorPoints {
            data: b,
            offset,
            len: b.read_u32(offset)?,
        })
    }

    /// Set of anchor points for a glyph.
    #[derive(Copy, Clone)]
    pub struct AnchorPoints<'a> {
        data: Bytes<'a>,
        offset: usize,
        len: u32,
    }

    impl<'a> AnchorPoints<'a> {
        /// Returns the number of anchor points.
        pub fn len(&self) -> u32 {
            self.len
        }
        /// Returns the x and y coordinates of the anchor point at the specified
        /// index.
        pub fn get(&self, index: u32) -> Option<(i16, i16)> {
            let offset = self.offset + 4 + index as usize * 4;
            let x = self.data.read::<i16>(offset)?;
            let y = self.data.read::<i16>(offset + 2)?;
            Some((x, y))
        }
    }
}

/// Kerning table.
pub mod kern {
    use super::*;

    pub fn subtables<'a>(data: &'a [u8], kern: u32) -> Subtables<'a> {
        let b = if kern == 0 {
            Bytes::new(&[])
        } else {
            Bytes::with_offset(data, kern as usize).unwrap_or_else(|| Bytes::new(&[]))
        };
        let version = b.read_or_default::<u16>(0);
        if version == 0 {
            let len = b.read_or_default::<u16>(2) as u32;
            Subtables {
                data: b,
                offset: 4,
                len,
                cur: 0,
                is_aat: false,
            }
        } else {
            let len = b.read_or_default::<u32>(4);
            Subtables {
                data: b,
                offset: 8,
                len,
                cur: 0,
                is_aat: true,
            }
        }
    }

    #[derive(Copy, Clone)]
    pub struct Subtables<'a> {
        data: Bytes<'a>,
        offset: usize,
        len: u32,
        cur: u32,
        is_aat: bool,
    }

    impl<'a> Iterator for Subtables<'a> {
        type Item = Subtable<'a>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.cur >= self.len {
                return None;
            }
            self.cur += 1;
            let offset = self.offset;
            let subtable = Subtable::new(&self.data, offset, self.is_aat)?;
            self.offset = self.offset.checked_add(subtable.size as usize)?;
            Some(subtable)
        }
    }

    /// Extended kerning subtable.
    #[derive(Copy, Clone)]
    pub struct Subtable<'a> {
        data: Bytes<'a>,
        offset: usize,
        size: u32,
        coverage: u16,
        is_aat: bool,
        is_horizontal: bool,
        cross_stream: bool,
        format: u8,
    }

    impl<'a> Subtable<'a> {
        fn new(data: &Bytes<'a>, mut offset: usize, is_aat: bool) -> Option<Self> {
            let data = Bytes::with_offset(data.data(), offset)?;
            let size = if is_aat {
                offset = 8;
                data.read_u32(0)?
            } else {
                offset = 6;
                data.read_u16(2)? as u32
            };
            let coverage = data.read_u16(4)?;
            let (is_horizontal, cross_stream, format) = if is_aat {
                let format = (coverage & 0xFF) as u8;
                let is_vertical = coverage & 0x8000 != 0;
                let cross_stream = coverage & 0x4000 != 0;
                (!is_vertical, cross_stream, format)
            } else {
                let format = (coverage & 0xFF00) >> 8;
                let coverage = coverage & 0xFF;
                let is_horizontal = coverage & (1 << 0) != 0;
                let cross_stream = coverage & (1 << 2) != 0;
                (is_horizontal, cross_stream, format as u8)
            };
            Some(Self {
                data,
                offset,
                size,
                coverage,
                is_aat,
                is_horizontal,
                cross_stream,
                format,
            })
        }

        pub fn is_horizontal(&self) -> bool {
            self.is_horizontal
        }

        pub fn cross_stream(&self) -> bool {
            self.cross_stream
        }

        pub fn kind(&self) -> Option<SubtableKind<'a>> {
            Some(match self.format {
                0 => SubtableKind::Format0(Format0::new(self)?),
                1 => SubtableKind::Format1(Format1::new(self)?),
                _ => return None,
            })
        }
    }

    #[derive(Copy, Clone)]
    pub enum SubtableKind<'a> {
        Format0(Format0<'a>),
        Format1(Format1<'a>),
    }

    #[derive(Copy, Clone)]
    pub struct Format0<'a> {
        data: Bytes<'a>,
        offset: usize,
        count: usize,
    }

    impl<'a> Format0<'a> {
        fn new(subtable: &Subtable<'a>) -> Option<Self> {
            let count = subtable.data.read_u16(subtable.offset)? as usize;
            Some(Self {
                data: subtable.data,
                offset: subtable.offset + 8,
                count,
            })
        }

        /// Returns the kerning adjustment for the specified pair of glyphs.
        pub fn get(&self, left: u16, right: u16) -> Option<i16> {
            let key = (left as u32) << 16 | right as u32;
            let base = self.offset;
            let reclen = 6;
            let b = &self.data;
            let mut l = 0;
            let mut h = self.count;
            while l < h {
                use core::cmp::Ordering::*;
                let i = (l + h) / 2;
                let pair = b.read::<u32>(base + i * reclen)?;
                match key.cmp(&pair) {
                    Greater => l = i + 1,
                    Less => h = i,
                    Equal => return b.read_i16(base + i * reclen + 4),
                }
            }
            None
        }
    }

    /// Contextual kerning subtable.
    #[derive(Copy, Clone)]
    #[allow(dead_code)]
    pub struct Format1<'a> {
        data: Bytes<'a>,
        state_table: StateTable<'a>,
        cross_stream: bool,
    }

    impl<'a> Format1<'a> {
        fn new(subtable: &Subtable<'a>) -> Option<Self> {
            let data = Bytes::with_offset(subtable.data.data(), subtable.offset)?;
            let state_table = StateTable::new(&data)?;
            Some(Self {
                data,
                state_table,
                cross_stream: subtable.cross_stream,
            })
        }

        pub fn next(
            &self,
            state: &mut Format1State,
            index: usize,
            glyph_id: u16,
            mut f: impl FnMut(usize, i16) -> Option<()>,
        ) -> Option<usize> {
            const PUSH: u16 = 0x8000;
            const DONT_ADVANCE: u16 = 0x4000;
            let class = self.state_table.class(glyph_id);
            let entry = self.state_table.entry::<()>(state.state, class)?;
            state.state = entry.new_state;
            if entry.flags & PUSH != 0 {
                if state.pos == state.stack.len() {
                    return None;
                }
                state.stack[state.pos] = index;
                state.pos += 1;
            } else if entry.flags == 0 {
                state.pos = 0;
            }
            let mut value_offset = (entry.flags & 0x3FFF) as usize;
            if value_offset != 0 {
                while state.pos > 0 {
                    let mut value = self.data.read_i16(value_offset)?;
                    let mut last = false;
                    if value & 1 != 0 {
                        last = true;
                        value &= !1;
                    }
                    let pos = state.pos - 1;
                    state.pos = pos;
                    if self.cross_stream && value as u16 == 0x8000 {
                        // Reset cross stream?
                    } else {
                        f(state.stack[pos], value)?;
                    }
                    if last {
                        state.pos = 0;
                        break;
                    }
                    value_offset += 2;
                }
            }
            let mut advance = entry.flags & DONT_ADVANCE == 0;
            if advance {
                state.cycles = 0;
            } else if state.cycles > MAX_CYCLES {
                state.cycles = 0;
                advance = true;
            } else {
                state.cycles += 1;
            }
            Some(advance as usize)
        }
    }

    /// State for a contextual kerning subtable.
    #[derive(Copy, Clone, Default)]
    pub struct Format1State {
        state: u16,
        stack: [usize; 8],
        pos: usize,
        cycles: u16,
    }

    impl Format1State {
        /// Creates a new contextual state.
        pub fn new() -> Self {
            Self::default()
        }
    }
}
