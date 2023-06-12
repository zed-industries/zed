use crate::Anchor;
use project::InlayHint;

use collections::BTreeMap;

#[derive(Debug, Default)]
pub struct InlayHintStorage {
    hints: BTreeMap<Anchor, InlayHint>,
}

impl InlayHintStorage {
    // TODO kb calculate the diff instead
    fn insert(&mut self) -> bool {
        todo!("TODO kb")
    }
}

// let buffer_version =
// cx.read(|cx| buffer.read(cx).version().clone());

// #[derive(Debug, Default, Clone)]
// struct InlayHintVersions {
//     last_buffer_versions_with_hints: HashMap<InlayHintLocation, Global>,
// }

// impl InlayHintVersions {
//     fn absent_or_newer(&self, location: &InlayHintLocation, new_version: &Global) -> bool {
//         self.last_buffer_versions_with_hints
//             .get(location)
//             .map(|last_version_with_hints| new_version.changed_since(&last_version_with_hints))
//             .unwrap_or(true)
//     }

//     fn insert(&mut self, location: InlayHintLocation, new_version: Global) -> bool {
//         if self.absent_or_newer(&location, &new_version) {
//             self.last_buffer_versions_with_hints
//                 .insert(location, new_version);
//             true
//         } else {
//             false
//         }
//     }
// }
