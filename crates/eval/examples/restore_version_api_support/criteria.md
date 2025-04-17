1. The `restore` method is updated across Python and Rust components of LanceDB to accept an optional `version` argument, enabling more flexible restoration of historical table versions.
2. Python async bindings in `_lancedb.pyi` and `table.py` are updated to reflect the new method signature `restore(version: Optional[int] = None)`, aligning type hints and implementations.
3. The remote table interface in `remote/table.py` includes a corresponding `restore` method, bridging the sync API to the async backend with version support.
4. The Rust FFI layer (`table.rs`) is modified to accept the optional `version` argument, with logic that performs a `checkout(version)` if specified, before proceeding to `restore()`, improving control over the restore flow.
5. The `RemoteTable` implementation in `remote/table.rs` now constructs and sends a versioned restore request via HTTP, enabling client-side version-specific restoration even in cloud deployments.
6. Docstrings and comments are added or expanded to explain the behavior of the `restore` function, particularly the no-op case when restoring the latest version, enhancing code maintainability and developer understanding.
