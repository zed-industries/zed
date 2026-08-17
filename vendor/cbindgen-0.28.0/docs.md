# cbindgen User Guide

cbindgen creates C/C++11 headers for Rust libraries which expose a public C API.

While you could do this by hand, it's not a particularly good use of your time. It's also much more likely to be error-prone than machine-generated headers that are based on your actual Rust code. The cbindgen developers have also worked closely with the developers of Rust to ensure that the headers we generate reflect actual guarantees about Rust's type layout and ABI.

C++ headers are nice because we can use operator overloads, constructors, enum classes, and templates to make the API more ergonomic and Rust-like. C headers are nice because you can be more confident that whoever you're interoperating with can handle them. With cbindgen *you don't need to choose*! You can just tell it to emit both from the same Rust library.

There are two ways to use cbindgen: as a standalone program, or as a library (presumably in your build.rs).
There isn't really much practical difference, because cbindgen is a simple rust library with no interesting dependencies. Using it as a program means people building your software will need it installed. Using it in your library means people may have to build cbindgen more frequently (e.g. every time they update their rust compiler).

It's worth noting that the development of cbindgen has been largely adhoc, as features have been added to support the usecases of the maintainers. This means cbindgen may randomly fail to support some particular situation simply because no one has put in the effort to handle it yet. [Please file an issue if you run into such a situation][file-it]. Although since we all have other jobs, you might need to do the implementation work too :)





# Quick Start

To install cbindgen, you just need to run

```text
cargo install --force cbindgen
```

