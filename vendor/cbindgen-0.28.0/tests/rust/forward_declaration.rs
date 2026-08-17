#[repr(C)]
struct TypeInfo {
    data: TypeData,
}

#[repr(C)]
enum TypeData {
    Primitive,
    Struct(StructInfo),
}

#[repr(C)]
struct StructInfo {
    fields: *const *const TypeInfo, // requires forward declaration
    num_fields: usize,
}

#[no_mangle]
pub extern "C" fn root(
    x: TypeInfo,
) {}
