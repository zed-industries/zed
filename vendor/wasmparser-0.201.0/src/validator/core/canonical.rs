//! Canonicalization of types.
//!
//! The unit of canonicalization is a recursion group. Having "unnecessary"
//! types in a recursion group can "break" canonicalization of other types
//! within that same recursion group, as can reordering types within a recursion
//! group.
//!
//! It is an invariant that all types defined before the recursion group we are
//! currently canonicalizing have already been canonicalized themselves.
//!
//! Canonicalizing a recursion group then proceeds as follows:
//!
//! * First we walk each of its `SubType` elements and put their type references
//!   (i.e. their `PackedIndex`es) into canonical form. Canonicalizing a
//!   `PackedIndex` means switching it from indexing into the Wasm module's
//!   types space into either
//!
//!   1. Referencing an already-canonicalized type, for types outside of this
//!      recursion group. Because inter-group type references can only go
//!      towards types defined before this recursion group, we know the type is
//!      already canonicalized and we have a `CoreTypeId` for each of those
//!      types. This updates the `PackedIndex` into a `CoreTypeId`.
//!
//!   2. Indexing into the current recursion group, for intra-group type
//!      references.
//!
//!   Note that (2) has the effect of making the "same" structure of mutual type
//!   recursion look identical across recursion groups:
//!
//!   ```wat
//!   ;; Before
//!   (rec (struct (field (module-type 1))) (struct (field (module-type 0))))
//!   (rec (struct (field (module-type 3))) (struct (field (module-type 2))))
//!
//!   ;; After
//!   (rec (struct (field (rec-group-type 1))) (struct (field (rec-group-type 0))))
//!   (rec (struct (field (rec-group-type 1))) (struct (field (rec-group-type 0))))
//!   ```
//!
//! * Now that the recursion group's elements are in canonical form, we can
//!   "simply" hash cons whole rec groups at a time. The `TypesList` morally
//!   maintains a hash map from `Vec<SubType>` to `RecGroupId` and we can do
//!   get-or-create operations on it. I say "morally" because we don't actually
//!   duplicate the `Vec<SubType>` key in that hash map since those elements are
//!   already stored in the `TypeList`'s internal `SnapshotList<CoreType>`. This
//!   means we need to do some low-level hash table fiddling with the
//!   `hashbrown` crate.
//!
//! And that's it! That is the whole canonicalization algorithm.
//!
//! Some more random things to note:
//!
//! * Because we essentially already have to do the check to canonicalize, and
//!   to avoid additional passes over the types, the canonicalization pass also
//!   checks that type references are in bounds. These are the only errors that
//!   can be returned from canonicalization.
//!
//! * Canonicalizing requires the `Module` to translate type indices to
//!   actual `CoreTypeId`s.
//!
//! * It is important that *after* we have canonicalized all types, we don't
//!   need the `Module` anymore. This makes sure that we can, for example,
//!   intern all types from the same store into the same `TypeList`. Which in
//!   turn lets us type check function imports of a same-store instance's
//!   exported functions and we don't need to translate from one module's
//!   canonical representation to another module's canonical representation or
//!   perform additional expensive checks to see if the types match or not
//!   (since the whole point of canonicalization is to avoid that!).

use super::{Module, RecGroupId, TypeAlloc};
use crate::{
    types::{CoreTypeId, TypeIdentifier},
    PackedIndex, RecGroup, Result, UnpackedIndex, WasmFeatures,
};

/// Canonicalize the rec group and return its id and whether it is a new group
/// (we added its types to the `TypeAlloc`) or not (we deduplicated it with an
/// existing canonical rec group).
pub(crate) fn canonicalize_and_intern_rec_group(
    features: &WasmFeatures,
    types: &mut TypeAlloc,
    module: &Module,
    mut rec_group: RecGroup,
    offset: usize,
) -> Result<(bool, RecGroupId)> {
    TypeCanonicalizer::new(module, offset)
        .with_features(features)
        .canonicalize_rec_group(&mut rec_group)?;
    Ok(types.intern_canonical_rec_group(rec_group))
}

/// The kind of canonicalization we are doing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CanonicalizationMode {
    /// Standard canonicalization: turns module indices into either (1)
    /// `CoreTypeId`s for inter-group references or (2) rec-group-local indices
    /// for intra-group references.
    HashConsing,

    /// Turns all type reference indices into `CoreTypeId`s, even from within
    /// the same rec group. Not useful for hash consing, but useful when
    /// exposing types to end users so they don't have to deal with
    /// rec-group-local indices.
    OnlyIds,
}

