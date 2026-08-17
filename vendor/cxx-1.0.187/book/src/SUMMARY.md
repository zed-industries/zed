# Summary

- [Rust ❤️ C++](index.md)

- [Core concepts](concepts.md)

- [Tutorial](tutorial.md)

- [Other Rust&ndash;C++ interop tools](context.md)

- [Multi-language build system options](building.md)
    - [Cargo](build/cargo.md)
    - [Bazel or Buck2](build/bazel.md)
    - [CMake](build/cmake.md)
    - [More...](build/other.md)

- [Reference: the bridge module](reference.md)
    - [extern "Rust"](extern-rust.md)
    - [extern "C++"](extern-c++.md)
    - [Shared types](shared.md)
    - [Attributes](attributes.md)
    - [Async functions](async.md)
    - [Error handling](binding/result.md)

- [Reference: built-in bindings](bindings.md)
    - [String &mdash; rust::String](binding/string.md)
    - [&str &mdash; rust::Str](binding/str.md)
    - [&&#91;T&#93;, &mut &#91;T&#93; &mdash; rust::Slice\<T\>](binding/slice.md)
    - [CxxString &mdash; std::string](binding/cxxstring.md)
    - [Box\<T\> &mdash; rust::Box\<T\>](binding/box.md)
    - [UniquePtr\<T\> &mdash; std::unique\_ptr\<T\>](binding/uniqueptr.md)
    - [SharedPtr\<T\> &mdash; std::shared\_ptr\<T\>](binding/sharedptr.md)
    - [Vec\<T\> &mdash; rust::Vec\<T\>](binding/vec.md)
    - [CxxVector\<T\> &mdash; std::vector\<T\>](binding/cxxvector.md)
    - [*mut T, *const T raw pointers](binding/rawptr.md)
    - [Function pointers](binding/fn.md)
    - [Result\<T\>](binding/result.md)
