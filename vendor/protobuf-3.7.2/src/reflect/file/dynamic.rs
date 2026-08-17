use std::collections::HashMap;
use std::sync::Arc;

use crate::descriptor::FileDescriptorProto;
use crate::owning_ref::OwningRef;
use crate::reflect::error::ReflectError;
use crate::reflect::file::index::FileDescriptorCommon;
use crate::reflect::FileDescriptor;

#[derive(Debug)]
pub(crate) struct DynamicFileDescriptor {
    pub(crate) proto: Arc<FileDescriptorProto>,
    pub(crate) common: FileDescriptorCommon,
}

impl DynamicFileDescriptor {
    pub(crate) fn new(
        proto: FileDescriptorProto,
        dependencies: &[FileDescriptor],
    ) -> crate::Result<DynamicFileDescriptor> {
        // Remove undeclared dependencies.
        let dependencies_index: HashMap<_, &FileDescriptor> =
            dependencies.iter().map(|d| (d.proto().name(), d)).collect();

        if dependencies_index.len() != dependencies.len() {
            return Err(ReflectError::NonUniqueDependencies(
                dependencies
                    .iter()
                    .map(|d| d.proto().name())
                    .collect::<Vec<_>>()
                    .join(", "),
            )
            .into());
        }

        let dependencies: Vec<FileDescriptor> = proto
            .dependency
            .iter()
            .map(|d| {
                let dep = dependencies_index.get(d.as_str());
                match dep {
                    Some(dep) => Ok((*dep).clone()),
                    None => Err(ReflectError::DependencyNotFound(
                        d.clone(),
                        proto.name().to_owned(),
                        dependencies
                            .iter()
                            .map(|d| d.proto().name())
                            .collect::<Vec<_>>()
                            .join(", "),
                    )
                    .into()),
                }
            })
            .collect::<crate::Result<Vec<_>>>()?;

        let proto = Arc::new(proto);

        let common = FileDescriptorCommon::new(OwningRef::new_arc(proto.clone()), dependencies)?;

        Ok(DynamicFileDescriptor { proto, common })
    }
}
