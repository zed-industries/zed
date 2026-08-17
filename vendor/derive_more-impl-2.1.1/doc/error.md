# Using `#[derive(Error)]`

Deriving `Error` will generate an `Error` implementation, that contains
(depending on the type) a `source()` and a `provide()` method. Please note,
at the time of writing `provide()` is only supported on nightly Rust. So you
have to use that to make use of it.

For a struct, these methods always do the same. For an `enum` they have separate
behaviour for each of the variants. The variant is first matched and then the
implementation will do the same as it would have done if the variant was a
struct.

Usually when you derive `Error` you will also want to [derive `Display`](crate::Display) and
often [`From` as well](crate::From).


### When and how does it derive `source()`?

1. It's a struct/variant with named fields and one is the fields is
   called `source`. Then it would return that field as the `source`.
2. It's a tuple struct/variant and there's exactly one field that is not used as
   the `backtrace`. So either a tuple struct with one field, or one with two where one
   is the `backtrace`. Then it returns this field as the `source`.
3. One of the fields is annotated with `#[error(source)]`. Then it would
   return that field as the `source`.

### When and how does it derive `provide()`?

1. It's a struct/variant with named fields and one of the fields is
   called `backtrace`. Then it would return that field as the `backtrace`.
2. It's a tuple struct/variant and the type of exactly one of the fields is
   called `Backtrace`. Then it would return that field as the `backtrace`.
3. One of the fields is annotated with `#[error(backtrace)]`. Then it would
   return that field as the `backtrace`.

### Ignoring fields for derives

It's possible to ignore a field or a whole enum variant completely for this
derive using the `#[error(ignore)]` attribute. This will ignore it both for
detecting `backtrace` and `source`. It's also possible to mark a field only
ignored for one of these methods by using `#[error(not(backtrace))]` or
`#[error(not(source))]`.


### What works in `no_std`?

`Error` derive fully works on `no_std` environments, except the `provide()`
method usage, because the `Backtrace` type is only available in `std`.


### `Option`al fields

Deriving `source()` is supported naturally for `Option<_>`-typed fields.

> **WARNING**: Field type is **considered as `Option<_>` only syntactically**,
> meaning that:
> - Neither renaming `std::option::Option` would work:
>   ```rust,compile_fail
>   # use derive_more::{Display, Error};
>   #
>   #[derive(Debug, Display, Error)]
>   struct Simple;
>
>   #[derive(Debug, Display, Error)]
>   #[display("Oops!")]
>   struct Generic(RenamedOption<Simple>); // not recognized as `Option`
>   type RenamedOption<T> = Option<T>;
>   ```
> - Nor using any custom type named as `Option` would:
>   ```rust,compile_fail
>   # use derive_more::{Display, Error};
>   #
>   # #[derive(Debug, Display, Error)]
>   # struct Simple;
>   #
>   #[derive(Debug, Display, Error)]
>   #[display("No way!")]
>   struct Generic(Option<Simple>);        // cannot use `?` syntax
>   struct Option<T>(T);
>   ```

> **TIP**: If `std::option::Option` for some reason needs to be renamed,
> annotate the field with the `#[error(source(optional))]` attribute:
> ```rust
> # use derive_more::{Display, Error};
> #
> #[derive(Debug, Display, Error)]
> struct Simple;
>
> #[derive(Debug, Display, Error)]
> #[display("Oops!")]
> struct Generic(#[error(source(optional))] RenamedOption<Simple>);
> type RenamedOption<T> = Option<T>;
>  ```




## Example usage

