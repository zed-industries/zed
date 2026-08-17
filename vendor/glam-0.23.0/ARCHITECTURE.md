# Architecture

This document describes the high-level architecture of `glam`. While `glam` is
not a large library there are some complexities to its implementation. The
rational and explanation of these follows.

## Design goals

There overarching design goals of glam are:

* Good out of the box performance using SIMD when available
* Has a simple public interface
* Is fast to compile
* Follow Rust [standard library] conventions and [API guidelines] where possible
* High quality [rustdoc] generated document

[standard library]: https://doc.rust-lang.org/std/index.html
[API guidelines]: https://rust-lang.github.io/api-guidelines
[rustdoc]: https://doc.rust-lang.org/rustdoc/index.html

### SIMD

One of the core premises of `glam` was that using SSE2 instructions on `x86` and
`x86_64` architectures gave better performance than using Rust's built in `f32`
type. For more on this finding see [Optimising path tracing with SIMD].

I also wanted to have a `f32` fallback when SIMD was not available.

[Optimising path tracing with SIMD]: https://bitshifter.github.io/2018/06/04/simd-path-tracing/#converting-vec3-to-sse2.

### No generics

Because internally storage could be a SIMD vector intrinsic like `__m128` on
`x86` or say an array of `f32` if SSE2 was not available, a simple generic
parameter like `Vec4<T>` could not be used. The `T` would specify the public
facing type, but not storage. Perhaps this could be achieved with a second
generic parameter for storage, e.g. `Vec4<f32, __m128>` or `Vec4<f32, [f32; 4]>`
but I felt that such a design would introduce a lot of complexity that end users
would ultimately be burdened with, so it's not something that was pursued.

Generics can also increase compile time and code size which is something glam
wants to avoid.

### No traits

`glam` also mostly avoids using traits in the public interface. Primarily
because there wasn't a good reason to. A `Vec3` is not an interface, it is a
concrete type. The secondary reason is traits fragment documentation. If the
functionality of a `Vec3` is implemented across a number of different traits
then the documentation of all of the `Vec3` methods will be on the individual
traits, not the `Vec3` itself. This makes it harder for users to find what
methods a struct actually implements as the documentation is not in one place.

Conversely `glam` does use traits for swizzle methods so that the documentation
for these methods is on the trait and not the `Vec2`, `Vec3`, `Vec4` and so on
structs. There are many swizzle methods which would clutter the documentation,
making them a trait means they won't pollute documentation.

### Support common primitives

Initially `glam` only supported `f32` which kept the internal implementation
relatively simple. However users also wanted support for other primitives types
like `f64`, `i32` and `u32`. Because `glam` avoids using `generics` adding
support for other primitive types without a lot of code duplication required
some additional complexity in implementation.

## High level structure

`glam` supports a number of permutations of vector, quaternion and matrix types
for `f32`, `f64`, `i32` and `u32` primitives, with SSE2 or wasm32 for some `f32`
types and scalar fallbacks if SIMD is not available.

### Component access via Deref

The `Deref` trait is used to provide direct access to SIMD vector components
like `.x`, `.y` and so on.  The `Deref` implementation will return `XYZ<T>`
structure on which the vector components are accessible. Unfortunately if users
dereference the public types they will see confusing errors messages about
`XYZ` types but this on balance seemed preferable to needing to setter and
getting methods to read and write component values.

## Code generation

See [codegen/README.md] for information on `glam`'s code generation process.
