// Copyright 2018 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::mem::{self, ManuallyDrop};
use std::thread;

/// Dummy type that panics when its destructor is invoked.
#[derive(Debug)]
struct DropBomb();

impl Drop for DropBomb {
    fn drop(&mut self) {
        let message = "Trying to drop an AliasedCell, which may still have aliases outstanding.";
        if thread::panicking() {
            eprintln!("{}", message);
        } else {
            panic!("{}", message);
        }
    }
}

/// A wrapper for unsafely aliased memory locations.
///
/// `AliasedCell' makes sure that its inner value
/// is completely inaccessible from safe code.
/// Once an `AliasedCell` has been constructed from some value,
/// until the value is moved out again with the (unsafe) `into_inner()` method,
/// the only way to access the wrapped value
/// is through the unsafe `alias_mut()` method.
///
/// This is useful for FFI calls that take raw pointers as input,
/// and hold on to them even after returning control to the caller.
/// Since Rust's type system is not aware of such aliases,
/// it cannot provide the usual guarantees about validity of pointers
/// and exclusiveness of mutable pointers.
/// This means that any code that has access to the memory in question
/// is inherently unsafe as long as such untracked aliases exist.
/// Putting the value in an `AliasedCell` before the FFI call,
/// and only taking it out again
/// once the caller has ensured that all aliases have been dropped
/// (most likely through another FFI call),
/// makes certain that any such unsafe access to the aliased value
/// can only happen from code marked as `unsafe`.
///
/// An `AliasedCell` should never be simply dropped in normal use.
/// Rather, it should always be freed explicitly with `into_inner()`,
/// signalling that there are no outstanding aliases anymore.
/// When an `AliasedCell` is dropped unexpectedly,
/// we have to assume that it likely still has untracked aliases,
/// and thus freeing the memory would probably be unsound.
/// Therefore, the `drop()` implementation of `AliasedCell`
/// leaks the inner value instead of freeing it;
/// and throws a panic.
/// Leaking the memory, while undesirable in general,
/// keeps the memory accessible to any outstanding aliases.
/// This is the only way to retain soundness during unwinding,
/// or when the panic gets caught.
///
/// Note that making FFI access through untracked aliases
/// requires the value to have a stable memory location --
/// typically by living on the heap rather than on the stack.
/// If the value isn't already in a heap-allocated container
/// such as `Box<>`, `Vec<>`, or `String`,
/// it is the caller's responsibility to wrap it in a `Box<>` explicitly.
/// `AliasedCell` itself cannot ensure that the address remains stable
/// when the `AliasedCell` gets moved around.
#[derive(Debug)]
pub struct AliasedCell<T> {
    value: ManuallyDrop<T>,
    drop_bomb: DropBomb,
}

impl<T> AliasedCell<T> {
    /// Wrap the provided value in an `AliasedCell`, making it inaccessible from safe code.
    pub fn new(value: T) -> AliasedCell<T> {
        AliasedCell {
            value: ManuallyDrop::new(value),
            drop_bomb: DropBomb(),
        }
    }

    /// Get a pointer to the inner value.
    ///
    /// Note that this yields a regular reference.
    /// To actually get an untracked alias,
    /// it needs to be cast or coerced into a raw pointer.
    /// This usually happens implicitly though
    /// when calling an FFI function (or any other function)
    /// taking a raw pointer as argument.
    ///
    /// `alias_mut()` can be called any number of times:
    /// the wrapper doesn't keep track of the number of outstanding aliases --
    /// the caller is responsible for making sure that no aliases are left
    /// before invoking `into_inner()`.
    /// If you need to track the number of aliases,
    /// wrap the inner value in an `Rc<>` or `Arc` --
    /// this way, the reference count will also be inaccessible from safe code.
    pub unsafe fn alias_mut(&mut self) -> &mut T {
        &mut self.value
    }

    /// Get a shared (immutable) pointer to the inner value.
    ///
    /// With this method it's possible to get an alias
    /// while only holding a shared reference to the `AliasedCell`.
    ///
    /// Since all the unsafe aliases are untracked,
    /// it's up to the callers to make sure no shared aliases are used
    /// while the data might actually be mutated elsewhere
    /// through some outstanding mutable aliases.
    pub unsafe fn alias(&self) -> &T {
        &self.value // orig &self.inner
    }

    /// Move out the wrapped value, making it accessible from safe code again.
    pub unsafe fn into_inner(self) -> T {
        mem::forget(self.drop_bomb);
        ManuallyDrop::into_inner(self.value)
    }
}

/// Some basic tests.
///
/// Note: These mostly just check that various expected usage scenarios
/// can be compiled and basically work.
/// We can't verify though that the usage is actually sound;
/// nor do we check whether invalid usage is indeed prevented by the compiler...
///
/// (The latter could probably be remedied though
/// with some sort of compile-fail tests.)
#[cfg(test)]
mod tests {
    use super::AliasedCell;

    unsafe fn mutate_value(addr: *mut [i32; 4]) {
        let value = addr.as_mut().unwrap();
        value[1] += value[3];
    }

    struct Mutator {
        addr: *mut [i32; 4],
        ascended: bool,
    }

    impl Mutator {
        unsafe fn new(addr: *mut [i32; 4]) -> Mutator {
            Mutator {
                addr,
                ascended: false,
            }
        }

        fn ascend(&mut self) {
            self.ascended = true;
        }

        unsafe fn mutate(&mut self) {
            let value = self.addr.as_mut().unwrap();
            if self.ascended {
                value[3] += value[2];
            } else {
                value[1] += value[3];
            }
        }
    }

