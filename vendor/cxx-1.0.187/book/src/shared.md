{{#title Shared types — Rust ♡ C++}}
# Shared types

Shared types enable *both* languages to have visibility into the internals of a
type. This is in contrast to opaque Rust types and opaque C++ types, for which
only one side gets to manipulate the internals.

Unlike opaque types, the FFI bridge is allowed to pass and return shared types
by value.

The order in which shared types are written is not important. C++ is order
sensitive but CXX will topologically sort and forward-declare your types as
necessary.

## Shared structs and enums

For enums, only C-like a.k.a. unit variants are currently supported.

```rust,noplayground
#[cxx::bridge]
mod ffi {
    struct PlayingCard {
        suit: Suit,
        value: u8,  // A=1, J=11, Q=12, K=13
    }

    enum Suit {
        Clubs,
        Diamonds,
        Hearts,
        Spades,
    }

    unsafe extern "C++" {
        fn deck() -> Vec<PlayingCard>;
        fn sort(cards: &mut Vec<PlayingCard>);
    }
}
```

## The generated data structures

Shared structs compile to an aggregate-initialization compatible C++ struct.

Shared enums compile to a C++ `enum class` with a sufficiently sized integral
base type decided by CXX.

```cpp
// generated header

struct PlayingCard final {
  Suit suit;
  uint8_t value;
};

enum class Suit : uint8_t {
  Clubs = 0,
  Diamonds = 1,
  Hearts = 2,
  Spades = 3,
};
```

Because it is not UB in C++ for an `enum class` to hold a value different from
all of the listed variants, we use a Rust representation for shared enums that
is compatible with this. The API you'll get is something like:

```rust,noplayground
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct Suit {
    pub repr: u8,
}
#[allow(non_upper_case_globals)]
impl Suit {
    pub const Clubs: Self = Suit { repr: 0 };
    pub const Diamonds: Self = Suit { repr: 1 };
    pub const Hearts: Self = Suit { repr: 2 };
    pub const Spades: Self = Suit { repr: 3 };
}
```

Notice you're free to treat the enum as an integer in Rust code via the public
`repr` field.

Pattern matching with `match` still works but will require you to write wildcard
arms to handle the situation of an enum value that is not one of the listed
variants.

```rust,noplayground
fn main() {
    let suit: Suit = /*...*/;
    match suit {
        Suit::Clubs => ...,
        Suit::Diamonds => ...,
        Suit::Hearts => ...,
        Suit::Spades => ...,
        _ => ...,  // fallback arm
    }
}
```

If a shared struct has generic lifetime parameters, the lifetimes are simply not
represented on the C++ side. C++ code will need care when working with borrowed
data (as usual in C++).

```rust,noplayground
#[cxx::bridge]
mod ffi {
    struct Borrowed<'a> {
        flags: &'a [&'a str],
    }
}
```

```cpp
// generated header

struct Borrowed final {
  rust::Slice<const rust::Str> flags;
};
```

## Enum discriminants

You may provide explicit discriminants for some or all of the enum variants, in
which case those numbers will be propagated into the generated C++ `enum class`.

```rust,noplayground
#[cxx::bridge]
mod ffi {
    enum SmallPrime {
        Two = 2,
        Three = 3,
        Five = 5,
        Seven = 7,
    }
}
```

Variants without an explicit discriminant are assigned the previous discriminant
plus 1. If the first variant has not been given an explicit discriminant, it is
assigned discriminant 0.

By default CXX represents your enum using the smallest integer type capable of
fitting all the discriminants (whether explicit or implicit). If you need a
different representation for reasons, provide a `repr` attribute.

```rust,noplayground
#[cxx::bridge]
mod ffi {
    #[repr(i32)]
    enum Enum {
        Zero,
        One,
        Five = 5,
        Six,
    }
}
```

```cpp
// generated header

enum class Enum : int32_t {
  Zero = 0,
  One = 1,
  Five = 5,
  Six = 6,
};
```

## Extern enums

If you need to interoperate with an already existing enum for which an existing
C++ definition is the source of truth, make sure that definition is provided by
some header in the bridge and then declare your enum *additionally* as an extern
C++ type.

```rust,noplayground
#[cxx::bridge]
mod ffi {
    enum Enum {
        Yes,
        No,
    }

    extern "C++" {
        include!("path/to/the/header.h");
        type Enum;
    }
}
```

CXX will recognize this pattern and, instead of generating a C++ definition of
the enum, will instead generate C++ static assertions asserting that the
variants and discriminant values and integer representation written in Rust all
correctly match the existing C++ enum definition.

Extern enums support all the same features as ordinary shared enums (explicit
discriminants, repr). Again, CXX will static assert that all of those things you
wrote are correct.

## Derives

The following standard traits are supported in `derive(...)` within the CXX
bridge module.

- `Clone`
- `Copy`
- `Debug`
- `Default`
- `Eq`
- `Hash`
- `Ord`
- `PartialEq`
- `PartialOrd`
- `BitAnd` (enums only)
- `BitOr` (enums only)
- `BitXor` (enums only)

Note that shared enums automatically always come with impls of `Copy`, `Clone`,
`Eq`, and `PartialEq`, so you're free to omit those derives on an enum.

```rust,noplayground
#[cxx::bridge]
mod ffi {
    #[derive(Clone, Debug, Hash)]
    struct ExampleStruct {
        x: u32,
        s: String,
    }

    #[derive(Hash, Ord, PartialOrd)]
    enum ExampleEnum {
        Yes,
        No,
    }
}
```

The derives naturally apply to *both* the Rust data type *and* the corresponding
C++ data type:

- `Hash` gives you a specialization of [`template <> struct std::hash<T>`][hash] in C++
- `PartialEq` produces `operator==` and `operator!=`
- `PartialOrd` produces `operator<`, `operator<=`, `operator>`, `operator>=`
- `BitAnd` produces `operator&`
- `BitOr` produces `operator|`
- `BitXor` produces `operator^`

[hash]: https://en.cppreference.com/w/cpp/utility/hash

## Alignment

The attribute `repr(align(…))` sets a minimum required alignment for a shared
struct. The alignment value must be a power of two in the range 2<sup>0</sup> to
2<sup>13</sup>.

This turns into an [`alignas`] specifier in C++.

[`alignas`]: https://en.cppreference.com/w/cpp/language/alignas.html

```rust,noplayground
#[cxx::bridge]
mod ffi {
    #[repr(align(4))]
    struct ExampleStruct {
        b: [u8; 4],
    }
}
```
