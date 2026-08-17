{{#title std::string — Rust ♡ C++}}
# std::string

The Rust binding of std::string is called **[`CxxString`]**. See the link for
documentation of the Rust API.

[`CxxString`]: https://docs.rs/cxx/*/cxx/struct.CxxString.html

### Restrictions:

Rust code can never obtain a CxxString by value. C++'s string requires a move
constructor and may hold internal pointers, which is not compatible with Rust's
move behavior. Instead in Rust code we will only ever look at a CxxString
through a reference or smart pointer, as in &CxxString or Pin\<&mut CxxString\>
or UniquePtr\<CxxString\>.

In order to construct a CxxString on the stack from Rust, you must use the
[`let_cxx_string!`] macro which will pin the string properly. The code below
uses this in one place, and the link covers the syntax.

[`let_cxx_string!`]: https://docs.rs/cxx/*/cxx/macro.let_cxx_string.html

## Example

This example uses C++17's std::variant to build a toy JSON type. JSON can hold
various types including strings, and JSON's object type is a map with string
keys. The example demonstrates Rust indexing into one of those maps.

```rust,noplayground
// src/main.rs

use cxx::let_cxx_string;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("example/include/json.h");

        #[cxx_name = "json"]
        type Json;
        #[cxx_name = "object"]
        type Object;

        fn isNull(self: &Json) -> bool;
        fn isNumber(self: &Json) -> bool;
        fn isString(self: &Json) -> bool;
        fn isArray(self: &Json) -> bool;
        fn isObject(self: &Json) -> bool;

        fn getNumber(self: &Json) -> f64;
        fn getString(self: &Json) -> &CxxString;
        fn getArray(self: &Json) -> &CxxVector<Json>;
        fn getObject(self: &Json) -> &Object;

        #[cxx_name = "at"]
        fn get<'a>(self: &'a Object, key: &CxxString) -> &'a Json;

        fn load_config() -> UniquePtr<Json>;
    }
}

fn main() {
    let config = ffi::load_config();

    let_cxx_string!(key = "name");
    println!("{}", config.getObject().get(&key).getString());
}
```

```cpp
// include/json.h

#pragma once
#include <map>
#include <memory>
#include <string>
#include <variant>
#include <vector>

class json final {
public:
  static const json null;
  using number = double;
  using string = std::string;
  using array = std::vector<json>;
  using object = std::map<string, json>;

  json() noexcept = default;
  json(const json &) = default;
  json(json &&) = default;
  template <typename... T>
  json(T &&...value) : value(std::forward<T>(value)...) {}

  bool isNull() const;
  bool isNumber() const;
  bool isString() const;
  bool isArray() const;
  bool isObject() const;

  number getNumber() const;
  const string &getString() const;
  const array &getArray() const;
  const object &getObject() const;

private:
  std::variant<std::monostate, number, string, array, object> value;
};

using object = json::object;

std::unique_ptr<json> load_config();
```

```cpp
// include/json.cc

#include "example/include/json.h"
#include <initializer_list>
#include <utility>

const json json::null{};
bool json::isNull() const { return std::holds_alternative<std::monostate>(value); }
bool json::isNumber() const { return std::holds_alternative<number>(value); }
bool json::isString() const { return std::holds_alternative<string>(value); }
bool json::isArray() const { return std::holds_alternative<array>(value); }
bool json::isObject() const { return std::holds_alternative<object>(value); }
json::number json::getNumber() const { return std::get<number>(value); }
const json::string &json::getString() const { return std::get<string>(value); }
const json::array &json::getArray() const { return std::get<array>(value); }
const json::object &json::getObject() const { return std::get<object>(value); }

std::unique_ptr<json> load_config() {
  return std::make_unique<json>(
      std::in_place_type<json::object>,
      std::initializer_list<std::pair<const std::string, json>>{
          {"name", "cxx-example"},
          {"edition", 2021.},
          {"repository", json::null}});
}
```