    #[test]
    fn noop_roundtrip() {
        let value = [1, 3, 3, 7];
        let cell = AliasedCell::new(Box::new(value));
        let new_value = unsafe { *cell.into_inner() };
        assert_eq!(new_value, [1, 3, 3, 7]);
    }

    #[test]
    fn unused_alias() {
        let value = [1, 3, 3, 7];
        let mut cell = AliasedCell::new(Box::new(value));
        unsafe {
            cell.alias_mut().as_mut();
        }
        let new_value = unsafe { *cell.into_inner() };
        assert_eq!(new_value, [1, 3, 3, 7]);
    }

    #[test]
    fn mutate() {
        let value = [1, 3, 3, 7];
        let mut cell = AliasedCell::new(Box::new(value));
        unsafe {
            mutate_value(cell.alias_mut().as_mut());
        }
        let new_value = unsafe { *cell.into_inner() };
        assert_eq!(new_value, [1, 10, 3, 7]);
    }

    /// Verify that we can take multiple aliases.
    #[test]
    fn mutate_twice() {
        let value = [1, 3, 3, 7];
        let mut cell = AliasedCell::new(Box::new(value));
        unsafe {
            mutate_value(cell.alias_mut().as_mut());
        }
        unsafe {
            mutate_value(cell.alias_mut().as_mut());
        }
        let new_value = unsafe { *cell.into_inner() };
        assert_eq!(new_value, [1, 17, 3, 7]);
    }

    /// Verify that we can do basic safe manipulations between unsafe blocks.
    #[test]
    fn moves() {
        let value = [1, 3, 3, 7];
        let mut cell = AliasedCell::new(Box::new(value));
        unsafe {
            mutate_value(cell.alias_mut().as_mut());
        }
        let mut cell2 = cell;
        unsafe {
            mutate_value(cell2.alias_mut().as_mut());
        }
        let cell3 = cell2;
        let new_value = unsafe { *cell3.into_inner() };
        assert_eq!(new_value, [1, 17, 3, 7]);
    }

    /// Verify that alias can be used at a later point.
    #[test]
    fn mutate_deferred() {
        let value = [1, 3, 3, 7];
        let mut cell = AliasedCell::new(Box::new(value));
        let mut mutator = unsafe { Mutator::new(cell.alias_mut().as_mut()) };
        unsafe {
            mutator.mutate();
        }
        let new_value = unsafe { *cell.into_inner() };
        assert_eq!(new_value, [1, 10, 3, 7]);
    }

    #[test]
    fn mutate_deferred_twice() {
        let value = [1, 3, 3, 7];
        let mut cell = AliasedCell::new(Box::new(value));
        let mut mutator = unsafe { Mutator::new(cell.alias_mut().as_mut()) };
        unsafe {
            mutator.mutate();
        }
        unsafe {
            mutator.mutate();
        }
        let new_value = unsafe { *cell.into_inner() };
        assert_eq!(new_value, [1, 17, 3, 7]);
    }

    /// Further safe manipulations.
    #[test]
    fn deferred_moves() {
        let value = [1, 3, 3, 7];
        let mut cell = AliasedCell::new(Box::new(value));
        let mutator = unsafe { Mutator::new(cell.alias_mut().as_mut()) };
        let cell2 = cell;
        let mut mutator2 = mutator;
        unsafe {
            mutator2.mutate();
        }
        let cell3 = cell2;
        let _mutator3 = mutator2;
        let new_value = unsafe { *cell3.into_inner() };
        assert_eq!(new_value, [1, 10, 3, 7]);
    }

    /// Non-trivial safe manipulation.
    #[test]
    fn safe_frobbing() {
        let value = [1, 3, 3, 7];
        let mut cell = AliasedCell::new(Box::new(value));
        let mut mutator = unsafe { Mutator::new(cell.alias_mut().as_mut()) };
        unsafe {
            mutator.mutate();
        }
        mutator.ascend();
        unsafe {
            mutator.mutate();
        }
        let new_value = unsafe { *cell.into_inner() };
        assert_eq!(new_value, [1, 10, 3, 10]);
    }

    /// Verify that two aliases can exist simultaneously.
    #[test]
    fn two_mutators() {
        let value = [1, 3, 3, 7];
        let mut cell = AliasedCell::new(Box::new(value));
        let mut mutator1 = unsafe { Mutator::new(cell.alias_mut().as_mut()) };
        unsafe {
            mutator1.mutate();
        }
        let mut mutator2 = unsafe { Mutator::new(cell.alias_mut().as_mut()) };
        unsafe {
            mutator2.mutate();
        }
        let new_value = unsafe { *cell.into_inner() };
        assert_eq!(new_value, [1, 17, 3, 7]);
    }

    #[test]
    #[should_panic(
        expected = "Trying to drop an AliasedCell, which may still have aliases outstanding."
    )]
    fn invalid_drop() {
        let value = [1, 3, 3, 7];
        let mut cell = AliasedCell::new(Box::new(value));
        unsafe {
            let mut mutator = Mutator::new(cell.alias_mut().as_mut());
            mutator.mutate();
            drop(cell);
        }
    }

    /// Verify that we skip the panic-on-drop while unwinding from another panic.
    #[test]
    #[should_panic(expected = "bye!")]
    fn panic() {
        let value = [1, 3, 3, 7];
        let mut cell = AliasedCell::new(Box::new(value));
        unsafe {
            let mut mutator = Mutator::new(cell.alias_mut().as_mut());
            mutator.mutate();
            panic!("bye!");
        }
    }
}
