use crate::ast::*;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SizeAlign {
    pub size: usize,
    pub align: usize,
}

impl SizeAlign {
    fn zero() -> SizeAlign {
        SizeAlign { size: 0, align: 0 }
    }
    fn append_field(&mut self, other: &SizeAlign) {
        self.align = self.align.max(other.align);
        self.size = align_to(self.size, other.align);
        self.size += other.size;
    }
}

pub trait Layout {
    fn mem_size_align(&self) -> SizeAlign;
    fn mem_size(&self) -> usize {
        self.mem_size_align().size
    }
    fn mem_align(&self) -> usize {
        self.mem_size_align().align
    }
}

impl TypeRef {
    fn layout(&self, cache: &mut HashMap<TypeRef, SizeAlign>) -> SizeAlign {
        if let Some(hit) = cache.get(self) {
            return *hit;
        }
        let layout = match &self {
            TypeRef::Name(nt) => nt.layout(cache),
            TypeRef::Value(v) => v.layout(cache),
        };
        cache.insert(self.clone(), layout);
        layout
    }
}

impl Layout for TypeRef {
    fn mem_size_align(&self) -> SizeAlign {
        let mut cache = HashMap::new();
        self.layout(&mut cache)
    }
}

impl NamedType {
    fn layout(&self, cache: &mut HashMap<TypeRef, SizeAlign>) -> SizeAlign {
        self.tref.layout(cache)
    }
}
impl Layout for NamedType {
    fn mem_size_align(&self) -> SizeAlign {
        let mut cache = HashMap::new();
        self.layout(&mut cache)
    }
}

impl Type {
    fn layout(&self, cache: &mut HashMap<TypeRef, SizeAlign>) -> SizeAlign {
        match &self {
            Type::Record(s) => match s.bitflags_repr() {
                Some(repr) => repr.mem_size_align(),
                None => s.layout(cache),
            },
            Type::Variant(s) => s.mem_size_align(),
            Type::Handle(h) => h.mem_size_align(),
            Type::List { .. } => SizeAlign { size: 8, align: 4 }, // Pointer and Length
            Type::Pointer { .. } | Type::ConstPointer { .. } => BuiltinType::S32.mem_size_align(),
            Type::Builtin(b) => b.mem_size_align(),
        }
    }
}

impl Layout for Type {
    fn mem_size_align(&self) -> SizeAlign {
        let mut cache = HashMap::new();
        self.layout(&mut cache)
    }
}

impl Layout for IntRepr {
    fn mem_size_align(&self) -> SizeAlign {
        self.to_builtin().mem_size_align()
    }
}

pub struct RecordMemberLayout<'a> {
    pub member: &'a RecordMember,
    pub offset: usize,
}

impl RecordDatatype {
    pub fn member_layout(&self) -> Vec<RecordMemberLayout> {
        self.member_layout_(&mut HashMap::new()).1
    }

    fn member_layout_(
        &self,
        cache: &mut HashMap<TypeRef, SizeAlign>,
    ) -> (SizeAlign, Vec<RecordMemberLayout>) {
        let mut members = Vec::new();
        let mut sa = SizeAlign::zero();
        for m in self.members.iter() {
            let member = m.tref.layout(cache);
            sa.append_field(&member);
            members.push(RecordMemberLayout {
                member: m,
                offset: sa.size - member.size,
            });
        }
        sa.size = align_to(sa.size, sa.align);
        (sa, members)
    }

    fn layout(&self, cache: &mut HashMap<TypeRef, SizeAlign>) -> SizeAlign {
        self.member_layout_(cache).0
    }
}

impl Layout for RecordDatatype {
    fn mem_size_align(&self) -> SizeAlign {
        match self.bitflags_repr() {
            Some(repr) => repr.mem_size_align(),
            None => {
                let mut cache = HashMap::new();
                self.layout(&mut cache)
            }
        }
    }
}

impl Layout for Variant {
    fn mem_size_align(&self) -> SizeAlign {
        let mut max = SizeAlign { size: 0, align: 0 };
        for case in self.cases.iter() {
            let mut size = self.tag_repr.mem_size_align();
            if let Some(payload) = &case.tref {
                size.append_field(&payload.mem_size_align());
            }
            size.size = align_to(size.size, size.align);
            max.size = max.size.max(size.size);
            max.align = max.align.max(size.align);
        }
        max
    }
}

impl Variant {
    pub fn payload_offset(&self) -> usize {
        let mut offset = self.tag_repr.mem_size_align().size;
        for case in self.cases.iter() {
            if let Some(payload) = &case.tref {
                offset = offset.max(align_to(offset, payload.mem_size_align().align));
            }
        }
        offset
    }
}

/// If the next free byte in the struct is `offs`, and the next
/// element has alignment `alignment`, determine the offset at
/// which to place that element.
fn align_to(offs: usize, alignment: usize) -> usize {
    offs + alignment - 1 - ((offs + alignment - 1) % alignment)
}

#[cfg(test)]
mod test {
    use super::align_to;
    #[test]
    fn align() {
        assert_eq!(0, align_to(0, 1));
        assert_eq!(0, align_to(0, 2));
        assert_eq!(0, align_to(0, 4));
        assert_eq!(0, align_to(0, 8));

        assert_eq!(1, align_to(1, 1));
        assert_eq!(2, align_to(1, 2));
        assert_eq!(4, align_to(1, 4));
        assert_eq!(8, align_to(1, 8));

        assert_eq!(2, align_to(2, 1));
        assert_eq!(2, align_to(2, 2));
        assert_eq!(4, align_to(2, 4));
        assert_eq!(8, align_to(2, 8));

        assert_eq!(5, align_to(5, 1));
        assert_eq!(6, align_to(5, 2));
        assert_eq!(8, align_to(5, 4));
        assert_eq!(8, align_to(5, 8));
    }
}

impl Layout for HandleDatatype {
    fn mem_size_align(&self) -> SizeAlign {
        BuiltinType::S32.mem_size_align()
    }
}

impl Layout for BuiltinType {
    fn mem_size_align(&self) -> SizeAlign {
        match self {
            BuiltinType::U8 { .. } | BuiltinType::S8 => SizeAlign { size: 1, align: 1 },
            BuiltinType::U16 | BuiltinType::S16 => SizeAlign { size: 2, align: 2 },
            BuiltinType::Char | BuiltinType::U32 { .. } | BuiltinType::S32 | BuiltinType::F32 => {
                SizeAlign { size: 4, align: 4 }
            }
            BuiltinType::U64 | BuiltinType::S64 | BuiltinType::F64 => {
                SizeAlign { size: 8, align: 8 }
            }
        }
    }
}
