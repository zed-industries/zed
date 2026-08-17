# json_dotpath

Access members of nested JSON arrays and objects using "dotted paths".

## Changes

### 1.1.0

Added `dot_has()` and `dot_has_checked()`

### 1.0.3

Replaced `failure` with `thiserror`, implement `std::error::Error` for the error type.

### 1.0.0

The API changed to return `Result<Option<T>>` instead of panicking internally on error.
The library is now much safer to use.
 
Further, all logic has been adjusted to be more robust and consistent.

Array append and prepend operators now use `<<` and `>>` instead of overloading `<` and `>`,
which now work the same way in all array accesses (getting the first and last element). 

## Dotted path

Dotted path represents a path from the root of a JSON object to one of its nodes.
Such path is represented by joining the object and array keys by dots:

Consider this example JSON:

```json
{
  "fruit": [
    {"name": "lemon", "color":  "yellow"},
    {"name": "apple", "color":  "green"},
    {"name": "cherry", "color":  "red"}
  ]
}
```

The following paths represent its parts:

- `""` ... the whole object
- `"fruit"` ... the fruits array
- `"fruit.0"` ... the first fruit object, `{"name": "lemon", "color":  "yellow"}`
- `"fruit.1.name"` ... the second (index is 0-based) fruit's name, `"apple"`

Special patterns may be used for object manipulation as well (see below).

## Object operations

Five principal methods are added by the `DotPaths` trait to `serde_json::Value`, 
`Vec<Value>` and `serde_json::Map<String, Value>` (the inner structs of `Value::Object` and `Value::Array`).

- `dot_get(path)` - get a value by path
- `dot_get_mut(path)` - get a mutable reference to an element of the JSON object
- `dot_set(path, value)` - set a new value, dropping the original (if any)
- `dot_replace(path, value)` - set a new value, returning the original (if any)
- `dot_take(path)` - remove a value by path, returning it (if any)

`dot_set()` supports array manipulation syntax not found in the other methods, namely the 
`>n` and `<n` pattern to insert an element before or after an index, shifting the rest of the `Vec`.

### Additional convenience methods

- `dot_remove(path)` - remove a value by path
- `dot_get_or(path, def)` - get value, or a custom default
- `dot_get_or_default(path)` - get value, or `Default::default()`
- `dot_has_checked(path)` - checks if a path is valid and a value exists there
- `dot_has(path)` - the same as above, but errors silently become `false`

All methods are generic and take care of serializing and deserializing the stored / retrieved
data. `dot_get_mut()` is an exception and returns `&mut Value`.

`dot_set()` and `dot_remove()` do not deserialize the original object, which makes them more
efficient than `dot_replace()` and `dot_take()` when the original value is not needed.

### Error reporting

All methods return `Result<json_dotpath::Error, T>` (aliased to `json_dotpath::Result<T>`), 
either as `json_dotpath::Result<()>`, or `json_dotpath::Result<Option<T>>` when a value is expected.

These results should be handled carefully, as they report structural errors (meaning the requested operation
could not be performed), or the path given was invalid.

### Dynamic object building

When a path that does not exist but could (e.g. an appended array element, a new object key), and one of the assignment
methods or `dot_get_mut()` are used, this element will be created automatically, including its parent elements as needed.

This is well illustrated in one of the unit tests:

```rust
let mut obj = Value::Null;
let _ = obj.dot_get_mut("foo.0").unwrap(); // get mut, here only for side effects
assert_eq!(json!({"foo": [null]}), obj);
```
 
Null can flexibly become an array or object in such situations (see "Special handling of Null" below).

## Dotted Path Syntax

Path is simply a sequence of path segment joined by periods (`.`).

Some symbols are ascribed special meaning by the library, depending on the method they're used in.
All symbols (including `.`) may be escaped using a backslash if their literal value is needed as part of the path.

### Array Patterns

Array keys must be numeric (integer), or one of the special patterns listed below.

- `<` ... first element
- `>` ... last element
- `-` or `<<` ... prepend
- `+` or `>>` ... append
- `<n`, e.g. `<5` ... insert before the n-th element
- `>n`, e.g. `>5` ... insert after the n-th element

It's possible to create nested arrays or objects by setting a non-existent path,
provided the key syntax rules are maintained. 

See unit tests for more examples.

### Special handling of Null

JSON null can transparently become an array or object by setting it's members (even nested), 
as if it was an empty array or object. Whether it should become an array or object depends on the key used to index into it.

- numeric key turns null into an array (only `0` and the special array operators are allowed, 
  as other numbers are out of range for an empty array)
- any other key turns it into a map
- any key starting with an escape creates a map as well (e.g. `\0.aaa` turns `null` into `{"0": {"aaa": …} }` )

JSON null is considered an empty value and is transformed into `Ok(None)` when retrieved, 
as it can not be deserialized.

Setting a value to `Value::Null` works as expected and places a JSON null in the object, the same
applies when getting a mutable reference.
