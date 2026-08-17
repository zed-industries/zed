//! Implementation of matching (structural subtyping) for core Wasm types.
//!
//! We only ever do structural matching for one link at a time in a subtype
//! chain. That is, we never recurse on new `SubType`s. This is because
//! subtyping relations are required to be declared, so the earlier links in the
//! chain were already checked when we processed those declarations.
//!
//! Note that while we don't recursively *match* on each sub- and supertype field
//! when checking whether a struct type matches another struct type, we do check
//! that either `field_type_a == field_type b` or that it was previously
//! declared that `field_type a <: field_type b`. The latter case means that we
//! previously checked that they matched when we saw the declaration, and we
//! don't need to match again; we just look at the declarations from now on.

use crate::{
    ArrayType, CompositeInnerType, CompositeType, ContType, FieldType, FuncType, RefType,
    StorageType, StructType, SubType, ValType,
    types::{CoreTypeId, RecGroupId, TypeList},
};

/// Wasm type matching.
pub trait Matches {
    /// Does `a` structurally match `b`?
    ///
    /// Both `a` and `b` must be canonicalized already.
    ///
    /// This is expected to recursively break down and inspect the *parts* of
    /// `Self` but should always bottom out in subtype checks, rather than
    /// looping back to new match calls on a *whole* new `Self`. This is what
    /// maintains the "single-link-in-the-chain" property mentioned in the
    /// module comment above.
    fn matches(types: &TypeList, a: Self, b: Self) -> bool;
}

/// A `T` with its containing `RecGroupId`.
///
/// The `RecGroupId` can be used to resolve canonicalized type references that
/// are indices into the local rec group.
#[derive(Debug, Copy, Clone)]
pub(crate) struct WithRecGroup<T> {
    inner: T,
    rec_group_id: RecGroupId,
}

impl<T> WithRecGroup<T> {
    #[inline]
    fn rec_group(x: Self) -> RecGroupId {
        x.rec_group_id
    }
}

impl<T> core::ops::Deref for WithRecGroup<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<T> core::ops::DerefMut for WithRecGroup<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl WithRecGroup<CoreTypeId> {
    /// Construct a new `WithRecGroup<CoreTypeId>` by looking up the
    /// `CoreTypeId`'s rec group id in the `TypeList`.
    pub(crate) fn new(types: &TypeList, id: CoreTypeId) -> Self {
        let rec_group_id = types.rec_group_id_of(id);
        WithRecGroup {
            inner: id,
            rec_group_id,
        }
    }
}

impl<T> WithRecGroup<T> {
    /// Project into a field of the inner value, while maintaining the
    /// `RecGroupId` context.
    pub(crate) fn map<U>(x: Self, f: impl FnOnce(T) -> U) -> WithRecGroup<U> {
        WithRecGroup {
            inner: f(x.inner),
            rec_group_id: x.rec_group_id,
        }
    }
}

impl<'a> Matches for WithRecGroup<&'a SubType> {
    fn matches(types: &TypeList, a: Self, b: Self) -> bool {
        // NB: matching does not check finality and supertypes. That is checked
        // once when we define types, not repeatedly every time we check
        // matches.
        Matches::matches(
            types,
            WithRecGroup::map(a, |a| &a.composite_type),
            WithRecGroup::map(b, |b| &b.composite_type),
        )
    }
}

impl<'a> Matches for WithRecGroup<&'a CompositeType> {
    fn matches(types: &TypeList, a: Self, b: Self) -> bool {
        use CompositeInnerType::*;
        if (*a).shared != (*b).shared {
            return false;
        }
        match (&(*a).inner, &(*b).inner) {
            (Func(fa), Func(fb)) => Matches::matches(
                types,
                WithRecGroup::map(a, |_| fa),
                WithRecGroup::map(b, |_| fb),
            ),
            (Func(_), _) => false,

            (Array(aa), Array(ab)) => Matches::matches(
                types,
                WithRecGroup::map(a, |_| *aa),
                WithRecGroup::map(b, |_| *ab),
            ),
            (Array(_), _) => false,

            (Struct(sa), Struct(sb)) => Matches::matches(
                types,
                WithRecGroup::map(a, |_| sa),
                WithRecGroup::map(b, |_| sb),
            ),
            (Struct(_), _) => false,

            (Cont(ca), Cont(cb)) => Matches::matches(
                types,
                WithRecGroup::map(a, |_| ca),
                WithRecGroup::map(b, |_| cb),
            ),
            (Cont(_), _) => false,
        }
    }
}

