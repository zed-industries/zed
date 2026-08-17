//! An explanation of the crate's naming conventions.
//!
//! This crate attempts to follow the general naming scheme of `verb_type` when
//! the operation is "simple", and `verb_description_words_type` when the
//! operation (op) needs to be more specific than normal. Like this:
//! * `add_m128`
//! * `add_saturating_i8_m128i`
//!
//! ## Types
//! Currently, only `x86` and `x86_64` types are supported. Among those types:
//! * `m128` and `m256` are always considered to hold `f32` lanes.
//! * `m128d` and `m256d` are always considered to hold `f64` lanes.
//! * `m128i` and `m256i` hold integer data, but each op specifies what lane
//!   width of integers the operation uses.
//! * If the type has `_s` on the end then it's a "scalar" operation that
//!   affects just the lowest lane. The other lanes are generally copied forward
//!   from one of the inputs, though the details there vary from op to op.
//! * The SIMD types are often referred to as "registers" because each SIMD
//!   typed value represents exactly one CPU register when you're doing work.
//!
//! ## Operations
//! There's many operations that can be performed. When possible, `safe_arch`
//! tries to follow normal Rust naming (eg: adding is still `add` and left
//! shifting is still `shl`), but if an operation doesn't normally exist at all
//! in Rust then we basically have to make something up.
//!
//! Many operations have more than one variant, such as `add` and also
//! `add_saturating`. In this case, `safe_arch` puts the "core operation" first
//! and then any "modifiers" go after, which isn't how you might normally say it
//! in English, but it makes the list of functions sort better.
//!
//! As a general note on SIMD terminology: When an operation uses the same
//! indexed lane in two _different_ registers to determine the output, that is a
//! "vertical" operation. When an operation uses more than one lane in the
//! _same_ register to determine the output, that is a "horizontal" operation.
//! * Vertical: `out[0] = a[0] + b[0]`, `out[1] = a[1] + b[1]`
//! * Horizontal: `out[0] = a[0] + a[1]`, `out[1] = b[0] + b[1]`
//!
//! ## Operation Glossary
//! Here follows the list of all the main operations and their explanations.
//!
//! * `abs`: Absolute value (wrapping).
//! * `add`: Addition. This is "wrapping" by default, though some other types of
//!   addition are available. Remember that wrapping signed addition is the same
//!   as wrapping unsigned addition.
//! * `average`: Averages the two inputs.
//! * `bitand`: Bitwise And, `a & b`, like [the trait](core::ops::BitAnd).
//! * `bitandnot`: Bitwise `(!a) & b`. This seems a little funny at first but
//!   it's useful for clearing bits. The output will be based on the `b` side's
//!   bit pattern, but with all active bits in `a` cleared:
//!   * `bitandnot(0b0010, 0b1011) == 0b1001`
//! * `bitor`: Bitwise Or, `a | b`, like [the trait](core::ops::BitOr).
//! * `bitxor`: Bitwise eXclusive Or, `a ^ b`, like [the
//!   trait](core::ops::BitXor).
//! * `blend`: Merge the data lanes of two SIMD values by taking either the `b`
//!   value or `a` value for each lane. Depending on the instruction, the blend
//!   mask can be either an immediate or a runtime value.
//! * `cast`: Convert between data types while preserving the exact bit
//!   patterns, like how [`transmute`](core::mem::transmute) works.
//! * `ceil`: "Ceiling", rounds towards positive infinity.
//! * `cmp`: Numeric comparisons of various kinds. This generally gives "mask"
//!   output where the output value is of the same data type as the inputs, but
//!   with all the bits in a "true" lane as 1 and all the bits in a "false" lane
//!   as 0. Remember that with floating point values all 1s bits is a NaN, and
//!   with signed integers all 1s bits is -1.
//!   * An "Ordered comparison" checks if _neither_ floating point value is NaN.
//!   * An "Unordered comparison" checks if _either_ floating point value is
//!     NaN.
//! * `convert`: This does some sort of numeric type change. The details can
//!   vary wildly. Generally, if the number of lanes goes down then the lowest
//!   lanes will be kept. If the number of lanes goes up then the new high lanes
//!   will be zero.
//! * `div`: Division.
//! * `dot_product`: This works like the matrix math operation. The lanes are
//!   multiplied and then the results are summed up into a single value.
//! * `duplicate`: Copy the even or odd indexed lanes to the other set of lanes.
//!   Eg, `[1, 2, 3, 4]` becomes `[1, 1, 3, 3]` or `[2, 2, 4, 4]`.
//! * `extract`: Get a value from the lane of a SIMD type into a scalar type.
//! * `floor`: Rounds towards negative infinity.
//! * `fused`: All the fused operations are a multiply as well as some sort of
//!   adding or subtracting. The details depend on which fused operation you
//!   select. The benefit of this operation over a non-fused operation are that
//!   it can compute slightly faster than doing the mul and add separately, and
//!   also the output can have higher accuracy in the result.
//! * `insert`: The opposite of `extract`, this puts a new value into a
//!   particular lane of a SIMD type.
//! * `load`: Reads an address and makes a SIMD register value. The details can
//!   vary because there's more than one type of `load`, but generally this is a
//!   `&T -> U` style operation.
//! * `max`: Picks the larger value from each of the two inputs.
//! * `min`: Picks the smaller value from each of the two inputs.
//! * `mul`: Multiplication. For floating point this is just "normal"
//!   multiplication, but for integer types you tend to have some options. An
//!   integer multiplication of X bits will produce a 2X bit output, so
//!   generally you'll get to pick if you want to keep the high half of that,
//!   the low half of that (a normal "wrapping" mul), or "widen" the outputs to
//!   be all the bits at the expense of not multiplying half the lanes the
//!   lanes.
//! * `pack`: Take the integers in the `a` and `b` inputs, reduce them to fit
//!   within the half-sized integer type (eg: `i16` to `i8`), and pack them all
//!   together into the output.
//! * `population`: The "population" operations refer to the bits within an
//!   integer. Either counting them or adjusting them in various ways.
//! * `rdrand`: Use the hardware RNG to make a random value of the given length.
//! * `rdseed`: Use the hardware RNG to make a random seed of the given length.
//!   This is less commonly available, but theoretically an improvement over
//!   `rdrand` in that if you have to combine more than one usage of this
//!   operation to make your full seed size then the guess difficulty rises at a
//!   multiplicative rate instead of just an additive rate. For example, two
//!   `u64` outputs concatenated to a single `u128` have a guess difficulty of
//!   2^(64*64) with `rdseed` but only 2^(64+64) with `rdrand`.
//! * `read_timestamp_counter`: Lets you read the CPU's cycle counter, which
//!   doesn't strictly mean anything in particular since even the CPU's clock
//!   rate isn't even stable over time, but you might find it interesting as an
//!   approximation during benchmarks, or something like that.
//! * `reciprocal`: Turns `x` into `1/x`. Can also be combined with a `sqrt`
//!   operation.
//! * `round`: Convert floating point values to whole numbers, according to one
//!   of several available methods.
//! * `set`: Places a list of scalar values into a SIMD lane. Conceptually
//!   similar to how building an array works in Rust.
//! * `splat`: Not generally an operation of its own, but a modifier to other
//!   operations such as `load` and `set`. This will copy a given value across a
//!   SIMD type as many times as it can be copied. For example, a 32-bit value
//!   splatted into a 128-bit register will be copied four times.
//! * `shl`: Bit shift left. New bits shifted in are always 0. Because the shift
//!   is the same for both signed and unsigned values, this crate simply marks
//!   left shift as always being an unsigned operation.
//!   * You can shift by an immediate value ("imm"), all lanes by the same value
//!     ("all"), or each lane by its own value ("each").
//! * `shr`: Bit shift right. This comes in two forms: "Arithmetic" shifts shift
//!   in the starting sign bit (which preserves the sign of the value), and
//!   "Logical" shifts shift in 0 regardless of the starting sign bit (so the
//!   result ends up being positive). With normal Rust types, signed integers
//!   use arithmetic shifts and unsigned integers use logical shifts, so these
//!   functions are marked as being for signed or unsigned integers
//!   appropriately.
//!   * As with `shl`, you can shift by an immediate value ("imm"), all lanes by
//!     the same value ("all"), or each lane by its own value ("each").
//! * `sign_apply`: Multiplies one set of values by the signum (1, 0, or -1) of
//!   another set of values.
//! * `sqrt`: Square Root.
//! * `store`: Writes a SIMD value to a memory location.
//! * `string_search`: A rather specialized instruction that lets you do byte
//!   based searching within a register. This lets you do some very high speed
//!   searching through ASCII strings when the stars align.
//! * `sub`: Subtract.
//! * `shuffle`: This lets you re-order the data lanes. Sometimes x86/x64 calls
//!   this is called "shuffle", and sometimes it's called "permute", and there's
//!   no particular reasoning behind the different names, so we just call them
//!   all shuffle.
//!   * `shuffle_{args}_{lane-type}_{lane-sources}_{simd-type}`.
//!   * "args" is the input arguments: `a` (one arg) or `ab` (two args), then
//!     either `v` (runtime-varying) or `i` (immediate). All the immediate
//!     shuffles are macros, of course.
//!   * "lane type" is `f32`, `f64`, `i8`, etc. If there's a `z` after the type
//!     then you'll also be able to zero an output position instead of making it
//!     come from a particular source lane.
//!   * "lane sources" is generally either "all" which means that all lanes can
//!     go to all other lanes, or "half" which means that each half of the lanes
//!     is isolated from the other half, and you can't cross data between the
//!     two halves, only within a half (this is how most of the 256-bit x86/x64
//!     shuffles work).
//! * `unpack`: Takes a SIMD value and gets out some of the lanes while widening
//!   them, such as converting `i16` to `i32`.
