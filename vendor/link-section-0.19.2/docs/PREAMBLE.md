A crate for defining linker-backed sections in Rust.

`link-section` provides two attributes:

- `#[section(...)]` defines a section handle. The handle is a `static` item used
  to inspect the section at runtime, usually as a slice. The handle's visibility
  determines where items may be submitted: public handles can be submitted from
  any module, while private handles can only be submitted from the module that
  defines them.
- `#[in_section(path::to::SECTION)]` submits an item to that section. A
  submitted item is an item annotated with `#[in_section(...)]`; depending on
  the section kind, it may also remain usable directly at the submission site.
  The `path::to::SECTION` must be visible to the submission site.

Together, these attributes let separately-declared items be collected into one
linker section and accessed through a single section handle.

## Visibility

Importantly, even though the linker is used to collect items, the visibility of
the section handle determines where items may be submitted: public handles can
be submitted from any crate (assuming the submitting crate references the one
with the collection handle), while private handles can only be submitted from
the crate that defines them.

The section name is generated from the name of the item and a location of the
item within the source tree. This means that you may have more than one
independent section with the item name in a project, and they will not conflict.

Note that if you are generating sections from a macro, you _must_ include at
least one token from the top-level macro call in the section definition to avoid
conflicts with tokens that are provided purely from the macro itself.

To allow for submission of items without visibility constraints, the crate
provides an `unsafe` option for the submission macro where the section's
name and attributes may be specified manually:

```rust
# mod root {
pub struct MyType(u8);

mod my_private_section {
    # use link_section::section;
    # use super::MyType;
    // Specify a section name so it can be used without a direct reference.
    #[section(typed, unsafe, name = my_crate::SECTION_NAME)]
    static MY_SECTION: link_section::TypedSection<MyType>;
}

mod elsewhere {
    # use link_section::in_section;
    # use super::MyType;
    // This must match the definition site!
    #[in_section(unsafe, name = my_crate::SECTION_NAME, type = typed)] // optionally: aux(main = MAIN_SECTION)
    static ITEM: MyType = MyType(42);
}
# }
```

### Syntax

Section definition:

 - `#[section(<kind>)]`
 - `#[section(<kind>, aux(main = <path::to::MAIN_SECTION>))]`
 - `#[section(unsafe, type = <kind>)]`
 - `#[section(unsafe, type = <kind>, name = <name>)]`

Section submission:

 - `#[in_section(path::to::SECTION)]`
 - `#[in_section(unsafe, type = <kind>, name = <name>)]`

## Section Kinds

There are five section kinds:

- `untyped`: Collects related code or data in one linker section without
  exposing a typed slice. This is useful for co-location, phase-specific code,
  or platform-specific section placement.
- `typed`: Stores values of one type and exposes them as an immutable slice.
- `mutable`: Stores values of one type and exposes them as a mutable slice.
- `reference`: Stores values of one type, exposes them as an immutable slice,
  and also lets each submitted item be used as a reference at its submission
  site.
- `movable`: Stores values of one type and exposes them as a mutable slice, and
  also lets each submitted item be used as a reference at its submission site.
  The entire section is available as a mutable slice, and items may be reordered
  during startup initialization (see [`TypedMovableSection`] for more details).

| Section Kind | Immutable Slice | Mutable Slice | `const` Items | `static` / Reference Items |
| ------------ | --------------- | ------------- | ------------- | -------------------------- |
| `untyped`    | ❌              | ❌            | ✅            | ✅                         |
| `typed`      | ✅              | ❌            | ✅            | ⚠️                         |
| `mutable`    | ✅              | ✅            | ✅            | ❌                         |
| `reference`  | ✅              | ❌            | ✅            | ✅                         |
| `movable`    | ✅              | ✅            | ❌            | ✅                         |

⚠️ Native targets support `static` submissions for `typed` sections; WASM uses
`const` submissions only.

## Submitting Items

Items are submitted with `#[in_section(SECTION)]`.

A `const` submission copies the value into the section. The original constant
remains usable as a normal Rust constant, and the section receives its own
stored copy.

```rust
# pub struct MyType(u8); impl MyType { const fn new() -> Self { Self(0) } }
# use link_section::{in_section, section};
# #[section(typed)] pub static MY_SECTION: link_section::TypedSection<MyType>;
#[in_section(MY_SECTION)]
pub const ITEM: MyType = MyType::new();
```

A `static` submission stores the `static` directly in the section. References to
the `static` and references obtained from the section slice point at the same
underlying object. `static` submissions are supported for typed sections on
native targets and for reference sections.

```rust
# pub struct MyType(u8); impl MyType { const fn new() -> Self { Self(0) } }
# use link_section::{in_section, section};
# #[section(typed)] pub static MY_SECTION: link_section::TypedSection<MyType>;
#[in_section(MY_SECTION)]
pub static ITEM: MyType = MyType::new();
```

