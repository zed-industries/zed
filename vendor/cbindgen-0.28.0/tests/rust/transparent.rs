struct DummyStruct;

// Transparent struct tuple wrapping a struct.
#[repr(transparent)]
struct TransparentComplexWrappingStructTuple(DummyStruct);

// Transparent struct tuple wrapping a primitive.
#[repr(transparent)]
struct TransparentPrimitiveWrappingStructTuple(u32);

// Transparent structure wrapping a struct.
#[repr(transparent)]
struct TransparentComplexWrappingStructure { only_field: DummyStruct }

// Transparent structure wrapping a primitive.
#[repr(transparent)]
struct TransparentPrimitiveWrappingStructure { only_field: u32 }

// Transparent struct wrapper with a marker wrapping a struct.
#[repr(transparent)]
struct TransparentComplexWrapper<T> {
    only_non_zero_sized_field: DummyStruct,
    marker: PhantomData<T>,
}

// Transparent struct wrapper with a marker wrapping a primitive.
#[repr(transparent)]
struct TransparentPrimitiveWrapper<T> {
    only_non_zero_sized_field: u32,
    marker: PhantomData<T>,
}

// Associated constant declared before struct declaration.
impl TransparentPrimitiveWithAssociatedConstants {
    pub const ZERO: TransparentPrimitiveWithAssociatedConstants = TransparentPrimitiveWithAssociatedConstants {
        bits: 0
    };
}

// Transparent structure wrapping a primitive with associated constants.
#[repr(transparent)]
struct TransparentPrimitiveWithAssociatedConstants { bits: u32 }

// Transparent zero-sized structs are legal rust, but there's no way to emit a typedef for one, so
// cbindgen should treat it as repr(C) instead and emit an empty struct definition.
#[repr(transparent)]
struct TransparentEmptyStructure;

// Associated constant declared after struct declaration.
impl TransparentPrimitiveWithAssociatedConstants {
    pub const ONE: TransparentPrimitiveWithAssociatedConstants = TransparentPrimitiveWithAssociatedConstants {
        bits: 1
    };
}

enum EnumWithAssociatedConstantInImpl { A }

impl EnumWithAssociatedConstantInImpl {
    pub const TEN: TransparentPrimitiveWrappingStructure = TransparentPrimitiveWrappingStructure { only_field: 10 };
}

#[no_mangle]
pub extern "C" fn root(
    a: TransparentComplexWrappingStructTuple,
    b: TransparentPrimitiveWrappingStructTuple,
    c: TransparentComplexWrappingStructure,
    d: TransparentPrimitiveWrappingStructure,
    e: TransparentComplexWrapper<i32>,
    f: TransparentPrimitiveWrapper<i32>,
    g: TransparentPrimitiveWithAssociatedConstants,
    h: TransparentEmptyStructure,
    i: EnumWithAssociatedConstantInImpl,
) { }
