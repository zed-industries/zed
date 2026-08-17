/**
 * \file wasmtime/val.hh
 */

#ifndef WASMTIME_VAL_HH
#define WASMTIME_VAL_HH

#include <optional>
#include <wasmtime/store.hh>
#include <wasmtime/types/val.hh>
#include <wasmtime/val.h>

namespace wasmtime {

/**
 * \brief Representation of a WebAssembly `externref` value.
 *
 * This class represents an value that cannot be forged by WebAssembly itself.
 * All `ExternRef` values are guaranteed to be created by the host and its
 * embedding. It's suitable to place private data structures in here which
 * WebAssembly will not have access to, only other host functions will have
 * access to them.
 *
 * Note that `ExternRef` values are rooted within a `Store` and must be manually
 * unrooted via the `unroot` function. If this is not used then values will
 * never be candidates for garbage collection.
 */
class ExternRef {
  friend class Val;

  wasmtime_externref_t val;

  static void finalizer(void *ptr) {
    std::unique_ptr<std::any> _ptr(static_cast<std::any *>(ptr));
  }

public:
  /// Creates a new `ExternRef` directly from its C-API representation.
  explicit ExternRef(wasmtime_externref_t val) : val(val) {}

  /// Creates a new `externref` value from the provided argument.
  ///
  /// Note that `val` should be safe to send across threads and should own any
  /// memory that it points to. Also note that `ExternRef` is similar to a
  /// `std::shared_ptr` in that there can be many references to the same value.
  template <typename T> explicit ExternRef(Store::Context cx, T val) {
    void *ptr = std::make_unique<std::any>(std::move(val)).release();
    bool ok = wasmtime_externref_new(cx.ptr, ptr, finalizer, &this->val);
    if (!ok) {
      fprintf(stderr, "failed to allocate a new externref");
      abort();
    }
  }

  /// Creates a new `ExternRef` which is separately rooted from this one.
  ExternRef clone(Store::Context cx) {
    wasmtime_externref_t other;
    wasmtime_externref_clone(cx.ptr, &val, &other);
    return ExternRef(other);
  }

  /// Returns the underlying host data associated with this `ExternRef`.
  std::any &data(Store::Context cx) {
    return *static_cast<std::any *>(wasmtime_externref_data(cx.ptr, &val));
  }

  /// Unroots this value from the context provided, enabling a future GC to
  /// collect the internal object if there are no more references.
  void unroot(Store::Context cx) { wasmtime_externref_unroot(cx.ptr, &val); }

  /// Returns the raw underlying C API value.
  ///
  /// This class still retains ownership of the pointer.
  const wasmtime_externref_t *raw() const { return &val; }
};

/**
 * \brief Representation of a WebAssembly `anyref` value.
 */
class AnyRef {
  friend class Val;

  wasmtime_anyref_t val;

public:
  /// Creates a new `AnyRef` directly from its C-API representation.
  explicit AnyRef(wasmtime_anyref_t val) : val(val) {}

  /// Creates a new `AnyRef` which is an `i31` with the given `value`,
  /// truncated if the upper bit is set.
  static AnyRef i31(Store::Context cx, uint32_t value) {
    wasmtime_anyref_t other;
    wasmtime_anyref_from_i31(cx.ptr, value, &other);
    return AnyRef(other);
  }

  /// Creates a new `AnyRef` which is separately rooted from this one.
  AnyRef clone(Store::Context cx) {
    wasmtime_anyref_t other;
    wasmtime_anyref_clone(cx.ptr, &val, &other);
    return AnyRef(other);
  }

  /// Unroots this value from the context provided, enabling a future GC to
  /// collect the internal object if there are no more references.
  void unroot(Store::Context cx) { wasmtime_anyref_unroot(cx.ptr, &val); }

  /// Returns the raw underlying C API value.
  ///
  /// This class still retains ownership of the pointer.
  const wasmtime_anyref_t *raw() const { return &val; }

  /// \brief If this is an `i31`, get the value zero-extended.
  std::optional<uint32_t> u31(Store::Context cx) const {
    uint32_t ret = 0;
    if (wasmtime_anyref_i31_get_u(cx.ptr, &val, &ret))
      return ret;
    return std::nullopt;
  }

  /// \brief If this is an `i31`, get the value sign-extended.
  std::optional<int32_t> i31(Store::Context cx) const {
    int32_t ret = 0;
    if (wasmtime_anyref_i31_get_s(cx.ptr, &val, &ret))
      return ret;
    return std::nullopt;
  }
};

/// \brief Container for the `v128` WebAssembly type.
struct V128 {
  /// \brief The little-endian bytes of the `v128` value.
  wasmtime_v128 v128;

  /// \brief Creates a new zero-value `v128`.
  V128() : v128{} { memset(&v128[0], 0, sizeof(wasmtime_v128)); }

  /// \brief Creates a new `V128` from its C API representation.
  V128(const wasmtime_v128 &v) : v128{} {
    memcpy(&v128[0], &v[0], sizeof(wasmtime_v128));
  }
};

class Func;

/**
 * \brief Representation of a generic WebAssembly value.
 *
 * This is roughly equivalent to a tagged union of all possible WebAssembly
 * values. This is later used as an argument with functions, globals, tables,
 * etc.
 *
 * Note that a `Val` can represent owned GC pointers. In this case the `unroot`
 * method must be used to ensure that they can later be garbage-collected.
 */
class Val {
  friend class Global;
  friend class Table;
  friend class Func;

  wasmtime_val_t val;

