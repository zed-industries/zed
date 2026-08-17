# What `#[derive(FromStr)]` generates

Deriving `FromStr` only works for enums/structs with no fields
or newtypes (structs with only a single field). The result is
that you will be able to call the `parse()` method on a string
to convert it to your newtype. This only works when the wrapped
type implements `FromStr` itself.




## Forwarding

Deriving forwarding implementation is only supported for newtypes
(structs with only a single field).


### Tuple structs

When deriving `FromStr` for a tuple struct with one field:
```rust
# use derive_more::FromStr;
#
#[derive(FromStr, Debug, Eq, PartialEq)]
struct MyInt(i32);

assert_eq!("5".parse::<MyInt>().unwrap(), MyInt(5));
```

Code like this is generated:
```rust
# struct MyInt(i32);
impl derive_more::core::str::FromStr for MyInt {
    type Err = <i32 as derive_more::core::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(i32::from_str(s)?))
    }
}
```


### Regular structs

When deriving `FromStr` for a regular struct with one field:
```rust
# use derive_more::FromStr;
#
#[derive(FromStr, Debug, Eq, PartialEq)]
struct Point1D {
    x: i32,
}

assert_eq!("100".parse::<Point1D>().unwrap(), Point1D { x: 100 });
```

Code like this is generated:
```rust
# struct Point1D {
#     x: i32,
# }
impl derive_more::core::str::FromStr for Point1D {
    type Err = <i32 as derive_more::core::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            x: i32::from_str(s)?,
        })
    }
}
```




## Flat representation

Deriving flat string representation is only supported for empty enums and
structs (with no fields).


### Empty enums

When deriving `FromStr` for enums with empty variants, it will generate a
`from_str()` method converting strings matching the variant name to the variant.
If using a case-insensitive match would give a unique variant (i.e. you don't have
both `MyEnum::Foo` and `MyEnum::foo` variants), then case-insensitive matching will
be used, otherwise it will fall back to exact string matching.

Since the string may not match any variants an error type is needed, so the
`derive_more::FromStrError` is used for that purpose.

Given the following enum:
```rust
# use derive_more::FromStr;
#
#[derive(FromStr, Debug, Eq, PartialEq)]
enum EnumNoFields {
    Foo,
    Bar,
    Baz,
    BaZ,
}

assert_eq!("foo".parse::<EnumNoFields>().unwrap(), EnumNoFields::Foo);
assert_eq!("Foo".parse::<EnumNoFields>().unwrap(), EnumNoFields::Foo);
assert_eq!("FOO".parse::<EnumNoFields>().unwrap(), EnumNoFields::Foo);

assert_eq!("Bar".parse::<EnumNoFields>().unwrap(), EnumNoFields::Bar);
assert_eq!("bar".parse::<EnumNoFields>().unwrap(), EnumNoFields::Bar);

assert_eq!("Baz".parse::<EnumNoFields>().unwrap(), EnumNoFields::Baz);
assert_eq!("BaZ".parse::<EnumNoFields>().unwrap(), EnumNoFields::BaZ);
assert_eq!(
    "other".parse::<EnumNoFields>().unwrap_err().to_string(),
    "Invalid `EnumNoFields` string representation",
);
```

Code like this is generated:
```rust
# use core::str::FromStr;
#
# enum EnumNoFields {
#     Foo,
#     Bar,
#     Baz,
#     BaZ,
# }
#
impl derive_more::core::str::FromStr for EnumNoFields {
    type Err = derive_more::FromStrError;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        Ok(match s.to_lowercase().as_str() {
            "foo" => Self::Foo,
            "bar" => Self::Bar,
            "baz" if s == "Baz" => Self::Baz,
            "baz" if s == "BaZ" => Self::BaZ,
            _ => return Err(derive_more::FromStrError::new("EnumNoFields")),
        })
    }
}
```


### Empty structs

Deriving `FromStr` for structs with no fields is similar to enums,
but involves only case-insensitive matching by now.

