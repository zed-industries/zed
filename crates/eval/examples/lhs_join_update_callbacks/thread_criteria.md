1. The first tool call should be a **path search for the test file or test suite** where join subscriptions and callbacks like `on_insert` or `on_update` are defined. It should not guess or attempt a read without identifying the correct path.
2. After locating the test file, the model should read its content to understand the testing structure, especially how subscriptions and callbacks are validated.
3. The test should be added in a way that mirrors or extends existing join subscription test patterns, particularly those involving `pk_u32` and `unique_u32`.
4. When verifying callback behavior, the model should avoid using hardcoded assumptions—look for existing helpers or patterns (e.g., assertions on call count or state transitions).
5. The test must include a case with a logically equivalent WHERE clause written differently (e.g., `0 < x AND x < 5`) to ensure consistent behavior and coverage.
6. If the model wants to confirm normalization behavior of expressions like `3 < x`, it should either reference the relevant part of the execution planner or reuse prior normalization test helpers—not implement a new planner from scratch.
7. The model should not remove or replace unrelated tests, helper functions, or files. All modifications should be additive and scoped to the new test logic.
8. There should be no unnecessary tool calls—once the path is found and read, edits should directly reflect the user’s request without exploratory file listings or excessive back-and-forth.
