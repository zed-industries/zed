/// A summary of the amount of resources required to instantiate a particular
/// [`Module`][crate::Module] or [`Component`][crate::component::Component].
///
/// Example uses of this information:
///
/// * Determining whether your pooling allocator configuration supports
///   instantiating this module.
///
/// * Deciding how many of which `Module` you want to instantiate within a
///   fixed amount of resources, e.g. determining whether to create 5
///   instances of module `X` or 10 instances of module `Y`.
pub struct ResourcesRequired {
    /// The number of memories that are required.
    pub num_memories: u32,
    /// The maximum initial size required by any memory, in units of Wasm pages.
    pub max_initial_memory_size: Option<u64>,
    /// The number of tables that are required.
    pub num_tables: u32,
    /// The maximum initial size required by any table.
    pub max_initial_table_size: Option<u64>,
}

impl ResourcesRequired {
    #[cfg(feature = "component-model")]
    pub(crate) fn add(&mut self, other: &ResourcesRequired) {
        self.num_memories += other.num_memories;
        self.max_initial_memory_size =
            core::cmp::max(self.max_initial_memory_size, other.max_initial_memory_size);
        self.num_tables += other.num_tables;
        self.max_initial_table_size =
            core::cmp::max(self.max_initial_table_size, other.max_initial_table_size);
    }
}
