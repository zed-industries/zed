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

This is done with a combination of Rust macros for generating the public facing
types and documentation, e.g. `Vec4` and inner storage types which have a number
of traits implemented for different kinds of storage.

### Inner types and traits

Many `glam` types may use SIMD storage where available, e.g. `Vec4` might use
`__m128` for storage if available or an inner storage struct `XYZW<f32>` for
the scalar implementation.

There are a number of internal traits defined in `core::traits` for scalar,
vector, matrix and quaternion functionality that glam needs. These traits are
implemented for the different storage types, e.g.
`core::traits::vector::Vector4` has an implementations for `__m128`, `simd128`
and the `XYZW` struct.

The traits will provide default definitions where possible so not every trait
method needs to be implemented for every storage type.

### Component access via Deref

Because `glam` uses an inner storage type which could be a simple struct or a
SIMD vector it is not possible to provide direct access to the vector's
component values (e.g. `.x`, `.y`, and `.z`). The `Deref` trait is used to work
around this and provide direct access to vector components like `.x`, `.y` and
so on.  The `Deref` implementation will return `XYZ<T>` structure on which the
vector components are accessible. Unfortunately if users dereference the public
types they will see confusing errors messages about `XYZ` types but this on
balance seemed preferable to needing to setter and getting methods to read and
write component values.

### Public types and macros

Macros are used to generate the public types to reuse common implementation
details where possible.

Generally the public type `struct` is declared then it's methods are populated
with multiple macros within a single `impl`. Methods are all declared within a
single `impl` definition for documentation purposes. While it is possible to
have multiple `impl` blocks for the same type this splits the method
documentation generated by `rustdoc`.

A lot of the motivation for removing duplication is documentation. It is
reasonably easy to find discrepancies in duplicated code through unit testing
but documentation is a lot harder to keep in sync.

## A walkthrough of 3D vectors

3D vectors are the most complicated case in `glam`. There are 5 different 3D
vector types, including `Vec3`, `Vec3A` (16 byte aligned SIMD), `DVec3`, `IVec3`
and `UVec3`. There is some common code used by all of these types and some that
is specific to the primitives and storage that they implement. Note that there
is also a `BVec3` but as that type is used as a mask it is quite separate.

### Storage

Store for a 3D vector may use `XYZ<T>` where `T` is one of `f32`, `f64`, `i32`,
or `u32` for the scalar case and `__m128` or `simd128` for the SIMD case for
SSE2 or wasm32 respectively. There is also `XYZF32A16` which is used as storage
for `Vec3A` when SIMD is not available.

### Traits

There are quite a few traits involved in implementing a 3D vector. To start with
there are scalar traits which set up some constants and expected operations for
scalar types in the `core::trait::scalar` module:

* `NumConstsEx` - defines `ZERO` and `ONE` constants
* `FloatConstEx` - defines `NEG_ONE`, `TWO` and `HALF` constants
* `NumEx` - base number trait, implemented for `f32`, `f64`, `i32` and `u32`
* `SignedEx` - signed number trait, implemented for `i32`, `f32` and `f64`
* `FloatEx` - float number trait, implemented for `f32` and `f64`

Then vector traits are defined in `core::trait::vector`:

* `VectorConst` - defines `ZERO` and `ONE` constants for vectors
* `Vector3Const` - defines 3D vector constants, `X`, `Y`, and `Z`
* `Vector` - defines methods for any size of vector, typically these methods
  can be implemented without needing to know the number of components, e.g.
  `splat`
* `SignedVector` - defines methods for signed vectors of any size, e.g. `neg`
* `Vector3` - defines methods for 3D vectors, e.g. `dot` needs to know how many
  components to operate on
* `SignedVector3` - defines methods for signed vector types with 3 components,
  in this case the default implementation of some methods needs to know the
  number of components, e.g. `abs`
* `FloatVector3` - defines methods for float vector types with 3 components, for
  when the implementation needs to know the number of components, e.g. `length`

Note that the `Vector<T>` trait also has an associated `Mask` type which is the
type used for returning from comparison operators. Different types are used for
scalar and SIMD types.

### Macros

The different 3D vector types are declared in the `vec3` module and macros are
used to implement the majority of their methods. Macros specific to 3D vectors
are found in the `vec3` module, most macros call other macros.

* `impl_f32_vec3` - implements methods and traits common to `Vec3` and `Vec3A`
* `impl_vec3_float_methods` -  implements methods for 3D vectors of floats
* `impl_vec3_signed_methods` - implements methods for 3D vectors of signed types
* `impl_vec3_common_methods` - implements common methods for 3D vectors
* `impl_vec3_float_traits` - implements traits for 3D vector float operators
* `impl_vec3_common_traits` - implements traits for common 3D vector operators

Macros that define functionality to vectors of any size are found in
`src/vec.rs`. These do not call other macros.

* `impl_vecn_float_methods` - implements common methods for vectors of floats
* `impl_vecn_signed_methods` - implements common methods for vectors of signed
* `imp_vecn_common_methods` - implements common methods for all vector types
* `impl_vecn_signed_traits` - implements trait operators for signed vector types
* `impl_vecn_common_traits` - implements common trait operators for all vectors

### Summary

Functionality is broken down to the point that there is very little duplicated
code. Macros are also used to avoid duplicating comments. Functionality is
implemented through traits operating on different storage types. Common
functionality is often implemented in default trait implementations.

## Limitations

Adding support for types other than `f32` greatly increased the complexity of
the internal implementation of the crate.

While the current  approach works well for keeping the public interface and
documentation simple and clean complexity exists under the surface. The use of
macros unfortunately obfuscates the source code for anyone trying to read it.
The largest issues being when users navigate to glam code in an IDE they are
usually presented with a very high level macro and need to manually hunt down
the actual implementation. The same issue exists when attempting to view source
in the `rustdoc` generated documentation.

One way to address this would be to use an offline code generator instead of a
compile time code generator (i.e. Rust macros) which would remove macros from
the code that users end up viewing. The main downside is it would take a while
to write this and it might be more confusing for contributors. Another option
might be to improve tooling so that IDEs and rustdoc navigate directly to the
code that was generated by the macro rather than the macro itself.

The use of `Deref` does occasionally cause confusion. It would be good if it was
only necessary to implement `Deref` on the SIMD types but currently that is not
possible.
