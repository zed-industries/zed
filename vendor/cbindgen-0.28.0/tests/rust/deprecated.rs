#[no_mangle]
#[deprecated]
pub extern "C" fn deprecated_without_note() {}

#[no_mangle]
#[deprecated = "This is a note"]
pub extern "C" fn deprecated_without_bracket() {}

#[no_mangle]
#[deprecated(note = "This is a note")]
pub extern "C" fn deprecated_with_note() {}

#[no_mangle]
#[deprecated(note = "This is a note", since = "1.0.0")]
pub extern "C" fn deprecated_with_note_and_since() {}

#[no_mangle]
#[deprecated(note = "This quote \" requires to be quoted, and this [\n] requires to be escaped")]
pub extern "C" fn deprecated_with_note_which_requires_to_be_escaped() {}

#[repr(i32)]
#[deprecated]
pub enum DeprecatedEnum {
    A = 0,
}

#[repr(i32)]
#[deprecated(note = "This is a note")]
pub enum DeprecatedEnumWithNote {
    B = 0,
}

#[repr(i32)]
pub enum EnumWithDeprecatedVariants {
    C = 0,
    #[deprecated]
    D = 1,
    #[deprecated(note = "This is a note")]
    E = 2,
    #[deprecated(note = "This is a note", since = "1.0.0")]
    F = 3,
}

#[repr(u8)]
enum EnumWithDeprecatedStructVariants {
    Foo(i16),
    #[deprecated]
    Bar { x: u8, y: i16 },
    #[deprecated(note = "This is a note")]
    Baz { x: u8, y: u8 },
}

#[repr(C)]
#[deprecated]
pub struct DeprecatedStruct {
    pub a: i32,
}

#[repr(C)]
#[deprecated(note = "This is a note")]
pub struct DeprecatedStructWithNote {
    pub a: i32,
}

#[no_mangle]
pub extern "C" fn dummy(
    a: DeprecatedEnum,
    b: DeprecatedEnumWithNote,
    c: EnumWithDeprecatedVariants,
    d: DeprecatedStruct,
    e: DeprecatedStructWithNote,
    f: EnumWithDeprecatedStructVariants,
) -> void {
}
