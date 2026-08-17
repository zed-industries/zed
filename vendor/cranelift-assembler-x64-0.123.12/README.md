# `cranelift-assembler-x64`

A Cranelift-specific x64 assembler. Unlike the existing `cranelift-codegen`
assembler, this assembler uses instructions, not instruction classes, as the
core abstraction.

### Use

Like `cranelift-codegen`, using this assembler starts with `enum Inst`. For
convenience, a `main.rs` script prints the path to this generated code:

```console
$ cat $(cargo run) | head
...
pub enum Inst<R:Registers> {
    andb_i(andb_i),
    andw_i(andw_i),
    andl_i(andl_i),
    ...
```

### Test

In order to check that this assembler emits correct machine code, we fuzz it
against a known-good disassembler. We can run a quick, one-second check:

```console
$ cargo test -- --nocapture
```

Or we can run the fuzzer indefinitely:

```console
$ cargo +nightly fuzz run -s none roundtrip -j16
```

