use ash::vk;

#[test]
fn debug_flags() {
    assert_eq!(
        format!(
            "{:?}",
            vk::AccessFlags::INDIRECT_COMMAND_READ | vk::AccessFlags::VERTEX_ATTRIBUTE_READ
        ),
        "INDIRECT_COMMAND_READ | VERTEX_ATTRIBUTE_READ"
    );
}

#[test]
fn debug_enum() {
    assert_eq!(format!("{:?}", vk::ChromaLocation::MIDPOINT), "MIDPOINT");
}