Given the following struct:
```rust
# use derive_more::FromStr;
#
#[derive(FromStr, Debug, Eq, PartialEq)]
struct Foo;

assert_eq!("foo".parse::<Foo>().unwrap(), Foo);
assert_eq!("Foo".parse::<Foo>().unwrap(), Foo);
assert_eq!("FOO".parse::<Foo>().unwrap(), Foo);
```

Code like this is generated:
```rust
# use core::str::FromStr;
#
# struct Foo;
#
impl derive_more::core::str::FromStr for Foo {
    type Err = derive_more::FromStrError;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        Ok(match s.to_lowercase().as_str() {
            "foo" => Self,
            _ => return Err(derive_more::FromStrError::new("Foo")),
        })
    }
}
```


### The `rename_all` attribute

To control the concrete string representation of the name verbatim,
the `#[from_str(rename_all = "...")]` attribute can be placed on structs,
enums and variants.

The available casings are:
- `lowercase`
- `UPPERCASE`
- `PascalCase`
- `camelCase`
- `snake_case`
- `SCREAMING_SNAKE_CASE`
- `kebab-case`
- `SCREAMING-KEBAB-CASE`

```rust
# use derive_more::FromStr;
#
#[derive(FromStr, Debug, Eq, PartialEq)]
#[from_str(rename_all = "lowercase")]
enum Enum {
    VariantOne,
    #[from_str(rename_all = "kebab-case")] // overrides the top-level one
    VariantTwo
}

assert_eq!("variantone".parse::<Enum>().unwrap(), Enum::VariantOne);
assert_eq!("variant-two".parse::<Enum>().unwrap(), Enum::VariantTwo);
```

> **NOTE**: Using `#[from_str(rename_all = "...")]` attribute disables
> any case-insensitivity where applied. This is also true for any enum
> variant whose name or string representation is similar to the variant
> being marked:
> ```rust
> # use derive_more::FromStr;
> #
> # #[allow(non_camel_case_types)]
> #[derive(FromStr, Debug, Eq, PartialEq)]
> enum Enum {
>    Foo,  // case-insensitive
>    #[from_str(rename_all = "SCREAMING_SNAKE_CASE")]
>    BaR,  // case-sensitive (marked with attribute)
>    Bar,  // case-sensitive (name is similar to the marked `BaR` variant)
>    Ba_R, // case-sensitive (string representation is similar to the marked `BaR` variant)
> }
> #
> # assert_eq!("Foo".parse::<Enum>().unwrap(), Enum::Foo);
> # assert_eq!("FOO".parse::<Enum>().unwrap(), Enum::Foo);
> # assert_eq!("foo".parse::<Enum>().unwrap(), Enum::Foo);
> #
> # assert_eq!("BA_R".parse::<Enum>().unwrap(), Enum::BaR);
> # assert_eq!("Bar".parse::<Enum>().unwrap(), Enum::Bar);
> # assert_eq!("Ba_R".parse::<Enum>().unwrap(), Enum::Ba_R);
> ```




## Custom error

The `#[from_str(error(<ty>[, <conv>]))]` attribute can be used to convert the `FromStr`' `Err` type
into a custom error type.

If the conversion function is not provided, the custom error type must implement `From<FromStr::Err>`.
The conversion function could be provided in the following forms:
- function (like `CustomError::new`);
- closure (like `|e| CustomError::new(e)`);
- function call (like `CustomError::factory()`).


### Forwarding

Given the following struct:
```rust
# use derive_more::{From, FromStr};
#
#[derive(From)]
struct CustomError(core::num::ParseIntError);

#[derive(FromStr, Debug, Eq, PartialEq)]
#[from_str(error(CustomError))]
struct MyInt(i32);
```
Code like this is generated:
```rust
# use core::str::FromStr;
# use derive_more::From;
#
# #[derive(From)]
# struct CustomError(core::num::ParseIntError);
#
# struct MyInt(i32);
#
impl derive_more::core::str::FromStr for MyInt {
    type Err = CustomError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        FromStr::from_str(s)
            .map(|v| Self(v))
            .map_err(Into::into)
    }
}
```

