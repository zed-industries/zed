/**
 * \file wasmtime/config.hh
 */

#ifndef WASMTIME_CONFIG_HH
#define WASMTIME_CONFIG_HH

#include <wasmtime/conf.h>
#include <wasmtime/config.h>
#include <wasmtime/error.hh>
#include <wasmtime/types/memory.hh>

namespace wasmtime {

/// \brief Strategies passed to `Config::strategy`
enum class Strategy {
  /// Automatically selects the compilation strategy
  Auto = WASMTIME_STRATEGY_AUTO,
  /// Requires Cranelift to be used for compilation
  Cranelift = WASMTIME_STRATEGY_CRANELIFT,
};

/// \brief Values passed to `Config::cranelift_opt_level`
enum class OptLevel {
  /// No extra optimizations performed
  None = WASMTIME_OPT_LEVEL_NONE,
  /// Optimize for speed
  Speed = WASMTIME_OPT_LEVEL_SPEED,
  /// Optimize for speed and generated code size
  SpeedAndSize = WASMTIME_OPT_LEVEL_SPEED_AND_SIZE,
};

/// \brief Values passed to `Config::profiler`
enum class ProfilingStrategy {
  /// No profiling enabled
  None = WASMTIME_PROFILING_STRATEGY_NONE,
  /// Profiling hooks via perf's jitdump
  Jitdump = WASMTIME_PROFILING_STRATEGY_JITDUMP,
  /// Profiling hooks via VTune
  Vtune = WASMTIME_PROFILING_STRATEGY_VTUNE,
  /// Profiling hooks via perfmap
  Perfmap = WASMTIME_PROFILING_STRATEGY_PERFMAP,
};

#ifdef WASMTIME_FEATURE_POOLING_ALLOCATOR
/**
 * \brief Pool allocation configuration for Wasmtime.
 *
 * For more information be sure to consult the [rust
 * documentation](https://docs.wasmtime.dev/api/wasmtime/struct.PoolingAllocationConfig.html).
 */
class PoolAllocationConfig {
  friend class Config;

  struct deleter {
    void operator()(wasmtime_pooling_allocation_config_t *p) const {
      wasmtime_pooling_allocation_config_delete(p);
    }
  };

  std::unique_ptr<wasmtime_pooling_allocation_config_t, deleter> ptr;

public:
  PoolAllocationConfig() : ptr(wasmtime_pooling_allocation_config_new()) {}

  /// \brief Configures the maximum number of “unused warm slots” to retain in
  /// the pooling allocator.
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.PoolingAllocationConfig.html#method.max_unused_warm_slots.
  void max_unused_warm_slots(uint32_t max) {
    wasmtime_pooling_allocation_config_max_unused_warm_slots_set(ptr.get(),
                                                                 max);
  }

  /// \brief The target number of decommits to do per batch.
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.PoolingAllocationConfig.html#method.decommit_batch_size.
  void decommit_batch_size(size_t batch_size) {
    wasmtime_pooling_allocation_config_decommit_batch_size_set(ptr.get(),
                                                               batch_size);
  }

#ifdef WASMTIME_FEATURE_ASYNC
  /// \brief How much memory, in bytes, to keep resident for async stacks
  /// allocated with the pooling allocator.
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.PoolingAllocationConfig.html#method.async_stack_keep_resident.
  void async_stack_keep_resident(size_t size) {
    wasmtime_pooling_allocation_config_async_stack_keep_resident_set(ptr.get(),
                                                                     size);
  }
#endif // WASMTIME_FEATURE_ASYNC

  /// \brief How much memory, in bytes, to keep resident for each linear memory
  /// after deallocation.
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.PoolingAllocationConfig.html#method.linear_memory_keep_resident.
  void linear_memory_keep_resident(size_t size) {
    wasmtime_pooling_allocation_config_linear_memory_keep_resident_set(
        ptr.get(), size);
  }

  /// \brief How much memory, in bytes, to keep resident for each table after
  /// deallocation.
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.PoolingAllocationConfig.html#method.table_keep_resident.
  void table_keep_resident(size_t size) {
    wasmtime_pooling_allocation_config_table_keep_resident_set(ptr.get(), size);
  }

  /// \brief The maximum number of concurrent component instances supported
  /// (default is 1000).
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.PoolingAllocationConfig.html#method.total_component_instances.
  void total_component_instances(uint32_t count) {
    wasmtime_pooling_allocation_config_total_component_instances_set(ptr.get(),
                                                                     count);
  }

