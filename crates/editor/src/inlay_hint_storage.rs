use crate::Anchor;
use project::InlayHint;

use collections::BTreeMap;

#[derive(Debug, Default)]
pub struct InlayHintStorage {
    hints: BTreeMap<Anchor, InlayHint>,
}

impl InlayHintStorage {
    fn insert(&mut self) -> bool {
        todo!("TODO kb")
    }
}
// TODO kb need to understand different inlay hint update cases:
// * new hints from the new excerpt (no need to invalidate the cache)
// * new hints after /refresh or a text edit (whole cache should be purged)
// ??? revert/reopened files could get a speedup, if we don't truly delete the hints, but hide them in another var?

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
