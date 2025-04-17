Add a new test case to validate join subscription updates when the **LHS table is updated**, and ensure correct invocation of reducer callbacks. The test should:

- Subscribe to a join query with a filter involving fields from both tables (e.g., `SELECT p.* FROM pk_u32 p JOIN unique_u32 u ON p.n = u.n WHERE u.data > 0 AND u.data < 5`).
- Insert rows into both LHS (`pk_u32`) and RHS (`unique_u32`) that satisfy the join condition.
- Verify the initial subscription callback is triggered via `on_insert`.
- Update the LHS (`pk_u32`) such that the row remains part of the join result.
- Validate that:
  - `on_update` is invoked correctly.
  - An immediate follow-up update back to the original value also triggers `on_update`.
- Repeat the above with disjoint filters (e.g., `u.n != 1`) and confirm behavior remains correct.

Also, ensure that literal-first SQL expressions like `3 < x` are correctly interpreted and inverted in the physical execution plan (converted to `x > 3`) and behave identically during query evaluation and execution.