For the explicitly specified error conversion:
```rust
# use derive_more::FromStr;
#
struct CustomError(core::num::ParseIntError);

impl CustomError {
    fn new(err: core::num::ParseIntError) -> Self {
        Self(err)
    }
}

#[derive(FromStr, Debug, Eq, PartialEq)]
#[from_str(error(CustomError, CustomError::new))]
struct MyInt(i32);
```
Code like this is generated:
```rust
# use core::str::FromStr;
#
# struct CustomError(core::num::ParseIntError);
#
# impl CustomError {
#     fn new(err: core::num::ParseIntError) -> Self {
#         Self(err)
#     }
# }
#
# struct MyInt(i32);
#
impl derive_more::core::str::FromStr for MyInt {
    type Err = CustomError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        FromStr::from_str(s)
            .map(|v| Self(v))
            .map_err(CustomError::new)
    }
}
```

Custom error for a newtype struct with one named field, *e.g*,
```rust
# use derive_more::{From, FromStr};
#
#[derive(From)]
struct CustomError(core::num::ParseIntError);

#[derive(FromStr)]
#[from_str(error(CustomError))]
struct Point1D {
    x: i32,
}
```
works similarly.


### Flat representation

Custom error type is also supported for empty enums and unit structs.

Given the following enum:
```rust
# use derive_more::{From, FromStr};
#
#[derive(From)]
struct CustomError(derive_more::FromStrError);

#[derive(FromStr, Debug, Eq, PartialEq)]
#[from_str(error(CustomError))]
enum EnumNoFields {
    Foo,
    Bar,
    Baz,
}
```
Code like this is generated:
```rust
# use core::str::FromStr;
# use derive_more::From;
#
# #[derive(From)]
# struct CustomError(derive_more::FromStrError);
#
# enum EnumNoFields {
#     Foo,
#     Bar,
#     Baz,
# }
#
impl derive_more::core::str::FromStr for EnumNoFields {
    type Err = CustomError;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        Ok(match s.to_lowercase().as_str() {
            "foo" => Self::Foo,
            "bar" => Self::Bar,
            "baz" => Self::Baz,
            _ => return Err(derive_more::FromStrError::new("EnumNoFields").into()),
        })
    }
}
```

For the explicitly specified error conversion:
```rust
# use derive_more::FromStr;
#
struct CustomError(derive_more::FromStrError);

impl CustomError {
   pub fn new(err: derive_more::FromStrError) -> Self {
       Self(err)
   }
}

#[derive(FromStr, Debug, Eq, PartialEq)]
#[from_str(error(CustomError, CustomError::new))]
enum EnumNoFields {
    Foo,
    Bar,
    Baz,
}
```
Code like this is generated:
```rust
# use core::str::FromStr;
#
# struct CustomError(derive_more::FromStrError);
#
# impl CustomError {
#    pub fn new(err: derive_more::FromStrError) -> Self {
#        Self(err)
#    }
# }
#
# enum EnumNoFields {
#     Foo,
#     Bar,
#     Baz,
# }
#
impl derive_more::core::str::FromStr for EnumNoFields {
    type Err = CustomError;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        Ok(match s.to_lowercase().as_str() {
            "foo" => Self::Foo,
            "bar" => Self::Bar,
            "baz" => Self::Baz,
            _ => return Err(CustomError::new(derive_more::FromStrError::new("EnumNoFields"))),
        })
    }
}
```

Custom error type for unit structs, *e.g*,
```rust
# use derive_more::{From, FromStr};
#
#[derive(From)]
struct CustomError(derive_more::FromStrError);

#[derive(FromStr)]
#[from_str(error(CustomError))]
struct Foo;
```
works similarly.