  /// \brief The maximum size, in bytes, allocated for a component instance’s
  /// VMComponentContext metadata.
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.PoolingAllocationConfig.html#method.max_component_instance_size.
  void max_component_instance_size(size_t size) {
    wasmtime_pooling_allocation_config_max_component_instance_size_set(
        ptr.get(), size);
  }

  /// \brief The maximum number of core instances a single component may contain
  /// (default is unlimited).
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.PoolingAllocationConfig.html#method.max_core_instances_per_component.
  void max_core_instances_per_component(uint32_t count) {
    wasmtime_pooling_allocation_config_max_core_instances_per_component_set(
        ptr.get(), count);
  }

  /// \brief The maximum number of Wasm linear memories that a single component
  /// may transitively contain (default is unlimited).
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.PoolingAllocationConfig.html#method.max_memories_per_component.
  void max_memories_per_component(uint32_t count) {
    wasmtime_pooling_allocation_config_max_memories_per_component_set(ptr.get(),
                                                                      count);
  }

  /// \brief The maximum number of tables that a single component may
  /// transitively contain (default is unlimited).
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.PoolingAllocationConfig.html#method.max_tables_per_component.
  void max_tables_per_component(uint32_t count) {
    wasmtime_pooling_allocation_config_max_tables_per_component_set(ptr.get(),
                                                                    count);
  }

  /// \brief The maximum number of concurrent Wasm linear memories supported
  /// (default is 1000).
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.PoolingAllocationConfig.html#method.total_memories.
  void total_memories(uint32_t count) {
    wasmtime_pooling_allocation_config_total_memories_set(ptr.get(), count);
  }

  /// \brief The maximum number of concurrent tables supported (default is
  /// 1000).
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.PoolingAllocationConfig.html#method.total_tables.
  void total_tables(uint32_t count) {
    wasmtime_pooling_allocation_config_total_tables_set(ptr.get(), count);
  }

#ifdef WASMTIME_FEATURE_ASYNC
  /// \brief The maximum number of execution stacks allowed for asynchronous
  /// execution, when enabled (default is 1000).
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.PoolingAllocationConfig.html#method.total_stacks.
  void total_stacks(uint32_t count) {
    wasmtime_pooling_allocation_config_total_stacks_set(ptr.get(), count);
  }
#endif // WASMTIME_FEATURE_ASYNC

  /// \brief The maximum number of concurrent core instances supported (default
  /// is 1000).
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.PoolingAllocationConfig.html#method.total_core_instances.
  void total_core_instances(uint32_t count) {
    wasmtime_pooling_allocation_config_total_core_instances_set(ptr.get(),
                                                                count);
  }

  /// \brief The maximum size, in bytes, allocated for a core instance’s
  /// VMContext metadata.
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.PoolingAllocationConfig.html#method.max_core_instance_size.
  void max_core_instance_size(size_t size) {
    wasmtime_pooling_allocation_config_max_core_instance_size_set(ptr.get(),
                                                                  size);
  }

  /// \brief The maximum number of defined tables for a core module (default is
  /// 1).
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.PoolingAllocationConfig.html#method.max_tables_per_module.
  void max_tables_per_module(uint32_t tables) {
    wasmtime_pooling_allocation_config_max_tables_per_module_set(ptr.get(),
                                                                 tables);
  }

  /// \brief The maximum table elements for any table defined in a module
  /// (default is 20000).
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.PoolingAllocationConfig.html#method.table_elements.
  void table_elements(size_t elements) {
    wasmtime_pooling_allocation_config_table_elements_set(ptr.get(), elements);
  }

  /// \brief The maximum number of defined linear memories for a module (default
  /// is 1).
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.PoolingAllocationConfig.html#method.max_memories_per_module.
  void max_memories_per_module(uint32_t memories) {
    wasmtime_pooling_allocation_config_max_memories_per_module_set(ptr.get(),
                                                                   memories);
  }

  /// \brief The maximum byte size that any WebAssembly linear memory may grow
  /// to.
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.PoolingAllocationConfig.html#method.max_memory_size.
  void max_memory_size(size_t bytes) {
    wasmtime_pooling_allocation_config_max_memory_size_set(ptr.get(), bytes);
  }