(--force just makes it update to the latest cbindgen if it's already installed)

To use cbindgen you need two things:

* A configuration (cbindgen.toml, which can be empty to start)
* A Rust crate with a public C API

Then all you need to do is run it:

```text
cbindgen --config cbindgen.toml --crate my_rust_library --output my_header.h
```

This produces a header file for C++.  For C, add the `--lang c` switch. \
`cbindgen` also supports generation of [Cython](https://cython.org) bindings,
use `--lang cython` for that.

See `cbindgen --help` for more options.

[Get a template cbindgen.toml here.](template.toml)



## build.rs

If you don't want to use cbindgen as an application, here's an example build.rs script:

```rust
extern crate cbindgen;

use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
      .with_crate(crate_dir)
      .generate()
      .expect("Unable to generate bindings")
      .write_to_file("bindings.h");
}
```

You can add configuration options using the [`Builder`](https://docs.rs/cbindgen/*/cbindgen/struct.Builder.html#methods) interface.

When actively working on code, you likely don't want cbindgen to fail the entire build. Instead of expect-ing the result of the header generation, you could [ignore parse errors](https://github.com/mozilla/cbindgen/issues/472#issuecomment-831439826) and let rustc or your code analysis bring up:

```rust
    // ...
    .generate()
    .map_or_else(
        |error| match error {
            cbindgen::Error::ParseSyntaxError { .. } => {}
            e => panic!("{:?}", e),
        },
        |bindings| {
            bindings.write_to_file("target/include/bindings.h");
        },
    );
}
```

Be sure to add the following section to your Cargo.toml:

```
[build-dependencies]
cbindgen = "0.24.0"
```

If you'd like to use a `build.rs` script with a `cbindgen.toml`, consider using [`cbindgen::generate()`](https://docs.rs/cbindgen/*/cbindgen/fn.generate.html) instead.

## Internal Representation

Some users may find it useful to access the **unstable** internal representation (IR) that cbindgen uses to parse and generate code. By default, the IR is private, but you can access it by enabling the `"unstable_ir"` feature flag like so:

```
[build-dependencies]
cbindgen = { version = "0.27.0", features = ["unstable_ir"] }
```

This opens up the `cbindgen::bindgen::ir` module.

Please remember that the IR is **not stable**, so if you use this feature, you will need to pin cbindgen to avoid breakages.

# Writing Your C API

cbindgen has a simple but effective strategy. It walks through your crate looking for:

* `#[no_mangle] pub extern fn` ("functions")
* `#[no_mangle] pub static` ("globals")
* `pub const` ("constants")

and generates a header declaring those items. But to declare those items, it needs to also be able to describe the layout and ABI of the types that appear in their signatures. So it will also spider through your crate (and optionally its dependencies) to try to find the definitions of every type used in your public API.

> 🚨 NOTE: A major limitation of cbindgen is that it does not understand Rust's module system or namespacing. This means that if cbindgen sees that it needs the definition for `MyType` and there exists two things in your project with the type name `MyType`, it won't know what to do. Currently, cbindgen's behaviour is unspecified if this happens. However this may be ok if they have [different cfgs][section-cfgs].

If a type is determined to have a guaranteed layout, a full definition will be emitted in the header. If the type doesn't have a guaranteed layout, only a forward declaration will be emitted. This may be fine if the type is intended to be passed around opaquely and by reference.




# Examples

🚧 🏗

It would be really nice to have some curated and clean examples, but we don't have those yet.

[The README has some useful links though](README.md#examples).



# Supported Types

Most things in Rust don't have a guaranteed layout by default. In most cases this is nice because it enables layout to be optimized in the majority of cases where type layout isn't that interesting. However this is problematic for our purposes. Thankfully Rust lets us opt into guaranteed layouts with the `repr` attribute.

You can learn about all of the different repr attributes [by reading Rust's reference][reference], but here's a quick summary:

* `#[repr(C)]`: give this struct/union/enum the same layout and ABI C would
* `#[repr(u8, u16, ... etc)]`: give this enum the same layout and ABI as the given integer type
* `#[repr(transparent)]`: give this single-field struct the same ABI as its field (useful for newtyping integers but keeping the integer ABI)

cbindgen supports the `#[repr(align(N))]` and `#[repr(packed)]` attributes, but currently does not support `#[repr(packed(N))]`.

cbindgen also supports using `repr(C)`/`repr(u8)` on non-C-like enums (enums with fields). This gives a C-compatible tagged union layout, as [defined by this RFC 2195][really-tagged-unions]. `repr(C)` will give a simpler layout that is perhaps more intuitive, while `repr(u8)` will produce a more compact layout.

If you ensure everything has a guaranteed repr, then cbindgen will generate definitions for:

* struct (named-style or tuple-style)
* enum (fieldless or with fields)
* union
* type
* `[T; n]` (arrays always have a guaranteed C-compatible layout)
* `&T`, `&mut T`, `*const T`, `*mut T`, `Option<&T>`, `Option<&mut T>` (all have the same pointer ABI)
* `fn()` (as an actual function pointer)
* `bitflags! { ... }` (if macro_expansion.bitflags is enabled)

structs, enums, unions, and type aliases may be generic, although certain generic substitutions may fail to resolve under certain configurations. In C mode generics are resolved through monomorphization and mangling, while in C++ mode generics are resolved with templates. cbindgen cannot support generic functions, as they do not actually have a single defined symbol.

cbindgen sadly cannot ever support anonymous tuples `(A, B, ...)`, as there is no way to guarantee their layout. You must use a tuple struct.

cbindgen also cannot support wide pointers like `&dyn Trait` or `&[T]`, as their layout and ABI is not guaranteed. In the case of slices you can at least decompose them into a pointer and length, and reconstruct them with `slice::from_raw_parts`.

If cbindgen determines that a type is zero-sized, it will erase all references to that type (so fields of that type simply won't be emitted). This won't work if that type appears as a function argument because C, C++, and Rust all have different definitions of what it means for a type to be empty.

Don't use the `[u64; 0]` trick to over-align a struct, we don't support this.

cbindgen contains the following hardcoded mappings (again completely ignoring namespacing, literally just looking at the name of the type):




## std types

* bool => bool
* char => uint32_t
* u8 => uint8_t
* u16 => uint16_t
* u32 => uint32_t
* u64 => uint64_t
* usize => uintptr_t
* i8 => int8_t
* i16 => int16_t
* i32 => int32_t
* i64 => int64_t
* isize => intptr_t
* f32 => float
* f64 => double
* VaList => va_list
* RawFd => int
* PhantomData => *evaporates*, can only appear as the field of a type
* PhantomPinned => *evaporates*, can only appear as the field of a type  
* () => *evaporates*, can only appear as the field of a type
* MaybeUninit<T>, ManuallyDrop<T>, and Pin<T> => T




## libc types

* c_void => void
* c_char => char
* c_schar => signed char
* c_uchar => unsigned char
* c_float => float
* c_double => double
* c_short => short
* c_int => int
* c_long => long
* c_longlong => long long
* c_ushort => unsigned short
* c_uint => unsigned int
* c_ulong => unsigned long
* c_ulonglong => unsigned long long



## stdint types

* uint8_t => uint8_t
* uint16_t => uint16_t
* uint32_t => uint32_t
* uint64_t => uint64_t
* uintptr_t => uintptr_t
* size_t => size_t
* int8_t => int8_t
* int16_t => int16_t
* int32_t => int32_t
* int64_t => int64_t
* intptr_t => intptr_t
* ssize_t => ssize_t
* ptrdiff_t => ptrdiff_t






# Configuring Your Header

cbindgen supports several different options for configuring the output of your header, including target language, styling, mangling, prefixing, includes, and defines.





## Defines and Cfgs

As cbindgen spiders through your crate, it will make note of all the cfgs it found on the path to every item. If it finds multiple declarations that share a single name but have different cfgs, it will then try to emit every version it found wrapped in defines that correspond to those cfgs. In this way platform-specific APIs or representations can be properly supported.

However cbindgen has no way of knowing how you want to map those cfgs to defines. You will need to use the `[defines]` section in your cbindgen.toml to specify all the different mappings. It natively understands concepts like any() and all(), so you only need to tell it how you want to translate base concepts like `target_os = "freebsd"` or `feature = "serde"`.

Note that because cbindgen just parses the source of your crate, you mostly don't need to worry about what crate features or what platform you're targetting. Every possible configuration should be visible to the parser. Our primitive mappings should also be completely platform agnostic (i32 is int32_t regardless of your target).

While modules within a crate form a tree with uniquely defined paths to each item, and therefore uniquely defined cfgs for those items, dependencies do not. If you depend on a crate in multiple ways, and those ways produce different cfgs, one of them will be arbitrarily chosen for any types found in that crate.




## Annotations

While output configuration is primarily done through the cbindgen.toml, in some cases you need to manually override your global settings. In those cases you can add inline annotations to your types, which are doc comments that start with `cbindgen:`. Here's an example of using annotations to rename a struct's fields and opt into overloading `operator==`:

```rust
/// cbindgen:field-names=[x, y]
/// cbindgen:derive-eq
#[repr(C)]
pub struct Point(pub f32, pub f32);
```

An annotation may be a bool, string (no quotes), or list of strings. If just the annotation's name is provided, `=true` is assumed. The annotation parser is currently fairly naive and lacks any capacity for escaping, so don't try to make any strings with `=`, `,`, `[` or `]`.

Most annotations are just local overrides for identical settings in the cbindgen.toml, but a few are unique because they don't make sense in a global context. The set of supported annotation are as follows:

### Ignore annotation

cbindgen will automatically ignore any `#[test]` or `#[cfg(test)]` item it
finds. You can manually ignore other stuff with the `ignore` annotation
attribute:

```rust
pub mod my_interesting_mod;

/// cbindgen:ignore
pub mod my_uninteresting_mod; // This won't be scanned by cbindgen.
```

### No export annotation

cbindgen will usually emit all items it finds, as instructed by the parse and export config sections. This annotation will make cbindgen skip this item from the output, while still being aware of it. This is useful for a) suppressing "Can't find" errors and b) emitting `struct my_struct` for types in a different header (rather than a bare `my_struct`).

There is no equivalent config for this annotation - by comparison, the export exclude config will cause cbindgen to not be aware of the item at all.

Note that cbindgen will still traverse `no-export` structs that are `repr(C)` to emit types present in the fields. You will need to manually exclude those types in your config if desired.

```
/// cbindgen:no-export
#[repr(C)]
pub struct Foo { .. }; // This won't be emitted by cbindgen in the header

#[repr(C)]
fn bar() -> Foo { .. } // Will be emitted as `struct foo bar();`
```

### Struct Annotations

* field-names=\[field1, field2, ...\] -- sets the names of all the fields in the output struct. These names will be output verbatim, and are not eligible for renaming.

The rest are just local overrides for the same options found in the cbindgen.toml:

* rename-all=RenameRule
* derive-constructor
* derive-eq
* derive-neq
* derive-lt
* derive-lte
* derive-gt
* derive-gte
* {eq,neq,lt,lte,gt,gte}-attributes: Takes a single identifier which will be
  emitted before the signature of the auto-generated `operator==` / `operator!=`
  / etc(if any). The idea is for this to be used to annotate the operator with
  attributes, for example:

```rust
/// cbindgen:eq-attributes=MY_ATTRIBUTES
#[repr(C)]
pub struct Foo { .. }
```

Will generate something like:

```
  MY_ATTRIBUTES bool operator==(const Foo& other) const {
    ...
  }
```

Combined with something like:

```
#define MY_ATTRIBUTES [[nodiscard]]
```

for example.

### Enum Annotations

* enum-trailing-values=\[variant1, variant2, ...\] -- add the following fieldless enum variants to the end of the enum's definition. These variant names *will* have the enum's renaming rules applied.

WARNING: if any of these values are ever passed into Rust, behaviour will be Undefined. Rust does not know about them, and will assume they cannot happen.

The rest are just local overrides for the same options found in the cbindgen.toml:

* rename-all=RenameRule
* add-sentinel
* derive-helper-methods
* derive-const-casts
* derive-mut-casts
* derive-tagged-enum-destructor
* derive-tagged-enum-copy-constructor
* enum-class
* prefix-with-name
* private-default-tagged-enum-constructor
* {destructor,copy-constructor,copy-assignment}-attributes: See the description
  of the struct attributes, these do the same for the respective generated code.

### Enum variant annotations

These apply to both tagged and untagged enum _variants_.

* variant-{constructor,const-cast,mut-cast,is}-attributes: See the description
  of the struct attributes. These do the same for the respective functions.

TODO: We should allow to override the `derive-{const,mut}-casts`, helper methods
et al. with per-variant annotations, probably.

### Union Annotations

* field-names=\[field1, field2, ...\] -- sets the names of all the fields in the output union. These names will be output verbatim, and are not eligible for renaming.

The rest are just local overrides for the same options found in the cbindgen.toml:

* rename-all=RenameRule



### Function Annotations

All function attributes are just local overrides for the same options found in the cbindgen.toml:

* rename-all=RenameRule
* prefix
* postfix
* ptrs-as-arrays=\[[ptr\_name1; array\_length1], [ptr\_name2; array\_length2], ...\] -- represents the pointer arguments of a function as arrays. Below how the mappings are performed:

```
arg: *const T --> const T arg[array_length]
arg: *mut T ---> T arg[array_length]
```

If `array_length` is not specified:

```
arg: *const T --> const T arg[]
arg: *mut T --> T arg[]
```

## Generating Swift Bindings

In addition to parsing function names in C/C++ header files, the Swift compiler can make use of the `swift_name` attribute on functions to generate more idiomatic names for imported functions and methods.

This attribute is commonly used in Objective-C/C/C++ via the `NS_SWIFT_NAME` and `CF_SWIFT_NAME` macros.

Given configuration in the cbindgen.toml, `cbindgen` can generate these attributes for you by guessing an appropriate method signature based on the existing function name (and type, if it is a method in an `impl` block).

This is controlled by the `swift_name_macro` option in the cbindgen.toml.

## cbindgen.toml

Most configuration happens through your cbindgen.toml file. Every value has a default (that is usually reasonable), so you can start with an empty cbindgen.toml and tweak it until you like the output you're getting.

Note that many options defined here only apply for one of C or C++. Usually it's an option specifying whether we should try to make use of a feature in C++'s type system or generate a helper method.

```toml
# The language to output bindings in
#
# possible values: "C", "C++", "Cython"
#
# default: "C++"
language = "C"




# Options for wrapping the contents of the header:

# An optional string of text to output at the beginning of the generated file
# default: doesn't emit anything
header = "/* Text to put at the beginning of the generated file. Probably a license. */"

# An optional string of text to output at the end of the generated file
# default: doesn't emit anything
trailer = "/* Text to put at the end of the generated file */"

# An optional name to use as an include guard
# default: doesn't emit an include guard
include_guard = "mozilla_wr_bindings_h"

# Whether to add a `#pragma once` guard
# default: doesn't emit a `#pragma once`
pragma_once = true

# An optional string of text to output between major sections of the generated
# file as a warning against manual editing
#
# default: doesn't emit anything
autogen_warning = "/* Warning, this file is autogenerated by cbindgen. Don't modify this manually. */"

# Whether to include a comment with the version of cbindgen used to generate the file
# default: false
include_version = true

# An optional namespace to output around the generated bindings
# default: doesn't emit a namespace
namespace = "ffi"

# An optional list of namespaces to output around the generated bindings
# default: []
namespaces = ["mozilla", "wr"]

# An optional list of namespaces to declare as using with "using namespace"
# default: []
using_namespaces = ["mozilla", "wr"]

# A list of sys headers to #include (with angle brackets)
# default: []
sys_includes = ["stdio", "string"]

# A list of headers to #include (with quotes)
# default: []
includes = ["my_great_lib.h"]

# Whether cbindgen's default C/C++ standard imports should be suppressed. These
# imports are included by default because our generated headers tend to require
# them (e.g. for uint32_t). Currently, the generated imports are:
#
# * for C: <stdarg.h>, <stdbool.h>, <stdint.h>, <stdlib.h>, <uchar.h>
#
# * for C++: <cstdarg>, <cstdint>, <cstdlib>, <new>, <cassert> (depending on config)
#
# default: false
no_includes = false

# Whether to make a C header C++ compatible.
# These will wrap generated functions into a `extern "C"` block, e.g.
#
# #ifdef __cplusplus
# extern "C" {
# #endif // __cplusplus
#
# // Generated functions.
#
# #ifdef __cplusplus
# } // extern "C"
# #endif // __cplusplus
#
# If the language is not C this option won't have any effect.
#
# default: false
cpp_compat = false

# A list of lines to add verbatim after the includes block
after_includes = "#define VERSION 1"



# Code Style Options

# The style to use for curly braces
#
# possible values: "SameLine", "NextLine"
#
# default: "SameLine"
braces = "SameLine"

# The desired length of a line to use when formatting lines
# default: 100
line_length = 80

# The amount of spaces to indent by
# default: 2
tab_width = 3

# Include doc comments from Rust as documentation
documentation = true

# How the generated documentation should be commented.
#
# possible values:
# * "c": /* like this */
# * "c99": // like this
# * "c++": /// like this
# * "doxy": like C, but with leading *'s on each line
# * "auto": "c++" if that's the language, "doxy" otherwise
#
# default: "auto"
documentation_style = "doxy"

# How much of the documentation for each item is output.
#
# possible values:
# * "short": Only the first line.
# * "full": The full documentation.
#
# default: "full"
documentation_length = "short"




# Codegen Options

# When generating a C header, the kind of declaration style to use for structs
# or enums.
#
# possible values:
# * "type": typedef struct { ... } MyType;
# * "tag": struct MyType { ... };
# * "both": typedef struct MyType { ... } MyType;
#
# default: "both"
style = "both"

# If this option is true `usize` and `isize` will be converted into `size_t` and `ptrdiff_t`
# instead of `uintptr_t` and `intptr_t` respectively.
usize_is_size_t = true

# A list of substitutions for converting cfg's to ifdefs. cfgs which aren't
# defined here will just be discarded.
#
# e.g.
# `#[cfg(target = "freebsd")] ...`
# becomes
# `#if defined(DEFINE_FREEBSD) ... #endif`
[defines]
"target_os = freebsd" = "DEFINE_FREEBSD"
"feature = serde" = "DEFINE_SERDE"





[export]
# A list of additional items to always include in the generated bindings if they're
# found but otherwise don't appear to be used by the public API.
#
# default: []
include = ["MyOrphanStruct", "MyGreatTypeRename"]

# A list of items to not include in the generated bindings
# default: []
exclude = ["Bad"]

# A prefix to add before the name of every item
# default: no prefix is added
prefix = "CAPI_"

# Types of items that we'll generate. If empty, then all types of item are emitted.
#
# possible items: (TODO: explain these in detail)
# * "constants":
# * "globals":
# * "enums":
# * "structs":
# * "unions":
# * "typedefs":
# * "opaque":
# * "functions":
#
# default: []
item_types = ["enums", "structs", "opaque", "functions"]

# Whether applying rules in export.rename prevents export.prefix from applying.
#
# e.g. given this toml:
#
# [export]
# prefix = "capi_"
# [export.rename]
# "MyType" = "my_cool_type"
#
# You get the following results:
#
# renaming_overrides_prefixing = true:
# "MyType" => "my_cool_type"
#
# renaming_overrides_prefixing = false:
# "MyType => capi_my_cool_type"
#
# default: false
renaming_overrides_prefixing = true

# Table of name conversions to apply to item names (lhs becomes rhs)
[export.rename]
"MyType" = "my_cool_type"
"my_function" = "BetterFunctionName"

# Table of things to prepend to the body of any struct, union, or enum that has the
# given name. This can be used to add things like methods which don't change ABI,
# mark fields private, etc
[export.pre_body]
"MyType" = """
  MyType() = delete;
private:
"""

# Table of things to append to the body of any struct, union, or enum that has the
# given name. This can be used to add things like methods which don't change ABI.
[export.body]
"MyType" = """
  void cppMethod() const;
"""

# Configuration for name mangling
[export.mangle]
# Whether the types should be renamed during mangling, for example
# c_char -> CChar, etc.
rename_types = "PascalCase"
# Whether the underscores from the mangled name should be omitted.
remove_underscores = false

[layout]
# A string that should come before the name of any type which has been marked
# as `#[repr(packed)]`. For instance, "__attribute__((packed))" would be a
# reasonable value if targeting gcc/clang. A more portable solution would
# involve emitting the name of a macro which you define in a platform-specific
# way. e.g. "PACKED"
#
# default: `#[repr(packed)]` types will be treated as opaque, since it would
# be unsafe for C callers to use a incorrectly laid-out union.
packed = "PACKED"

# A string that should come before the name of any type which has been marked
# as `#[repr(align(n))]`. This string must be a function-like macro which takes
# a single argument (the requested alignment, `n`). For instance, a macro
# `#define`d as `ALIGNED(n)` in `header` which translates to
# `__attribute__((aligned(n)))` would be a reasonable value if targeting
# gcc/clang.
#
# default: `#[repr(align(n))]` types will be treated as opaque, since it
# could be unsafe for C callers to use a incorrectly-aligned union.
aligned_n = "ALIGNED"


[fn]
# An optional prefix to put before every function declaration
# default: no prefix added
prefix = "WR_START_FUNC"

# An optional postfix to put after any function declaration
# default: no postix added
postfix = "WR_END_FUNC"

# How to format function arguments
#
# possible values:
# * "horizontal": place all arguments on the same line
# * "vertical": place each argument on its own line
# * "auto": only use vertical if horizontal would exceed line_length
#
# default: "auto"
args = "horizontal"

# An optional string that should prefix function declarations which have been
# marked as `#[must_use]`. For instance, "__attribute__((warn_unused_result))"
# would be a reasonable value if targeting gcc/clang. A more portable solution
# would involve emitting the name of a macro which you define in a
# platform-specific way. e.g. "MUST_USE_FUNC"
# default: nothing is emitted for must_use functions
must_use = "MUST_USE_FUNC"

# An optional string that should prefix function declarations which have been
# marked as `#[deprecated]` without note. For instance, "__attribute__((deprecated))"
# would be a reasonable value if targeting gcc/clang. A more portable solution
# would involve emitting the name of a macro which you define in a
# platform-specific way. e.g. "DEPRECATED_FUNC"
# default: nothing is emitted for deprecated functions
deprecated = "DEPRECATED_FUNC"

# An optional string that should prefix function declarations which have been
# marked as `#[deprecated(note = "reason")]`. `{}` will be replaced with the
# double-quoted string. For instance, "__attribute__((deprecated({})))"
# would be a reasonable value if targeting gcc/clang. A more portable solution
# would involve emitting the name of a macro which you define in a
# platform-specific way. e.g. "DEPRECATED_FUNC_WITH_NOTE(note)"
# default: nothing is emitted for deprecated functions
deprecated_with_notes = "DEPRECATED_FUNC_WITH_NOTE"

# An optional string that will be used in the attribute position for functions
# that don't return (that return `!` in Rust).
#
# For instance, `__attribute__((noreturn))` would be a reasonable value if
# targeting gcc/clang.
no_return = "NO_RETURN"

# An optional string that, if present, will be used to generate Swift function
# and method signatures for generated functions, for example "CF_SWIFT_NAME".
# If no such macro is available in your toolchain, you can define one using the
# `header` option in cbindgen.toml
# default: no swift_name function attributes are generated
swift_name_macro = "CF_SWIFT_NAME"

# A rule to use to rename function argument names. The renaming assumes the input
# is the Rust standard snake_case, however it accepts all the different rename_args
# inputs. This means many options here are no-ops or redundant.
#
# possible values (that actually do something):
# * "CamelCase": my_arg => myArg
# * "PascalCase": my_arg => MyArg
# * "GeckoCase": my_arg => aMyArg
# * "ScreamingSnakeCase": my_arg => MY_ARG
# * "None": apply no renaming
#
# technically possible values (that shouldn't have a purpose here):
# * "SnakeCase": apply no renaming
# * "LowerCase": apply no renaming (actually applies to_lowercase, is this bug?)
# * "UpperCase": same as ScreamingSnakeCase in this context
# * "QualifiedScreamingSnakeCase" => same as ScreamingSnakeCase in this context
#
# default: "None"
rename_args = "PascalCase"

# This rule specifies the order in which functions will be sorted.
#
# "Name": sort by the name of the function
# "None": keep order in which the functions have been parsed
#
# default: "None"
sort_by = "Name"

[struct]
# A rule to use to rename struct field names. The renaming assumes the input is
# the Rust standard snake_case, however it acccepts all the different rename_args
# inputs. This means many options here are no-ops or redundant.
#
# possible values (that actually do something):
# * "CamelCase": my_arg => myArg
# * "PascalCase": my_arg => MyArg
# * "GeckoCase": my_arg => mMyArg
# * "ScreamingSnakeCase": my_arg => MY_ARG
# * "None": apply no renaming
#
# technically possible values (that shouldn't have a purpose here):
# * "SnakeCase": apply no renaming
# * "LowerCase": apply no renaming (actually applies to_lowercase, is this bug?)
# * "UpperCase": same as ScreamingSnakeCase in this context
# * "QualifiedScreamingSnakeCase" => same as ScreamingSnakeCase in this context
#
# default: "None"
rename_fields = "PascalCase"

# An optional string that should come before the name of any struct which has been
# marked as `#[must_use]`. For instance, "__attribute__((warn_unused))"
# would be a reasonable value if targeting gcc/clang. A more portable solution
# would involve emitting the name of a macro which you define in a
# platform-specific way. e.g. "MUST_USE_STRUCT"
#
# default: nothing is emitted for must_use structs
must_use = "MUST_USE_STRUCT"

# An optional string that should come before the name of any struct which has been
# marked as `#[deprecated]` without note. For instance, "__attribute__((deprecated))"
# would be a reasonable value if targeting gcc/clang. A more portable solution
# would involve emitting the name of a macro which you define in a
# platform-specific way. e.g. "DEPRECATED_STRUCT"
# default: nothing is emitted for deprecated structs
deprecated = "DEPRECATED_STRUCT"

# An optional string that should come before the name of any struct which has been
# marked as `#[deprecated(note = "reason")]`. `{}` will be replaced with the
# double-quoted string. For instance, "__attribute__((deprecated({})))"
# would be a reasonable value if targeting gcc/clang. A more portable solution
# would involve emitting the name of a macro which you define in a
# platform-specific way. e.g. "DEPRECATED_STRUCT_WITH_NOTE(note)"
# default: nothing is emitted for deprecated structs
deprecated_with_notes = "DEPRECATED_STRUCT_WITH_NOTE"

# Whether a Rust type with associated consts should emit those consts inside the
# type's body. Otherwise they will be emitted trailing and with the type's name
# prefixed. This does nothing if the target is C, or if
# [const]allow_static_const = false
#
# default: false
# associated_constants_in_body: false

# Whether to derive a simple constructor that takes a value for every field.
# default: false
derive_constructor = true

# Whether to derive an operator== for all structs
# default: false
derive_eq = false

# Whether to derive an operator!= for all structs
# default: false
derive_neq = false

# Whether to derive an operator< for all structs
# default: false
derive_lt = false

# Whether to derive an operator<= for all structs
# default: false
derive_lte = false

# Whether to derive an operator> for all structs
# default: false
derive_gt = false

# Whether to derive an operator>= for all structs
# default: false
derive_gte = false





[enum]
# A rule to use to rename enum variants, and the names of any fields those
# variants have. This should probably be split up into two separate options, but
# for now, they're the same! See the documentation for `[struct]rename_fields`
# for how this applies to fields. Renaming of the variant assumes that the input
# is the Rust standard PascalCase. In the case of QualifiedScreamingSnakeCase,
# it also assumed that the enum's name is PascalCase.
#
# possible values (that actually do something):
# * "CamelCase": MyVariant => myVariant
# * "SnakeCase": MyVariant => my_variant
# * "ScreamingSnakeCase": MyVariant => MY_VARIANT
# * "QualifiedScreamingSnakeCase": MyVariant => ENUM_NAME_MY_VARIANT
# * "LowerCase": MyVariant => myvariant
# * "UpperCase": MyVariant => MYVARIANT
# * "None": apply no renaming
#
# technically possible values (that shouldn't have a purpose for the variants):
# * "PascalCase": apply no renaming
# * "GeckoCase": apply no renaming
#
# default: "None"
rename_variants = "None"

# Whether an extra "sentinel" enum variant should be added to all generated enums.
# Firefox uses this for their IPC serialization library.
#
# WARNING: if the sentinel is ever passed into Rust, behaviour will be Undefined.
# Rust does not know about this value, and will assume it cannot happen.
#
# default: false
add_sentinel = false

# Whether enum variant names should be prefixed with the name of the enum.
# default: false
prefix_with_name = false

# Whether to emit enums using "enum class" when targeting C++.
# default: true
enum_class = true

# Whether to generate static `::MyVariant(..)` constructors and `bool IsMyVariant()`
# methods for enums with fields.
#
# default: false
derive_helper_methods = false

# Whether to generate `const MyVariant& AsMyVariant() const` methods for enums with fields.
# default: false
derive_const_casts = false

# Whether to generate `MyVariant& AsMyVariant()` methods for enums with fields
# default: false
derive_mut_casts = false

# The name of the macro/function to use for asserting `IsMyVariant()` in the body of
# derived `AsMyVariant()` cast methods.
#
# default: "assert" (but also causes `<cassert>` to be included by default)
cast_assert_name = "MOZ_RELEASE_ASSERT"

# An optional string that should come before the name of any enum which has been
# marked as `#[must_use]`. For instance, "__attribute__((warn_unused))"
# would be a reasonable value if targeting gcc/clang. A more portable solution
# would involve emitting the name of a macro which you define in a
# platform-specific way. e.g. "MUST_USE_ENUM"
#
# Note that this refers to the *output* type. That means this will not apply to an enum
# with fields, as it will be emitted as a struct. `[struct]must_use` will apply there.
#
# default: nothing is emitted for must_use enums
must_use = "MUST_USE_ENUM"

# An optional string that should come before the name of any enum which has been
# marked as `#[deprecated]` without note. For instance, "__attribute__((deprecated))"
# would be a reasonable value if targeting gcc/clang. A more portable solution
# would involve emitting the name of a macro which you define in a
# platform-specific way. e.g. "DEPRECATED_ENUM"
# default: nothing is emitted for deprecated enums
deprecated = "DEPRECATED_ENUM"

# An optional string that should come before the name of any enum which has been
# marked as `#[deprecated(note = "reason")]`. `{}` will be replaced with the
# double-quoted string. For instance, "__attribute__((deprecated({})))"
# would be a reasonable value if targeting gcc/clang. A more portable solution
# would involve emitting the name of a macro which you define in a
# platform-specific way. e.g. "DEPRECATED_ENUM_WITH_NOTE(note)"
# default: nothing is emitted for deprecated enums
deprecated_with_notes = "DEPRECATED_ENUM_WITH_NOTE"

# An optional string that should come after the name of any enum variant which has been
# marked as `#[deprecated]` without note. For instance, "__attribute__((deprecated))"
# would be a reasonable value if targeting gcc/clang. A more portable solution would
# involve emitting the name of a macro which you define in a platform-specific
# way. e.g. "DEPRECATED_ENUM_VARIANT"
# default: nothing is emitted for deprecated enum variants
deprecated_variant = "DEPRECATED_ENUM_VARIANT"

# An optional string that should come after the name of any enum variant which has been
# marked as `#[deprecated(note = "reason")]`. `{}` will be replaced with the
# double-quoted string. For instance, "__attribute__((deprecated({})))" would be a
# reasonable value if targeting gcc/clang. A more portable solution would involve
# emitting the name of a macro which you define in a platform-specific
# way. e.g. "DEPRECATED_ENUM_WITH_NOTE(note)"
# default: nothing is emitted for deprecated enum variants
deprecated_variant_with_notes = "DEPRECATED_ENUM_VARIANT_WITH_NOTE({})"

# Whether enums with fields should generate destructors. This exists so that generic
# enums can be properly instantiated with payloads that are C++ types with
# destructors. This isn't necessary for structs because C++ has rules to
# automatically derive the correct constructors and destructors for those types.
#
# Care should be taken with this option, as Rust and C++ cannot
# properly interoperate with eachother's notions of destructors. Also, this may
# change the ABI for the type. Either your destructor-full enums must live
# exclusively within C++, or they must only be passed by-reference between
# C++ and Rust.
#
# default: false
derive_tagged_enum_destructor = false

# Whether enums with fields should generate copy-constructor. See the discussion on
# derive_tagged_enum_destructor for why this is both useful and very dangerous.
#
# default: false
derive_tagged_enum_copy_constructor = false
# Whether enums with fields should generate copy-assignment operators.
#
# This depends on also deriving copy-constructors, and it is highly encouraged
# for this to be set to true.
#
# default: false
derive_tagged_enum_copy_assignment = false

# Whether enums with fields should generate an empty, private destructor.
# This allows the auto-generated constructor functions to compile, if there are
# non-trivially constructible members. This falls in the same family of
# dangerousness as `derive_tagged_enum_copy_constructor` and co.
#
# default: false
private_default_tagged_enum_constructor = false





[const]
# Whether a generated constant can be a static const in C++ mode. I have no
# idea why you would turn this off.
#
# default: true
allow_static_const = true

# Whether a generated constant can be constexpr in C++ mode.
#
# default: true
allow_constexpr = false

# This rule specifies the order in which constants will be sorted.
#
# "Name": sort by the name of the constant
# "None": keep order in which the constants have been parsed
#
# default: "None"
sort_by = "Name"




[macro_expansion]
# Whether bindings should be generated for instances of the bitflags! macro.
# default: false
bitflags = true






# Options for how your Rust library should be parsed

[parse]
# Whether to parse dependent crates and include their types in the output
# default: false
parse_deps = true

# A white list of crate names that are allowed to be parsed. If this is defined,
# only crates found in this list will ever be parsed.
#
# default: there is no whitelist (NOTE: this is the opposite of [])
include = ["webrender", "webrender_traits"]

# A black list of crate names that are not allowed to be parsed.
# default: []
exclude = ["libc"]

# Whether to use a new temporary target directory when running `rustc -Zunpretty=expanded`.
# This may be required for some build processes.
#
# default: false
clean = false

# Which crates other than the top-level binding crate we should generate
# bindings for.
#
# default: []
extra_bindings = ["my_awesome_dep"]

[parse.expand]
# A list of crate names that should be run through `cargo expand` before
# parsing to expand any macros. Note that if a crate is named here, it
# will always be parsed, even if the blacklist/whitelist says it shouldn't be.
#
# default: []
crates = ["euclid"]

# If enabled,  use the `--all-features` option when expanding. Ignored when
# `features` is set. For backwards-compatibility, this is forced on if
# `expand = ["euclid"]` shorthand is used.
#
# default: false
all_features = false

# When `all_features` is disabled and this is also disabled, use the
# `--no-default-features` option when expanding.
#
# default: true
default_features = true

# A list of feature names that should be used when running `cargo expand`. This
# combines with `default_features` like in your `Cargo.toml`. Note that the features
# listed here are features for the current crate being built, *not* the crates
# being expanded. The crate's `Cargo.toml` must take care of enabling the
# appropriate features in its dependencies
#
# default: []
features = ["cbindgen"]

[ptr]
# An optional string to decorate all pointers that are
# required to be non null. Nullability is inferred from the Rust type: `&T`,
# `&mut T` and `NonNull<T>` all require a valid pointer value.
non_null_attribute = "_Nonnull"

# Options specific to Cython bindings.

[cython]

# Header specified in the top level `cdef extern from header:` declaration.
#
# default: *
header = '"my_header.h"'

# `from module cimport name1, name2` declarations added in the same place
# where you'd get includes in C.
[cython.cimports]
module = ["name1", "name2"]
```





[reference]: https://doc.rust-lang.org/nightly/reference/type-layout.html#representations
[really-tagged-unions]: https://github.com/rust-lang/rfcs/blob/master/text/2195-really-tagged-unions.md
[section-cfgs]: #defines-and-cfgs
[file-it]: https://github.com/mozilla/cbindgen/issues/new
