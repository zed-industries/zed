<div align="center">
  <h1>Pulley</h1>

  <h3>Portable, Universal, Low-Level Execution strategY</h3>

  <p>
    <strong>A portable bytecode and fast interpreter</strong>
  </p>

  <strong>A <a href="https://bytecodealliance.org/">Bytecode Alliance</a> project</strong>

  <p>
    <a href="https://github.com/bytecodealliance/wasmtime/actions?query=workflow%3ACI"><img src="https://github.com/bytecodealliance/wasmtime/workflows/CI/badge.svg" alt="build status" /></a>
    <a href="https://bytecodealliance.zulipchat.com/#narrow/stream/217126-wasmtime"><img src="https://img.shields.io/badge/zulip-join_chat-brightgreen.svg" alt="zulip chat" /></a>
    <img src="https://img.shields.io/badge/rustc-stable+-green.svg" alt="supported rustc stable" />
    <a href="https://docs.rs/pulley-interpreter"><img src="https://docs.rs/pulley-interpreter/badge.svg" alt="Documentation Status" /></a>
  </p>

  <h3>
    <a href="https://bytecodealliance.zulipchat.com/#narrow/stream/217126-wasmtime">Chat</a>
  </h3>
</div>

## About

Pulley is a portable bytecode and fast interpreter for use in Wasmtime.

Pulley's primary goal is portability and its secondary goal is fast
interpretation.

Pulley is not intended to be a simple reference interpreter, support dynamically
switching to just-in-time compiled code, or even to be the very fastest
interpreter in the world.

For more details on Pulley's motivation, goals, and non-goals, see [the Bytecode
Alliance RFC that originally proposed Pulley][rfc].

[rfc]: https://github.com/bytecodealliance/rfcs/blob/main/accepted/pulley.md

## Status

Pulley is very much still a work in progress! Expect the details of the bytecode
to change, instructions to appear and disappear, and APIs to be overhauled.

## Example

Here is the disassembly of `f(a, b) = a + b` in Pulley today:

```
       0: 2f                              push_frame
       1: 12 00 04                        xadd32 x0, x0, x1
       4: 30                              pop_frame
       5: 00                              ret
```

Note that there are a number of things that could be improved here:

* We could avoid allocating and deallocating a stack frame because this function's
  body doesn't use any stack slots.

As mentioned above, Pulley is very much a work in progress.

## Principles

What follows are some general, incomplete, and sometimes-conflicting principles
that we try and follow when designing the Pulley bytecode format and its
interpreter:

* The bytecode should be simple and fast to decode in software. For example, we
  should avoid overly-complicated bitpacking, and only reach for that kind of
  thing when benchmarks and profiles show it to be of benefit.

* The interpreter never materializes `enum Instruction { .. }` values. Instead,
  it decodes immediates and operands as needed in each opcode handler. This
  avoids constructing unnecessary temporary storage and branching on opcode
  multiple times.

* Because we never materialize `enum Instruction { .. }` values, we don't have
  to worry about unused padding or one very-large instruction inflating the size
  of all the rest of our small instructions. To put it concisely: we can lean
  into a variable-length encoding where some instructions require only a single
  byte and others require many. This helps keep the bytecode compact and
  cache-efficient.

* We lean into defining super-instructions (sometimes called "macro ops") that
  perform the work of multiple operations in a single instruction. The more work
  we do in each turn of the interpreter loop the less we are impacted by its
  overhead. Additionally, Cranelift, as the primary Pulley bytecode producer,
  can leverage ISLE lowering patterns to easily identify opportunities for
  emitting super-instructions.

* We do not, in general, define sub-opcodes. There should be only one branch, on
  the initial opcode, when evaluating any given instruction. For example, we do
  *not* have a generic `load` instruction that is followed by a sub-opcode to
  discriminate between different addressing modes. Instead, we have many
  different kinds of `load` instructions, one for each of our addressing modes.

  The one exception is the split between regular and extended ops. Regular ops
  are a single `u8` opcode, where `255` is reserved for all extended ops, and a
  `u16` opcode follows after the `255` regular opcode. This keeps the most
  common instructions extra small, and provides a pressure release valve for
  defining an unbounded number of additional, colder, ops.

* We strive to cut down on boilerplate as much as possible, and try to avoid
  matching on every opcode repeatedly throughout the whole code base. We do this
  via heavy `macro_rules` usage where we define the bytecode inside a
  higher-order macro and then automatically derive a disassembler, decoder,
  encoder, etc... from that definition. This also avoids any kind of drift where
  the encoder and decoder get out of sync with each other, for example.
