{{#title Function pointers — Rust ♡ C++}}
# Function pointers

### Public API:

```cpp,hidelines=...
// rust/cxx.h
...
...namespace rust {

template <typename Signature>
class Fn;

template <typename Ret, typename... Args>
class Fn<Ret(Args...)> final {
public:
  Ret operator()(Args... args) const noexcept;
  Fn operator*() const noexcept;
};
...
...} // namespace rust
```

### Restrictions:

Function pointers with a Result return type are not implemented yet.

Passing a function pointer from C++ to Rust is not implemented yet, only from
Rust to an `extern "C++"` function is implemented.

## Example

Function pointers are commonly useful for implementing [async functions over
FFI](../async.md). See the example code on that page.
