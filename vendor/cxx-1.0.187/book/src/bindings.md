{{#title Built-in bindings — Rust ♡ C++}}
# Built-in bindings reference

In addition to all the primitive types (i32 &lt;=&gt; int32_t), the following
common types may be used in the fields of shared structs and the arguments and
returns of extern functions.

<br>

<table>
<tr><th>name in Rust</th><th>name in C++</th><th>restrictions</th></tr>
<tr><td style="padding:3px 6px">String</td><td style="padding:3px 6px"><b><a href="binding/string.md">rust::String</a></b></td><td style="padding:3px 6px"></td></tr>
<tr><td style="padding:3px 6px">&amp;str</td><td style="padding:3px 6px"><b><a href="binding/str.md">rust::Str</a></b></td><td style="padding:3px 6px"></td></tr>
<tr><td style="padding:3px 6px">&amp;[T]</td><td style="padding:3px 6px"><b><a href="binding/slice.md">rust::Slice&lt;const&nbsp;T&gt;</a></b></td><td style="padding:3px 6px"><sup><i>cannot hold opaque C++ type</i></sup></td></tr>
<tr><td style="padding:3px 6px">&amp;mut [T]</td><td style="padding:3px 6px"><b><a href="binding/slice.md">rust::Slice&lt;T&gt;</a></b></td><td style="padding:3px 6px"><sup><i>cannot hold opaque C++ type</i></sup></td></tr>
<tr><td style="padding:3px 6px"><b><a href="binding/cxxstring.md">CxxString</a></b></td><td style="padding:3px 6px">std::string</td><td style="padding:3px 6px"><sup><i>cannot be passed by value</i></sup></td></tr>
<tr><td style="padding:3px 6px">Box&lt;T&gt;</td><td style="padding:3px 6px"><b><a href="binding/box.md">rust::Box&lt;T&gt;</a></b></td><td style="padding:3px 6px"><sup><i>cannot hold opaque C++ type</i></sup></td></tr>
<tr><td style="padding:3px 6px"><b><a href="binding/uniqueptr.md">UniquePtr&lt;T&gt;</a></b></td><td style="padding:3px 6px">std::unique_ptr&lt;T&gt;</td><td style="padding:3px 6px"><sup><i>cannot hold opaque Rust type</i></sup></td></tr>
<tr><td style="padding:3px 6px"><b><a href="binding/sharedptr.md">SharedPtr&lt;T&gt;</a></b></td><td style="padding:3px 6px">std::shared_ptr&lt;T&gt;</td><td style="padding:3px 6px"><sup><i>cannot hold opaque Rust type</i></sup></td></tr>
<tr><td style="padding:3px 6px">[T; N]</td><td style="padding:3px 6px">std::array&lt;T, N&gt;</td><td style="padding:3px 6px"><sup><i>cannot hold opaque C++ type</i></sup></td></tr>
<tr><td style="padding:3px 6px">Vec&lt;T&gt;</td><td style="padding:3px 6px"><b><a href="binding/vec.md">rust::Vec&lt;T&gt;</a></b></td><td style="padding:3px 6px"><sup><i>cannot hold opaque C++ type</i></sup></td></tr>
<tr><td style="padding:3px 6px"><b><a href="binding/cxxvector.md">CxxVector&lt;T&gt;</a></b></td><td style="padding:3px 6px">std::vector&lt;T&gt;</td><td style="padding:3px 6px"><sup><i>cannot be passed by value, cannot hold opaque Rust type</i></sup></td></tr>
<tr><td style="padding:3px 6px"><b><a href="binding/rawptr.md">*mut T, *const T</a></b></td><td style="padding:3px 6px">T*, const T*</td><td style="padding:3px 6px"><sup><i>fn with a raw pointer argument must be declared unsafe to call</i></sup></td></tr>
<tr><td style="padding:3px 6px">fn(T, U) -&gt; V</td><td style="padding:3px 6px"><b><a href="binding/fn.md">rust::Fn&lt;V(T, U)&gt;</a></b></td><td style="padding:3px 6px"><sup><i>only passing from Rust to C++ is implemented so far</i></sup></td></tr>
<tr><td style="padding:3px 6px"><b><a href="binding/result.md">Result&lt;T&gt;</a></b></td><td style="padding:3px 6px">throw/catch</td><td style="padding:3px 6px"><sup><i>allowed as return type only</i></sup></td></tr>
</table>

<br>

The C++ API of the `rust` namespace is defined by the *include/cxx.h* file in
the CXX GitHub repo. You will need to include this header in your C++ code when
working with those types. **When using Cargo and the cxx-build crate, the header
is made available to you at `#include "rust/cxx.h"`.**

The `rust` namespace additionally provides lowercase type aliases of all the
types mentioned in the table, for use in codebases preferring that style. For
example `rust::String`, `rust::Vec` may alternatively be written `rust::string`,
`rust::vec` etc.

## Pending bindings

The following types are intended to be supported "soon" but are just not
implemented yet. I don't expect any of these to be hard to make work but it's a
matter of designing a nice API for each in its non-native language.

<br>

<table>
<tr><th>name in Rust</th><th>name in C++</th></tr>
<tr><td>BTreeMap&lt;K, V&gt;</td><td><sup><i>tbd</i></sup></td></tr>
<tr><td>HashMap&lt;K, V&gt;</td><td><sup><i>tbd</i></sup></td></tr>
<tr><td>Arc&lt;T&gt;</td><td><sup><i>tbd</i></sup></td></tr>
<tr><td>Option&lt;T&gt;</td><td><sup><i>tbd</i></sup></td></tr>
<tr><td><sup><i>tbd</i></sup></td><td>std::map&lt;K, V&gt;</td></tr>
<tr><td><sup><i>tbd</i></sup></td><td>std::unordered_map&lt;K, V&gt;</td></tr>
</table>
