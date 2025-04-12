1. The changes add support for I64 data type in the Metal backend's gather operation by adding new pattern matches for various DType combinations involving I64 (both as input and output types) in `metal_backend/mod.rs`.
2. The changes implement the corresponding Metal kernel operations for the new I64 gather functionality by adding new GATHER_OP macros in `indexing.metal`, including support for different combinations with I64 and handling of the BF16 case with preprocessor directives.
3. The changes fix two syntax issues in `scaled_dot_product_attention.metal`: removing an extra comma in a function call and removing an unnecessary ampersand in a method call.
4. The changes adjust the test tolerance in `test_ops.rs` from 1e-4 to 1e-3 for an assertion comparing actual and expected values.
