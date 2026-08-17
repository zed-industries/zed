/**
 * \file wasmtime/store.hh
 */

#ifndef WASMTIME_STORE_HH
#define WASMTIME_STORE_HH

#include <any>
#include <memory>
#include <optional>
#include <wasmtime/conf.h>
#include <wasmtime/engine.hh>
#include <wasmtime/error.hh>
#include <wasmtime/store.h>
#include <wasmtime/wasi.hh>

namespace wasmtime {

class Caller;

/**
 * \brief Owner of all WebAssembly objects
 *
 * A `Store` owns all WebAssembly objects such as instances, globals, functions,
 * memories, etc. A `Store` is one of the main central points about working with
 * WebAssembly since it's an argument to almost all APIs. The `Store` serves as
 * a form of "context" to give meaning to the pointers of `Func` and friends.
 *
 * A `Store` can be sent between threads but it cannot generally be shared
 * concurrently between threads. Memory associated with WebAssembly instances
 * will be deallocated when the `Store` is deallocated.
 */
class Store {
  struct deleter {
    void operator()(wasmtime_store_t *p) const { wasmtime_store_delete(p); }
  };

  std::unique_ptr<wasmtime_store_t, deleter> ptr;

  static void finalizer(void *ptr) {
    std::unique_ptr<std::any> _ptr(static_cast<std::any *>(ptr));
  }

public:
  /// Creates a new `Store` within the provided `Engine`.
  explicit Store(Engine &engine)
      : ptr(wasmtime_store_new(engine.ptr.get(), nullptr, finalizer)) {}

  /**
   * \brief An interior pointer into a `Store`.
   *
   * A `Context` object is created from either a `Store` or a `Caller`. It is an
   * interior pointer into a `Store` and cannot be used outside the lifetime of
   * the original object it was created from.
   *
   * This object is an argument to most APIs in Wasmtime but typically doesn't
   * need to be constructed explicitly since it can be created from a `Store&`
   * or a `Caller&`.
   */
  class Context {
    friend class Global;
    friend class Table;
    friend class Memory;
    friend class Func;
    friend class Instance;
    friend class Linker;
    friend class ExternRef;
    friend class AnyRef;
    friend class Val;
    friend class Store;
    wasmtime_context_t *ptr;

    Context(wasmtime_context_t *ptr) : ptr(ptr) {}

  public:
    /// Creates a context referencing the provided `Store`.
    Context(Store &store) : Context(wasmtime_store_context(store.ptr.get())) {}
    /// Creates a context referencing the provided `Store`.
    Context(Store *store) : Context(*store) {}
    /// Creates a context referencing the provided `Caller`.
    Context(Caller &caller);
    /// Creates a context referencing the provided `Caller`.
    Context(Caller *caller);

    /// Runs a garbage collection pass in the referenced store to collect loose
    /// `externref` values, if any are available.
    void gc() { wasmtime_context_gc(ptr); }

    /// Injects fuel to be consumed within this store.
    ///
    /// Stores start with 0 fuel and if `Config::consume_fuel` is enabled then
    /// this is required if you want to let WebAssembly actually execute.
    ///
    /// Returns an error if fuel consumption isn't enabled.
    Result<std::monostate> set_fuel(uint64_t fuel) {
      auto *error = wasmtime_context_set_fuel(ptr, fuel);
      if (error != nullptr) {
        return Error(error);
      }
      return std::monostate();
    }

    /// Returns the amount of fuel consumed so far by executing WebAssembly.
    ///
    /// Returns `std::nullopt` if fuel consumption is not enabled.
    Result<uint64_t> get_fuel() const {
      uint64_t fuel = 0;
      auto *error = wasmtime_context_get_fuel(ptr, &fuel);
      if (error != nullptr) {
        return Error(error);
      }
      return fuel;
    }

    /// Set user specified data associated with this store.
    void set_data(std::any data) const {
      finalizer(static_cast<std::any *>(wasmtime_context_get_data(ptr)));
      wasmtime_context_set_data(
          ptr, std::make_unique<std::any>(std::move(data)).release());
    }

    /// Get user specified data associated with this store.
    std::any &get_data() const {
      return *static_cast<std::any *>(wasmtime_context_get_data(ptr));
    }

#ifdef WASMTIME_FEATURE_WASI
    /// Configures the WASI state used by this store.
    ///
    /// This will only have an effect if used in conjunction with
    /// `Linker::define_wasi` because otherwise no host functions will use the
    /// WASI state.
    Result<std::monostate> set_wasi(WasiConfig config) {
      auto *error = wasmtime_context_set_wasi(ptr, config.ptr.release());
      if (error != nullptr) {
        return Error(error);
      }
      return std::monostate();
    }
#endif // WASMTIME_FEATURE_WASI

    /// Configures this store's epoch deadline to be the specified number of
    /// ticks beyond the engine's current epoch.
    ///
    /// By default the deadline is the current engine's epoch, immediately
    /// interrupting code if epoch interruption is enabled. This must be called
    /// to extend the deadline to allow interruption.
    void set_epoch_deadline(uint64_t ticks_beyond_current) {
      wasmtime_context_set_epoch_deadline(ptr, ticks_beyond_current);
    }

    /// Returns the raw context pointer for the C API.
    wasmtime_context_t *raw_context() { return ptr; }
  };

  /// \brief Provides limits for a store. Used by hosts to limit resource
  /// consumption of instances. Use negative value to keep the default value
  /// for the limit.
  ///
  /// \param memory_size the maximum number of bytes a linear memory can grow
  /// to. Growing a linear memory beyond this limit will fail. By default,
  /// linear memory will not be limited.
  ///
  /// \param table_elements the maximum number of elements in a table.
  /// Growing a table beyond this limit will fail. By default, table elements
  /// will not be limited.
  ///
  /// \param instances the maximum number of instances that can be created
  /// for a Store. Module instantiation will fail if this limit is exceeded.
  /// This value defaults to 10,000.
  ///
  /// \param tables the maximum number of tables that can be created for a
  /// Store. Module instantiation will fail if this limit is exceeded. This
  /// value defaults to 10,000.
  ///
  /// \param memories the maximum number of linear
  /// memories that can be created for a Store. Instantiation will fail with an
  /// error if this limit is exceeded. This value defaults to 10,000.
  ///
  /// Use any negative value for the parameters that should be kept on
  /// the default values.
  ///
  /// Note that the limits are only used to limit the creation/growth of
  /// resources in the future, this does not retroactively attempt to apply
  /// limits to the store.
  void limiter(int64_t memory_size, int64_t table_elements, int64_t instances,
               int64_t tables, int64_t memories) {
    wasmtime_store_limiter(ptr.get(), memory_size, table_elements, instances,
                           tables, memories);
  }

  /// Explicit function to acquire a `Context` from this store.
  Context context() { return this; }
};

} // namespace wasmtime

#endif // WASMTIME_STORE_HH
