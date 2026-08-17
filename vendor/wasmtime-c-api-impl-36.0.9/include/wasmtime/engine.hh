/**
 * \file wasmtime/engine.hh
 */

#ifndef WASMTIME_ENGINE_HH
#define WASMTIME_ENGINE_HH

#include <memory>
#include <wasmtime/config.hh>
#include <wasmtime/engine.h>

namespace wasmtime {

/**
 * \brief Global compilation state in Wasmtime.
 *
 * Created with either default configuration or with a specified instance of
 * configuration, an `Engine` is used as an umbrella "session" for all other
 * operations in Wasmtime.
 */
class Engine {
  friend class Store;
  friend class Module;
  friend class Linker;

  struct deleter {
    void operator()(wasm_engine_t *p) const { wasm_engine_delete(p); }
  };

  std::unique_ptr<wasm_engine_t, deleter> ptr;

public:
  /// \brief Creates an engine with default compilation settings.
  Engine() : ptr(wasm_engine_new()) {}
  /// \brief Creates an engine with the specified compilation settings.
  explicit Engine(Config config)
      : ptr(wasm_engine_new_with_config(config.ptr.release())) {}

  /// Copies another engine into this one.
  Engine(const Engine &other) : ptr(wasmtime_engine_clone(other.ptr.get())) {}
  /// Copies another engine into this one.
  Engine &operator=(const Engine &other) {
    ptr.reset(wasmtime_engine_clone(other.ptr.get()));
    return *this;
  }
  ~Engine() = default;
  /// Moves resources from another engine into this one.
  Engine(Engine &&other) = default;
  /// Moves resources from another engine into this one.
  Engine &operator=(Engine &&other) = default;

  /// \brief Increments the current epoch which may result in interrupting
  /// currently executing WebAssembly in connected stores if the epoch is now
  /// beyond the configured threshold.
  void increment_epoch() const { wasmtime_engine_increment_epoch(ptr.get()); }

  /// \brief Returns whether this engine is using Pulley for execution.
  void is_pulley() const { wasmtime_engine_is_pulley(ptr.get()); }
};

} // namespace wasmtime

#endif // WASMTIME_ENGINE_HH
