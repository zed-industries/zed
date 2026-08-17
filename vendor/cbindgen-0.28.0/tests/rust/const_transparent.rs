#[repr(transparent)]
struct TransparentStruct { field: u8 }

impl TransparentStruct {
    pub const ASSOC_STRUCT_FOO: i64 = 1;
    pub const ASSOC_STRUCT_BAR: TransparentStruct = TransparentStruct { field: 2 };

    // TODO: Only C++ supports template constants so far.
    pub const ASSOC_STRUCT_BAZ: Wrapper<TransparentStruct> = Wrapper { field: TransparentStruct { field: 3 } };
}

#[repr(transparent)]
struct TransparentTupleStruct(u8);

#[repr(transparent)]
struct Wrapper<T> { field: T }

pub const STRUCT_FOO: TransparentStruct = TransparentStruct { field: 4 };
pub const STRUCT_BAR: TransparentTupleStruct = TransparentTupleStruct(5);

// TODO: Only C++ supports template constants so far.
pub const STRUCT_BAZ: Wrapper<TransparentStruct> = Wrapper { field: TransparentStruct { field: 6 } };

#[repr(transparent)]
struct TransparentStructWithErasedField<T> {
    field: Wrapper<T>,
}

// TODO: Only C++ supports template constants so far.
pub const COMPLEX: TransparentStructWithErasedField<TransparentStruct> = TransparentStructWithErasedField {
    field: Wrapper { field: TransparentStruct { field: 7 } }
};
