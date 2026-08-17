{{#title Multi-language build system options — Rust ♡ C++}}
# Multi-language build system options

CXX is designed to be convenient to integrate into a variety of build systems.

If you are working in a project that does not already have a preferred build
system for its C++ code *or* which will be relying heavily on open source
libraries from the Rust package registry, you're likely to have the easiest
experience with Cargo which is the build system commonly used by open source
Rust projects. Refer to the ***[Cargo](build/cargo.md)*** chapter about CXX's
Cargo support.

Among build systems designed for first class multi-language support, Bazel is a
solid choice. Refer to the ***[Bazel](build/bazel.md)*** chapter.

If your codebase is already invested in CMake, refer to the
***[CMake](build/cmake.md)*** chapter.

If you have some other build system that you'd like to try to make work with
CXX, see [this page](build/other.md) for notes.
