/**
 * \file wasmtime/types/global.hh
 */

#ifndef WASMTIME_TYPES_GLOBAL_HH
#define WASMTIME_TYPES_GLOBAL_HH

#include <wasmtime/types/val.hh>

namespace wasmtime {

/**
 * \brief Type information about a WebAssembly global
 */
class GlobalType {
  friend class Global;

  struct deleter {
    void operator()(wasm_globaltype_t *p) const { wasm_globaltype_delete(p); }
  };

  std::unique_ptr<wasm_globaltype_t, deleter> ptr;

public:
  /// Non-owning reference to a `Global`, must not be used after the original
  /// owner is deleted.
  class Ref {
    friend class GlobalType;
    const wasm_globaltype_t *ptr;

  public:
    /// Creates a new reference from the raw underlying C API representation.
    Ref(const wasm_globaltype_t *ptr) : ptr(ptr) {}
    /// Creates a new reference to the specified type.
    Ref(const GlobalType &ty) : Ref(ty.ptr.get()) {}

    /// Returns whether or not this global type is mutable.
    bool is_mutable() const {
      return wasm_globaltype_mutability(ptr) == WASM_VAR;
    }

    /// Returns the type of value stored within this global type.
    ValType::Ref content() const { return wasm_globaltype_content(ptr); }
  };

private:
  Ref ref;
  GlobalType(wasm_globaltype_t *ptr) : ptr(ptr), ref(ptr) {}

public:
  /// Creates a new global type from the specified value type and mutability.
  GlobalType(ValType ty, bool mut)
      : GlobalType(wasm_globaltype_new(
            ty.ptr.release(),
            (wasm_mutability_t)(mut ? WASM_VAR : WASM_CONST))) {}
  /// Clones a reference into a uniquely owned global type.
  GlobalType(Ref other) : GlobalType(wasm_globaltype_copy(other.ptr)) {}
  /// Copies other type information into this one.
  GlobalType(const GlobalType &other)
      : GlobalType(wasm_globaltype_copy(other.ptr.get())) {}
  /// Copies other type information into this one.
  GlobalType &operator=(const GlobalType &other) {
    ptr.reset(wasm_globaltype_copy(other.ptr.get()));
    return *this;
  }
  ~GlobalType() = default;
  /// Moves the underlying type information from another global into this one.
  GlobalType(GlobalType &&other) = default;
  /// Moves the underlying type information from another global into this one.
  GlobalType &operator=(GlobalType &&other) = default;

  /// \brief Returns the underlying `Ref`, a non-owning reference pointing to
  /// this instance.
  Ref *operator->() { return &ref; }
  /// \brief Returns the underlying `Ref`, a non-owning reference pointing to
  /// this instance.
  Ref *operator*() { return &ref; }
};

}; // namespace wasmtime

#endif // WASMTIME_TYPES_GLOBAL_HH
