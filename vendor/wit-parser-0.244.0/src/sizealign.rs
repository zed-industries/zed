use std::{
    cmp::Ordering,
    num::NonZeroUsize,
    ops::{Add, AddAssign},
};

use crate::{FlagsRepr, Int, Resolve, Type, TypeDef, TypeDefKind};

/// Architecture specific alignment
#[derive(Eq, PartialEq, Clone, Copy)]
pub enum Alignment {
    /// This represents 4 byte alignment on 32bit and 8 byte alignment on 64bit architectures
    Pointer,
    /// This alignment is architecture independent (derived from integer or float types)
    Bytes(NonZeroUsize),
}

impl Default for Alignment {
    fn default() -> Self {
        Alignment::Bytes(NonZeroUsize::new(1).unwrap())
    }
}

impl std::fmt::Debug for Alignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Alignment::Pointer => f.write_str("ptr"),
            Alignment::Bytes(b) => f.write_fmt(format_args!("{}", b.get())),
        }
    }
}

impl PartialOrd for Alignment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Alignment {
    /// Needed for determining the max alignment of an object from its parts.
    /// The ordering is: Bytes(1) < Bytes(2) < Bytes(4) < Pointer < Bytes(8)
    /// as a Pointer is either four or eight byte aligned, depending on the architecture
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Alignment::Pointer, Alignment::Pointer) => std::cmp::Ordering::Equal,
            (Alignment::Pointer, Alignment::Bytes(b)) => {
                if b.get() > 4 {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            }
            (Alignment::Bytes(b), Alignment::Pointer) => {
                if b.get() > 4 {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Less
                }
            }
            (Alignment::Bytes(a), Alignment::Bytes(b)) => a.cmp(b),
        }
    }
}

impl Alignment {
    /// for easy migration this gives you the value for wasm32
    pub fn align_wasm32(&self) -> usize {
        match self {
            Alignment::Pointer => 4,
            Alignment::Bytes(bytes) => bytes.get(),
        }
    }

    pub fn align_wasm64(&self) -> usize {
        match self {
            Alignment::Pointer => 8,
            Alignment::Bytes(bytes) => bytes.get(),
        }
    }

    pub fn format(&self, ptrsize_expr: &str) -> String {
        match self {
            Alignment::Pointer => ptrsize_expr.into(),
            Alignment::Bytes(bytes) => format!("{}", bytes.get()),
        }
    }
}

/// Architecture specific measurement of position,
/// the combined amount in bytes is
/// `bytes + pointers * core::mem::size_of::<*const u8>()`
#[derive(Default, Clone, Copy, Eq, PartialEq)]
pub struct ArchitectureSize {
    /// architecture independent bytes
    pub bytes: usize,
    /// amount of pointer sized units to add
    pub pointers: usize,
}

impl Add<ArchitectureSize> for ArchitectureSize {
    type Output = ArchitectureSize;

    fn add(self, rhs: ArchitectureSize) -> Self::Output {
        ArchitectureSize::new(self.bytes + rhs.bytes, self.pointers + rhs.pointers)
    }
}

impl AddAssign<ArchitectureSize> for ArchitectureSize {
    fn add_assign(&mut self, rhs: ArchitectureSize) {
        self.bytes += rhs.bytes;
        self.pointers += rhs.pointers;
    }
}

impl From<Alignment> for ArchitectureSize {
    fn from(align: Alignment) -> Self {
        match align {
            Alignment::Bytes(bytes) => ArchitectureSize::new(bytes.get(), 0),
            Alignment::Pointer => ArchitectureSize::new(0, 1),
        }
    }
}

impl std::fmt::Debug for ArchitectureSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.format("ptrsz"))
    }
}

impl ArchitectureSize {
    pub fn new(bytes: usize, pointers: usize) -> Self {
        Self { bytes, pointers }
    }

    pub fn max<B: std::borrow::Borrow<Self>>(&self, other: B) -> Self {
        let other = other.borrow();
        let self32 = self.size_wasm32();
        let self64 = self.size_wasm64();
        let other32 = other.size_wasm32();
        let other64 = other.size_wasm64();
        if self32 >= other32 && self64 >= other64 {
            *self
        } else if self32 <= other32 && self64 <= other64 {
            *other
        } else {
            // we can assume a combination of bytes and pointers, so align to at least pointer size
            let new32 = align_to(self32.max(other32), 4);
            let new64 = align_to(self64.max(other64), 8);
            ArchitectureSize::new(new32 + new32 - new64, (new64 - new32) / 4)
        }
    }

