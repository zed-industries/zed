use crate::{KeyBinding, KeyBindingContextPredicate, Keystroke, NoAction};
use collections::HashSet;
use smallvec::SmallVec;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub struct KeymapVersion(usize);

#[derive(Default)]
pub struct Keymap {
    bindings: Vec<KeyBinding>,
    binding_indices_by_action_id: HashMap<TypeId, SmallVec<[usize; 3]>>,
    disabled_keystrokes:
        HashMap<SmallVec<[Keystroke; 2]>, HashSet<Option<KeyBindingContextPredicate>>>,
    version: KeymapVersion,
}

impl Keymap {
    pub fn new(bindings: Vec<KeyBinding>) -> Self {
        let mut this = Self::default();
        this.add_bindings(bindings);
        this
    }

    pub fn version(&self) -> KeymapVersion {
        self.version
    }

    pub fn bindings_for_action(&self, action_id: TypeId) -> impl Iterator<Item = &'_ KeyBinding> {
        self.binding_indices_by_action_id
            .get(&action_id)
            .map(SmallVec::as_slice)
            .unwrap_or(&[])
            .iter()
            .map(|ix| &self.bindings[*ix])
            .filter(|binding| !self.binding_disabled(binding))
    }

    pub fn add_bindings<T: IntoIterator<Item = KeyBinding>>(&mut self, bindings: T) {
        let no_action_id = &(NoAction {}).type_id();
        let mut new_bindings = Vec::new();
        let mut has_new_disabled_keystrokes = false;
        for binding in bindings {
            if binding.action.type_id() == *no_action_id {
                has_new_disabled_keystrokes |= self
                    .disabled_keystrokes
                    .entry(binding.keystrokes)
                    .or_default()
                    .insert(binding.context_predicate);
            } else {
                new_bindings.push(binding);
            }
        }

        if has_new_disabled_keystrokes {
            self.binding_indices_by_action_id.retain(|_, indices| {
                indices.retain(|ix| {
                    let binding = &self.bindings[*ix];
                    match self.disabled_keystrokes.get(&binding.keystrokes) {
                        Some(disabled_predicates) => {
                            !disabled_predicates.contains(&binding.context_predicate)
                        }
                        None => true,
                    }
                });
                !indices.is_empty()
            });
        }

        for new_binding in new_bindings {
            if !self.binding_disabled(&new_binding) {
                self.binding_indices_by_action_id
                    .entry(new_binding.action().as_any().type_id())
                    .or_default()
                    .push(self.bindings.len());
                self.bindings.push(new_binding);
            }
        }

        self.version.0 += 1;
    }

    pub fn clear(&mut self) {
        self.bindings.clear();
        self.binding_indices_by_action_id.clear();
        self.disabled_keystrokes.clear();
        self.version.0 += 1;
    }

    pub fn bindings(&self) -> Vec<&KeyBinding> {
        self.bindings
            .iter()
            .filter(|binding| !self.binding_disabled(binding))
            .collect()
    }

    fn binding_disabled(&self, binding: &KeyBinding) -> bool {
        match self.disabled_keystrokes.get(&binding.keystrokes) {
            Some(disabled_predicates) => disabled_predicates.contains(&binding.context_predicate),
            None => false,
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::actions;

//     use super::*;

//     actions!(
//         keymap_test,
//         [Present1, Present2, Present3, Duplicate, Missing]
//     );

//     #[test]
//     fn regular_keymap() {
//         let present_1 = Binding::new("ctrl-q", Present1 {}, None);
//         let present_2 = Binding::new("ctrl-w", Present2 {}, Some("pane"));
//         let present_3 = Binding::new("ctrl-e", Present3 {}, Some("editor"));
//         let keystroke_duplicate_to_1 = Binding::new("ctrl-q", Duplicate {}, None);
//         let full_duplicate_to_2 = Binding::new("ctrl-w", Present2 {}, Some("pane"));
//         let missing = Binding::new("ctrl-r", Missing {}, None);
//         let all_bindings = [
//             &present_1,
//             &present_2,
//             &present_3,
//             &keystroke_duplicate_to_1,
//             &full_duplicate_to_2,
//             &missing,
//         ];

//         let mut keymap = Keymap::default();
//         assert_absent(&keymap, &all_bindings);
//         assert!(keymap.bindings().is_empty());

//         keymap.add_bindings([present_1.clone(), present_2.clone(), present_3.clone()]);
//         assert_absent(&keymap, &[&keystroke_duplicate_to_1, &missing]);
//         assert_present(
//             &keymap,
//             &[(&present_1, "q"), (&present_2, "w"), (&present_3, "e")],
//         );

//         keymap.add_bindings([
//             keystroke_duplicate_to_1.clone(),
//             full_duplicate_to_2.clone(),
//         ]);
//         assert_absent(&keymap, &[&missing]);
//         assert!(
//             !keymap.binding_disabled(&keystroke_duplicate_to_1),
//             "Duplicate binding 1 was added and should not be disabled"
//         );
//         assert!(
//             !keymap.binding_disabled(&full_duplicate_to_2),
//             "Duplicate binding 2 was added and should not be disabled"
//         );

//         assert_eq!(
//             keymap
//                 .bindings_for_action(keystroke_duplicate_to_1.action().id())
//                 .map(|binding| &binding.keystrokes)
//                 .flatten()
//                 .collect::<Vec<_>>(),
//             vec![&Keystroke {
//                 ctrl: true,
//                 alt: false,
//                 shift: false,
//                 cmd: false,
//                 function: false,
//                 key: "q".to_string(),
//                 ime_key: None,
//             }],
//             "{keystroke_duplicate_to_1:?} should have the expected keystroke in the keymap"
//         );
//         assert_eq!(
//             keymap
//                 .bindings_for_action(full_duplicate_to_2.action().id())
//                 .map(|binding| &binding.keystrokes)
//                 .flatten()
//                 .collect::<Vec<_>>(),
//             vec![
//                 &Keystroke {
//                     ctrl: true,
//                     alt: false,
//                     shift: false,
//                     cmd: false,
//                     function: false,
//                     key: "w".to_string(),
//                     ime_key: None,
//                 },
//                 &Keystroke {
//                     ctrl: true,
//                     alt: false,
//                     shift: false,
//                     cmd: false,
//                     function: false,
//                     key: "w".to_string(),
//                     ime_key: None,
//                 }
//             ],
//             "{full_duplicate_to_2:?} should have a duplicated keystroke in the keymap"
//         );

//         let updated_bindings = keymap.bindings();
//         let expected_updated_bindings = vec![
//             &present_1,
//             &present_2,
//             &present_3,
//             &keystroke_duplicate_to_1,
//             &full_duplicate_to_2,
//         ];
//         assert_eq!(
//             updated_bindings.len(),
//             expected_updated_bindings.len(),
//             "Unexpected updated keymap bindings {updated_bindings:?}"
//         );
//         for (i, expected) in expected_updated_bindings.iter().enumerate() {
//             let keymap_binding = &updated_bindings[i];
//             assert_eq!(
//                 keymap_binding.context_predicate, expected.context_predicate,
//                 "Unexpected context predicate for keymap {i} element: {keymap_binding:?}"
//             );
//             assert_eq!(
//                 keymap_binding.keystrokes, expected.keystrokes,
//                 "Unexpected keystrokes for keymap {i} element: {keymap_binding:?}"
//             );
//         }

//         keymap.clear();
//         assert_absent(&keymap, &all_bindings);
//         assert!(keymap.bindings().is_empty());
//     }

//     #[test]
//     fn keymap_with_ignored() {
//         let present_1 = Binding::new("ctrl-q", Present1 {}, None);
//         let present_2 = Binding::new("ctrl-w", Present2 {}, Some("pane"));
//         let present_3 = Binding::new("ctrl-e", Present3 {}, Some("editor"));
//         let keystroke_duplicate_to_1 = Binding::new("ctrl-q", Duplicate {}, None);
//         let full_duplicate_to_2 = Binding::new("ctrl-w", Present2 {}, Some("pane"));
//         let ignored_1 = Binding::new("ctrl-q", NoAction {}, None);
//         let ignored_2 = Binding::new("ctrl-w", NoAction {}, Some("pane"));
//         let ignored_3_with_other_context =
//             Binding::new("ctrl-e", NoAction {}, Some("other_context"));

//         let mut keymap = Keymap::default();

//         keymap.add_bindings([
//             ignored_1.clone(),
//             ignored_2.clone(),
//             ignored_3_with_other_context.clone(),
//         ]);
//         assert_absent(&keymap, &[&present_3]);
//         assert_disabled(
//             &keymap,
//             &[
//                 &present_1,
//                 &present_2,
//                 &ignored_1,
//                 &ignored_2,
//                 &ignored_3_with_other_context,
//             ],
//         );
//         assert!(keymap.bindings().is_empty());
//         keymap.clear();

//         keymap.add_bindings([
//             present_1.clone(),
//             present_2.clone(),
//             present_3.clone(),
//             ignored_1.clone(),
//             ignored_2.clone(),
//             ignored_3_with_other_context.clone(),
//         ]);
//         assert_present(&keymap, &[(&present_3, "e")]);
//         assert_disabled(
//             &keymap,
//             &[
//                 &present_1,
//                 &present_2,
//                 &ignored_1,
//                 &ignored_2,
//                 &ignored_3_with_other_context,
//             ],
//         );
//         keymap.clear();

//         keymap.add_bindings([
//             present_1.clone(),
//             present_2.clone(),
//             present_3.clone(),
//             ignored_1.clone(),
//         ]);
//         assert_present(&keymap, &[(&present_2, "w"), (&present_3, "e")]);
//         assert_disabled(&keymap, &[&present_1, &ignored_1]);
//         assert_absent(&keymap, &[&ignored_2, &ignored_3_with_other_context]);
//         keymap.clear();

//         keymap.add_bindings([
//             present_1.clone(),
//             present_2.clone(),
//             present_3.clone(),
//             keystroke_duplicate_to_1.clone(),
//             full_duplicate_to_2.clone(),
//             ignored_1.clone(),
//             ignored_2.clone(),
//             ignored_3_with_other_context.clone(),
//         ]);
//         assert_present(&keymap, &[(&present_3, "e")]);
//         assert_disabled(
//             &keymap,
//             &[
//                 &present_1,
//                 &present_2,
//                 &keystroke_duplicate_to_1,
//                 &full_duplicate_to_2,
//                 &ignored_1,
//                 &ignored_2,
//                 &ignored_3_with_other_context,
//             ],
//         );
//         keymap.clear();
//     }

//     #[track_caller]
//     fn assert_present(keymap: &Keymap, expected_bindings: &[(&Binding, &str)]) {
//         let keymap_bindings = keymap.bindings();
//         assert_eq!(
//             expected_bindings.len(),
//             keymap_bindings.len(),
//             "Unexpected keymap bindings {keymap_bindings:?}"
//         );
//         for (i, (expected, expected_key)) in expected_bindings.iter().enumerate() {
//             assert!(
//                 !keymap.binding_disabled(expected),
//                 "{expected:?} should not be disabled as it was added into keymap for element {i}"
//             );
//             assert_eq!(
//                 keymap
//                     .bindings_for_action(expected.action().id())
//                     .map(|binding| &binding.keystrokes)
//                     .flatten()
//                     .collect::<Vec<_>>(),
//                 vec![&Keystroke {
//                     ctrl: true,
//                     alt: false,
//                     shift: false,
//                     cmd: false,
//                     function: false,
//                     key: expected_key.to_string(),
//                     ime_key: None,
//                 }],
//                 "{expected:?} should have the expected keystroke with key '{expected_key}' in the keymap for element {i}"
//             );

//             let keymap_binding = &keymap_bindings[i];
//             assert_eq!(
//                 keymap_binding.context_predicate, expected.context_predicate,
//                 "Unexpected context predicate for keymap {i} element: {keymap_binding:?}"
//             );
//             assert_eq!(
//                 keymap_binding.keystrokes, expected.keystrokes,
//                 "Unexpected keystrokes for keymap {i} element: {keymap_binding:?}"
//             );
//         }
//     }

//     #[track_caller]
//     fn assert_absent(keymap: &Keymap, bindings: &[&Binding]) {
//         for binding in bindings.iter() {
//             assert!(
//                 !keymap.binding_disabled(binding),
//                 "{binding:?} should not be disabled in the keymap where was not added"
//             );
//             assert_eq!(
//                 keymap.bindings_for_action(binding.action().id()).count(),
//                 0,
//                 "{binding:?} should have no actions in the keymap where was not added"
//             );
//         }
//     }

//     #[track_caller]
//     fn assert_disabled(keymap: &Keymap, bindings: &[&Binding]) {
//         for binding in bindings.iter() {
//             assert!(
//                 keymap.binding_disabled(binding),
//                 "{binding:?} should be disabled in the keymap"
//             );
//             assert_eq!(
//                 keymap.bindings_for_action(binding.action().id()).count(),
//                 0,
//                 "{binding:?} should have no actions in the keymap where it was disabled"
//             );
//         }
//     }
// }
