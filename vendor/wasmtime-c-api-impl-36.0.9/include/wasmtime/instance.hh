/**
 * \file wasmtime/instance.hh
 */

#ifndef WASMTIME_INSTANCE_HH
#define WASMTIME_INSTANCE_HH

#include <wasmtime/extern.hh>
#include <wasmtime/func.hh>
#include <wasmtime/global.hh>
#include <wasmtime/instance.h>
#include <wasmtime/memory.hh>
#include <wasmtime/module.hh>
#include <wasmtime/store.hh>
#include <wasmtime/table.hh>

namespace wasmtime {

/**
 * \brief A WebAssembly instance.
 *
 * This class represents a WebAssembly instance, created by instantiating a
 * module. An instance is the collection of items exported by the module, which
 * can be accessed through the `Store` that owns the instance.
 *
 * Note that this type does not itself own any resources. It points to resources
 * owned within a `Store` and the `Store` must be passed in as the first
 * argument to the functions defined on `Instance`. Note that if the wrong
 * `Store` is passed in then the process will be aborted.
 */
class Instance {
  friend class Linker;
  friend class Caller;

  wasmtime_instance_t instance;

public:
  /// Creates a new instance from the raw underlying C API representation.
  Instance(wasmtime_instance_t instance) : instance(instance) {}

  /**
   * \brief Instantiates the module `m` with the provided `imports`
   *
   * \param cx the store in which to instantiate the provided module
   * \param m the module to instantiate
   * \param imports the list of imports to use to instantiate the module
   *
   * This `imports` parameter is expected to line up 1:1 with the imports
   * required by the `m`. The type of `m` can be inspected to determine in which
   * order to provide the imports. Note that this is a relatively low-level API
   * and it's generally recommended to use `Linker` instead for name-based
   * instantiation.
   *
   * This function can return an error if any of the `imports` have the wrong
   * type, or if the wrong number of `imports` is provided.
   */
  static TrapResult<Instance> create(Store::Context cx, const Module &m,
                                     const std::vector<Extern> &imports) {
    std::vector<wasmtime_extern_t> raw_imports;
    for (const auto &item : imports) {
      raw_imports.push_back(wasmtime_extern_t{});
      auto &last = raw_imports.back();
      detail::cvt_extern(item, last);
    }
    wasmtime_instance_t instance;
    wasm_trap_t *trap = nullptr;
    auto *error = wasmtime_instance_new(cx.ptr, m.ptr.get(), raw_imports.data(),
                                        raw_imports.size(), &instance, &trap);
    if (error != nullptr) {
      return TrapError(Error(error));
    }
    if (trap != nullptr) {
      return TrapError(Trap(trap));
    }
    return Instance(instance);
  }

  /**
   * \brief Load an instance's export by name.
   *
   * This function will look for an export named `name` on this instance and, if
   * found, return it as an `Extern`.
   */
  std::optional<Extern> get(Store::Context cx, std::string_view name) {
    wasmtime_extern_t e;
    if (!wasmtime_instance_export_get(cx.ptr, &instance, name.data(),
                                      name.size(), &e)) {
      return std::nullopt;
    }
    return detail::cvt_extern(e);
  }

  /**
   * \brief Load an instance's export by index.
   *
   * This function will look for the `idx`th export of this instance. This will
   * return both the name of the export as well as the exported item itself.
   */
  std::optional<std::pair<std::string_view, Extern>> get(Store::Context cx,
                                                         size_t idx) {
    wasmtime_extern_t e;
    // I'm not sure why clang-tidy thinks this is using va_list or anything
    // related to that...
    // NOLINTNEXTLINE(cppcoreguidelines-pro-type-vararg)
    char *name = nullptr;
    size_t len = 0;
    if (!wasmtime_instance_export_nth(cx.ptr, &instance, idx, &name, &len,
                                      &e)) {
      return std::nullopt;
    }
    std::string_view n(name, len);
    return std::pair(n, detail::cvt_extern(e));
  }
};

} // namespace wasmtime

#endif // WASMTIME_INSTANCE_HH
