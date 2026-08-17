{{#title Other Rust–C++ interop tools — Rust ♡ C++}}
# Context: other Rust&ndash;C++ interop tools

When it comes to interacting with an idiomatic Rust API or idiomatic C++ API
from the other language, the generally applicable approaches outside of the CXX
crate are:

- Build a C-compatible wrapper around the code (expressed using `extern "C"`
  signatures, primitives, C-compatible structs, raw pointers). Translate that
  manually to equivalent `extern "C"` declarations in the other language and
  keep them in sync. Preferably, build a safe/idiomatic wrapper around the
  translated `extern "C"` signatures for callers to use.

- Build a C wrapper around the C++ code and use **[bindgen]** to translate that
  programmatically to `extern "C"` Rust signatures. Preferably, build a
  safe/idiomatic Rust wrapper on top.

- Build a C-compatible Rust wrapper around the Rust code and use **[cbindgen]**
  to translate that programmatically to an `extern "C"` C++ header. Preferably,
  build an idiomatic C++ wrapper.

**If the code you are binding is already *"effectively C"*, the above has you
covered.** You should use bindgen or cbindgen, or manually translated C
signatures if there aren't too many and they seldom change.

[bindgen]: https://github.com/rust-lang/rust-bindgen
[cbindgen]: https://github.com/eqrion/cbindgen

## C++ vs C

Bindgen has some basic support for C++. It can reason about classes, member
functions, and the layout of templated types. However, everything it does
related to C++ is best-effort only. Bindgen starts from a point of wanting to
generate declarations for everything, so any C++ detail that it hasn't
implemented will cause a crash if you are lucky ([bindgen#388]) or more likely
silently emit an incompatible signature ([bindgen#380], [bindgen#607],
[bindgen#652], [bindgen#778], [bindgen#1194]) which will do arbitrary
memory-unsafe things at runtime whenever called.

[bindgen#388]: https://github.com/rust-lang/rust-bindgen/issues/388
[bindgen#380]: https://github.com/rust-lang/rust-bindgen/issues/380
[bindgen#607]: https://github.com/rust-lang/rust-bindgen/issues/607
[bindgen#652]: https://github.com/rust-lang/rust-bindgen/issues/652
[bindgen#778]: https://github.com/rust-lang/rust-bindgen/issues/778
[bindgen#1194]: https://github.com/rust-lang/rust-bindgen/issues/1194

Thus using bindgen correctly requires not just juggling all your pointers
correctly at the language boundary, but also understanding ABI details and their
workarounds and reliably applying them. For example, the programmer will
discover that their program sometimes segfaults if they call a function that
returns std::unique\_ptr\<T\> through bindgen. Why? Because unique\_ptr, despite
being "just a pointer", has a different ABI than a pointer or a C struct
containing a pointer ([bindgen#778]) and is not directly expressible in Rust.
Bindgen emitted something that *looks* reasonable and you will have a hell of a
time in gdb working out what went wrong. Eventually people learn to avoid
anything involving a non-trivial copy constructor, destructor, or inheritance,
and instead stick to raw pointers and primitives and trivial structs only
&mdash; in other words C.

## Geometric intuition for why there is so much opportunity for improvement

The CXX project attempts a different approach to C++ FFI.

Imagine Rust and C and C++ as three vertices of a scalene triangle, with length
of the edges being related to similarity of the languages when it comes to
library design.

The most similar pair (the shortest edge) is Rust&ndash;C++. These languages
have largely compatible concepts of things like ownership, vectors, strings,
fallibility, etc that translate clearly from signatures in either language to
signatures in the other language.

When we make a binding for an idiomatic C++ API using bindgen, and we fall down
to raw pointers and primitives and trivial structs as described above, what we
are really doing is coding the two longest edges of the triangle: getting from
C++ down to C, and C back up to Rust. The Rust&ndash;C edge always involves a
great deal of `unsafe` code, and the C++&ndash;C edge similarly requires care
just for basic memory safety. Something as basic as "how do I pass ownership of
a string to the other language?" becomes a strap-yourself-in moment,
particularly for someone not already an expert in one or both sides.

You should think of the `cxx` crate as being the midpoint of the Rust&ndash;C++
edge. Rather than coding the two long edges, you will code half the short edge
in Rust and half the short edge in C++, in both cases with the library playing
to the strengths of the Rust type system *and* the C++ type system to help
assure correctness.

If you've already been through the tutorial in the previous chapter, take a
moment to appreciate that the C++ side *really* looks like we are just writing
C++ and the Rust side *really* looks like we are just writing Rust. Anything you
could do wrong in Rust, and almost anything you could reasonably do wrong in
C++, will be caught by the compiler. This highlights that we are on the "short
edge of the triangle".

But it all still boils down to the same things: it's still FFI from one piece of
native code to another, nothing is getting serialized or allocated or
runtime-checked in between.

## Role of CXX

The role of CXX is to capture the language boundary with more fidelity than what
`extern "C"` is able to represent. You can think of CXX as being a replacement
for `extern "C"` in a sense.

From this perspective, CXX is a lower level tool than the bindgens. Just as
bindgen and cbindgen are built on top of `extern "C"`, it makes sense to think
about higher level tools built on top of CXX. Such a tool might consume a C++
header and/or Rust module (and/or IDL like Thrift) and emit the corresponding
safe cxx::bridge language boundary, leveraging CXX's static analysis and
underlying implementation of that boundary. We are beginning to see this space
explored by the [autocxx] tool, though nothing yet ready for broad use in the
way that CXX on its own is.

[autocxx]: https://github.com/google/autocxx

But note in other ways CXX is higher level than the bindgens, with rich support
for common standard library types. CXX's types serve as an intuitive vocabulary
for designing a good boundary between components in different languages.