A `fn` submitted to a typed section is stored as a function pointer. The
function body itself is not placed into the typed data section.

```rust
# use link_section::{in_section, section};
#[section(typed)]
pub static FUNCTIONS: link_section::TypedSection<fn()>;

#[in_section(FUNCTIONS)]
pub fn callback() {
    // ...
}
```

## Platform Support

| Platform                 | Support                                         |
| ------------------------ | ----------------------------------------------- |
| Linux                    | ✅ Supported, uses orphan section handling (§1) |
| \*BSD                    | ✅ Supported, uses orphan section handling (§1) |
| macOS                    | ✅ Fully supported                              |
| Windows                  | ✅ Fully supported                              |
| WASM                     | ✅ Fully supported, via emulation (§2)          |
| AIX                      | ✅ Supported (§3) (§4)                          |
| Other LLVM/GCC platforms | ✅ Supported, uses orphan section handling (§1) |

(§1) Orphan section handling is a feature of the linker that allows sections to
be defined without a pre-defined name.

(§2) WASM requires `const` items, and uses `ctor`-like initialization to copy
data to a contiguous section. To access link-section slices in WASM in `#[ctor]`
functions, make sure to use at least `#[ctor(priority = 1)]`.

(§3) AIX requires `-C link-arg=-bdbg:namedsects:ss` which enables functionality
similar to LLVM/GCC's orphan section handling.

(§4) Empty sections are not currently supported: ensure every section has at least
one item, or pass the `-C link-arg=-berok` linker flag to ignore errors.

## Platform Details

Each platform has a slightly different implementation of section control.

### Linux and other LLVM/GCC platforms

- Has start/end symbols: ✅ (C-compatible names only)
- Supports linker sorting: ❌

On Linux and other LLVM/GCC platforms, the linker supports orphan sections,
which allow sections to be defined without a pre-defined name. These sections
are emitted as if they were r/w `.data`. For sections with C-compatible names,
the linker will emit start/end symbols for the section.

Orphan sections are not sorted via numeric suffix (e.g.: `SECTION.1`,
`SECTION.2`, etc.) with the default linker script.

### macOS

- Has start/end symbols: ✅
- Supports linker sorting: ❌

On macOS, sections are configured via `__DATA` or `__TEXT` prefix and option
suffixes (`regular`, `no_dead_strip`, etc.). The linker emits start and stop
symbols, but Rust requires a (somewhat-stable) `\x01` prefix to avoid mangling
the section name. macOS does not support ordering in the linker.

### Windows

- Has start/end symbols: ❌
- Supports linker sorting: ✅

On Windows, the linker does not emit start/end symbols, but all sections with a
common prefix are automatically sorted by suffix, allowing us to use suffixes to
control placement of start/stop symbols that we emit.

