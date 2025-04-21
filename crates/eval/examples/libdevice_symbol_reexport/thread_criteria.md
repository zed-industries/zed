1. The first tool call should be a path search to locate the file or module that defines or contains `LIBDEVICE_BITCODE`. This is necessary to confirm the current source of the symbol (`cust_raw::nvvm_sys`) before refactoring.
2. Once the relevant path (e.g., inside `cust_raw::nvvm_sys`) is confirmed, the model should read the contents of that file to identify how `LIBDEVICE_BITCODE` is currently defined or reexported.
3. The model should then path search or navigate to the `nvvm` crate root (e.g., `crates/nvvm/src/lib.rs`) and modify it to include a `pub use` statement that reexports `LIBDEVICE_BITCODE` publicly from its original location.
4. The model should locate all usages of `cust_raw::nvvm_sys::LIBDEVICE_BITCODE` in the `rustc_codegen_nvvm` crate and replace them with the new `nvvm::LIBDEVICE_BITCODE` path.
5. The `rustc_codegen_nvvm/Cargo.toml` file should be read and modified to remove the dependency on `cust_raw` if it is no longer used directly.
6. After updating imports, the model should clean up any `use` statements in `rustc_codegen_nvvm` files that reference `cust_raw` and are now redundant.
7. At no point should the model attempt to remove or spin up irrelevant files or dependencies—tool use should be precise, focused on `nvvm`, `cust_raw`, and `rustc_codegen_nvvm` only.
8. The model must not attempt to guess paths or rely on heuristics when looking for crate files—it should perform path search or directory listing when uncertain.
