use std::collections::HashMap;
use std::collections::HashSet;

use crate::descriptor::field_descriptor_proto::Label;
use crate::descriptor::FileDescriptorProto;
use crate::reflect::field::index::FieldIndex;
use crate::reflect::field::index::ForwardProtobufFieldType;
use crate::reflect::field::index::ForwardProtobufTypeBox;
use crate::reflect::file::index::MessageIndices;
use crate::reflect::MessageDescriptor;
use crate::reflect::RuntimeType;
use crate::reflect::Syntax;

pub(crate) fn compute_is_initialized_is_always_true(
    messages: &mut [MessageIndices],
    file_fields: &[FieldIndex],
    file: &FileDescriptorProto,
) {
    for message in messages.iter_mut() {
        message.is_initialized_is_always_true =
            is_initialized_is_always_true_ignoring_deps(message, file);
    }

    // Map from a message to messages who include it. E.g. for:
    // ```
    // 0: message A {}
    // 1: message B { A a = 10; }
    // ```
    // This map will contain: `{0: [1]}`
    let mut rdeps: HashMap<usize, Vec<usize>> = HashMap::new();

    for i in 0..messages.len() {
        let message = &mut messages[i];

        if !message.is_initialized_is_always_true {
            continue;
        }

        let mut is_initialized_is_always_true = true;
        for ft in message_field_messages(message, file_fields) {
            match ft {
                MessageType::ThisFile(j) => {
                    rdeps.entry(j).or_default().push(i);
                }
                MessageType::OtherFile(m) => {
                    if !m.is_initialized_is_always_true() {
                        is_initialized_is_always_true = false;
                    }
                }
            }
        }
        message.is_initialized_is_always_true = is_initialized_is_always_true;
    }

    let mut invalidated: HashSet<usize> = HashSet::new();
    let mut invalidate_stack: Vec<usize> = Vec::new();

    for i in 0..messages.len() {
        let message = &messages[i];
        if message.is_initialized_is_always_true {
            continue;
        }

        invalidate_stack.push(i);
    }

    while let Some(i) = invalidate_stack.pop() {
        if !invalidated.insert(i) {
            continue;
        }

        messages[i].is_initialized_is_always_true = false;
        let next = rdeps.get(&i).map(|v| v.as_slice()).unwrap_or_default();
        for next in next {
            invalidate_stack.push(*next);
        }
    }
}

enum MessageType<'m> {
    ThisFile(usize),
    OtherFile(&'m MessageDescriptor),
}

fn message_field_messages<'a>(
    message: &'a MessageIndices,
    file_fields: &'a [FieldIndex],
) -> impl Iterator<Item = MessageType<'a>> + 'a {
    message_field_types(message, file_fields).filter_map(|f| match f {
        ForwardProtobufTypeBox::ProtobufTypeBox(t) => match t.runtime() {
            RuntimeType::Message(m) => Some(MessageType::OtherFile(m)),
            _ => None,
        },
        ForwardProtobufTypeBox::CurrentFileEnum(_) => None,
        ForwardProtobufTypeBox::CurrentFileMessage(i) => Some(MessageType::ThisFile(*i)),
    })
}

fn message_field_types<'a>(
    message: &'a MessageIndices,
    file_fields: &'a [FieldIndex],
) -> impl Iterator<Item = &'a ForwardProtobufTypeBox> {
    enum Either<A, B> {
        Left(A),
        Right(B),
    }

    impl<T, A: Iterator<Item = T>, B: Iterator<Item = T>> Iterator for Either<A, B> {
        type Item = T;

        fn next(&mut self) -> Option<T> {
            match self {
                Either::Left(a) => a.next(),
                Either::Right(b) => b.next(),
            }
        }
    }

    message
        .message_index
        .slice_fields(file_fields)
        .iter()
        .flat_map(|f| match &f.field_type {
            ForwardProtobufFieldType::Singular(t) => Either::Left([t].into_iter()),
            ForwardProtobufFieldType::Repeated(t) => Either::Left([t].into_iter()),
            ForwardProtobufFieldType::Map(k, v) => Either::Right([k, v].into_iter()),
        })
}

fn is_initialized_is_always_true_ignoring_deps(
    message: &MessageIndices,
    file: &FileDescriptorProto,
) -> bool {
    // Shortcut.
    if Syntax::of_file(file) == Syntax::Proto3 {
        return true;
    }

    // We don't support extensions properly but if we did,
    // extensions should have been checked for `is_initialized`.
    if !message.proto.extension_range.is_empty() {
        return false;
    }

    for field in &message.proto.field {
        if field.label() == Label::LABEL_REQUIRED {
            return false;
        }
    }
    true
}