  /// \brief The maximum number of concurrent GC heaps supported (default is
  /// 1000).
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.PoolingAllocationConfig.html#method.total_gc_heaps.
  void total_gc_heaps(uint32_t count) {
    wasmtime_pooling_allocation_config_total_gc_heaps_set(ptr.get(), count);
  }
};
#endif // WASMTIME_FEATURE_POOLING_ALLOCATOR

/**
 * \brief Configuration for Wasmtime.
 *
 * This class is used to configure Wasmtime's compilation and various other
 * settings such as enabled WebAssembly proposals.
 *
 * For more information be sure to consult the [rust
 * documentation](https://docs.wasmtime.dev/api/wasmtime/struct.Config.html).
 */
class Config {
  friend class Engine;

  struct deleter {
    void operator()(wasm_config_t *p) const { wasm_config_delete(p); }
  };

  std::unique_ptr<wasm_config_t, deleter> ptr;

public:
  /// \brief Creates configuration with all the default settings.
  Config() : ptr(wasm_config_new()) {}

  /// \brief Configures whether dwarf debuginfo is emitted for assisting
  /// in-process debugging.
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.Config.html#method.debug_info
  void debug_info(bool enable) {
    wasmtime_config_debug_info_set(ptr.get(), enable);
  }

  /// \brief Configures whether epochs are enabled which can be used to
  /// interrupt currently executing WebAssembly.
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.Config.html#method.epoch_interruption
  void epoch_interruption(bool enable) {
    wasmtime_config_epoch_interruption_set(ptr.get(), enable);
  }

  /// \brief Configures whether WebAssembly code will consume fuel and trap when
  /// it runs out.
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.Config.html#method.consume_fuel
  void consume_fuel(bool enable) {
    wasmtime_config_consume_fuel_set(ptr.get(), enable);
  }

  /// \brief Configures the maximum amount of native stack wasm can consume.
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.Config.html#method.max_wasm_stack
  void max_wasm_stack(size_t stack) {
    wasmtime_config_max_wasm_stack_set(ptr.get(), stack);
  }

#ifdef WASMTIME_FEATURE_THREADS
  /// \brief Configures whether the WebAssembly threads proposal is enabled
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.Config.html#method.wasm_threads
  void wasm_threads(bool enable) {
    wasmtime_config_wasm_threads_set(ptr.get(), enable);
  }
#endif // WASMTIME_FEATURE_THREADS

  /// \brief Configures whether the WebAssembly tail call proposal is enabled
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.Config.html#method.wasm_tail_call
  void wasm_tail_call(bool enable) {
    wasmtime_config_wasm_tail_call_set(ptr.get(), enable);
  }

  /// \brief Configures whether the WebAssembly reference types proposal is
  /// enabled
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.Config.html#method.wasm_reference_types
  void wasm_reference_types(bool enable) {
    wasmtime_config_wasm_reference_types_set(ptr.get(), enable);
  }

  /// \brief Configures whether the WebAssembly simd proposal is enabled
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.Config.html#method.wasm_simd
  void wasm_simd(bool enable) {
    wasmtime_config_wasm_simd_set(ptr.get(), enable);
  }

  /// \brief Configures whether the WebAssembly relaxed simd proposal is enabled
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.Config.html#method.wasm_relaxed_simd
  void wasm_relaxed_simd(bool enable) {
    wasmtime_config_wasm_relaxed_simd_set(ptr.get(), enable);
  }

  /// \brief Configures whether the WebAssembly relaxed simd proposal supports
  /// its deterministic behavior.
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.Config.html#method.wasm_relaxed_simd_deterministic
  void wasm_relaxed_simd_deterministic(bool enable) {
    wasmtime_config_wasm_relaxed_simd_deterministic_set(ptr.get(), enable);
  }

  /// \brief Configures whether the WebAssembly bulk memory proposal is enabled
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.Config.html#method.wasm_bulk_memory
  void wasm_bulk_memory(bool enable) {
    wasmtime_config_wasm_bulk_memory_set(ptr.get(), enable);
  }

  /// \brief Configures whether the WebAssembly multi value proposal is enabled
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.Config.html#method.wasm_multi_value
  void wasm_multi_value(bool enable) {
    wasmtime_config_wasm_multi_value_set(ptr.get(), enable);
  }

  /// \brief Configures whether the WebAssembly multi memory proposal is enabled
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.Config.html#method.wasm_multi_memory
  void wasm_multi_memory(bool enable) {
    wasmtime_config_wasm_multi_memory_set(ptr.get(), enable);
  }

  /// \brief Configures whether the WebAssembly memory64 proposal is enabled
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.Config.html#method.wasm_memory64
  void wasm_memory64(bool enable) {
    wasmtime_config_wasm_memory64_set(ptr.get(), enable);
  }