    pub fn add_bytes(&self, b: usize) -> Self {
        Self::new(self.bytes + b, self.pointers)
    }

    /// The effective offset/size is
    /// `constant_bytes() + core::mem::size_of::<*const u8>() * pointers_to_add()`
    pub fn constant_bytes(&self) -> usize {
        self.bytes
    }

    pub fn pointers_to_add(&self) -> usize {
        self.pointers
    }

    /// Shortcut for compatibility with previous versions
    pub fn size_wasm32(&self) -> usize {
        self.bytes + self.pointers * 4
    }

    pub fn size_wasm64(&self) -> usize {
        self.bytes + self.pointers * 8
    }

    /// prefer this over >0
    pub fn is_empty(&self) -> bool {
        self.bytes == 0 && self.pointers == 0
    }

    // create a suitable expression in bytes from a pointer size argument
    pub fn format(&self, ptrsize_expr: &str) -> String {
        self.format_term(ptrsize_expr, false)
    }

    // create a suitable expression in bytes from a pointer size argument,
    // extended API with optional brackets around the sum
    pub fn format_term(&self, ptrsize_expr: &str, suppress_brackets: bool) -> String {
        if self.pointers != 0 {
            if self.bytes > 0 {
                // both
                if suppress_brackets {
                    format!(
                        "{}+{}*{ptrsize_expr}",
                        self.constant_bytes(),
                        self.pointers_to_add()
                    )
                } else {
                    format!(
                        "({}+{}*{ptrsize_expr})",
                        self.constant_bytes(),
                        self.pointers_to_add()
                    )
                }
            } else if self.pointers == 1 {
                // one pointer
                ptrsize_expr.into()
            } else {
                // only pointer
                if suppress_brackets {
                    format!("{}*{ptrsize_expr}", self.pointers_to_add())
                } else {
                    format!("({}*{ptrsize_expr})", self.pointers_to_add())
                }
            }
        } else {
            // only bytes
            format!("{}", self.constant_bytes())
        }
    }
}

/// Information per structure element
#[derive(Default)]
pub struct ElementInfo {
    pub size: ArchitectureSize,
    pub align: Alignment,
}

impl From<Alignment> for ElementInfo {
    fn from(align: Alignment) -> Self {
        ElementInfo {
            size: align.into(),
            align,
        }
    }
}

impl ElementInfo {
    fn new(size: ArchitectureSize, align: Alignment) -> Self {
        Self { size, align }
    }
}

/// Collect size and alignment for sub-elements of a structure
#[derive(Default)]
pub struct SizeAlign {
    map: Vec<ElementInfo>,
}

impl SizeAlign {
    pub fn fill(&mut self, resolve: &Resolve) {
        self.map = Vec::new();
        for (_, ty) in resolve.types.iter() {
            let pair = self.calculate(ty);
            self.map.push(pair);
        }
    }

