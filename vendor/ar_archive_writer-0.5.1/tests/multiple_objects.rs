use object::write;

mod common;

/// Tests creating an archive with multiple objects.
/// Note that `func_overlapping` exists in both objects - this is to test
/// deduplication of symbols in the symbol table (where supported).
#[test]
fn basic_multiple_objects() {
    common::generate_archive_and_compare(
        "basic_multiple_objects",
        |architecture, subarch, endianness, binary_format| {
            let mut object1 = write::Object::new(binary_format, architecture, endianness);
            object1.set_sub_architecture(subarch);
            common::add_file_with_functions_to_object(
                &mut object1,
                b"file1.c",
                &[b"func1", b"func2", b"func_overlapping"],
            );

            let mut object2 = write::Object::new(binary_format, architecture, endianness);
            object2.set_sub_architecture(subarch);
            common::add_file_with_functions_to_object(
                &mut object2,
                b"file2.c",
                &[b"func3", b"func4", b"func_overlapping"],
            );

            vec![
                ("file1.o", object1.write().unwrap()),
                ("file2.o", object2.write().unwrap()),
            ]
        },
    );
}

/// Tests creating an archive with multiple objects with the same name.
/// This is important for Mach), which uses the timestamp when in deterministic
/// mode to differentiate the two objects.
#[test]
fn multiple_objects_same_name() {
    common::generate_archive_and_compare(
        "multiple_objects_same_name",
        |architecture, subarch, endianness, binary_format| {
            let mut object1 = write::Object::new(binary_format, architecture, endianness);
            object1.set_sub_architecture(subarch);
            common::add_file_with_functions_to_object(&mut object1, b"file1.c", &[b"func1"]);

            let mut object2 = write::Object::new(binary_format, architecture, endianness);
            object2.set_sub_architecture(subarch);
            common::add_file_with_functions_to_object(&mut object2, b"file2.c", &[b"func2"]);

            vec![
                ("1/file.o", object1.write().unwrap()),
                ("2/file.o", object2.write().unwrap()),
            ]
        },
    );
}
