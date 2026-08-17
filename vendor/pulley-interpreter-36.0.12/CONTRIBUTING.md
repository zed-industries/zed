# Contributing

For general contribution to Wasmtime, see Wasmtime's [contributing docs][docs].

[docs]: https://docs.wasmtime.dev/contributing.html

## Adding an instruction to Pulley

So you want to add an instruction to Pulley. If you're reading this in the
not-so-distant future Pulley probably doesn't support all of WebAssembly yet and
you're interested in helping to improve the situation. This is intended to be a
small guide about how to add an instruction to Pulley through an early example
of doing so.

#### Choose a test to get passing

First off find a test in this repository, probably a `*.wast` test, which isn't
currently passing. Check out the `WastTest::should_fail` method in
`crates/wast-util/src/lib.rs` which has a list of `unsupported` tests for
Pulley. Here we're going to select `./tests/misc_testsuite/control-flow.wast`
as it's a reasonably small test.

#### See the test failure

Run this command:

```
$ cargo run --features pulley -- wast --target pulley64 ./tests/misc_testsuite/control-flow.wast
```

This builds the `wasmtime` CLI with Pulley support enabled (`--features
pulley`), runs the `wast` subcommand, executes with the pulley target
(`--target pulley64`), and then runs our test. As of now this shows:

```
$ cargo run --features pulley -- wast --target pulley64 ./tests/misc_testsuite/control-flow.wast
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/wasmtime wast --target pulley64 ./tests/misc_testsuite/control-flow.wast`
Error: failed to run script file './tests/misc_testsuite/control-flow.wast'

Caused by:
    0: failed directive on ./tests/misc_testsuite/control-flow.wast:77:1
    1: Compilation error: Unsupported feature: should be implemented in ISLE: inst = `v5 = sdiv.i32 v2, v3`, type = `Some(types::I32)`
```

Note that if you run `cargo test --test wast control-flow.wast` it'll also run
the same test under a variety of configurations, but the test is expected to
fail under Pulley. You can update the `WastTest::should_fail` method in
`crates/wast-util/src/lib.rs` to say the test is expected to pass, and then you
can see a similar failure.

#### Adding an instruction: Pulley

Here the failure is what's the most common failure in Pulley right now -- the
Pulley Cranelift backend is not yet complete and is missing a lowering. This
means that there is CLIF that cannot be lowered to Pulley bytecode just yet.

The first thing to do is to probably add a new opcode to Pulley itself as
Pulley probably can't execute this operation just yet. Here we're interested in
signed 32-bit division.

Pull up `pulley/src/lib.rs` and you'll be editing the `for_each_op!` macro
definition. If this is a "rare" opcode you can edit the `for_each_extended_op!`
macro instead. The syntax is the same between the two macros.

Here this is a simple instruction, so let's add it directly:

```rust
/// `dst = src1 / src2` (signed)
xdiv32_s = XDiv32S { operands: BinaryOperands<XReg> };
```

This defines the snake-case name of the instruction (`xdiv32_s`) that is used
by the disassembler and visitor trait, the upper-camel-case name of the
instruction (`XDiv32S`) used for the Rust type and `enum` variant, and
immediates and operands in the instruction itself. In this case it's a binary
operation using integer ("x") registers.

> Note: By convention, we tend to include the class ("x") and width ("32") of
> registers operated upon by an instruction in its name. This distinguishes
> between, for example, 32-bit integer addition (`xadd32`) and 64-bit floating
> point addition (`fadd64`).

Rerun our test command and we see:

```
$ cargo run --features pulley -- wast --target pulley64 ./tests/misc_testsuite/control-flow.wast
   Compiling pulley-interpreter v29.0.0 (/home/alex/code/wasmtime/pulley)
error[E0046]: not all trait items implemented, missing: `xdiv32_s`
   --> pulley/src/interp.rs:807:1
    |
807 | impl OpVisitor for Interpreter<'_> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `xdiv32_s` in implementation
    |
   ::: pulley/src/decode.rs:574:17
    |
574 |                 fn $snake_name(&mut self $( $( , $field : $field_ty )* )? ) -> Self::Return;
    |                 ---------------------------------------------------------------------------- `xdiv32_s` from trait

   Compiling cranelift-codegen-meta v0.116.0 (/home/alex/code/wasmtime/cranelift/codegen/meta)
For more information about this error, try `rustc --explain E0046`.
```

This indicates that we need to actually implement the new opcode in the
interpreter. Open up `pulley/src/interp.rs` and append to `impl OpVisitor for
Interpreter` or `impl ExtendedOpVisitor for Interpreter` as appropriate. Here
we'll add:

```rust
fn xdiv32_s(&mut self, operands: BinaryOperands<XReg>) -> ControlFlow<Done> {
    let a = self.state[operands.src1].get_i32();
    let b = self.state[operands.src2].get_i32();
    match a.checked_div(b) {
        Some(result) => {
            self.state[operands.dst].set_i32(result);
            ControlFlow::Continue(())
        }
        None => self.done_trap::<crate::XDiv32S>(),
    }
}
```

Note that division needs to handle the case that the divisor is 0 or causes an
overflow, hence the use of `checked_div` here. If that happens then a trap is
returned, otherwise interpretation will continue. Also note that the `get_i32`
method is used to specifically match the width and signedness of the
instruction itself, signed 32-bit division. Look around at other instructions
in `interp.rs` for inspiration of how to do various operations.

Running our test again we get the same error as before!

```
$ cargo run --features pulley -- wast --target pulley64 ./tests/misc_testsuite/control-flow.wast
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/wasmtime wast --target pulley64 ./tests/misc_testsuite/control-flow.wast`
Error: failed to run script file './tests/misc_testsuite/control-flow.wast'

Caused by:
    0: failed directive on ./tests/misc_testsuite/control-flow.wast:77:1
    1: Compilation error: Unsupported feature: should be implemented in ISLE: inst = `v5 = sdiv.i32 v2, v3`, type = `Some(types::I32)`
```

