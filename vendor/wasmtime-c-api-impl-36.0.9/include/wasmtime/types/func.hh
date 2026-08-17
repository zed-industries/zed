/**
 * \file wasmtime/types/func.hh
 */

#ifndef WASMTIME_TYPES_FUNC_HH
#define WASMTIME_TYPES_FUNC_HH

#include <wasmtime/types/val.hh>

namespace wasmtime {

/**
 * \brief Type information for a WebAssembly function.
 */
class FuncType {
  friend class Func;
  friend class Linker;

  struct deleter {
    void operator()(wasm_functype_t *p) const { wasm_functype_delete(p); }
  };

  std::unique_ptr<wasm_functype_t, deleter> ptr;

public:
  /// Non-owning reference to a `FuncType`, must not be used after the original
  /// owner has been deleted.
  class Ref {
    friend class FuncType;
    const wasm_functype_t *ptr;

  public:
    /// Creates a new reference from the underlying C API representation.
    Ref(const wasm_functype_t *ptr) : ptr(ptr) {}
    /// Creates a new reference to the given type.
    Ref(const FuncType &ty) : Ref(ty.ptr.get()) {}

    /// Returns the list of types this function type takes as parameters.
    ValType::ListRef params() const { return wasm_functype_params(ptr); }

    /// Returns the list of types this function type returns.
    ValType::ListRef results() const { return wasm_functype_results(ptr); }
  };

private:
  Ref ref;
  FuncType(wasm_functype_t *ptr) : ptr(ptr), ref(ptr) {}

public:
  /// Creates a new function type from the given list of parameters and results.
  FuncType(std::initializer_list<ValType> params,
           std::initializer_list<ValType> results)
      : ref(nullptr) {
    *this = FuncType::from_iters(params, results);
  }

  /// Copies a reference into a uniquely owned function type.
  FuncType(Ref other) : FuncType(wasm_functype_copy(other.ptr)) {}
  /// Copies another type's information into this one.
  FuncType(const FuncType &other)
      : FuncType(wasm_functype_copy(other.ptr.get())) {}
  /// Copies another type's information into this one.
  FuncType &operator=(const FuncType &other) {
    ptr.reset(wasm_functype_copy(other.ptr.get()));
    return *this;
  }
  ~FuncType() = default;
  /// Moves type information from another type into this one.
  FuncType(FuncType &&other) = default;
  /// Moves type information from another type into this one.
  FuncType &operator=(FuncType &&other) = default;

  /// Creates a new function type from the given list of parameters and results.
  template <typename P, typename R>
  static FuncType from_iters(P params, R results) {
    wasm_valtype_vec_t param_vec;
    wasm_valtype_vec_t result_vec;
    wasm_valtype_vec_new_uninitialized(&param_vec, params.size());
    wasm_valtype_vec_new_uninitialized(&result_vec, results.size());
    size_t i = 0;

    for (auto val : params) {
      param_vec.data[i++] = val.ptr.release(); // NOLINT
    }
    i = 0;
    for (auto val : results) {
      result_vec.data[i++] = val.ptr.release(); // NOLINT
    }

    return wasm_functype_new(&param_vec, &result_vec);
  }

  /// \brief Returns the underlying `Ref`, a non-owning reference pointing to
  /// this instance.
  Ref *operator->() { return &ref; }
  /// \brief Returns the underlying `Ref`, a non-owning reference pointing to
  /// this instance.
  Ref *operator*() { return &ref; }
};

}; // namespace wasmtime

#endif // WASMTIME_TYPES_FUNC_HH
