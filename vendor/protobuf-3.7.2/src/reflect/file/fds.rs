use std::collections::HashMap;
use std::collections::HashSet;
use std::mem;

use protobuf_support::toposort::toposort;

use crate::descriptor::FileDescriptorProto;
use crate::reflect::error::ReflectError;
use crate::reflect::FileDescriptor;

pub(crate) fn build_fds(
    protos: Vec<FileDescriptorProto>,
    dependencies: &[FileDescriptor],
) -> crate::Result<Vec<FileDescriptor>> {
    let mut index_by_name: HashMap<&str, usize> = HashMap::new();
    for (i, proto) in protos.iter().enumerate() {
        let prev = index_by_name.insert(proto.name(), i);
        if prev.is_some() {
            return Err(ReflectError::NonUniqueFileDescriptor(proto.name().to_owned()).into());
        }
    }

    let sorted = match toposort(0..protos.len(), |&i| {
        protos[i]
            .dependency
            .iter()
            .filter_map(|d| index_by_name.get(d.as_str()).copied())
    }) {
        Ok(s) => s,
        Err(_) => return Err(ReflectError::CycleInFileDescriptors.into()),
    };

    let mut built_descriptors_by_index = vec![None; protos.len()];

    let mut protos: Vec<Option<FileDescriptorProto>> = protos.into_iter().map(Some).collect();

    let mut all_descriptors = dependencies.to_vec();
    for f in sorted {
        let proto = mem::take(&mut protos[f]).unwrap();
        let d = FileDescriptor::new_dynamic(proto, &all_descriptors)?;
        all_descriptors.push(d.clone());
        built_descriptors_by_index[f] = Some(d);
    }

    Ok(built_descriptors_by_index
        .into_iter()
        .map(Option::unwrap)
        .collect())
}

pub(crate) fn fds_extend_with_public(file_descriptors: Vec<FileDescriptor>) -> Vec<FileDescriptor> {
    let mut visited = HashSet::new();

    let mut r = Vec::new();
    let mut stack = file_descriptors;
    stack.reverse();

    while let Some(f) = stack.pop() {
        if !visited.insert(f.proto().name().to_owned()) {
            continue;
        }

        stack.extend(f.public_deps());

        r.push(f);
    }
    r
}
