use crate::syntax::set::UnorderedSet as Set;
use std::sync::{Mutex, OnceLock, PoisonError};

#[derive(Copy, Clone, Default)]
pub(crate) struct InternedString(&'static str);

impl InternedString {
    pub(crate) fn str(self) -> &'static str {
        self.0
    }
}

pub(crate) fn intern(s: &str) -> InternedString {
    static INTERN: OnceLock<Mutex<Set<&'static str>>> = OnceLock::new();

    let mut set = INTERN
        .get_or_init(|| Mutex::new(Set::new()))
        .lock()
        .unwrap_or_else(PoisonError::into_inner);

    InternedString(match set.get(s) {
        Some(interned) => *interned,
        None => {
            let interned = Box::leak(Box::from(s));
            set.insert(interned);
            interned
        }
    })
}