That leads us to the next part...

#### Adding a Cranelift Lowering

Next up we need to actually fix the error at hand, a new lowering rule needs to
be added to Cranelift. Here we'll be working in
`cranelift/codegen/src/isa/pulley_shared/lower.isle`.

Our ISLE lowering rules are generally written like this:

```lisp
(rule (lower <CLIF>)
  <Pulley>)
```

This means "when lowering Cranelift's mid-level IR down to Pulley bytecode, and
we match the snippet `<CLIF>`, replace it with `<Pulley>`". (For more details,
see the [ISLE Language
Reference](https://github.com/bytecodealliance/wasmtime/blob/main/cranelift/isle/docs/language-reference.md)
and [How ISLE is Integrated with
Cranelift](https://github.com/bytecodealliance/wasmtime/blob/main/cranelift/docs/isle-integration.md))


In our case, we need to match an `sdiv` CLIF instruction that is operating on
32-bit values (this is the `(has_type $I32 ...)` bit) and then replace it with
our new Pulley `xdiv32_s` instruction:

```
;;;; Rules for `idiv` ;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(rule (lower (has_type $I32 (sdiv a b)))
      (pulley_xdiv32_s a b))
```

Note that ISLE constructors for Pulley instructions, including the
`pulley_xdivs32_s` constructor for our new `xdiv32_s` Pulley instruction, are
automatically generated from the `for_each_op!` macro.

Running our test again yields:

```
Error: failed to run script file './tests/misc_testsuite/control-flow.wast'

Caused by:
    0: failed directive on ./tests/misc_testsuite/control-flow.wast:83:1
    1: Compilation error: Unsupported feature: should be implemented in ISLE: inst = `v26 = band.i32 v2, v13  ; v13 = 3`, type = `Some(types::I32)`
```

Progress! This is a different error than before. Now it's time to rinse and
repeat these steps. Be sure to skim the rest of `lower.isle` for inspiration on
how to implement lowering rules. You can also look at `lower.isle` for other
architecture backends too for inspiration.

#### Flagging a test as passing

After implementing a lowering for `band.i32` our test case is now passing:

```
$ cargo run --features pulley -- wast --target pulley64 ./tests/misc_testsuite/control-flow.wast
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 13.50s
     Running `target/debug/wasmtime wast --target pulley64 ./tests/misc_testsuite/control-flow.wast`
```

If we run the test suite though we'll see:

```
$ cargo test --test wast control-flow.wast
    Finished `test` profile [unoptimized + debuginfo] target(s) in 29.14s
     Running tests/wast.rs (target/debug/deps/wast-f83a3ee5e5dbacde)

running 6 tests
...F.F
failures:

---- CraneliftPulley/./tests/misc_testsuite/control-flow.wast ----
this test is flagged as should-fail but it succeeded

---- CraneliftPulley/pooling/./tests/misc_testsuite/control-flow.wast ----
this test is flagged as should-fail but it succeeded


failures:
    CraneliftPulley/./tests/misc_testsuite/control-flow.wast
    CraneliftPulley/pooling/./tests/misc_testsuite/control-flow.wast

test result: FAILED. 4 passed; 2 failed; 0 ignored; 0 measured; 4086 filtered out; finished in 0.05s

error: test failed, to rerun pass `--test wast`
```

This indicates that the test was previously flagged as "should fail", but that
assertion is no longer true! Update the `WastTest::should_fail` method in
`crates/wast-util/src/lib.rs` so that it expects the test to pass by deleting
the tests from the `unsupported` list. Then we'll see:

```
$ cargo test --test wast control-flow.wast
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.74s
     Running tests/wast.rs (target/debug/deps/wast-f83a3ee5e5dbacde)

running 6 tests
......
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 4086 filtered out; finished in 0.05s
```

Success!

But we aren't quite done yet: the new lowerings we added might have made
additional tests that previously failed start passing as well. If so, then we
also want to mark those tests as expected to pass now. We can double check by
running the full `wast` test suite:

```
$ cargo test --test wast Pulley
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.74s
     Running tests/wast.rs (target/debug/deps/wast-f83a3ee5e5dbacde)


running 1364 tests
....................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................
test result: ok. 1364 passed; 0 failed; 0 ignored; 0 measured; 2728 filtered out; finished in 0.93s
```

Alas, maybe next time!

#### Clean up and make a PR

All that's left now is to clean things up, document anything necessary, and make
a pull request.

To view the complete pull request that implemented `xdiv32_s` and `xand32`
Pulley instructions and got `./tests/misc_testsuite/control-flow.wast` passing
(and also introduced this documentation) check out
[#9765](https://github.com/bytecodealliance/wasmtime/pull/9765).

Thanks for helping out!
