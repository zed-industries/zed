//-
// Copyright 2018 Jason Lingle
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Produce a hashable identifier unique to the particular macro invocation
/// which is stable across processes of the same executable.
///
/// This is usually the best thing to pass for the `fork_id` argument of
/// [`fork`](fn.fork.html).
///
/// The type of the expression this macro expands to is
/// [`RustyForkId`](struct.RustyForkId.html).
#[macro_export]
macro_rules! rusty_fork_id { () => { {
    struct _RustyForkId;
    $crate::RustyForkId::of(::std::any::TypeId::of::<_RustyForkId>())
} } }

/// The type of the value produced by
/// [`rusty_fork_id!`](macro.rusty_fork_id.html).
#[derive(Clone, Hash, PartialEq, Debug)]
pub struct RustyForkId(::std::any::TypeId);
impl RustyForkId {
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub fn of(id: ::std::any::TypeId) -> Self {
        RustyForkId(id)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn ids_are_actually_distinct() {
        assert_ne!(rusty_fork_id!(), rusty_fork_id!());
    }
}