    fn calculate(&self, ty: &TypeDef) -> ElementInfo {
        match &ty.kind {
            TypeDefKind::Type(t) => ElementInfo::new(self.size(t), self.align(t)),
            TypeDefKind::FixedSizeList(t, size) => {
                let field_align = self.align(t);
                let field_size = self.size(t);
                ElementInfo::new(
                    ArchitectureSize::new(
                        field_size.bytes.checked_mul(*size as usize).unwrap(),
                        field_size.pointers.checked_mul(*size as usize).unwrap(),
                    ),
                    field_align,
                )
            }
            TypeDefKind::List(_) => {
                ElementInfo::new(ArchitectureSize::new(0, 2), Alignment::Pointer)
            }
            TypeDefKind::Map(_, _) => {
                ElementInfo::new(ArchitectureSize::new(0, 2), Alignment::Pointer)
            }
            TypeDefKind::Record(r) => self.record(r.fields.iter().map(|f| &f.ty)),
            TypeDefKind::Tuple(t) => self.record(t.types.iter()),
            TypeDefKind::Flags(f) => match f.repr() {
                FlagsRepr::U8 => int_size_align(Int::U8),
                FlagsRepr::U16 => int_size_align(Int::U16),
                FlagsRepr::U32(n) => ElementInfo::new(
                    ArchitectureSize::new(n * 4, 0),
                    Alignment::Bytes(NonZeroUsize::new(4).unwrap()),
                ),
            },
            TypeDefKind::Variant(v) => self.variant(v.tag(), v.cases.iter().map(|c| c.ty.as_ref())),
            TypeDefKind::Enum(e) => self.variant(e.tag(), []),
            TypeDefKind::Option(t) => self.variant(Int::U8, [Some(t)]),
            TypeDefKind::Result(r) => self.variant(Int::U8, [r.ok.as_ref(), r.err.as_ref()]),
            // A resource is represented as an index.
            // A future is represented as an index.
            // A stream is represented as an index.
            // An error is represented as an index.
            TypeDefKind::Handle(_) | TypeDefKind::Future(_) | TypeDefKind::Stream(_) => {
                int_size_align(Int::U32)
            }
            // This shouldn't be used for anything since raw resources aren't part of the ABI -- just handles to
            // them.
            TypeDefKind::Resource => ElementInfo::new(
                ArchitectureSize::new(usize::MAX, 0),
                Alignment::Bytes(NonZeroUsize::new(usize::MAX).unwrap()),
            ),
            TypeDefKind::Unknown => unreachable!(),
        }
    }

    pub fn size(&self, ty: &Type) -> ArchitectureSize {
        match ty {
            Type::Bool | Type::U8 | Type::S8 => ArchitectureSize::new(1, 0),
            Type::U16 | Type::S16 => ArchitectureSize::new(2, 0),
            Type::U32 | Type::S32 | Type::F32 | Type::Char | Type::ErrorContext => {
                ArchitectureSize::new(4, 0)
            }
            Type::U64 | Type::S64 | Type::F64 => ArchitectureSize::new(8, 0),
            Type::String => ArchitectureSize::new(0, 2),
            Type::Id(id) => self.map[id.index()].size,
        }
    }

    pub fn align(&self, ty: &Type) -> Alignment {
        match ty {
            Type::Bool | Type::U8 | Type::S8 => Alignment::Bytes(NonZeroUsize::new(1).unwrap()),
            Type::U16 | Type::S16 => Alignment::Bytes(NonZeroUsize::new(2).unwrap()),
            Type::U32 | Type::S32 | Type::F32 | Type::Char | Type::ErrorContext => {
                Alignment::Bytes(NonZeroUsize::new(4).unwrap())
            }
            Type::U64 | Type::S64 | Type::F64 => Alignment::Bytes(NonZeroUsize::new(8).unwrap()),
            Type::String => Alignment::Pointer,
            Type::Id(id) => self.map[id.index()].align,
        }
    }

    pub fn field_offsets<'a>(
        &self,
        types: impl IntoIterator<Item = &'a Type>,
    ) -> Vec<(ArchitectureSize, &'a Type)> {
        let mut cur = ArchitectureSize::default();
        types
            .into_iter()
            .map(|ty| {
                let ret = align_to_arch(cur, self.align(ty));
                cur = ret + self.size(ty);
                (ret, ty)
            })
            .collect()
    }

    pub fn payload_offset<'a>(
        &self,
        tag: Int,
        cases: impl IntoIterator<Item = Option<&'a Type>>,
    ) -> ArchitectureSize {
        let mut max_align = Alignment::default();
        for ty in cases {
            if let Some(ty) = ty {
                max_align = max_align.max(self.align(ty));
            }
        }
        let tag_size = int_size_align(tag).size;
        align_to_arch(tag_size, max_align)
    }

    pub fn record<'a>(&self, types: impl IntoIterator<Item = &'a Type>) -> ElementInfo {
        let mut size = ArchitectureSize::default();
        let mut align = Alignment::default();
        for ty in types {
            let field_size = self.size(ty);
            let field_align = self.align(ty);
            size = align_to_arch(size, field_align) + field_size;
            align = align.max(field_align);
        }
        ElementInfo::new(align_to_arch(size, align), align)
    }

    pub fn params<'a>(&self, types: impl IntoIterator<Item = &'a Type>) -> ElementInfo {
        self.record(types.into_iter())
    }

    fn variant<'a>(
        &self,
        tag: Int,
        types: impl IntoIterator<Item = Option<&'a Type>>,
    ) -> ElementInfo {
        let ElementInfo {
            size: discrim_size,
            align: discrim_align,
        } = int_size_align(tag);
        let mut case_size = ArchitectureSize::default();
        let mut case_align = Alignment::default();
        for ty in types {
            if let Some(ty) = ty {
                case_size = case_size.max(&self.size(ty));
                case_align = case_align.max(self.align(ty));
            }
        }
        let align = discrim_align.max(case_align);
        let discrim_aligned = align_to_arch(discrim_size, case_align);
        let size_sum = discrim_aligned + case_size;
        ElementInfo::new(align_to_arch(size_sum, align), align)
    }
}

