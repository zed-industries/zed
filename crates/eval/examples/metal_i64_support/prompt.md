<context> The following items were attached by the user. User the read_file tool to load the context into the thread. Load the files before doing anything else.

<files>
candle-examples/examples/codegeex4-9b/main.rs
candle-transformers/src/models/codegeex4_9b.rs
candle-transformers/src/models/glm4.rs
<files>
</context>

I'm currently making a set of code changes to improve the Metal backend in the Candle framework, focusing on expanding tensor operation support for additional data types. Specifically, I’m working on extending the gather operation to handle more dtype combinations, including support for i64—both as indices and values. This includes enabling combinations like u32 indices with i64 values, and i64 indices with types such as f32, f16, bf16, u32, and i64.

As part of this update, I’m also cleaning up minor syntax issues in the Metal kernels. This includes removing extra commas in function parameters and eliminating unnecessary ampersands in method calls within the scaled dot product attention code. One of the test tolerances may also require slight adjustment to account for acceptable numerical variance.

These changes span multiple files in candle-core and candle-metal-kernels, following the current macro-based pattern used for Metal shader definitions and their Rust bindings. Could you take a look and let me know if this approach aligns with the framework’s design goals or if there are other factors I should consider after making the code changes for me?
