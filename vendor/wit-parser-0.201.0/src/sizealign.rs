use crate::{FlagsRepr, Int, Resolve, Type, TypeDef, TypeDefKind};

#[derive(Default)]
pub struct SizeAlign {
    map: Vec<(usize, usize)>,
}

impl SizeAlign {
    pub fn fill(&mut self, resolve: &Resolve) {
        self.map = Vec::new();
        for (_, ty) in resolve.types.iter() {
            let pair = self.calculate(ty);
            self.map.push(pair);
        }
    }

    fn calculate(&self, ty: &TypeDef) -> (usize, usize) {
        match &ty.kind {
            TypeDefKind::Type(t) => (self.size(t), self.align(t)),
            TypeDefKind::List(_) => (8, 4),
            TypeDefKind::Record(r) => self.record(r.fields.iter().map(|f| &f.ty)),
            TypeDefKind::Tuple(t) => self.record(t.types.iter()),
            TypeDefKind::Flags(f) => match f.repr() {
                FlagsRepr::U8 => (1, 1),
                FlagsRepr::U16 => (2, 2),
                FlagsRepr::U32(n) => (n * 4, 4),
            },
            TypeDefKind::Variant(v) => self.variant(v.tag(), v.cases.iter().map(|c| c.ty.as_ref())),
            TypeDefKind::Enum(e) => self.variant(e.tag(), []),
            TypeDefKind::Option(t) => self.variant(Int::U8, [Some(t)]),
            TypeDefKind::Result(r) => self.variant(Int::U8, [r.ok.as_ref(), r.err.as_ref()]),
            // A resource is represented as an index.
            TypeDefKind::Handle(_) => (4, 4),
            // A future is represented as an index.
            TypeDefKind::Future(_) => (4, 4),
            // A stream is represented as an index.
            TypeDefKind::Stream(_) => (4, 4),
            // This shouldn't be used for anything since raw resources aren't part of the ABI -- just handles to
            // them.
            TypeDefKind::Resource => (usize::MAX, usize::MAX),
            TypeDefKind::Unknown => unreachable!(),
        }
    }

    pub fn size(&self, ty: &Type) -> usize {
        match ty {
            Type::Bool | Type::U8 | Type::S8 => 1,
            Type::U16 | Type::S16 => 2,
            Type::U32 | Type::S32 | Type::Float32 | Type::Char => 4,
            Type::U64 | Type::S64 | Type::Float64 | Type::String => 8,
            Type::Id(id) => self.map[id.index()].0,
        }
    }

    pub fn align(&self, ty: &Type) -> usize {
        match ty {
            Type::Bool | Type::U8 | Type::S8 => 1,
            Type::U16 | Type::S16 => 2,
            Type::U32 | Type::S32 | Type::Float32 | Type::Char | Type::String => 4,
            Type::U64 | Type::S64 | Type::Float64 => 8,
            Type::Id(id) => self.map[id.index()].1,
        }
    }

    pub fn field_offsets<'a>(
        &self,
        types: impl IntoIterator<Item = &'a Type>,
    ) -> Vec<(usize, &'a Type)> {
        let mut cur = 0;
        types
            .into_iter()
            .map(|ty| {
                let ret = align_to(cur, self.align(ty));
                cur = ret + self.size(ty);
                (ret, ty)
            })
            .collect()
    }

    pub fn payload_offset<'a>(
        &self,
        tag: Int,
        cases: impl IntoIterator<Item = Option<&'a Type>>,
    ) -> usize {
        let mut max_align = 1;
        for ty in cases {
            if let Some(ty) = ty {
                max_align = max_align.max(self.align(ty));
            }
        }
        let tag_size = int_size_align(tag).0;
        align_to(tag_size, max_align)
    }

    pub fn record<'a>(&self, types: impl Iterator<Item = &'a Type>) -> (usize, usize) {
        let mut size = 0;
        let mut align = 1;
        for ty in types {
            let field_size = self.size(ty);
            let field_align = self.align(ty);
            size = align_to(size, field_align) + field_size;
            align = align.max(field_align);
        }
        (align_to(size, align), align)
    }

    pub fn params<'a>(&self, types: impl IntoIterator<Item = &'a Type>) -> (usize, usize) {
        self.record(types.into_iter())
    }

    fn variant<'a>(
        &self,
        tag: Int,
        types: impl IntoIterator<Item = Option<&'a Type>>,
    ) -> (usize, usize) {
        let (discrim_size, discrim_align) = int_size_align(tag);
        let mut case_size = 0;
        let mut case_align = 1;
        for ty in types {
            if let Some(ty) = ty {
                case_size = case_size.max(self.size(ty));
                case_align = case_align.max(self.align(ty));
            }
        }
        let align = discrim_align.max(case_align);
        (
            align_to(align_to(discrim_size, case_align) + case_size, align),
            align,
        )
    }
}

fn int_size_align(i: Int) -> (usize, usize) {
    match i {
        Int::U8 => (1, 1),
        Int::U16 => (2, 2),
        Int::U32 => (4, 4),
        Int::U64 => (8, 8),
    }
}

pub(crate) fn align_to(val: usize, align: usize) -> usize {
    (val + align - 1) & !(align - 1)
}