impl<'a> Matches for WithRecGroup<&'a FuncType> {
    fn matches(types: &TypeList, a: Self, b: Self) -> bool {
        if a.params().len() != b.params().len() || a.results().len() != b.results().len() {
            return false;
        }

        // A quick recap of covariance, contravariance, and how it applies to
        // subtyping function types, because I (and almost everyone else, it
        // seems) always need a reminder:
        //
        // *Covariance* is when `a <: b` and `a' <: b'`. That is, the subtyping
        // checks on the nested things (`a'` and `b'`) goes the same way as the
        // outer things (`a` and `b`).
        //
        // *Contravariance* is when `a <: b` and `b' <: a'`. That is, the
        // subtyping on the nested things is reversed compared to the outer
        // things.
        //
        // Now, when we are checking subtyping for function types, we have the
        // following variance:
        //
        // * Parameters are contravariant: `(a -> c) <: (b -> c)` when `b <: a`.
        //
        //   For example, we can substitute a `Cat -> Cat` function with a
        //   `Animal -> Cat` function because `Cat <: Animal` and so all
        //   arguments that could be given to the function are still valid.
        //
        //   We can't do the opposite and replace an `Animal -> Cat` function
        //   with a `Cat -> Cat` function. What would the `Cat -> Cat` function
        //   do when given a `Dog`? It is unsound.
        //
        // * Results are covariant: `(a -> b) <: (a -> c)` when `b <: c`.
        //
        //   For example, we can substitute a `Cat -> Animal` function with a
        //   `Cat -> Cat` function because callers expect to be returned an
        //   `Animal` and all `Cat`s are `Animal`s. (Also: all `Cat`s are
        //   `Beautiful`!)
        //
        //   We cannot do the opposite and substitute a `Cat -> Cat` function
        //   with a `Cat -> Animal` function, since callers expect a `Cat` but
        //   the new function could return a `Pig`.
        //
        // As always, Wikipedia is also helpful:
        // https://en.wikipedia.org/wiki/Covariance_and_contravariance_(computer_science)

        let params_match = a.params().iter().zip(b.params()).all(|(pa, pb)| {
            // Parameters are contravariant.
            Matches::matches(
                types,
                WithRecGroup::map(b, |_| *pb),
                WithRecGroup::map(a, |_| *pa),
            )
        });
        if !params_match {
            return false;
        }

        a.results().iter().zip(b.results()).all(|(ra, rb)| {
            // Results are covariant.
            Matches::matches(
                types,
                WithRecGroup::map(a, |_| *ra),
                WithRecGroup::map(b, |_| *rb),
            )
        })
    }
}

impl Matches for WithRecGroup<ArrayType> {
    fn matches(types: &TypeList, a: Self, b: Self) -> bool {
        Matches::matches(
            types,
            WithRecGroup::map(a, |a| a.0),
            WithRecGroup::map(b, |b| b.0),
        )
    }
}

impl<'a> Matches for WithRecGroup<&'a StructType> {
    fn matches(types: &TypeList, a: Self, b: Self) -> bool {
        // Note: Struct types support width and depth subtyping.
        a.fields.len() >= b.fields.len()
            && a.fields.iter().zip(b.fields.iter()).all(|(fa, fb)| {
                Matches::matches(
                    types,
                    WithRecGroup::map(a, |_| *fa),
                    WithRecGroup::map(b, |_| *fb),
                )
            })
    }
}

impl Matches for WithRecGroup<FieldType> {
    fn matches(types: &TypeList, a: Self, b: Self) -> bool {
        // `a`'s storage type must match `b`'s storage type.
        if !Matches::matches(
            types,
            WithRecGroup::map(a, |a| a.element_type),
            WithRecGroup::map(b, |b| b.element_type),
        ) {
            return false;
        }

        // And either both fields are immutable...
        if !a.mutable && !b.mutable {
            return true;
        }

        // Or both fields are mutable and `b`'s storage type must match `a`'s
        // storage type, a.k.a. they must be the exact same storage type.
        a.mutable
            && b.mutable
            && Matches::matches(
                types,
                WithRecGroup::map(b, |b| b.element_type),
                WithRecGroup::map(a, |a| a.element_type),
            )
    }
}

impl Matches for WithRecGroup<StorageType> {
    fn matches(types: &TypeList, a: Self, b: Self) -> bool {
        use StorageType as ST;
        match (*a, *b) {
            (ST::I8, ST::I8) | (ST::I16, ST::I16) => true,
            (ST::I8 | ST::I16, _) => false,

            (ST::Val(va), ST::Val(vb)) => Matches::matches(
                types,
                WithRecGroup::map(a, |_| va),
                WithRecGroup::map(b, |_| vb),
            ),
            (ST::Val(_), _) => false,
        }
    }
}

impl<'a> Matches for WithRecGroup<&'a ContType> {
    fn matches(types: &TypeList, a: Self, b: Self) -> bool {
        match (*a, *b) {
            (ContType(ca), ContType(cb)) => {
                ca == cb
                    || types.reftype_is_subtype_impl(
                        RefType::concrete(false, *ca),
                        Some(WithRecGroup::rec_group(a)),
                        RefType::concrete(false, *cb),
                        Some(WithRecGroup::rec_group(b)),
                    )
            }
        }
    }
}

impl Matches for WithRecGroup<ValType> {
    fn matches(types: &TypeList, a: Self, b: Self) -> bool {
        match (*a, *b) {
            (ValType::Ref(ra), ValType::Ref(rb)) => Matches::matches(
                types,
                WithRecGroup::map(a, |_| ra),
                WithRecGroup::map(b, |_| rb),
            ),
            (ValType::Ref(_), _) => false,

            (ValType::I32 | ValType::I64 | ValType::F32 | ValType::F64 | ValType::V128, _) => {
                *a == *b
            }
        }
    }
}

impl Matches for WithRecGroup<RefType> {
    fn matches(types: &TypeList, a: Self, b: Self) -> bool {
        types.reftype_is_subtype_impl(
            *a,
            Some(WithRecGroup::rec_group(a)),
            *b,
            Some(WithRecGroup::rec_group(b)),
        )
    }
}
