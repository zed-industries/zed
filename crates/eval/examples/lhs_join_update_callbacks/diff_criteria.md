1. A `JOIN` query with conditions on both sides (LHS and RHS) correctly triggers subscription updates when only the LHS table is updated.
2. Callback functions (`on_insert`, `on_update`) are invoked exactly once and in the expected order.
3. Queries with logically equivalent WHERE conditions (e.g., `x > 0 and x < 5` vs. `0 < x and x < 5`) yield consistent subscription behavior.
4. Complex disjoint queries that restrict the RHS via additional constraints (e.g., `u.n != 1`) still properly identify matching LHS updates.
5. Type inference and expression normalization correctly handle literals on the left-hand side of binary operations in WHERE clauses.
6. Physical execution plans normalize expressions like `3 < l.x` into `l.x > 3` with appropriate operator inversion (`Lt ↔ Gt`, `Lte ↔ Gte`), maintaining logical correctness.
