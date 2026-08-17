//! Strategies a caller may use to collect errors from subfunctions
//!
//! Currently, the strategies contained in this module are:
//!
//! -   [DontCare]: The caller will ignore any non-fatal errors in subfunction.
//!     [WriteErrorList::push] is effectively a no-op.
//!
//! -   [ErrorOccurred]: Keeps track of a single boolean about whether an error occurred or not.
//!     [WriteErrorList::push] essentially just sets a flag.
//!
//! -   [Sublist]: A full-fledged list of all non-fatal errors in subfunction. Will be mapped to
//!     the caller's error type with a map function and pushed into the caller's error list.
use {
    crate::{private, ErrorList, WriteErrorList},
    std::ops::{Deref, DerefMut},
};

/// A sublist that maps a list of errors into a parent error type
///
/// When an object of this type is dropped, it will call the given `MapFn` object with a
/// [`ErrorList<E>`][ErrorList] containing all the errors that were pushed into it. The map
/// function will be used to map that error list to a single `ParentErr` object which will then
/// be pushed onto the parent's error list.
///
/// This type implements [DerefMut] to an [ErrorList], so it can basically be thought of as
/// an [ErrorList] with a fancy destructor.
pub struct Sublist<'a, E, MapFn, Parent, ParentErr>
where
    MapFn: FnOnce(ErrorList<E>) -> ParentErr,
    Parent: WriteErrorList<ParentErr>,
{
    list: ErrorList<E>,
    map_fn_and_parent: Option<(MapFn, &'a mut Parent)>,
}

impl<'a, E, MapFn, Parent, ParentErr> Sublist<'a, E, MapFn, Parent, ParentErr>
where
    MapFn: FnOnce(ErrorList<E>) -> ParentErr,
    Parent: WriteErrorList<ParentErr>,
{
    /// Create a new sublist that maps a list of subfunction errors to the parent error
    ///
    /// `map_fn` is a function that accepts an `ErrorList<E>` and returns a `ParentErr`, which
    /// is then pushed into the parent's error list.
    ///
    /// It is recommended use [WriteErrorList::sublist] instead of this.
    pub fn new(map_fn: MapFn, parent: &'a mut Parent) -> Self {
        Self {
            list: ErrorList::default(),
            map_fn_and_parent: Some((map_fn, parent)),
        }
    }
}

impl<'a, E, MapFn, Parent, ParentErr> Drop for Sublist<'a, E, MapFn, Parent, ParentErr>
where
    MapFn: FnOnce(ErrorList<E>) -> ParentErr,
    Parent: WriteErrorList<ParentErr>,
{
    fn drop(&mut self) {
        if !self.list.is_empty() {
            let list = std::mem::take(&mut self.list);
            let (map_fn, parent) = self.map_fn_and_parent.take().unwrap();
            let parent_error = map_fn(list);
            parent.push(parent_error);
        }
    }
}

impl<'a, E, MapFn, Parent, ParentErr> Deref for Sublist<'a, E, MapFn, Parent, ParentErr>
where
    MapFn: FnOnce(ErrorList<E>) -> ParentErr,
    Parent: WriteErrorList<ParentErr>,
{
    type Target = ErrorList<E>;
    fn deref(&self) -> &Self::Target {
        &self.list
    }
}

impl<'a, E, MapFn, Parent, ParentErr> DerefMut for Sublist<'a, E, MapFn, Parent, ParentErr>
where
    MapFn: FnOnce(ErrorList<E>) -> ParentErr,
    Parent: WriteErrorList<ParentErr>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.list
    }
}

impl<'a, E, MapFn, Parent, ParentErr> private::Sealed<E>
    for Sublist<'a, E, MapFn, Parent, ParentErr>
where
    MapFn: FnOnce(ErrorList<E>) -> ParentErr,
    Parent: WriteErrorList<ParentErr>,
{
}

impl<'a, E, MapFn, Parent, ParentErr> WriteErrorList<E> for Sublist<'a, E, MapFn, Parent, ParentErr>
where
    MapFn: FnOnce(ErrorList<E>) -> ParentErr,
    Parent: WriteErrorList<ParentErr>,
{
    fn push(&mut self, error: E) {
        self.list.push(error)
    }
    fn subwriter<'sub, SubMapFn, SubErr: 'sub>(
        &'sub mut self,
        map_fn: SubMapFn,
    ) -> impl WriteErrorList<SubErr> + 'sub
    where
        SubMapFn: FnOnce(ErrorList<SubErr>) -> E + 'sub,
    {
        self.sublist(map_fn)
    }
}

/// An error list writer that ignores errors
///
/// Any call to [WriteErrorList::push] does nothing but drop the given error.
pub struct DontCare;

impl<E> private::Sealed<E> for DontCare {}

impl<E> WriteErrorList<E> for DontCare {
    fn push(&mut self, _error: E) {}
    fn subwriter<'sub, SubMapFn, SubErr: 'sub>(
        &'sub mut self,
        _map_fn: SubMapFn,
    ) -> impl WriteErrorList<SubErr> + 'sub
    where
        SubMapFn: FnOnce(ErrorList<SubErr>) -> E + 'sub,
    {
        DontCare
    }
}

/// An error list writer that only notes that an error occurred
///
/// [ErrorOccurred::as_bool] will return `true` if the subfunction encountered a non-fatal error
/// `false` otherwise
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct ErrorOccurred(bool);

impl ErrorOccurred {
    pub fn as_bool(&self) -> bool {
        self.0
    }
}

impl<E> private::Sealed<E> for ErrorOccurred {}

impl<E> WriteErrorList<E> for ErrorOccurred {
    fn push(&mut self, _error: E) {
        self.0 = true;
    }
    fn subwriter<'sub, SubMapFn, SubErr: 'sub>(
        &'sub mut self,
        _map_fn: SubMapFn,
    ) -> impl WriteErrorList<SubErr> + 'sub
    where
        SubMapFn: FnOnce(ErrorList<SubErr>) -> E + 'sub,
    {
        self
    }
}