See
[this blog post](https://devblogs.microsoft.com/oldnewthing/20181107-00/?p=100155)
and
[this blog post](https://devblogs.microsoft.com/oldnewthing/20181108-00/?p=100165)
for more details about the alphabetical sorting rule.

### WASM

- Has start/end symbols: ❌
- Supports linker sorting: ❌

On WASM platforms there are no linker-provided start/end symbols and no linker
ordering. Normally, WASM does not support placing arbitrary data in link
sections - only non-pointer data is supported. To work around this, the WASM
support uses `const` items and pre-`main` construction functions to gather each
entry into a contiguous section allocated at startup.

Each submitted item emits a plain (linear-memory) static "list node" plus an
`.init_array.0` constructor that threads the node onto an intrusive linked list
rooted in the section. Because these nodes are interior-mutable and have their
address taken, LLVM cannot merge them even under fat LTO, so the item
count is always exact.

The list is materialised into one contiguous allocation by an eager,
ordered constructor. Item submissions run at `.init_array.0` (priority 0), and
each section emits a finalization constructor at `.init_array.1` (priority 1).
Because wasm-ld orders `.init_array.*` by the integer value of the suffix, the
finalizer is guaranteed to run after every submission and before any
`#[ctor(priority >= 2)]` and before `main` — a well-defined pre-`main` time
rather than "whenever the section is first read".

[`Ref`] / [`MovableRef`] handles carry a pointer to their owning section and also
flatten it on first dereference. That lazy path is an idempotent backstop for the
one remaining tie: an explicit `#[ctor(priority = 1)]` that runs before the
finalizer. As a result the `#[ctor]` priority requirement is the same as
elsewhere: to read a link section or dereference a [`Ref`] / [`MovableRef`]
handle from a `#[ctor]`, use at least `#[ctor(priority = 1)]`. Reads from `main`
(or any later constructor) are always safe.

A `#[ctor(priority = 0)]` that reads section *bounds* (not a [`Ref`]) is the one
corner that breaks: it interleaves with `.init_array.0` submissions, triggers an
early lazy flatten, and then any later submission panics (the section is already
materialised). The contract — use at least priority 1 — is unchanged, but the
panic now names the section and the likely cause.

### AIX

- Has start/end symbols: ✅
- Supports linker sorting: ❌

AIX maps Rust's `#[link_section]` to `csect`s (Control Sections), which act like
subsections of the larger `.text` and `.data` sections
<sup>[↳](https://www.ibm.com/docs/kk/aix/7.2.0?topic=program-understanding-programming-toc)</sup>.
A `csect` is the smallest, indivisible unit of code or data.

By default, AIX does not have section start/stop symbols, but the most recent
versions of the linker added a new `-bdbg:namedsects:ss` flag which enables
section start/stop symbols
<sup>[↳](https://reviews.llvm.org/D124857?id=427067)</sup>.

This flag can be set with `-C link-arg=-bdbg:namedsects:ss` (or by upgrading to
a recent Rust version that sets this automatically
<sup>[↳](https://rust.googlesource.com/rust/+/ad582a586550bf2c72e963939f61a71df1af7c0c%5E%21/#F0)</sup>
to support link sections.

The linker will report an error like this if the start/stop symbols are not
found:

```text
= note: ld: 0711-317 ERROR: Undefined symbol: __start__data_link_section_DATABASES
        ld: 0711-317 ERROR: Undefined symbol: __stop__data_link_section_DATABASES
        ld: 0711-345 Use the -bloadmap or -bnoquiet option to obtain more information.
```

In addition, the linker may report the same errors if a section is empty. It is
recommended that you either (1) provide a sentinel item for AIX that can be skipped
in the slice, or (2) pass the `-C link-arg=-berok` linker flag to ignore the error.

For debugging AIX link-section issues, `-C link-arg=-bmap:[path]/linker.out` and
`-C link-arg=-bnoquiet` may also be useful.

AIX supports a special mode to strip (`strip -r`) that preserves structural
symbols like `csect`s and exports. A future version of `link-section` may add
support for loading `csect` bounds from the binary's symbol table.

```toml
[target.powerpc64-ibm-aix]
rustflags = [
    "-C", "link-arg=-bdbg:namedsects:ss",   # required
    "-C", "link-arg=-bmap:linker.out",      # for debugging
    "-C", "link-arg=-bnoquiet",             # for debugging
]
```

## Typed Sections

Typed sections provide a section where all items are of a specific, sized type.
The typed section may be accessed as a slice of the type at zero cost if
desired.

A typed section can be created from either `static` or `const` items.

For `const` items: a copy of the `const` is materialised at link time, while the
constant itself remains available for use as a constant in `const` contexts.

For `static` items: the static is stored directly in the link section.

`fn` items are special-cased and stored as function pointers in the typed
section.

## Exclusive Access

Mutable sections (ie: [`TypedMutableSection`] and [`TypedMovableSection`])
require exclusive access to the section's memory while calling
[`TypedMutableSection::as_mut_slice`] or [`TypedMovableSection::as_mut_slice`].

This is normally satisfied only during pre-`main` initialization (for example
inside a `#[ctor]`). After `main`, the caller must guarantee no concurrent reads
or writes from other threads and no active Rust references into the section.

It is highly recommended not to access the mutable references after `main` has
started.

## Usage

Create an untyped section using the `#[section]` macro that keeps related items
in close proximity:

```rust
use link_section::{in_section, section};

#[section(untyped)]
pub static CODE_SECTION: link_section::Section;

#[in_section(CODE_SECTION)]
pub fn link_section_function() {
    println!("link_section_function");
}
```

Create a typed section using the `#[section]` macro that stores items of a
specific, sized type from `static` or `const` items:

```rust
mod my_registry {
    use link_section::{in_section, section};

    pub struct MyStruct {
        name: &'static str,
    }

    #[section(typed)]
    pub static MY_REGISTRY: link_section::TypedSection<MyStruct>;

    // Registers a `const` item.
    mod register_a_constant {
        use super::*;

        // A copy of this constant is registered in the link section.
        #[in_section(MY_REGISTRY)]
        pub const LINKED_MY_STRUCT: MyStruct = MyStruct { name: "my_struct" };
    }

    // Registers a `static` item.
    mod register_a_static {
        use super::*;

        // This static lives directly in the link section.
        #[in_section(MY_REGISTRY)]
        pub static LINKED_MY_STRUCT: MyStruct = MyStruct { name: "my_struct_2" };
    }
}
```

## Inspiration

`link-section` would have been far more challenging to implement without dtolnay's great `linkme` project paving the way.
