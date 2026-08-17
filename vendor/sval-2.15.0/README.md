# `sval`: Streaming, structured values

[![Rust](https://github.com/sval-rs/sval/workflows/sval/badge.svg)](https://github.com/sval-rs/sval/actions)
[![Latest version](https://img.shields.io/crates/v/sval.svg)](https://crates.io/crates/sval)
[![Documentation Latest](https://docs.rs/sval/badge.svg)](https://docs.rs/sval)

## What is it?

`sval` is a serialization-only framework for Rust. It has a simple, but expressive design that can express any Rust data structure, plus some it can't yet. It was originally designed as a niche framework for structured logging, targeting serialization to [JSON](https://www.json.org, [protobuf](https://protobuf.dev), and Rust's [Debug-style formatting](https://doc.rust-lang.org/std/fmt/trait.Debug.html). The project has evolved beyond this point into a fully general and capable framework for introspecting runtime data.

The core of `sval` is the [`Stream`](https://docs.rs/sval/2.15.0/sval/trait.Stream.html) trait. It defines the data model and features of the framework. `sval` makes a few different API design decisions compared to [`serde`](https://serde.rs), the de-facto choice, to better accommodate the needs of Rust diagnostic frameworks:

1. **Simple API.** The [`Stream`](https://docs.rs/sval/2.15.0/sval/trait.Stream.html) trait has only a few required members that all values are forwarded to. This makes it easy to write bespoke handling for specific data types without needing to implement unrelated methods.
2. **`dyn`-friendly.** The [`Stream`](https://docs.rs/sval/2.15.0/sval/trait.Stream.html) trait is internally mutable, so is trivial to make `dyn`-compatible without intermediate boxing, making it possible to use in no-std environments.
3. **Buffer-friendly.** The [`Stream`](https://docs.rs/sval/2.15.0/sval/trait.Stream.html) trait is non-recursive, so values can be buffered as a flat stream of tokens and replayed later.
4. **Borrowing as an optimization.** The [`Stream`](https://docs.rs/sval/2.15.0/sval/trait.Stream.html) trait may accept borrowed text or binary fragments for a specific lifetime `'sval`, but is also required to accept temporary ones too. This makes it possible to optimize away allocations where possible, but still force them if it's required.
5. **Broadly compatible.** `sval` imposes very few constraints of its own, so it can trivially translate implementations of [`serde::Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) into implementations of [`sval::Value`](https://docs.rs/sval/2.15.0/sval/trait.Value.html).

`sval`'s data model takes inspiration from [CBOR](https://cbor.io), specifically:

1. **Small core.** The base data model of `sval` is small. The required members of the [`Stream`](https://docs.rs/sval/2.15.0/sval/trait.Stream.html) trait only includes nulls, booleans, text, 64-bit signed integers, 64-bit floating point numbers, and sequences. All other types, like arbitrary-precision floating point numbers, records, and tuples, are representable in the base model.
2. **Extensible tags.** Users can define _tags_ that extend `sval`'s data model with new semantics. Examples of tags include Rust's `Some` and `None` variants, constant-sized arrays, text that doesn't require JSON escaping, and anything else you might need.

## Getting started

This section is a high-level guided tour of `sval`'s design and API. To get started, add `sval` to your `Cargo.toml`:

```toml
[dependencies.sval]
version = "2.15.0"
features = ["derive"]
```

### Serializing values

As a quick example, here's how you can use `sval` to serialize a runtime value as JSON.

First, add [`sval_json`](https://docs.rs/sval_json/2.15.0/sval_json/index.html) to your `Cargo.toml`:

```toml
[dependencies.sval_json]
version = "2.15.0"
features = ["std"]
```

Next, derive the [`Value`](https://docs.rs/sval/2.15.0/sval/trait.Value.html) trait on the type you want to serialize, including on any other types it uses in its fields:

```rust
#[derive(sval::Value)]
pub struct MyRecord<'a> {
    field_0: i32,
    field_1: bool,
    field_2: &'a str,
}
```

Finally, use [`stream_to_string`](https://docs.rs/sval_json/2.15.0/sval_json/fn.stream_to_string.html) to serialize an instance of your type as JSON:

```rust
let my_record = MyRecord {
    field_0: 1,
    field_1: true,
    field_2: "some text",
};

// Produces:
//
// {"field_0":1,"field_1":true,"field_2":"some text"}
let json: String = sval_json::stream_to_string(my_record)?;
```

### The `Value` trait

The previous example didn't reveal a lot of detail about how `sval` works, only that there's a [`Value`](https://docs.rs/sval/2.15.0/sval/trait.Value.html) trait involved, and it somehow allows us to convert an instance of the `MyRecord` struct into a JSON object. Using [`cargo expand`](https://github.com/dtolnay/cargo-expand), we can peek behind the covers and see what the `Value` trait does. The previous example expands to something like this:

```rust
impl<'a> sval::Value for MyRecord<'a> {
    fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> sval::Result {
        let type_label = Some(&sval::Label::new("MyRecord").with_tag(&sval::tags::VALUE_IDENT));
        let type_index = None;

        stream.record_tuple_begin(None, type_label, type_index, Some(3))?;

        let mut field_index = 0;

        // field_0
        {
            field_index += 1;

            let field_index = &sval::Index::from(field_index - 1).with_tag(&sval::tags::VALUE_OFFSET);
            let field_label = &sval::Label::new("field_0").with_tag(&sval::tags::VALUE_IDENT);

            stream.record_tuple_value_begin(None, field_label, field_index)?;
            stream.value(&self.field_0)?;
            stream.record_tuple_value_end(None, field_label, field_index)?;
        }

        // field_1
        {
            field_index += 1;

            let field_index = &sval::Index::from(field_index - 1).with_tag(&sval::tags::VALUE_OFFSET);
            let field_label = &sval::Label::new("field_1").with_tag(&sval::tags::VALUE_IDENT);

            stream.record_tuple_value_begin(None, field_label, field_index)?;
            stream.value(&self.field_1)?;
            stream.record_tuple_value_end(None, field_label, field_index)?;
        }

        // field_2
        {
            field_index += 1;

            let field_index = &sval::Index::from(field_index - 1).with_tag(&sval::tags::VALUE_OFFSET);
            let field_label = &sval::Label::new("field_2").with_tag(&sval::tags::VALUE_IDENT);
            
            stream.record_tuple_value_begin(None, field_label, field_index)?;
            stream.value(&self.field_2)?;
            stream.record_tuple_value_end(None, field_label, field_index)?;
        }

        stream.record_tuple_end(None, type_label, type_index)
    }
}
```

The [`Value`](https://docs.rs/sval/2.15.0/sval/trait.Value.html) trait has a single required method, `stream`, which is responsible for driving an instance of a [`Stream`](https://docs.rs/sval/2.15.0/sval/trait.Stream.html) with its fields. The [`Stream`](https://docs.rs/sval/2.15.0/sval/trait.Stream.html) trait defines `sval`'s data model and the mechanics of how data is described in it. In this example, the `MyRecord` struct is represented as a _record tuple_, a type that can be either a _record_ with fields named by a [`Label`](https://docs.rs/sval/2.15.0/sval/struct.Label.html), or a _tuple_ with fields indexed by an [`Index`](https://docs.rs/sval/2.15.0/sval/struct.Index.html). Labels and indexes can be annotated with a [`Tag`](https://docs.rs/sval/2.15.0/sval/struct.Tag.html) which add user-defined semantics to them. In this case, the labels carry the [`VALUE_IDENT`](https://docs.rs/sval/2.15.0/sval/tags/constant.VALUE_IDENT.html) tag meaning they're valid Rust identifiers, and the indexes carry the [`VALUE_OFFSET`](https://docs.rs/sval/2.15.0/sval/tags/constant.VALUE_OFFSET.html) tag meanings they're zero-indexed field offsets. The specific type of [`Stream`](https://docs.rs/sval/2.15.0/sval/trait.Stream.html) can decide whether to treat the `MyRecord` type as either a record (in the case of JSON) or a tuple (in the case of protobuf), and whether it understands that tags it sees or not.

### The `Stream` trait

Something to notice about the [`Stream`](https://docs.rs/sval/2.15.0/sval/trait.Stream.html) API in the expanded `MyRecord` example is that it is _flat_. The call to `record_tuple_begin` doesn't return a new type like `serde`'s `struct_begin` does. The implementor of [`Value`](https://docs.rs/sval/2.15.0/sval/trait.Value.html) is responsible for issuing the correct sequence of [`Stream`](https://docs.rs/sval/2.15.0/sval/trait.Stream.html) calls as it works through its structure. The [`Stream`](https://docs.rs/sval/2.15.0/sval/trait.Stream.html) can then rely on markers like `record_tuple_value_begin` and `record_tuple_value_end` to know what position within a value it is without needing to track that state itself. The flat API makes dyn-compatibility and buffering simpler, but makes implementing non-trivial streams more difficult, because you can't rely on recursive to manage state.

Recall the way `MyRecord` was converted into JSON earlier:

```rust
let json: String = sval_json::stream_to_string(my_record)?;
```

Internally, [`stream_to_string`](https://docs.rs/sval_json/2.15.0/sval_json/fn.stream_to_string.html) uses an instance of [`Stream`](https://docs.rs/sval/2.15.0/sval/trait.Stream.html) that writes JSON tokens for each piece of the value it encounters. For example, `record_tuple_begin` and `record_tuple_end` will emit the corresponding `{` `}` characters for a JSON object.

`sval`'s data model is _layered_. The required methods on [`Stream`](https://docs.rs/sval/2.15.0/sval/trait.Stream.html) represent the base data model that more expressive constructs map down to. Here's what a minimal [`Stream`](https://docs.rs/sval/2.15.0/sval/trait.Stream.html) that just formats values in `sval`'s base data model looks like:

```rust
pub struct MyStream;

impl<'sval> sval::Stream<'sval> for MyStream {
    fn null(&mut self) -> sval::Result {
        print!("null");
        Ok(())
    }

    fn bool(&mut self, v: bool) -> sval::Result {
        print!("{}", v);
        Ok(())
    }

    fn i64(&mut self, v: i64) -> sval::Result {
        print!("{}", v);
        Ok(())
    }

    fn text_begin(&mut self, _: Option<usize>) -> sval::Result {
        print!("\"");
        Ok(())
    }

    fn text_fragment_computed(&mut self, fragment: &str) -> sval::Result {
        print!("{}", fragment.escape_debug());

        Ok(())
    }

    fn text_end(&mut self) -> sval::Result {
        print!("\"");
        Ok(())
    }

    fn seq_begin(&mut self, _: Option<usize>) -> sval::Result {
        print!("[");
        Ok(())
    }

    fn seq_value_begin(&mut self) -> sval::Result {
        Ok(())
    }

    fn seq_value_end(&mut self) -> sval::Result {
        print!(",");
        Ok(())
    }

    fn seq_end(&mut self) -> sval::Result {
        print!("]");
        Ok(())
    }
}

let my_record = MyRecord {
    field_0: 1,
    field_1: true,
    field_2: "some text",
};

// Prints:
//
// [["field_0",1,],["field_1",true,],["field_2","some text",],],
sval::stream(&mut MyStream, my_record);
```

Recall that the `MyRecord` struct mapped to a `record_tuple` in `sval`'s data model. `record_tuple`s in turn are represented in the base data model as a sequence of 2-dimensional sequences where the first element is the field label and the second is its value.

[`Stream`](https://docs.rs/sval/2.15.0/sval/trait.Stream.html)s aren't limited to just serializing data into interchange formats. They can manipulate or interrogate a value any way it likes. Here's an example of a [`Stream`](https://docs.rs/sval/2.15.0/sval/trait.Stream.html) that attempts to extract a specific field of a value as an `i32`:

```rust
pub fn get_i32<'sval>(field: &str, value: impl sval::Value) -> Option<i32> {
    struct Extract<'a> {
        depth: usize,
        field: &'a str,
        matched_field: bool,
        extracted: Option<i32>,
    }

    impl<'a, 'sval> sval::Stream<'sval> for Extract<'a> {
        // Rust structs that derive `Value`, like `MyRecord` from earlier, are records in `sval`'s data model.
        // Each field of the record starts with a call to `record_value_begin` with its name.
        fn record_value_begin(&mut self, _: Option<&sval::Tag>, label: &sval::Label) -> sval::Result {
            self.matched_field = label.as_str() == self.field;
            Ok(())
        }

        fn record_value_end(&mut self, _: Option<&sval::Tag>, _: &sval::Label) -> sval::Result {
            Ok(())
        }

        // We're looking for an `i32`, so will attempt to cast an integer we find.
        // `sval` will forward any convertible integer to `i64` by default.
        // We could also override the `i32` method here.
        fn i64(&mut self, v: i64) -> sval::Result {
            if self.matched_field {
                self.extracted = v.try_into().ok();
            }

            Ok(())
        }

        // `sval` will forward all complex types as sequences by default.
        // We're only interested in top-level fields of records here, so whenever
        // we encounter a sequence we increment/decrement our depth to tell how
        // deeply nested we are.
        fn seq_begin(&mut self, _: Option<usize>) -> sval::Result {
            self.depth += 1;
            Ok(())
        }
    
        fn seq_value_begin(&mut self) -> sval::Result {
            Ok(())
        }
    
        fn seq_value_end(&mut self) -> sval::Result {
            Ok(())
        }
    
        fn seq_end(&mut self) -> sval::Result {
            self.depth -= 1;
            Ok(())
        }

        // These other methods are required members of `Stream`.
        // We're not interested in them in this example.
        fn null(&mut self) -> sval::Result {
            Ok(())
        }
    
        fn bool(&mut self, _: bool) -> sval::Result {
            Ok(())
        }
    
        fn text_begin(&mut self, _: Option<usize>) -> sval::Result {
            Ok(())
        }
    
        fn text_fragment_computed(&mut self, _: &str) -> sval::Result {
            Ok(())
        }
    
        fn text_end(&mut self) -> sval::Result {
            Ok(())
        }
    }

    let mut stream = Extract {
        depth: 0,
        field,
        matched_field: false,
        extracted: None,
    };

    sval::stream(&mut stream, &value).ok()?;
    stream.extracted
}

let my_record = MyRecord {
    field_0: 1,
    field_1: true,
    field_2: "some text",
};

assert_eq!(Some(1), get_i32("field_0", &my_record));
```

### Streaming text

Strings in `sval` don't need to be streamed in a single call. As an example, say we have a template type like this:

```rust
enum Part<'a> {
    Literal(&'a str),
    Property(&'a str),
}

pub struct Template<'a>(&'a [Part<'a>]);
```

If we wanted to serialize `Template` to a string, we could implement [`Value`](https://docs.rs/sval/2.15.0/sval/trait.Value.html), handling each literal and property as a separate fragment:

```rust
impl<'a> sval::Value for Template<'a> {
    fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> sval::Result {
        stream.text_begin(None)?;

        for part in self.0 {
            match part {
                Part::Literal(lit) => stream.text_fragment(lit)?,
                Part::Property(prop) => {
                    stream.text_fragment("{")?;
                    stream.text_fragment(prop)?;
                    stream.text_fragment("}")?;
                }
            }
        }

        stream.text_end()
    }
}
```

When streamed as JSON, `Template` would produce something like this:

```rust
let template = Template(&[
    Part::Literal("some literal text and "),
    Part::Property("x"),
    Part::Literal(" and more literal text"),
]);

// Produces:
//
// "some literal text and {x} and more literal text"
let json = sval_json::stream_to_string(template)?;
```

### Borrowed data

The [`Stream`](https://docs.rs/sval/2.15.0/sval/trait.Stream.html) trait carries a `'sval` lifetime it can use to accept borrowed text and binary values. Borrowing in `sval` is an optimization. Even if a [`Stream`](https://docs.rs/sval/2.15.0/sval/trait.Stream.html) uses a concrete `'sval` lifetime, it still needs to handle computed values. Here's an example of a [`Stream`](https://docs.rs/sval/2.15.0/sval/trait.Stream.html) that attempts to extract a borrowed string from a value by making use of the `'sval` lifetime:

```rust
pub fn to_text(value: &(impl Value + ?Sized)) -> Option<&str> {
    struct Extract<'sval> {
        extracted: Option<&'sval str>,
        seen_fragment: bool,
    }

    impl<'sval> Stream<'sval> for Extract<'sval> {
        fn text_begin(&mut self, _: Option<usize>) -> Result {
            Ok(())
        }

        // `text_fragment` accepts a string borrowed for `'sval`.
        //
        // Implementations of `Value` will send borrowed data if they can.
        fn text_fragment(&mut self, fragment: &'sval str) -> Result {
            // Allow either independent strings, or fragments of a single borrowed string.
            if !self.seen_fragment {
                self.extracted = Some(fragment);
                self.seen_fragment = true;
            } else {
                self.extracted = None;
            }

            Ok(())
        }

        // `text_fragment_computed` accepts a string for an arbitrarily short lifetime.
        //
        // The fragment can't be borrowed outside of the function call, so would need to
        // be buffered.
        fn text_fragment_computed(&mut self, _: &str) -> Result {
            self.extracted = None;
            self.seen_fragment = true;

            sval::error()
        }

        fn text_end(&mut self) -> Result {
            Ok(())
        }

        fn null(&mut self) -> Result {
            sval::error()
        }

        fn bool(&mut self, _: bool) -> Result {
            sval::error()
        }

        fn i64(&mut self, _: i64) -> Result {
            sval::error()
        }

        fn seq_begin(&mut self, _: Option<usize>) -> Result {
            sval::error()
        }

        fn seq_value_begin(&mut self) -> Result {
            sval::error()
        }

        fn seq_value_end(&mut self) -> Result {
            sval::error()
        }

        fn seq_end(&mut self) -> Result {
            sval::error()
        }
    }

    let mut extract = Extract {
        extracted: None,
        seen_fragment: false,
    };

    value.stream(&mut extract).ok()?;
    extract.extracted
}
```

Implementations of [`Value`](https://docs.rs/sval/2.15.0/sval/trait.Value.html) should provide a [`Stream`](https://docs.rs/sval/2.15.0/sval/trait.Stream.html) with borrowed data where possible, and only compute it if it needs to.

### Error handling

`sval`'s [`Error`](https://docs.rs/sval/2.15.0/sval/struct.Error.html) type doesn't carry any state of its own. It only signals early termination of the [`Stream`](https://docs.rs/sval/2.15.0/sval/trait.Stream.html) which may be because its job is done, or because it failed. It's up to the [`Stream`](https://docs.rs/sval/2.15.0/sval/trait.Stream.html) to carry whatever state it needs to provide meaningful errors.

## Data model

This section descibes `sval`'s data model in detail using examples in Rust syntax. Some types in `sval`'s model aren't representable in Rust yet, so they use pseudo syntax.

### Base model

#### Nulls

```rust
null
```

```rust
stream.null()?;
```

#### Booleans

```rust
bool
```

```rust
stream.bool(true)?;
```

#### 64bit signed integers

```rust
i64
```

```rust
stream.i64(-1)?;
```

#### Text

```rust
[str]
```

```rust
stream.text_begin(None)?;

stream.text_fragment("Hello, ")?;
stream.text_fragment("World")?;

stream.text_end()?;
```

Note that `sval` text is an array of strings.

#### Sequences

```rust
[dyn T]
```

```rust
stream.seq_begin(None)?;

stream.seq_value_begin()?;
stream.i64(-1)?;
stream.seq_value_end()?;

stream.seq_value_begin()?;
stream.bool(true)?;
stream.seq_value_end()?;

stream.seq_end()?;
```

Note that Rust arrays are homogeneous, but `sval` sequences are heterogeneous.

### Extended model

#### 8bit unsigned integers

```rust
u8
```

```rust
stream.u8(1)?;
```

8bit unsigned integers reduce to 64bit signed integers in the base model.

#### 16bit unsigned integers

```rust
u16
```

```rust
stream.u16(1)?;
```

16bit unsigned integers reduce to 64bit signed integers in the base model.

#### 32bit unsigned integers

```rust
u32
```

```rust
stream.u32(1)?;
```

32bit unsigned integers reduce to 64bit signed integers in the base model.

#### 64bit unsigned integers

```rust
u64
```

```rust
stream.u64(1)?;
```

64bit unsigned integers reduce to 64bit signed integers in the base data model if they fit, or base10 ASCII text if they don't.

#### 128bit unsigned integers

```rust
u128
```

```rust
stream.u128(1)?;
```

128bit unsigned integers reduce to 64bit signed integers in the base data model if they fit, or base10 ASCII text if they don't.

#### 8bit signed integers

```rust
i8
```

```rust
stream.i8(1)?;
```

8bit signed integers reduce to 64bit signed integers in the base model.

#### 16bit signed integers

```rust
i16
```

```rust
stream.i16(1)?;
```

16bit signed integers reduce to 64bit signed integers in the base model.

#### 32bit signed integers

```rust
i32
```

```rust
stream.i32(1)?;
```

32bit signed integers reduce to 64bit signed integers in the base model.

#### 128bit signed integers

```rust
i128
```

```rust
stream.i128(1)?;
```

128bit signed integers reduce to 64bit signed integers in the base data model if they fit, or base10 ASCII text if they don't.

#### 32bit binary floating point numbers

```rust
f32
```

```rust
stream.f32(1)?;
```

32bit binary floating point numbers reduce to base10 ASCII text in the base model.

#### 64bit binary floating point numbers

```rust
f64
```

```rust
stream.f64(1)?;
```

64bit binary floating point numbers reduce to base10 ASCII text in the base model.

#### Binary

```rust
[[u8]]
```

```rust
stream.binary_begin(None)?;

stream.binary_fragment(b"Hello, ")?;
stream.binary_fragment(b"World")?;

stream.binary_end()?;
```

Binary values reduce to sequences of numbers in the base model.

#### Maps

```rust
[(dyn K, dyn V)]
```

```rust
stream.map_begin(None)?;

stream.map_key_begin()?;
stream.i64(0)?;
stream.map_key_end()?;

stream.map_value_begin()?;
stream.bool(false)?;
stream.map_value_end()?;

stream.map_key_begin()?;
stream.i64(1)?;
stream.map_key_end()?;

stream.map_value_begin()?;
stream.bool(true)?;
stream.map_value_end()?;

stream.map_end()?;
```

Note that most Rust maps are homogeneous, but `sval` maps are heterogeneous.

Maps reduce to a sequence of 2D sequences in the base model.

#### Tags

```rust
struct Tag
```

```rust
stream.tag(None, Some(&sval::Label::new("Tag")), None)?;
```

Tags reduce to null in the base model.

#### Tagged values

```rust
struct Tagged(i64);
```

```rust
stream.tagged_begin(None, Some(&sval::Label::new("Tagged")), None)?;
stream.i64(1)?;
stream.tagged_end(None, Some(&sval::Label::new("Tagged")), None)?;
```

Tagged values reduce to their wrapped value in the base model.

#### Tuples

```rust
struct Tuple(i64, bool)
```

```rust
stream.tuple_begin(None, Some(&sval::Label::new("Tuple")), None, None)?;

stream.tuple_value_begin(None, &sval::Index::new(0))?;
stream.i64(1)?;
stream.tuple_value_end(None, &sval::Index::new(0))?;

stream.tuple_value_begin(None, &sval::Index::new(1))?;
stream.bool(true)?;
stream.tuple_value_end(None, &sval::Index::new(1))?;

stream.tuple_end(None, Some(&sval::Label::new("Tuple")), None)?;
```

`sval` tuples may also be unnamed:

```rust
(i64, bool)
```

```rust
stream.tuple_begin(None, None, None, None)?;

stream.tuple_value_begin(None, &sval::Index::new(0))?;
stream.i64(1)?;
stream.tuple_value_end(None, &sval::Index::new(0))?;

stream.tuple_value_begin(None, &sval::Index::new(1))?;
stream.bool(true)?;
stream.tuple_value_end(None, &sval::Index::new(1))?;

stream.tuple_end(None, None, None)?;
```

Tuples reduce to sequences in the base model.

#### Records

```rust
struct Record { a: i64, b: bool }
```

```rust
stream.record_begin(None, Some(&sval::Label::new("Record")), None, None)?;

stream.record_value_begin(None, &sval::Label::new("a"))?;
stream.i64(1)?;
stream.record_value_end(None, &sval::Label::new("a"))?;

stream.record_value_begin(None, &sval::Label::new("b"))?;
stream.bool(true)?;
stream.record_value_end(None, &sval::Label::new("b"))?;

stream.record_end(None, Some(&sval::Label::new("Record")), None)?;
```

`sval` records may also be unnamed:

```rust
{ a: i64, b: bool }
```

```rust
stream.record_begin(None, None, None, None)?;

stream.record_value_begin(None, &sval::Label::new("a"))?;
stream.i64(1)?;
stream.record_value_end(None, &sval::Label::new("a"))?;

stream.record_value_begin(None, &sval::Label::new("b"))?;
stream.bool(true)?;
stream.record_value_end(None, &sval::Label::new("b"))?;

stream.record_end(None, None, None)?;
```

Records reduce to a sequence of 2D sequences in the base model.

#### Enums

`sval` enums wrap a variant, which may be any of the following types:

- Tags
- Tagged values
- Records
- Tuples
- Enums

```rust
Enum::Tag
```

```rust
stream.enum_begin(None, Some(&sval::Label::new("Enum")), None)?;

stream.tag(None, Some(&sval::Label::new("Tag")), Some(&sval::Index::new(0)))?;

stream.enum_end(None, Some(&sval::Label::new("Enum")), None)?;
```

```rust
Enum::Tagged(i64)
```

```rust
stream.enum_begin(None, Some(&sval::Label::new("Enum")), None)?;

stream.tagged_begin(None, Some(&sval::Label::new("Tagged")), Some(&sval::Index::new(1)))?;
stream.i64(1)?;
stream.tagged_end(None, Some(&sval::Label::new("Tagged")), Some(&sval::Index::new(1)))?;

stream.enum_end(None, Some(&sval::Label::new("Enum")), None)?;
```

```rust
Enum::Tuple(i64, bool)
```

```rust
stream.enum_begin(None, Some(&sval::Label::new("Enum")), None)?;

stream.tuple_begin(None, Some(&sval::Label::new("Tuple")), Some(&sval::Index::new(2)), None)?;

stream.tuple_value_begin(None, &sval::Index::new(0))?;
stream.i64(1)?;
stream.tuple_value_end(None, &sval::Index::new(0))?;

stream.tuple_value_begin(None, &sval::Index::new(1))?;
stream.bool(true)?;
stream.tuple_value_end(None, &sval::Index::new(1))?;

stream.tuple_end(None, Some(&sval::Label::new("Tuple")), Some(&sval::Index::new(2)))?;

stream.enum_end(None, Some(&sval::Label::new("Enum")), None)?;
```

```rust
Enum::Record { a: i64, b: bool }
```

```rust
stream.enum_begin(None, Some(&sval::Label::new("Enum")), None)?;

stream.record_begin(None, Some(&sval::Label::new("Record")), Some(&sval::Index::new(3)), None)?;

stream.record_value_begin(None, &sval::Label::new("a"))?;
stream.i64(1)?;
stream.record_value_end(None, &sval::Label::new("a"))?;

stream.record_value_begin(None, &sval::Label::new("b"))?;
stream.bool(true)?;
stream.record_value_end(None, &sval::Label::new("b"))?;

stream.record_end(None, Some(&sval::Label::new("Record")), Some(&sval::Index::new(3)))?;

stream.enum_end(None, Some(&sval::Label::new("Enum")), None)?;
```

`sval` enum variants may also be unnamed:

```rust
Enum::<i32>
```

```rust
stream.enum_begin(None, Some(&sval::Label::new("Enum")), None)?;

stream.tagged_begin(None, None, None)?;
stream.i64(1)?;
stream.tagged_end(None, None, None)?;

stream.enum_end(None, Some(&sval::Label::new("Enum")), None)?;
```

```rust
Enum::<(i64, bool)>
```

```rust
stream.enum_begin(None, Some(&sval::Label::new("Enum")), None)?;

stream.tuple_begin(None, None, None, None)?;

stream.tuple_value_begin(None, &sval::Index::new(0))?;
stream.i64(1)?;
stream.tuple_value_end(None, &sval::Index::new(0))?;

stream.tuple_value_begin(None, &sval::Index::new(1))?;
stream.bool(true)?;
stream.tuple_value_end(None, &sval::Index::new(1))?;

stream.tuple_end(None, None, None)?;

stream.enum_end(None, Some(&sval::Label::new("Enum")), None)?;
```

```rust
Enum::<{ a: i64, b: bool }>
```

```rust
stream.enum_begin(None, Some(&sval::Label::new("Enum")), None)?;

stream.record_begin(None, None, None, None)?;

stream.record_value_begin(None, &sval::Label::new("a"))?;
stream.i64(1)?;
stream.record_value_end(None, &sval::Label::new("a"))?;

stream.record_value_begin(None, &sval::Label::new("b"))?;
stream.bool(true)?;
stream.record_value_end(None, &sval::Label::new("b"))?;

stream.record_end(None, None, None)?;

stream.enum_end(None, Some(&sval::Label::new("Enum")), None)?;
```

`sval` enum variants may be other enums:

```rust
Enum::Inner::Tagged(i64)
```

```rust
stream.enum_begin(None, Some(&sval::Label::new("Enum")), None)?;

stream.enum_begin(None, Some(&sval::Label::new("Inner")), Some(&sval::Index::new(0)))?;

stream.tagged_begin(None, Some(&sval::Label::new("Tagged")), Some(&sval::Index::new(1)))?;
stream.i64(1)?;
stream.tagged_end(None, Some(&sval::Label::new("Tagged")), Some(&sval::Index::new(1)))?;

stream.enum_end(None, Some(&sval::Label::new("Inner")), Some(&sval::Index::new(0)))?;

stream.enum_end(None, Some(&sval::Label::new("Enum")), None)?;
```

#### User-defined tags

`sval` tags, tagged values, records, tuples, enums, and their values can carry a user-defined [`Tag`](https://docs.rs/sval/2.15.0/sval/struct.Tag.html) that alters their semantics. A [`Stream`](https://docs.rs/sval/2.15.0/sval/trait.Stream.html) may understand a [`Tag`](https://docs.rs/sval/2.15.0/sval/struct.Tag.html) and treat its annotated value differently, or it may ignore them. An example of a [`Tag`](https://docs.rs/sval/2.15.0/sval/struct.Tag.html) is [`NUMBER`](https://docs.rs/sval/2.15.0/sval/tags/constant.NUMBER.html), which is for text that encodes an arbitrary-precision decimal floating point number with a standardized format. A [`Stream`](https://docs.rs/sval/2.15.0/sval/trait.Stream.html) may parse these numbers and encode them differently to regular text.

Here's an example of a user-defined [`Tag`](https://docs.rs/sval/2.15.0/sval/struct.Tag.html) for treating integers as Unix timestamps, and a [`Stream`](https://docs.rs/sval/2.15.0/sval/trait.Stream.html) that understands them:

```rust
// Define a tag as a constant.
//
// Tags are expected to have unique names.
//
// The rules of our tag are that 64bit unsigned integers that carry it are seconds since
// the Unix epoch.
pub const UNIX_TIMESTAMP: sval::Tag = sval::Tag::new("unixts");

// Derive `Value` on a type, annotating it with our tag.
//
// We could also implement `Value` manually using `stream.tagged_begin(Some(&UNIX_TIMESTAMP), ..)`.
#[derive(Value)]
#[sval(tag = "UNIX_TIMESTAMP")]
pub struct Timestamp(u64);

// Here's an example of a `Stream` that understands our tag.
pub struct MyStream {
    is_unix_ts: bool,
}

impl<'sval> sval::Stream<'sval> for MyStream {
    fn tagged_begin(
        &mut self,
        tag: Option<&sval::Tag>,
        _: Option<&sval::Label>,
        _: Option<&sval::Index>,
    ) -> sval::Result {
        // When beginning a tagged value, check to see if it's a tag we understand.
        if let Some(&UNIX_TIMESTAMP) = tag {
            self.is_unix_ts = true;
        }

        Ok(())
    }

    fn tagged_end(
        &mut self,
        tag: Option<&sval::Tag>,
        _: Option<&sval::Label>,
        _: Option<&sval::Index>,
    ) -> sval::Result {
        if let Some(&UNIX_TIMESTAMP) = tag {
            self.is_unix_ts = false;
        }

        Ok(())
    }

    fn u64(&mut self, v: u64) -> sval::Result {
        // If the value is tagged as a Unix timestamp then print it using a human-readable RFC3339 format.
        if self.is_unix_ts {
            print!(
                "{}",
                humantime::format_rfc3339(
                    std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(v)
                )
            );
        }

        Ok(())
    }

    fn null(&mut self) -> sval::Result {
        Ok(())
    }

    fn bool(&mut self, _: bool) -> sval::Result {
        Ok(())
    }

    fn i64(&mut self, _: i64) -> sval::Result {

        Ok(())
    }

    fn text_begin(&mut self, _: Option<usize>) -> sval::Result {
        Ok(())
    }

    fn text_fragment_computed(&mut self, fragment: &str) -> sval::Result {
        Ok(())
    }

    fn text_end(&mut self) -> sval::Result {
        Ok(())
    }

    fn seq_begin(&mut self, _: Option<usize>) -> sval::Result {
        Ok(())
    }

    fn seq_value_begin(&mut self) -> sval::Result {
    }

    fn seq_value_end(&mut self) -> sval::Result {
        Ok(())
    }

    fn seq_end(&mut self) -> sval::Result {
        Ok(())
    }
}
```

The [`Label`](https://docs.rs/sval/2.15.0/sval/struct.Label.html) and [`Index`](https://docs.rs/sval/2.15.0/sval/struct.Index.html) types can also carry a [`Tag`](https://docs.rs/sval/2.15.0/sval/struct.Tag.html). An example of a [`Tag`](https://docs.rs/sval/2.15.0/sval/struct.Tag.html) you might use on a [`Label`](https://docs.rs/sval/2.15.0/sval/struct.Label.html) is [`VALUE_IDENT`](https://docs.rs/sval/2.15.0/sval/tags/constant.VALUE_IDENT.html), for labels that hold a valid Rust identifier.

### Type system

`sval` has an implicit structural type system based on the sequence of calls a [`Stream`](https://docs.rs/sval/2.15.0/sval/trait.Stream.html) receives, and the values of any [`Label`](https://docs.rs/sval/2.15.0/sval/struct.Label.html), [`Index`](https://docs.rs/sval/2.15.0/sval/struct.Index.html), or [`Tag`](https://docs.rs/sval/2.15.0/sval/struct.Tag.html) on them, with the following exceptions:

- Text type does not depend on the composition of fragments, or on their length.
- Binary type does not depend on the composition of fragments, or on their length.
- Sequences are untyped. Their type doesn't depend on the types of their elements, or on their length.
- Maps are untyped. Their type doesn't depend on the types of their keys or values, or on their length.
- Enums holding differently typed variants have the same type.

These rules may be better formalized in the future.

## Ecosystem

`sval` is a general framework with specific serialization formats and utilities provided as external libraries:

- [`sval_fmt`](https://docs.rs/sval_json/2.15.0/sval_fmt/index.html): Colorized Rust-style debug formatting.
- [`sval_json`](https://docs.rs/sval_json/2.15.0/sval_json/index.html): Serialize values as JSON in a `serde`-compatible format.
- [`sval_protobuf`](https://docs.rs/sval_protobuf/latest/sval_protobuf/): Serialize values as protobuf messages.
- [`sval_serde`](https://docs.rs/sval_json/2.15.0/sval_serde/index.html): Convert between `serde` and `sval`.
- [`sval_buffer`](https://docs.rs/sval_json/2.15.0/sval_buffer/index.html): Losslessly buffers any [`Value`](https://docs.rs/sval/2.15.0/sval/trait.Value.html) into an owned, thread-safe variant.
- [`sval_flatten`](https://docs.rs/sval_json/2.15.0/sval_flatten/index.html): Flatten the fields of a value onto its parent, like `#[serde(flatten)]`.
- [`sval_nested`](https://docs.rs/sval_json/2.15.0/sval_nested/index.html): Buffer `sval`'s flat [`Stream`](https://docs.rs/sval/2.15.0/sval/trait.Stream.html) API into a recursive one like `serde`'s. For types that `#[derive(Value)]`, the translation is non-allocating.
- [`sval_ref`](https://docs.rs/sval_json/2.15.0/sval_ref/index.html): A variant of [`Value`](https://docs.rs/sval/2.15.0/sval/trait.Value.html) for types that are internally borrowed (like `MyType<'a>`) instead of externally (like `&'a MyType`).
