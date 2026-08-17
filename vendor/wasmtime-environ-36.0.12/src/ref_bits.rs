//! Definitions for bits in the in-memory / in-table representation of references.

/// An "initialized bit" in a funcref table.
///
/// We lazily initialize tables of funcrefs, and this mechanism
/// requires us to interpret zero as "uninitialized", triggering a
/// slowpath on table read to possibly initialize the element. (This
/// has to be *zero* because that is the only value we can cheaply
/// initialize, e.g. with newly mmap'd memory.)
///
/// However, the user can also store a null reference into a table. We
/// have to interpret this as "actually null", and not "lazily
/// initialize to the original funcref that this slot had".
///
/// To do so, we rewrite nulls into the "initialized null" value. Note
/// that this should *only exist inside the table*: whenever we load a
/// value out of a table, we immediately mask off the low bit that
/// contains the initialized-null flag. Conversely, when we store into
/// a table, we have to translate a true null into an "initialized
/// null".
///
/// We can generalize a bit in order to simply the table-set logic: we
/// can set the LSB of *all* explicitly stored values to 1 in order to
/// note that they are indeed explicitly stored. We then mask off this
/// bit every time we load.
///
/// Note that we take care to set this bit and mask it off when
/// accessing tables directly in fastpaths in generated code as well.
pub const FUNCREF_INIT_BIT: usize = 1;

/// The mask we apply to all refs loaded from funcref tables.
///
/// This allows us to use the LSB as an "initialized flag" (see below)
/// to distinguish from an uninitialized element in a
/// lazily-initialized funcref table.
pub const FUNCREF_MASK: usize = !FUNCREF_INIT_BIT;