pub(crate) struct TypeCanonicalizer<'a> {
    module: &'a Module,
    features: Option<&'a WasmFeatures>,
    rec_group_start: u32,
    rec_group_len: u32,
    offset: usize,
    mode: CanonicalizationMode,
    within_rec_group: Option<std::ops::Range<CoreTypeId>>,
}

impl<'a> TypeCanonicalizer<'a> {
    pub fn new(module: &'a Module, offset: usize) -> Self {
        // These defaults will work for when we are canonicalizing types from
        // outside of a rec group definition, forcing all `PackedIndex`es to be
        // canonicalized to `CoreTypeId`s.
        let rec_group_start = u32::MAX;
        let rec_group_len = 0;

        Self {
            module,
            features: None,
            rec_group_start,
            rec_group_len,
            offset,
            mode: CanonicalizationMode::HashConsing,
            within_rec_group: None,
        }
    }

    pub fn with_features(&mut self, features: &'a WasmFeatures) -> &mut Self {
        debug_assert!(self.features.is_none());
        self.features = Some(features);
        self
    }

    fn allow_gc(&self) -> bool {
        self.features.map_or(true, |f| f.gc)
    }

    fn canonicalize_rec_group(&mut self, rec_group: &mut RecGroup) -> Result<()> {
        // Re-initialize these fields so that we properly canonicalize
        // intra-rec-group type references into indices into the rec group
        // rather than as `CoreTypeId`s.
        self.rec_group_start = u32::try_from(self.module.types.len()).unwrap();
        self.rec_group_len = u32::try_from(rec_group.types().len()).unwrap();

        for (rec_group_local_index, ty) in rec_group.types_mut().enumerate() {
            let rec_group_local_index = u32::try_from(rec_group_local_index).unwrap();
            let type_index = self.rec_group_start + rec_group_local_index;

            if let Some(sup) = ty.supertype_idx.as_mut() {
                if sup.as_module_index().map_or(false, |i| i >= type_index) {
                    bail!(self.offset, "supertypes must be defined before subtypes");
                }
            }

            ty.remap_indices(&mut |idx| self.canonicalize_type_index(idx))?;
        }

        Ok(())
    }

    fn canonicalize_type_index(&self, ty: &mut PackedIndex) -> Result<()> {
        match ty.unpack() {
            UnpackedIndex::Id(_) => return Ok(()),
            UnpackedIndex::Module(index) => {
                if index < self.rec_group_start || self.mode == CanonicalizationMode::OnlyIds {
                    let id = self.module.type_id_at(index, self.offset)?;
                    if let Some(id) = PackedIndex::from_id(id) {
                        *ty = id;
                        return Ok(());
                    } else {
                        bail!(
                            self.offset,
                            "implementation limit: too many types in `TypeList`"
                        )
                    }
                }

                // When GC is not enabled the `rec_group_len == 1` so any rec group
                // local type references will be direct self references. But any kind of
                // type recursion, including self references, is not allowed in the
                // typed function references proposal, only the GC proposal.
                debug_assert!(self.allow_gc() || self.rec_group_len == 1);
                let local = index - self.rec_group_start;
                if self.allow_gc() && local < self.rec_group_len {
                    if let Some(id) = PackedIndex::from_rec_group_index(local) {
                        *ty = id;
                        return Ok(());
                    } else {
                        bail!(
                            self.offset,
                            "implementation limit: too many types in a recursion group"
                        )
                    }
                }

                bail!(
                    self.offset,
                    "unknown type {index}: type index out of bounds"
                )
            }
            UnpackedIndex::RecGroup(local_index) => match self.mode {
                CanonicalizationMode::HashConsing => Ok(()),
                CanonicalizationMode::OnlyIds => {
                    let rec_group_elems = self.within_rec_group.as_ref().expect(
                        "configured to canonicalize all type reference indices to `CoreTypeId`s \
                         and found rec-group-local index, but missing `within_rec_group` context",
                    );

                    let rec_group_len = rec_group_elems.end.index() - rec_group_elems.start.index();
                    let rec_group_len = u32::try_from(rec_group_len).unwrap();
                    assert!(local_index < rec_group_len);

                    let rec_group_start = u32::try_from(rec_group_elems.start.index()).unwrap();

                    let id = CoreTypeId::from_index(rec_group_start + local_index);
                    *ty = PackedIndex::from_id(id).expect(
                        "should fit in impl limits since we already have the end of the rec group \
                         constructed successfully",
                    );
                    Ok(())
                }
            },
        }
    }
}
