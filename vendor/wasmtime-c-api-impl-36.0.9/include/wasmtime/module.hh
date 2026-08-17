/**
 * \file wasmtime/module.hh
 */

#ifndef WASMTIME_MODULE_HH
#define WASMTIME_MODULE_HH

#include <memory>
#include <string_view>
#include <wasmtime/engine.hh>
#include <wasmtime/module.h>
#include <wasmtime/span.hh>
#include <wasmtime/types/export.hh>
#include <wasmtime/types/import.hh>
#include <wasmtime/wat.hh>

namespace wasmtime {

/**
 * \brief Representation of a compiled WebAssembly module.
 *
 * This type contains JIT code of a compiled WebAssembly module. A `Module` is
 * connected to an `Engine` and can only be instantiated within that `Engine`.
 * You can inspect a `Module` for its type information. This is passed as an
 * argument to other APIs to instantiate it.
 */
class Module {
  friend class Store;
  friend class Instance;
  friend class Linker;

  struct deleter {
    void operator()(wasmtime_module_t *p) const { wasmtime_module_delete(p); }
  };

  std::unique_ptr<wasmtime_module_t, deleter> ptr;

  Module(wasmtime_module_t *raw) : ptr(raw) {}

public:
  /// Copies another module into this one.
  Module(const Module &other) : ptr(wasmtime_module_clone(other.ptr.get())) {}
  /// Copies another module into this one.
  Module &operator=(const Module &other) {
    ptr.reset(wasmtime_module_clone(other.ptr.get()));
    return *this;
  }
  ~Module() = default;
  /// Moves resources from another module into this one.
  Module(Module &&other) = default;
  /// Moves resources from another module into this one.
  Module &operator=(Module &&other) = default;

#ifdef WASMTIME_FEATURE_COMPILER
  /**
   * \brief Compiles a module from the WebAssembly text format.
   *
   * This function will automatically use `wat2wasm` on the input and then
   * delegate to the #compile function.
   */
  static Result<Module> compile(Engine &engine, std::string_view wat) {
    auto wasm = wat2wasm(wat);
    if (!wasm) {
      return wasm.err();
    }
    auto bytes = wasm.ok();
    return compile(engine, bytes);
  }

  /**
   * \brief Compiles a module from the WebAssembly binary format.
   *
   * This function compiles the provided WebAssembly binary specified by `wasm`
   * within the compilation settings configured by `engine`. This method is
   * synchronous and will not return until the module has finished compiling.
   *
   * This function can fail if the WebAssembly binary is invalid or doesn't
   * validate (or similar).
   */
  static Result<Module> compile(Engine &engine, Span<uint8_t> wasm) {
    wasmtime_module_t *ret = nullptr;
    auto *error =
        wasmtime_module_new(engine.ptr.get(), wasm.data(), wasm.size(), &ret);
    if (error != nullptr) {
      return Error(error);
    }
    return Module(ret);
  }

  /**
   * \brief Validates the provided WebAssembly binary without compiling it.
   *
   * This function will validate whether the provided binary is indeed valid
   * within the compilation settings of the `engine` provided.
   */
  static Result<std::monostate> validate(Engine &engine, Span<uint8_t> wasm) {
    auto *error =
        wasmtime_module_validate(engine.ptr.get(), wasm.data(), wasm.size());
    if (error != nullptr) {
      return Error(error);
    }
    return std::monostate();
  }
#endif // WASMTIME_FEATURE_COMPILER

  /**
   * \brief Deserializes a previous list of bytes created with `serialize`.
   *
   * This function is intended to be much faster than `compile` where it uses
   * the artifacts of a previous compilation to quickly create an in-memory
   * module ready for instantiation.
   *
   * It is not safe to pass arbitrary input to this function, it is only safe to
   * pass in output from previous calls to `serialize`. For more information see
   * the Rust documentation -
   * https://docs.wasmtime.dev/api/wasmtime/struct.Module.html#method.deserialize
   */
  static Result<Module> deserialize(Engine &engine, Span<uint8_t> wasm) {
    wasmtime_module_t *ret = nullptr;
    auto *error = wasmtime_module_deserialize(engine.ptr.get(), wasm.data(),
                                              wasm.size(), &ret);
    if (error != nullptr) {
      return Error(error);
    }
    return Module(ret);
  }

  /**
   * \brief Deserializes a module from an on-disk file.
   *
   * This function is the same as `deserialize` except that it reads the data
   * for the serialized module from the path on disk. This can be faster than
   * the alternative which may require copying the data around.
   *
   * It is not safe to pass arbitrary input to this function, it is only safe to
   * pass in output from previous calls to `serialize`. For more information see
   * the Rust documentation -
   * https://docs.wasmtime.dev/api/wasmtime/struct.Module.html#method.deserialize
   */
  static Result<Module> deserialize_file(Engine &engine,
                                         const std::string &path) {
    wasmtime_module_t *ret = nullptr;
    auto *error =
        wasmtime_module_deserialize_file(engine.ptr.get(), path.c_str(), &ret);
    if (error != nullptr) {
      return Error(error);
    }
    return Module(ret);
  }

  /// Returns the list of types imported by this module.
  ImportType::List imports() const {
    ImportType::List list;
    wasmtime_module_imports(ptr.get(), &list.list);
    return list;
  }

  /// Returns the list of types exported by this module.
  ExportType::List exports() const {
    ExportType::List list;
    wasmtime_module_exports(ptr.get(), &list.list);
    return list;
  }

#ifdef WASMTIME_FEATURE_COMPILER
  /**
   * \brief Serializes this module to a list of bytes.
   *
   * The returned bytes can then be used to later pass to `deserialize` to
   * quickly recreate this module in a different process perhaps.
   */
  Result<std::vector<uint8_t>> serialize() const {
    wasm_byte_vec_t bytes;
    auto *error = wasmtime_module_serialize(ptr.get(), &bytes);
    if (error != nullptr) {
      return Error(error);
    }
    std::vector<uint8_t> ret;
    // NOLINTNEXTLINE TODO can this be done without triggering lints?
    Span<uint8_t> raw(reinterpret_cast<uint8_t *>(bytes.data), bytes.size);
    ret.assign(raw.begin(), raw.end());
    wasm_byte_vec_delete(&bytes);
    return ret;
  }
#endif // WASMTIME_FEATURE_COMPILER
};

} // namespace wasmtime

#endif // WASMTIME_MODULE_HH
