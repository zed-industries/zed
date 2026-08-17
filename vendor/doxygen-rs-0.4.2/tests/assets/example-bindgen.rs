extern "C" {
    #[must_use]
    #[doc = "@name Memory management"]
    #[doc = "@{"]
    #[doc = "**"]
    #[doc = "* @brief Controls memory mapping"]
    #[doc = "* @param[out] addr_out The virtual address resulting from the operation. Usually the same as addr0."]
    #[doc = "* @param addr0    The virtual address to be used for the operation."]
    #[doc = "* @param addr1    The virtual address to be (un)mirrored by @p addr0 when using @ref MEMOP_MAP or @ref MEMOP_UNMAP."]
    #[doc = "*                 It has to be pointing to a RW memory."]
    #[doc = "*                 Use NULL if the operation is @ref MEMOP_FREE or @ref MEMOP_ALLOC."]
    #[doc = "* @param size     The requested size for @ref MEMOP_ALLOC and @ref MEMOP_ALLOC_LINEAR."]
    #[doc = "* @param op       Operation flags. See @ref MemOp."]
    #[doc = "* @param perm     A combination of @ref MEMPERM_READ and @ref MEMPERM_WRITE. Using MEMPERM_EXECUTE will return an error."]
    #[doc = "*                 Value 0 is used when unmapping memory."]
    #[doc = "*"]
    #[doc = "* If a memory is mapped for two or more addresses, you have to use MEMOP_UNMAP before being able to MEMOP_FREE it."]
    #[doc = "* MEMOP_MAP will fail if @p addr1 was already mapped to another address."]
    #[doc = "*"]
    #[doc = "* More information is available at http://3dbrew.org/wiki/SVC#Memory_Mapping."]
    #[doc = "*"]
    #[doc = "* @sa svcControlProcessMemory"]
    #[doc = "*/"]
    pub fn svcControlMemory(
        addr_out: *mut u32_,
        addr0: u32_,
        addr1: u32_,
        size: u32_,
        op: MemOp,
        perm: MemPerm,
    ) -> Result;
}