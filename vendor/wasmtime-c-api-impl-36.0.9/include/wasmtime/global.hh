/**
 * \file wasmtime/global.hh
 */

#ifndef WASMTIME_GLOBAL_HH
#define WASMTIME_GLOBAL_HH

#include <wasmtime/error.hh>
#include <wasmtime/global.h>
#include <wasmtime/store.hh>
#include <wasmtime/types/global.hh>
#include <wasmtime/val.hh>

namespace wasmtime {

/**
 * \brief A WebAssembly global.
 *
 * This class represents a WebAssembly global, either created through
 * instantiating a module or a host global. Globals contain a WebAssembly value
 * and can be read and optionally written to.
 *
 * Note that this type does not itself own any resources. It points to resources
 * owned within a `Store` and the `Store` must be passed in as the first
 * argument to the functions defined on `Global`. Note that if the wrong `Store`
 * is passed in then the process will be aborted.
 */
class Global {
  friend class Instance;
  wasmtime_global_t global;

public:
  /// Creates as global from the raw underlying C API representation.
  Global(wasmtime_global_t global) : global(global) {}

  /**
   * \brief Create a new WebAssembly global.
   *
   * \param cx the store in which to create the global
   * \param ty the type that this global will have
   * \param init the initial value of the global
   *
   * This function can fail if `init` does not have a value that matches `ty`.
   */
  static Result<Global> create(Store::Context cx, const GlobalType &ty,
                               const Val &init) {
    wasmtime_global_t global;
    auto *error = wasmtime_global_new(cx.ptr, ty.ptr.get(), &init.val, &global);
    if (error != nullptr) {
      return Error(error);
    }
    return Global(global);
  }

  /// Returns the type of this global.
  GlobalType type(Store::Context cx) const {
    return wasmtime_global_type(cx.ptr, &global);
  }

  /// Returns the current value of this global.
  Val get(Store::Context cx) const {
    Val val;
    wasmtime_global_get(cx.ptr, &global, &val.val);
    return std::move(val);
  }

  /// Sets this global to a new value.
  ///
  /// This can fail if `val` has the wrong type or if this global isn't mutable.
  Result<std::monostate> set(Store::Context cx, const Val &val) const {
    auto *error = wasmtime_global_set(cx.ptr, &global, &val.val);
    if (error != nullptr) {
      return Error(error);
    }
    return std::monostate();
  }

  /// Returns the raw underlying C API global this is using.
  const wasmtime_global_t &capi() const { return global; }
};

} // namespace wasmtime

#endif // WASMTIME_GLOBAL_HH
