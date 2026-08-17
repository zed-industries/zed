/**
 * \file wasmtime/types/val.hh
 */

#ifndef WASMTIME_TYPES_VAL_HH
#define WASMTIME_TYPES_VAL_HH

#include <memory>
#include <ostream>
#include <wasm.h>
#include <wasmtime/val.h>

namespace wasmtime {

/// Different kinds of types accepted by Wasmtime.
enum class ValKind {
  /// WebAssembly's `i32` type
  I32,
  /// WebAssembly's `i64` type
  I64,
  /// WebAssembly's `f32` type
  F32,
  /// WebAssembly's `f64` type
  F64,
  /// WebAssembly's `v128` type from the simd proposal
  V128,
  /// WebAssembly's `externref` type from the reference types
  ExternRef,
  /// WebAssembly's `funcref` type from the reference types
  FuncRef,
  /// WebAssembly's `anyref` type
  AnyRef,
};

/// Helper X macro to construct statement for each enumerator in `ValKind`.
/// X(enumerator in `ValKind`, name string, enumerator in `wasm_valkind_t`)
#define WASMTIME_FOR_EACH_VAL_KIND(X)                                          \
  X(I32, "i32", WASM_I32)                                                      \
  X(I64, "i64", WASM_I64)                                                      \
  X(F32, "f32", WASM_F32)                                                      \
  X(F64, "f64", WASM_F64)                                                      \
  X(ExternRef, "externref", WASM_EXTERNREF)                                    \
  X(FuncRef, "funcref", WASM_FUNCREF)                                          \
  X(AnyRef, "anyref", WASMTIME_ANYREF)                                         \
  X(V128, "v128", WASMTIME_V128)

/// \brief Used to print a ValKind.
inline std::ostream &operator<<(std::ostream &os, const ValKind &e) {
  switch (e) {
#define CASE_KIND_PRINT_NAME(kind, name, ignore)                               \
  case ValKind::kind:                                                          \
    os << name;                                                                \
    break;
    WASMTIME_FOR_EACH_VAL_KIND(CASE_KIND_PRINT_NAME)
#undef CASE_KIND_PRINT_NAME
  default:
    abort();
  }
  return os;
}

/**
 * \brief Type information about a WebAssembly value.
 *
 * Currently mostly just contains the `ValKind`.
 */
class ValType {
  friend class TableType;
  friend class GlobalType;
  friend class FuncType;

  struct deleter {
    void operator()(wasm_valtype_t *p) const { wasm_valtype_delete(p); }
  };

  std::unique_ptr<wasm_valtype_t, deleter> ptr;

  static wasm_valkind_t kind_to_c(ValKind kind) {
    switch (kind) {
#define CASE_KIND_TO_C(kind, ignore, ckind)                                    \
  case ValKind::kind:                                                          \
    return ckind;
      WASMTIME_FOR_EACH_VAL_KIND(CASE_KIND_TO_C)
#undef CASE_KIND_TO_C
    default:
      abort();
    }
  }

public:
  /// \brief Non-owning reference to a `ValType`, must not be used after the
  /// original `ValType` is deleted.
  class Ref {
    friend class ValType;

    const wasm_valtype_t *ptr;

  public:
    /// \brief Instantiates from the raw C API representation.
    Ref(const wasm_valtype_t *ptr) : ptr(ptr) {}
    /// Copy constructor
    Ref(const ValType &ty) : Ref(ty.ptr.get()) {}

    /// \brief Returns the corresponding "kind" for this type.
    ValKind kind() const {
      switch (wasm_valtype_kind(ptr)) {
#define CASE_C_TO_KIND(kind, ignore, ckind)                                    \
  case ckind:                                                                  \
    return ValKind::kind;
        WASMTIME_FOR_EACH_VAL_KIND(CASE_C_TO_KIND)
#undef CASE_C_TO_KIND
      }
      std::abort();
    }
  };

  /// \brief Non-owning reference to a list of `ValType` instances. Must not be
  /// used after the original owner is deleted.
  class ListRef {
    const wasm_valtype_vec_t *list;

  public:
    /// Creates a list from the raw underlying C API.
    ListRef(const wasm_valtype_vec_t *list) : list(list) {}

    /// This list iterates over a list of `ValType::Ref` instances.
    typedef const Ref *iterator;

    /// Pointer to the beginning of iteration
    iterator begin() const {
      return reinterpret_cast<Ref *>(&list->data[0]); // NOLINT
    }

    /// Pointer to the end of iteration
    iterator end() const {
      return reinterpret_cast<Ref *>(&list->data[list->size]); // NOLINT
    }

    /// Returns how many types are in this list.
    size_t size() const { return list->size; }
  };

private:
  Ref ref;
  ValType(wasm_valtype_t *ptr) : ptr(ptr), ref(ptr) {}

public:
  /// Creates a new type from its kind.
  ValType(ValKind kind) : ValType(wasm_valtype_new(kind_to_c(kind))) {}
  /// Copies a `Ref` to a new owned value.
  ValType(Ref other) : ValType(wasm_valtype_copy(other.ptr)) {}
  /// Copies one type to a new one.
  ValType(const ValType &other) : ValType(wasm_valtype_copy(other.ptr.get())) {}
  /// Copies the contents of another type into this one.
  ValType &operator=(const ValType &other) {
    ptr.reset(wasm_valtype_copy(other.ptr.get()));
    ref = other.ref;
    return *this;
  }
  ~ValType() = default;
  /// Moves the memory owned by another value type into this one.
  ValType(ValType &&other) = default;
  /// Moves the memory owned by another value type into this one.
  ValType &operator=(ValType &&other) = default;

  /// \brief Returns the underlying `Ref`, a non-owning reference pointing to
  /// this instance.
  Ref *operator->() { return &ref; }
  /// \brief Returns the underlying `Ref`, a non-owning reference pointing to
  /// this instance.
  Ref *operator*() { return &ref; }
};

#undef WASMTIME_FOR_EACH_VAL_KIND

}; // namespace wasmtime

#endif // WASMTIME_TYPES_VAL_HH
