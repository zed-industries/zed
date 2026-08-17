# ISLE: Instruction Selection / Lowering Expressions

ISLE is a domain specific language (DSL) for instruction selection and lowering
clif instructions to vcode's `MachInst`s in Cranelift.

ISLE is a statically-typed term-rewriting language. You define rewriting rules
that map input terms (clif instructions) into output terms (`MachInst`s). These
rules get compiled down into Rust source test that uses a tree of `match`
expressions as good as or better than what you would have written by hand.
