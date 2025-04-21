1. The first tool call should search for the file containing the text generation entry point—most likely something like `main.rs`, `cli.rs`, or another file in the binary or CLI layer—rather than guessing and reading files blindly.
2. When modifying argument parsing, the model should not remove fields like `temperature` or `top_p`—instead, it should set reasonable default values and ensure they are non-optional in downstream code.
3. When changing `verbose_prompt` to `verbose`, the model must update all places in the codebase where `verbose_prompt` was previously referenced, not just the CLI argument itself.
4. When updating how optional paths (`cache_path`, `weight_path`) are handled, the logic should gracefully fall back to defaults rather than panic or unwrap without checks.
5. Deserialization from a JSON config should be added using Serde. The tool should avoid hardcoding configuration values and should prefer loading from a file with a fallback to sensible defaults using helper functions.
6. The `Config` struct should be extended with a `rope_ratio` field, which includes a `Default` implementation or similar mechanism (e.g. `fn default_rope_ratio() -> f32`) to allow for clean deserialization.
7. Any reordering of imports or cleanup should not introduce functional regressions or changes to logic; these changes should only enhance code clarity and consistency.
8. The model should avoid spinning—repeated unnecessary tool calls such as rereading the same files or re-requesting already loaded information—and should move forward once relevant context has been gathered.
