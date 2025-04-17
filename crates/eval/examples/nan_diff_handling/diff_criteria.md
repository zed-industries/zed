1. **NaN Comparison Logic Update**:
The diff modifies the comparison function to explicitly handle NaN values as equivalent. Previously, the function relied on string conversion for NaN comparison, but now it first checks if both values are NaN using Number.isNaN() before proceeding with other comparison logic. This change ensures consistent behavior when comparing NaN values in objects.
2. **New NaN Test Suite - Object Operations**:
A comprehensive test suite is added to verify NaN handling in object operations. The tests cover: creating new objects with NaN values, changing NaN values to other numbers, verifying no changes when NaN values remain the same, and removing properties with NaN values. Each test case validates the diff output structure and type of operation.
3. **New NaN Test Suite - Array Operations**:
The test suite extends to array operations with similar test cases as objects but adapted for array contexts. It tests: adding NaN to arrays, replacing NaN with other numbers, maintaining arrays with unchanged NaN values, and removing NaN elements from arrays. The tests ensure consistent behavior between object and array operations involving NaN values.
