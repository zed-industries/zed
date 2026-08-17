use crate::descriptor::DescriptorProto;
use crate::descriptor::EnumDescriptorProto;
use crate::descriptor::FileDescriptorProto;

pub(crate) enum MessageOrEnum<'a> {
    Message(&'a DescriptorProto),
    Enum(&'a EnumDescriptorProto),
}

impl<'a> MessageOrEnum<'a> {
    fn from_two_options(
        m: Option<&'a DescriptorProto>,
        e: Option<&'a EnumDescriptorProto>,
    ) -> Option<MessageOrEnum<'a>> {
        match (m, e) {
            (Some(_), Some(_)) => panic!("enum and message with the same name"),
            (Some(m), None) => Some(MessageOrEnum::Message(m)),
            (None, Some(e)) => Some(MessageOrEnum::Enum(e)),
            (None, None) => None,
        }
    }
}

pub(crate) fn find_message_or_enum<'a>(
    file: &'a FileDescriptorProto,
    name_to_package: &str,
) -> Option<(String, MessageOrEnum<'a>)> {
    assert!(!name_to_package.starts_with("."));
    assert!(!name_to_package.is_empty());

    let mut path = name_to_package.split('.');
    let first = path.next().unwrap();
    let child_message = file.message_type.iter().find(|m| m.name() == first);
    let child_enum = file.enum_type.iter().find(|e| e.name() == first);

    let mut package_to_name = String::new();
    let mut me = MessageOrEnum::from_two_options(child_message, child_enum)?;

    for name in path {
        let message = match me {
            MessageOrEnum::Message(m) => m,
            MessageOrEnum::Enum(_) => panic!("enum has no children"),
        };

        if !package_to_name.is_empty() {
            package_to_name.push_str(".");
        }
        package_to_name.push_str(message.name());

        let child_message = message.nested_type.iter().find(|m| m.name() == name);
        let child_enum = message.enum_type.iter().find(|e| e.name() == name);
        me = MessageOrEnum::from_two_options(child_message, child_enum)?;
    }

    Some((package_to_name, me))
}