```rust
# #![cfg_attr(nightly, feature(error_generic_member_access))]
# // Nightly requires enabling this feature:
# // #![feature(error_generic_member_access)]
# #[cfg(not(nightly))] fn main() {}
# #[cfg(nightly)] fn main() {
# use core::error::{request_ref, request_value, Error as __};
# use std::backtrace::Backtrace;
#
# use derive_more::{Display, Error, From};
#
# type RenamedOption<T> = Option<T>;
#
// `std::error::Error` requires `std::fmt::Debug` and `std::fmt::Display`,
// so we can also use `derive_more::Display` for fully declarative
// error-type definitions.

#[derive(Default, Debug, Display, Error)]
struct Simple;

#[derive(Default, Debug, Display, Error)]
struct WithSource {
    source: Simple,
}
#[derive(Default, Debug, Display, Error)]
#[display("WithOptionalSource")]
struct WithOptionalSource {
    source: Option<Simple>,
}
#[derive(Default, Debug, Display, Error)]
struct WithExplicitSource {
    #[error(source)]
    explicit_source: Simple,
}
#[derive(Default, Debug, Display, Error)]
#[display("WithExplicitOptionalSource")]
struct WithExplicitOptionalSource {
    #[error(source)]
    explicit_source: Option<Simple>,
}
#[derive(Default, Debug, Display, Error)]
#[display("WithExplicitOptionalSource")]
struct WithExplicitRenamedOptionalSource {
    #[error(source(optional))]
    explicit_source: RenamedOption<Simple>,
}

#[derive(Default, Debug, Display, Error)]
struct Tuple(Simple);

#[derive(Default, Debug, Display, Error)]
struct WithoutSource(#[error(not(source))] i32);

#[derive(Debug, Display, Error)]
#[display("An error with a backtrace")]
struct WithSourceAndBacktrace {
    source: Simple,
    backtrace: Backtrace,
}

// derive_more::From fits nicely into this pattern as well
#[derive(Debug, Display, Error, From)]
enum CompoundError {
    Simple,
    WithSource {
        source: Simple,
    },
    #[display("WithOptionalSource")]
    WithOptionalSource {
        source: Option<Simple>,
    },
    #[from(ignore)]
    WithBacktraceFromSource {
        #[error(backtrace)]
        source: Simple,
    },
    #[display("{source}")]
    WithDifferentBacktrace {
        source: Simple,
        backtrace: Backtrace,
    },
    WithExplicitSource {
        #[error(source)]
        explicit_source: WithSource,
    },
    #[display("WithExplicitOptionalSource")]
    WithExplicitOptionalSource {
        #[error(source)]
        explicit_source: Option<WithSource>,
    },
    #[display("WithExplicitRenamedOptionalSource")]
    WithExplicitRenamedOptionalSource {
        #[error(source(optional))]
        explicit_source: RenamedOption<WithExplicitRenamedOptionalSource>,
    },
    #[from(ignore)]
    WithBacktraceFromExplicitSource {
        #[error(backtrace, source)]
        explicit_source: WithSource,
    },
    Tuple(WithExplicitSource),
    #[display("TupleOptional")]
    TupleOptional(Option<WithExplicitSource>),
    WithoutSource(#[error(not(source))] Tuple),
}

assert!(Simple.source().is_none());
assert!(request_ref::<Backtrace>(&Simple).is_none());

assert!(WithSource::default().source().is_some());
assert!(WithOptionalSource { source: Some(Simple) }.source().is_some());

assert!(WithExplicitSource::default().source().is_some());
assert!(WithExplicitOptionalSource { explicit_source: Some(Simple) }.source().is_some());
assert!(WithExplicitRenamedOptionalSource { explicit_source: Some(Simple) }.source().is_some());

assert!(Tuple::default().source().is_some());

assert!(WithoutSource::default().source().is_none());

let with_source_and_backtrace = WithSourceAndBacktrace {
    source: Simple,
    backtrace: Backtrace::capture(),
};
assert!(with_source_and_backtrace.source().is_some());
assert!(request_ref::<Backtrace>(&with_source_and_backtrace).is_some());

assert!(CompoundError::Simple.source().is_none());

assert!(CompoundError::from(Simple).source().is_some());
assert!(CompoundError::from(Some(Simple)).source().is_some());

assert!(CompoundError::from(WithSource::default()).source().is_some());
assert!(CompoundError::from(Some(WithSource::default())).source().is_some());

assert!(CompoundError::from(WithExplicitSource::default()).source().is_some());
assert!(CompoundError::from(Some(WithExplicitSource::default())).source().is_some());
assert!(CompoundError::from(Some(WithExplicitRenamedOptionalSource { explicit_source: Some(Simple) })).source().is_some());

assert!(CompoundError::from(Tuple::default()).source().is_none());
# }
```