  /// \brief Configures whether the WebAssembly Garbage Collection proposal will
  /// be enabled
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.Config.html#method.wasm_gc
  void wasm_gc(bool enable) { wasmtime_config_wasm_gc_set(ptr.get(), enable); }

  /// \brief Configures whether the WebAssembly function references proposal
  /// will be enabled
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.Config.html#method.wasm_function_references
  void wasm_function_references(bool enable) {
    wasmtime_config_wasm_function_references_set(ptr.get(), enable);
  }

  /// \brief Configures whether the WebAssembly wide arithmetic proposal will be
  /// enabled
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.Config.html#method.wasm_wide_arithmetic
  void wasm_wide_arithmetic(bool enable) {
    wasmtime_config_wasm_wide_arithmetic_set(ptr.get(), enable);
  }

#ifdef WASMTIME_FEATURE_COMPONENT_MODEL
  /// \brief Configures whether the WebAssembly component model proposal will be
  /// enabled
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.Config.html#method.wasm_component_model
  void wasm_component_model(bool enable) {
    wasmtime_config_wasm_component_model_set(ptr.get(), enable);
  }
#endif // WASMTIME_FEATURE_COMPONENT_MODEL

#ifdef WASMTIME_FEATURE_PARALLEL_COMPILATION
  /// \brief Configure whether wasmtime should compile a module using multiple
  /// threads.
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.Config.html#method.parallel_compilation
  void parallel_compilation(bool enable) {
    wasmtime_config_parallel_compilation_set(ptr.get(), enable);
  }
#endif // WASMTIME_FEATURE_PARALLEL_COMPILATION

#ifdef WASMTIME_FEATURE_COMPILER
  /// \brief Configures compilation strategy for wasm code.
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.Config.html#method.strategy
  void strategy(Strategy strategy) {
    wasmtime_config_strategy_set(ptr.get(),
                                 static_cast<wasmtime_strategy_t>(strategy));
  }

  /// \brief Configures whether cranelift's debug verifier is enabled
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.Config.html#method.cranelift_debug_verifier
  void cranelift_debug_verifier(bool enable) {
    wasmtime_config_cranelift_debug_verifier_set(ptr.get(), enable);
  }

  /// \brief Configures whether cranelift's nan canonicalization
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.Config.html#method.cranelift_nan_canonicalization
  void cranelift_nan_canonicalization(bool enable) {
    wasmtime_config_cranelift_nan_canonicalization_set(ptr.get(), enable);
  }

  /// \brief Configures cranelift's optimization level
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.Config.html#method.cranelift_opt_level
  void cranelift_opt_level(OptLevel level) {
    wasmtime_config_cranelift_opt_level_set(
        ptr.get(), static_cast<wasmtime_opt_level_t>(level));
  }

  /// \brief Enable the specified Cranelift flag
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.Config.html#method.cranelift_flag_enable
  void cranelift_flag_enable(const std::string &flag) {
    wasmtime_config_cranelift_flag_enable(ptr.get(), flag.c_str());
  }

  /// \brief Configure the specified Cranelift flag
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.Config.html#method.cranelift_flag_set
  void cranelift_flag_set(const std::string &flag, const std::string &value) {
    wasmtime_config_cranelift_flag_set(ptr.get(), flag.c_str(), value.c_str());
  }
#endif // WASMTIME_FEATURE_COMPILER

  /// \brief Configures an active wasm profiler
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.Config.html#method.profiler
  void profiler(ProfilingStrategy profiler) {
    wasmtime_config_profiler_set(
        ptr.get(), static_cast<wasmtime_profiling_strategy_t>(profiler));
  }

  /// \brief Configures the size of the initial linear memory allocation.
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.Config.html#method.memory_reservation
  void memory_reservation(size_t size) {
    wasmtime_config_memory_reservation_set(ptr.get(), size);
  }

  /// \brief Configures the size of the bytes to reserve beyond the end of
  /// linear memory to grow into.
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.Config.html#method.memory_reservation_for_growth
  void memory_reservation_for_growth(size_t size) {
    wasmtime_config_memory_reservation_for_growth_set(ptr.get(), size);
  }

  /// \brief Configures the size of memory's guard region
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.Config.html#method.memory_guard_size
  void memory_guard_size(size_t size) {
    wasmtime_config_memory_guard_size_set(ptr.get(), size);
  }

