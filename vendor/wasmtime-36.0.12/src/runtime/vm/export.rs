/// The value of an export passed from one instance to another.
pub enum Export {
    /// A function export value.
    Function(crate::Func),

    /// A table export value.
    Table(crate::Table),

    /// A memory export value.
    Memory { memory: crate::Memory, shared: bool },

    /// A global export value.
    Global(crate::Global),

    /// A tag export value.
    Tag(crate::Tag),
}
