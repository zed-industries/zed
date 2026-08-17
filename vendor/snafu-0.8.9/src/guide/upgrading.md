# Upgrading from previous releases

- [Version 0.7 → 0.8](#version-07--08)
- [Version 0.6 → 0.7](#version-06--07)
- [Version 0.5 → 0.6](#version-05--06)
- [Version 0.4 → 0.5](#version-04--05)
- [Version 0.3 → 0.4](#version-03--04)
- [Version 0.2 → 0.3](#version-02--03)
- [Version 0.1 → 0.2](#version-01--02)

## Version 0.7 → 0.8

### Fields named `location` are no longer automatically implicitly generated

Previously, fields named `location` would be implicitly
generated. However, this proved to be confusing and usually not what
users wanted. If you have `#[snafu(implicit(false))]` on a field named
`location`, that can be removed. If you are using this functionality,
you will need to add `#[snafu(implicit)]` on those fields.

#### Before

```rust,ignore
#[derive(Debug, Snafu)]
struct ErrorWithGeneratedLocation {
    location: snafu::Location,
}

#[derive(Debug, Snafu)]
struct ErrorWithNonGeneratedLocation {
    #[snafu(implicit(false))]
    location: usize,
}
```

#### After

```rust,ignore
#[derive(Debug, Snafu)]
struct ErrorWithGeneratedLocation {
    #[snafu(implicit)]
    location: snafu::Location,
}

#[derive(Debug, Snafu)]
struct ErrorWithNonGeneratedLocation {
    location: usize,
}
```

### The default implementation of `Display` no longer includes the source

To better follow the Error Handling Project Group's suggested
[guideline][], the generated implementation of `Display` no longer
includes the underlying source's `Display`. High quality error types
already define their own `Display` format strings via
`snafu(display(...))`, so this should not impact many users.

To combine all `Display` messages in the entire error chain, you can
use higher-level tools like [`report`](macro@report) or [`Report`][] or
lower-level tools like [`CleanedErrorText`][].

If you wish to ignore the suggested guideline, you will need to add a
`Display` implementation that explicitly includes the source text.

[guideline]: https://blog.rust-lang.org/inside-rust/2021/07/01/What-the-error-handling-project-group-is-working-towards.html#guidelines-for-implementing-displayfmt-and-errorsource

#### Before

```rust,ignore
#[derive(Debug, Snafu)]
struct ErrorWithDefaultDisplay {
    source: std::io::Error,
}
```

#### After

```rust,ignore
#[derive(Debug, Snafu)]
#[snafu(display("ErrorWithDefaultDisplay: {source}"))]
struct ErrorWithDefaultDisplay {
    source: std::io::Error,
}
```

### Minimum supported version of Rust is now 1.56

If you are writing a library, you will need to increase your minimum
supported version of Rust to 1.56 or better. If you are writing an
application, you should be able to upgrade your installed compiler by
the same mechanism that you installed it.

## Version 0.6 → 0.7

Upgrading should be a tedious but straightforward process. To assist
upgrading your code, you can use the [snafu-upgrade-assistant], which
attempts to automatically update breaking changes.

[snafu-upgrade-assistant]: https://github.com/shepmaster/snafu-upgrade-assistant

### Context selector names have changed

Previously, context selector names for enum errors exactly matched
their corresponding enum variant names. This caused a large amount of
confusion for people new to SNAFU. It was also inconsistent with
context selector names for struct errors.

Now, context selectors for both enum and struct errors use the `Snafu`
suffix. Any existing `Error` suffix is removed before `Snafu` is
added.

#### Before

```rust,ignore
#[derive(Debug, Snafu)]
struct StructError;

#[derive(Debug, Snafu)]
enum EnumError {
    VariantError,
}

ensure!(false, StructContext);
ensure!(false, VariantError);
```

#### After

```rust,ignore
#[derive(Debug, Snafu)]
struct StructError;

#[derive(Debug, Snafu)]
enum EnumError {
    VariantError,
}

ensure!(false, StructSnafu);
ensure!(false, VariantSnafu);
```

### `with_context` takes an argument

`ResultExt::with_context`, `TryFutureExt::with_context`, and
`TryStreamExt::with_context` now pass the error into the closure.

#### Before

```rust,ignore
some_result.with_context(|| ContextSelector);
```

#### After

```rust,ignore
some_result.with_context(|_| ContextSelector);
```

### String attribute parsing is no longer supported

Previously, SNAFU allowed an alternate attribute specification format
to support versions of Rust before 1.34. Since the minimum version has
been increased, this format is no longer required. Use the
parenthesized format instead:

#### Before

```rust,ignore
#[snafu(display = r#"("a format string with arguments: {}", info)"#)]
```

#### After

```rust,ignore
#[snafu(display("a format string with arguments: {}", info))]
```

### Minimum supported version of Rust is now 1.34

If you are writing a library, you will need to increase your minimum
supported version of Rust to 1.34 or better. If you are writing an
application, you should be able to upgrade your installed compiler by
the same mechanism that you installed it.

## Version 0.5 → 0.6

### Minimum supported version of Rust is now 1.31

If you are writing a library, you will need to increase your minimum
supported version of Rust to 1.31 or better. If you are writing an
application, you should be able to upgrade your installed compiler by
the same mechanism that you installed it.

### Backtraces

The `Backtrace` type is now always available, so it is encouraged to
make liberal use of it in your errors. If you are writing an
application that displays backtraces, make sure to enable the
[`backtrace` feature flag](crate::guide::feature_flags) so that
backtraces are populated when they are created.

Implementations of `Backtrace::default` and `Backtrace::new` have been
removed and replaced with `GenerateBacktrace::generate`.

The `backtrace-crate` feature flag has been renamed to
`backtraces-impl-backtrace-crate`. The backtrace returned by
`ErrorCompat::backtrace` is now the `backtrace::Backtrace` type when
this flag is enabled, so the implementation of `AsRef` has been
removed.

### Futures

Support for the standard library features has been stabilized, so the
feature flag has been renamed from `unstable-futures` to `futures`.

## Version 0.4 → 0.5

### `backtrace(delegate)` replaced with `backtrace`

Previously, if you wanted to delegate backtrace creation to
another error, you would specify `#[snafu(backtrace(delegate))]`
on the source field that references the other error.

Now, you specify the simpler `#[snafu(backtrace)]`.  Since source
fields must be error types, and backtrace fields must be
`Backtrace` types, this is unambiguous and simplifies the API.

#### Before

```rust,ignore
#[derive(Debug, Snafu)]
enum Error {
    MyVariant {
        #[snafu(backtrace(delegate))]
        source: OtherError,
    },
}
```

#### After

```rust,ignore
#[derive(Debug, Snafu)]
enum Error {
    MyVariant {
        #[snafu(backtrace)]
        source: OtherError,
    },
}
```

### `source(from)` implies `source`

Previously, if you had wanted to treat a field that wasn't named
"source" as a source field, *and* you wanted to transform the
field from another type, you had to specify both
`#[snafu(source)]` and `#[snafu(source(from(...)))]`.

Now, `#[snafu(source(from(...)))]` implies `#[snafu(source)]` --
it automatically treats the field as a source field regardless of
its name, so you can remove the `#[snafu(source)]` attribute.

#### Before

```rust,ignore
#[derive(Debug, Snafu)]
enum Error {
    CauseIsAnError {
        #[snafu(source)]
        #[snafu(source(from(Error, Box::new)))]
        cause: Box<Error>,
    },
}
```

#### After

```rust,ignore
#[derive(Debug, Snafu)]
enum Error {
    CauseIsAnError {
        #[snafu(source(from(Error, Box::new)))]
        cause: Box<Error>,
    },
}
```

### New errors for attribute misuse and duplication

Previously, SNAFU would ignore `#[snafu(...)]` attributes that
were used in invalid locations.  If attributes were duplicated,
either the first or last would apply (depending on the attribute)
and the rest would be ignored.

One example is specifying `#[snafu(source(from(...)))]` on an
enum variant instead of the source field in that variant:

```rust,ignore
#[derive(Debug, Snafu)]
enum Error {
    // This used to be ignored, and will now cause an error:
    #[snafu(source(from(Error, Box::new)))]
    MyVariant {
        source: Box<Error>,
    },
}
```

Now, compiler errors will be emitted that point to any misused or
duplicated attributes.

## Version 0.3 → 0.4

### `Context` vs. `IntoError`

The `Context` type and related `From` implementations have been
removed in favor of the [`IntoError`] trait.
If you were making use of this for custom conversions, you will need
to update your trait bounds:

#### Before

```rust,ignore
fn example<C, E>(context: C) -> MyType<E>
where
    snafu::Context<SomeError, C>: Into<E>;
```

#### After

```rust,ignore
fn example<C, E>(context: C) -> MyType<E>
where
    C: snafu::IntoError<E, Source = SomeError>,
    E: std::error::Error + snafu::ErrorCompat;
```

### `Borrow<std::error::Error>`

SNAFU no longer generates `Borrow<std::error::Error>`
implementations for SNAFU error types (sorry for the whiplash if
you were affected by this when upgrading to 0.3).

## Version 0.2 → 0.3

Minimal changes should be required: if you previously implemented
`Borrow<std::error::Error>` for a SNAFU error type, you should
remove that implementation and allow SNAFU to implement it for
you.

## Version 0.1 → 0.2

Support for the `snafu::display` attribute was removed as this
type of attribute was [never intended to be
supported][oops]. Since this required a SemVer-incompatible
version, the attribute format has also been updated and
normalized.

1. Attributes have been renamed
    - `snafu_display` and `snafu::display` became `snafu(display)`.
    - `snafu_visibility` became `snafu(visibility)`
    - `snafu_backtrace` became `snafu(backtrace)`

1. Support for `snafu_display` with individually-quoted format
   arguments was removed. Migrate to either the "clean" or "all
   one string" styles, depending on what version of Rust you are
   targeting.

[oops]: https://github.com/rust-lang/rust/pull/58899

### Before

```rust,ignore
#[derive(Debug, Snafu)]
enum DisplayUpdate {
    #[snafu::display("Format and {}", argument)]
    CleanStyle { argument: i32 },

    #[snafu_display("Format and {}", "argument")]
    QuotedArgumentStyle { argument: i32 },

    #[snafu_display = r#"("Format and {}", argument)"#]
    AllOneStringStyle { argument: i32 },
}
```

```rust,ignore
#[derive(Debug, Snafu)]
enum VisibilityUpdate {
    #[snafu_visibility(pub(crate))]
    CleanStyle,

    #[snafu_visibility = "pub(crate)"]
    AllOneStringStyle,
}
```

### After

```rust,ignore
# use snafu::Snafu;
#[derive(Debug, Snafu)]
enum DisplayUpdate {
    #[snafu(display("Format and {}", argument))]
    CleanStyle { argument: i32 },

    #[snafu(display = r#"("Format and {}", argument)"#)]
    QuotedArgumentStyle { argument: i32 },

    #[snafu(display = r#"("Format and {}", argument)"#)]
    AllOneStringStyle { argument: i32 },
}
```

```rust,ignore
# use snafu::Snafu;
#[derive(Debug, Snafu)]
enum VisibilityUpdate {
    #[snafu(visibility(pub(crate)))]
    CleanStyle,

    #[snafu(visibility = "pub(crate)")]
    AllOneStringStyle,
}
```