  /// \brief Configures whether the base pointer of linear memory is allowed to
  /// move.
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.Config.html#method.memory_may_move
  void memory_may_move(bool enable) {
    wasmtime_config_memory_may_move_set(ptr.get(), enable);
  }

  /// \brief Configures whether CoW is enabled.
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.Config.html#method.memory_init_cow
  void memory_init_cow(bool enable) {
    wasmtime_config_memory_init_cow_set(ptr.get(), enable);
  }

  /// \brief Configures whether native unwind information is emitted.
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.Config.html#method.native_unwind_info
  void native_unwind_info(bool enable) {
    wasmtime_config_native_unwind_info_set(ptr.get(), enable);
  }

  /// \brief Configures whether mach ports are used on macOS
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.Config.html#method.macos_use_mach_ports
  void macos_use_mach_ports(bool enable) {
    wasmtime_config_macos_use_mach_ports_set(ptr.get(), enable);
  }

#ifdef WASMTIME_FEATURE_CACHE
  /// \brief Loads the default cache configuration present on the system.
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.Config.html#method.cache_config_load_default
  Result<std::monostate> cache_load_default() {
    auto *error = wasmtime_config_cache_config_load(ptr.get(), nullptr);
    if (error != nullptr) {
      return Error(error);
    }
    return std::monostate();
  }

  /// \brief Loads cache configuration from the specified filename.
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.Config.html#method.cache_config_load
  Result<std::monostate> cache_load(const std::string &path) {
    auto *error = wasmtime_config_cache_config_load(ptr.get(), path.c_str());
    if (error != nullptr) {
      return Error(error);
    }
    return std::monostate();
  }
#endif // WASMTIME_FEATURE_CACHE

private:
  template <typename T> static void raw_finalize(void *env) {
    std::unique_ptr<T> ptr(reinterpret_cast<T *>(env));
  }

  template <typename M>
  static uint8_t *raw_get_memory(void *env, size_t *byte_size,
                                 size_t *byte_capacity) {
    M *memory = reinterpret_cast<M *>(env);
    return memory->get_memory(byte_size, byte_capacity);
  }

  template <typename M>
  static wasmtime_error_t *raw_grow_memory(void *env, size_t new_size) {
    M *memory = reinterpret_cast<M *>(env);
    Result<std::monostate> result = memory->grow_memory(new_size);
    if (!result)
      return result.err().release();
    return nullptr;
  }

  template <typename T>
  static wasmtime_error_t *
  raw_new_memory(void *env, const wasm_memorytype_t *ty, size_t minimum,
                 size_t maximum, size_t reserved_size_in_bytes,
                 size_t guard_size_in_bytes,
                 wasmtime_linear_memory_t *memory_ret) {
    using Memory = typename T::Memory;
    T *creator = reinterpret_cast<T *>(env);
    Result<Memory> result =
        creator->new_memory(MemoryType::Ref(ty), minimum, maximum,
                            reserved_size_in_bytes, guard_size_in_bytes);
    if (!result) {
      return result.err().release();
    }
    Memory memory = result.unwrap();
    memory_ret->env = std::make_unique<Memory>(memory).release();
    memory_ret->finalizer = raw_finalize<Memory>;
    memory_ret->get_memory = raw_get_memory<Memory>;
    memory_ret->grow_memory = raw_grow_memory<Memory>;
    return nullptr;
  }

public:
  /// \brief Configures a custom memory creator for this configuration and
  /// eventual Engine.
  ///
  /// This can be used to use `creator` to allocate linear memories for the
  /// engine that this configuration will be used for.
  template <typename T> void host_memory_creator(T creator) {
    wasmtime_memory_creator_t config = {0};
    config.env = std::make_unique<T>(creator).release();
    config.finalizer = raw_finalize<T>;
    config.new_memory = raw_new_memory<T>;
    wasmtime_config_host_memory_creator_set(ptr.get(), &config);
  }

#ifdef WASMTIME_FEATURE_POOLING_ALLOCATOR
  /// \brief Enables and configures the pooling allocation strategy.
  ///
  /// https://docs.wasmtime.dev/api/wasmtime/struct.Config.html#method.allocation_strategy
  void pooling_allocation_strategy(const PoolAllocationConfig &config) {
    wasmtime_pooling_allocation_strategy_set(ptr.get(), config.ptr.get());
  }
#endif // WASMTIME_FEATURE_POOLING_ALLOCATOR
};

} // namespace wasmtime

#endif // WASMTIME_CONFIG_HH
