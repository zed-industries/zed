use crate::runtime::vm::vmcontext::{
    VMFunctionImport, VMGlobalImport, VMMemoryImport, VMTableImport, VMTagImport,
};

/// Resolved import pointers.
///
/// Note that some of these fields are slices, not `PrimaryMap`. They should be
/// stored in index-order as with the module that we're providing the imports
/// for, and indexing is all done the same way as the main module's index
/// spaces.
///
/// Also note that the way we compile modules means that for the module linking
/// proposal all `alias` directives should map to imported items. This means
/// that each of these items aren't necessarily directly imported, but may be
/// aliased.
#[derive(Default)]
pub struct Imports<'a> {
    /// Resolved addresses for imported functions.
    pub functions: &'a [VMFunctionImport],

    /// Resolved addresses for imported tables.
    pub tables: &'a [VMTableImport],

    /// Resolved addresses for imported memories.
    pub memories: &'a [VMMemoryImport],

    /// Resolved addresses for imported globals.
    pub globals: &'a [VMGlobalImport],

    /// Resolved addresses for imported tags.
    pub tags: &'a [VMTagImport],
}
