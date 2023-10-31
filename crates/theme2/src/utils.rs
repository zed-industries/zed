/// This macro generates a struct and a corresponding struct with optional fields.
///
/// It takes as input the name of the struct to be generated, the name of the struct with optional fields,
/// and a list of field names along with their types.
///
/// # Example
/// ```
/// generate_struct_with_overrides!(
///     MyStruct,
///     MyStructOverride,
///     field1: i32,
///     field2: String
/// );
/// ```
/// This will generate the following structs:
/// ```
/// pub struct MyStruct {
///     pub field1: i32,
///     pub field2: String,
/// }
///
/// pub struct MyStructOverride {
///     pub field1: Option<i32>,
///     pub field2: Option<String>,
/// }
/// ```
#[macro_export]
macro_rules! generate_struct_with_overrides {
    ($struct_name:ident, $struct_override_name:ident, $($field:ident: $type:ty),*) => {
        pub struct $struct_name {
            $(
                pub $field: $type,
            )*
        }

        pub struct $struct_override_name {
            $(
                pub $field: Option<$type>,
            )*
        }
    };
}