  Val() : val{} {
    val.kind = WASMTIME_I32;
    val.of.i32 = 0;
  }
  Val(wasmtime_val_t val) : val(val) {}

public:
  /// Creates a new `i32` WebAssembly value.
  Val(int32_t i32) : val{} {
    val.kind = WASMTIME_I32;
    val.of.i32 = i32;
  }
  /// Creates a new `i64` WebAssembly value.
  Val(int64_t i64) : val{} {
    val.kind = WASMTIME_I64;
    val.of.i64 = i64;
  }
  /// Creates a new `f32` WebAssembly value.
  Val(float f32) : val{} {
    val.kind = WASMTIME_F32;
    val.of.f32 = f32;
  }
  /// Creates a new `f64` WebAssembly value.
  Val(double f64) : val{} {
    val.kind = WASMTIME_F64;
    val.of.f64 = f64;
  }
  /// Creates a new `v128` WebAssembly value.
  Val(const V128 &v128) : val{} {
    val.kind = WASMTIME_V128;
    memcpy(&val.of.v128[0], &v128.v128[0], sizeof(wasmtime_v128));
  }
  /// Creates a new `funcref` WebAssembly value.
  Val(std::optional<Func> func);
  /// Creates a new `funcref` WebAssembly value which is not `ref.null func`.
  Val(Func func);
  /// Creates a new `externref` value.
  Val(std::optional<ExternRef> ptr) : val{} {
    val.kind = WASMTIME_EXTERNREF;
    if (ptr) {
      val.of.externref = ptr->val;
    } else {
      wasmtime_externref_set_null(&val.of.externref);
    }
  }
  /// Creates a new `anyref` value.
  Val(std::optional<AnyRef> ptr) : val{} {
    val.kind = WASMTIME_ANYREF;
    if (ptr) {
      val.of.anyref = ptr->val;
    } else {
      wasmtime_anyref_set_null(&val.of.anyref);
    }
  }
  /// Creates a new `externref` WebAssembly value which is not `ref.null
  /// extern`.
  Val(ExternRef ptr);
  /// Creates a new `anyref` WebAssembly value which is not `ref.null
  /// any`.
  Val(AnyRef ptr);

  /// Returns the kind of value that this value has.
  ValKind kind() const {
    switch (val.kind) {
    case WASMTIME_I32:
      return ValKind::I32;
    case WASMTIME_I64:
      return ValKind::I64;
    case WASMTIME_F32:
      return ValKind::F32;
    case WASMTIME_F64:
      return ValKind::F64;
    case WASMTIME_FUNCREF:
      return ValKind::FuncRef;
    case WASMTIME_EXTERNREF:
      return ValKind::ExternRef;
    case WASMTIME_ANYREF:
      return ValKind::AnyRef;
    case WASMTIME_V128:
      return ValKind::V128;
    }
    std::abort();
  }

  /// Returns the underlying `i32`, requires `kind() == KindI32` or aborts the
  /// process.
  int32_t i32() const {
    if (val.kind != WASMTIME_I32) {
      std::abort();
    }
    return val.of.i32;
  }

  /// Returns the underlying `i64`, requires `kind() == KindI64` or aborts the
  /// process.
  int64_t i64() const {
    if (val.kind != WASMTIME_I64) {
      std::abort();
    }
    return val.of.i64;
  }

  /// Returns the underlying `f32`, requires `kind() == KindF32` or aborts the
  /// process.
  float f32() const {
    if (val.kind != WASMTIME_F32) {
      std::abort();
    }
    return val.of.f32;
  }

  /// Returns the underlying `f64`, requires `kind() == KindF64` or aborts the
  /// process.
  double f64() const {
    if (val.kind != WASMTIME_F64) {
      std::abort();
    }
    return val.of.f64;
  }

  /// Returns the underlying `v128`, requires `kind() == KindV128` or aborts
  /// the process.
  V128 v128() const {
    if (val.kind != WASMTIME_V128) {
      std::abort();
    }
    return val.of.v128;
  }

  /// Returns the underlying `externref`, requires `kind() == KindExternRef` or
  /// aborts the process.
  ///
  /// Note that `externref` is a nullable reference, hence the `optional` return
  /// value.
  std::optional<ExternRef> externref(Store::Context cx) const {
    if (val.kind != WASMTIME_EXTERNREF) {
      std::abort();
    }
    if (wasmtime_externref_is_null(&val.of.externref)) {
      return std::nullopt;
    }
    wasmtime_externref_t other;
    wasmtime_externref_clone(cx.ptr, &val.of.externref, &other);
    return ExternRef(other);
  }

  /// Returns the underlying `anyref`, requires `kind() == KindAnyRef` or
  /// aborts the process.
  ///
  /// Note that `anyref` is a nullable reference, hence the `optional` return
  /// value.
  std::optional<AnyRef> anyref(Store::Context cx) const {
    if (val.kind != WASMTIME_ANYREF) {
      std::abort();
    }
    if (wasmtime_anyref_is_null(&val.of.anyref)) {
      return std::nullopt;
    }
    wasmtime_anyref_t other;
    wasmtime_anyref_clone(cx.ptr, &val.of.anyref, &other);
    return AnyRef(other);
  }

  /// Returns the underlying `funcref`, requires `kind() == KindFuncRef` or
  /// aborts the process.
  ///
  /// Note that `funcref` is a nullable reference, hence the `optional` return
  /// value.
  std::optional<Func> funcref() const;

  /// Unroots any GC references this `Val` points to within the `cx` provided.
  void unroot(Store::Context cx) { wasmtime_val_unroot(cx.ptr, &val); }
};

} // namespace wasmtime

// fill in `Func` constructors for `Val`
#include <wasmtime/func.hh>

#endif // WASMTIME_VAL_HH