fn int_size_align(i: Int) -> ElementInfo {
    match i {
        Int::U8 => Alignment::Bytes(NonZeroUsize::new(1).unwrap()),
        Int::U16 => Alignment::Bytes(NonZeroUsize::new(2).unwrap()),
        Int::U32 => Alignment::Bytes(NonZeroUsize::new(4).unwrap()),
        Int::U64 => Alignment::Bytes(NonZeroUsize::new(8).unwrap()),
    }
    .into()
}

/// Increase `val` to a multiple of `align`;
/// `align` must be a power of two
pub(crate) fn align_to(val: usize, align: usize) -> usize {
    (val + align - 1) & !(align - 1)
}

/// Increase `val` to a multiple of `align`, with special handling for pointers;
/// `align` must be a power of two or `Alignment::Pointer`
pub fn align_to_arch(val: ArchitectureSize, align: Alignment) -> ArchitectureSize {
    match align {
        Alignment::Pointer => {
            let new32 = align_to(val.bytes, 4);
            if new32 != align_to(new32, 8) {
                ArchitectureSize::new(new32 - 4, val.pointers + 1)
            } else {
                ArchitectureSize::new(new32, val.pointers)
            }
        }
        Alignment::Bytes(align_bytes) => {
            let align_bytes = align_bytes.get();
            if align_bytes > 4 && (val.pointers & 1) != 0 {
                let new_bytes = align_to(val.bytes, align_bytes);
                if (new_bytes - val.bytes) >= 4 {
                    // up to four extra bytes fit together with a the extra 32 bit pointer
                    // and the 64 bit pointer is always 8 bytes (so no change in value)
                    ArchitectureSize::new(new_bytes - 8, val.pointers + 1)
                } else {
                    // there is no room to combine, so the odd pointer aligns to 8 bytes
                    ArchitectureSize::new(new_bytes + 8, val.pointers - 1)
                }
            } else {
                ArchitectureSize::new(align_to(val.bytes, align_bytes), val.pointers)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn align() {
        // u8 + ptr
        assert_eq!(
            align_to_arch(ArchitectureSize::new(1, 0), Alignment::Pointer),
            ArchitectureSize::new(0, 1)
        );
        // u8 + u64
        assert_eq!(
            align_to_arch(
                ArchitectureSize::new(1, 0),
                Alignment::Bytes(NonZeroUsize::new(8).unwrap())
            ),
            ArchitectureSize::new(8, 0)
        );
        // u8 + u32
        assert_eq!(
            align_to_arch(
                ArchitectureSize::new(1, 0),
                Alignment::Bytes(NonZeroUsize::new(4).unwrap())
            ),
            ArchitectureSize::new(4, 0)
        );
        // ptr + u64
        assert_eq!(
            align_to_arch(
                ArchitectureSize::new(0, 1),
                Alignment::Bytes(NonZeroUsize::new(8).unwrap())
            ),
            ArchitectureSize::new(8, 0)
        );
        // u32 + ptr
        assert_eq!(
            align_to_arch(ArchitectureSize::new(4, 0), Alignment::Pointer),
            ArchitectureSize::new(0, 1)
        );
        // u32, ptr + u64
        assert_eq!(
            align_to_arch(
                ArchitectureSize::new(0, 2),
                Alignment::Bytes(NonZeroUsize::new(8).unwrap())
            ),
            ArchitectureSize::new(0, 2)
        );
        // ptr, u8 + u64
        assert_eq!(
            align_to_arch(
                ArchitectureSize::new(1, 1),
                Alignment::Bytes(NonZeroUsize::new(8).unwrap())
            ),
            ArchitectureSize::new(0, 2)
        );
        // ptr, u8 + ptr
        assert_eq!(
            align_to_arch(ArchitectureSize::new(1, 1), Alignment::Pointer),
            ArchitectureSize::new(0, 2)
        );
        // ptr, ptr, u8 + u64
        assert_eq!(
            align_to_arch(
                ArchitectureSize::new(1, 2),
                Alignment::Bytes(NonZeroUsize::new(8).unwrap())
            ),
            ArchitectureSize::new(8, 2)
        );
        assert_eq!(
            align_to_arch(
                ArchitectureSize::new(30, 3),
                Alignment::Bytes(NonZeroUsize::new(8).unwrap())
            ),
            ArchitectureSize::new(40, 2)
        );

        assert_eq!(
            ArchitectureSize::new(12, 0).max(&ArchitectureSize::new(0, 2)),
            ArchitectureSize::new(8, 1)
        );
        assert_eq!(
            ArchitectureSize::new(10, 0).max(&ArchitectureSize::new(0, 2)),
            ArchitectureSize::new(8, 1)
        );

        assert_eq!(
            align_to_arch(
                ArchitectureSize::new(2, 0),
                Alignment::Bytes(NonZeroUsize::new(8).unwrap())
            ),
            ArchitectureSize::new(8, 0)
        );
        assert_eq!(
            align_to_arch(ArchitectureSize::new(2, 0), Alignment::Pointer),
            ArchitectureSize::new(0, 1)
        );
    }

    #[test]
    fn resource_size() {
        // keep it identical to the old behavior
        let obj = SizeAlign::default();
        let elem = obj.calculate(&TypeDef {
            name: None,
            kind: TypeDefKind::Resource,
            owner: crate::TypeOwner::None,
            docs: Default::default(),
            stability: Default::default(),
        });
        assert_eq!(elem.size, ArchitectureSize::new(usize::MAX, 0));
        assert_eq!(
            elem.align,
            Alignment::Bytes(NonZeroUsize::new(usize::MAX).unwrap())
        );
    }
    #[test]
    fn result_ptr_10() {
        let mut obj = SizeAlign::default();
        let mut resolve = Resolve::default();
        let tuple = crate::Tuple {
            types: vec![Type::U16, Type::U16, Type::U16, Type::U16, Type::U16],
        };
        let id = resolve.types.alloc(TypeDef {
            name: None,
            kind: TypeDefKind::Tuple(tuple),
            owner: crate::TypeOwner::None,
            docs: Default::default(),
            stability: Default::default(),
        });
        obj.fill(&resolve);
        let my_result = crate::Result_ {
            ok: Some(Type::String),
            err: Some(Type::Id(id)),
        };
        let elem = obj.calculate(&TypeDef {
            name: None,
            kind: TypeDefKind::Result(my_result),
            owner: crate::TypeOwner::None,
            docs: Default::default(),
            stability: Default::default(),
        });
        assert_eq!(elem.size, ArchitectureSize::new(8, 2));
        assert_eq!(elem.align, Alignment::Pointer);
    }
    #[test]
    fn result_ptr_64bit() {
        let obj = SizeAlign::default();
        let my_record = crate::Record {
            fields: vec![
                crate::Field {
                    name: String::new(),
                    ty: Type::String,
                    docs: Default::default(),
                },
                crate::Field {
                    name: String::new(),
                    ty: Type::U64,
                    docs: Default::default(),
                },
            ],
        };
        let elem = obj.calculate(&TypeDef {
            name: None,
            kind: TypeDefKind::Record(my_record),
            owner: crate::TypeOwner::None,
            docs: Default::default(),
            stability: Default::default(),
        });
        assert_eq!(elem.size, ArchitectureSize::new(8, 2));
        assert_eq!(elem.align, Alignment::Bytes(NonZeroUsize::new(8).unwrap()));
    }
}
