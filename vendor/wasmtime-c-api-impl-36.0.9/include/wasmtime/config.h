/**
 * \file wasmtime/config.h
 *
 * \brief Wasmtime-specific extensions to #wasm_config_t
 */

#ifndef WASMTIME_CONFIG_H
#define WASMTIME_CONFIG_H

#include <wasm.h>
#include <wasmtime/conf.h>
#include <wasmtime/error.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \brief Specifier for how Wasmtime will compile code, values are in
 * #wasmtime_strategy_enum
 */
typedef uint8_t wasmtime_strategy_t;

/**
 * \brief Different ways that Wasmtime can compile WebAssembly
 *
 * The default value is #WASMTIME_STRATEGY_AUTO.
 */
enum wasmtime_strategy_enum { // Strategy
  /// Automatically picks the compilation backend, currently always defaulting
  /// to Cranelift.
  WASMTIME_STRATEGY_AUTO,

  /// Indicates that Wasmtime will unconditionally use Cranelift to compile
  /// WebAssembly code.
  WASMTIME_STRATEGY_CRANELIFT,
};

/**
 * \brief Specifier of what optimization level to use for generated JIT code.
 *
 * See #wasmtime_opt_level_enum for possible values.
 */
typedef uint8_t wasmtime_opt_level_t;

/**
 * \brief Different ways Wasmtime can optimize generated code.
 *
 * The default value is #WASMTIME_OPT_LEVEL_SPEED.
 */
enum wasmtime_opt_level_enum { // OptLevel
  /// Generated code will not be optimized at all.
  WASMTIME_OPT_LEVEL_NONE,
  /// Generated code will be optimized purely for speed.
  WASMTIME_OPT_LEVEL_SPEED,
  /// Generated code will be optimized, but some speed optimizations are
  /// disabled if they cause the generated code to be significantly larger.
  WASMTIME_OPT_LEVEL_SPEED_AND_SIZE,
};

/**
 * \brief Different ways wasmtime can enable profiling JIT code.
 *
 * See #wasmtime_profiling_strategy_enum for possible values.
 */
typedef uint8_t wasmtime_profiling_strategy_t;

/**
 * \brief Different ways to profile JIT code.
 *
 * The default is #WASMTIME_PROFILING_STRATEGY_NONE.
 */
enum wasmtime_profiling_strategy_enum { // ProfilingStrategy
  /// No profiling is enabled at runtime.
  WASMTIME_PROFILING_STRATEGY_NONE,
  /// Linux's "jitdump" support in `perf` is enabled and when Wasmtime is run
  /// under `perf` necessary calls will be made to profile generated JIT code.
  WASMTIME_PROFILING_STRATEGY_JITDUMP,
  /// Support for VTune will be enabled and the VTune runtime will be informed,
  /// at runtime, about JIT code.
  ///
  /// Note that this isn't always enabled at build time.
  WASMTIME_PROFILING_STRATEGY_VTUNE,
  /// Linux's simple "perfmap" support in `perf` is enabled and when Wasmtime is
  /// run under `perf` necessary calls will be made to profile generated JIT
  /// code.
  WASMTIME_PROFILING_STRATEGY_PERFMAP,
};

#define WASMTIME_CONFIG_PROP(ret, name, ty)                                    \
  WASM_API_EXTERN ret wasmtime_config_##name##_set(wasm_config_t *, ty);

/**
 * \brief Configures whether DWARF debug information is constructed at runtime
 * to describe JIT code.
 *
 * This setting is `false` by default. When enabled it will attempt to inform
 * native debuggers about DWARF debugging information for JIT code to more
 * easily debug compiled WebAssembly via native debuggers. This can also
 * sometimes improve the quality of output when profiling is enabled.
 */
WASMTIME_CONFIG_PROP(void, debug_info, bool)

/**
 * \brief Whether or not fuel is enabled for generated code.
 *
 * This setting is `false` by default. When enabled it will enable fuel counting
 * meaning that fuel will be consumed every time a wasm instruction is executed,
 * and trap when reaching zero.
 */
WASMTIME_CONFIG_PROP(void, consume_fuel, bool)

/**
 * \brief Whether or not epoch-based interruption is enabled for generated code.
 *
 * This setting is `false` by default. When enabled wasm code will check the
 * current epoch periodically and abort if the current epoch is beyond a
 * store-configured limit.
 *
 * Note that when this setting is enabled all stores will immediately trap and
 * need to have their epoch deadline otherwise configured with
 * #wasmtime_context_set_epoch_deadline.
 *
 * Note that the current epoch is engine-local and can be incremented with
 * #wasmtime_engine_increment_epoch.
 */
WASMTIME_CONFIG_PROP(void, epoch_interruption, bool)

/**
 * \brief Configures the maximum stack size, in bytes, that JIT code can use.
 *
 * This setting is 2MB by default. Configuring this setting will limit the
 * amount of native stack space that JIT code can use while it is executing. If
 * you're hitting stack overflow you can try making this setting larger, or if
 * you'd like to limit wasm programs to less stack you can also configure this.
 *
 * Note that this setting is not interpreted with 100% precision. Additionally
 * the amount of stack space that wasm takes is always relative to the first
 * invocation of wasm on the stack, so recursive calls with host frames in the
 * middle will all need to fit within this setting.
 */
WASMTIME_CONFIG_PROP(void, max_wasm_stack, size_t)

#ifdef WASMTIME_FEATURE_THREADS

/**
 * \brief Configures whether the WebAssembly threading proposal is enabled.
 *
 * This setting is `false` by default.
 *
 * Note that threads are largely unimplemented in Wasmtime at this time.
 */
WASMTIME_CONFIG_PROP(void, wasm_threads, bool)

#endif // WASMTIME_FEATURE_THREADS

/**
 * \brief Configures whether the WebAssembly tail call proposal is enabled.
 *
 * This setting is `false` by default.
 */
WASMTIME_CONFIG_PROP(void, wasm_tail_call, bool)

/**
 * \brief Configures whether the WebAssembly reference types proposal is
 * enabled.
 *
 * This setting is `false` by default.
 */
WASMTIME_CONFIG_PROP(void, wasm_reference_types, bool)

/**
 * \brief Configures whether the WebAssembly typed function reference types
 * proposal is enabled.
 *
 * This setting is `false` by default.
 */
WASMTIME_CONFIG_PROP(void, wasm_function_references, bool)

/**
 * \brief Configures whether the WebAssembly GC proposal is enabled.
 *
 * This setting is `false` by default.
 */
WASMTIME_CONFIG_PROP(void, wasm_gc, bool)

/**
 * \brief Configures whether the WebAssembly SIMD proposal is
 * enabled.
 *
 * This setting is `false` by default.
 */
WASMTIME_CONFIG_PROP(void, wasm_simd, bool)

/**
 * \brief Configures whether the WebAssembly relaxed SIMD proposal is
 * enabled.
 *
 * This setting is `false` by default.
 */
WASMTIME_CONFIG_PROP(void, wasm_relaxed_simd, bool)

/**
 * \brief Configures whether the WebAssembly relaxed SIMD proposal is
 * in deterministic mode.
 *
 * This setting is `false` by default.
 */
WASMTIME_CONFIG_PROP(void, wasm_relaxed_simd_deterministic, bool)

/**
 * \brief Configures whether the WebAssembly bulk memory proposal is
 * enabled.
 *
 * This setting is `false` by default.
 */
WASMTIME_CONFIG_PROP(void, wasm_bulk_memory, bool)

/**
 * \brief Configures whether the WebAssembly multi value proposal is
 * enabled.
 *
 * This setting is `true` by default.
 */
WASMTIME_CONFIG_PROP(void, wasm_multi_value, bool)

/**
 * \brief Configures whether the WebAssembly multi-memory proposal is
 * enabled.
 *
 * This setting is `false` by default.
 */
WASMTIME_CONFIG_PROP(void, wasm_multi_memory, bool)

/**
 * \brief Configures whether the WebAssembly memory64 proposal is
 * enabled.
 *
 * This setting is `false` by default.
 */
WASMTIME_CONFIG_PROP(void, wasm_memory64, bool)

/**
 * \brief Configures whether the WebAssembly wide-arithmetic proposal is
 * enabled.
 *
 * This setting is `false` by default.
 */
WASMTIME_CONFIG_PROP(void, wasm_wide_arithmetic, bool)

#ifdef WASMTIME_FEATURE_COMPILER

/**
 * \brief Configures whether the WebAssembly stack switching
 * proposal is enabled.
 *
 * This setting is `false` by default.
 */
WASMTIME_CONFIG_PROP(void, wasm_stack_switching, bool)

/**
 * \brief Configures how JIT code will be compiled.
 *
 * This setting is #WASMTIME_STRATEGY_AUTO by default.
 */
WASMTIME_CONFIG_PROP(void, strategy, wasmtime_strategy_t)

#endif // WASMTIME_FEATURE_COMPILER

#ifdef WASMTIME_FEATURE_PARALLEL_COMPILATION

/**
 * \brief Configure whether wasmtime should compile a module using multiple
 * threads.
 *
 * For more information see the Rust documentation at
 * https://docs.wasmtime.dev/api/wasmtime/struct.Config.html#method.parallel_compilation.
 */
WASMTIME_CONFIG_PROP(void, parallel_compilation, bool)

#endif // WASMTIME_FEATURE_PARALLEL_COMPILATION

#ifdef WASMTIME_FEATURE_COMPILER

/**
 * \brief Configures whether Cranelift's debug verifier is enabled.
 *
 * This setting in `false` by default.
 *
 * When cranelift is used for compilation this enables expensive debug checks
 * within Cranelift itself to verify it's correct.
 */
WASMTIME_CONFIG_PROP(void, cranelift_debug_verifier, bool)

/**
 * \brief Configures whether Cranelift should perform a NaN-canonicalization
 * pass.
 *
 * When Cranelift is used as a code generation backend this will configure
 * it to replace NaNs with a single canonical value. This is useful for users
 * requiring entirely deterministic WebAssembly computation.
 *
 * This is not required by the WebAssembly spec, so it is not enabled by
 * default.
 *
 * The default value for this is `false`
 */
WASMTIME_CONFIG_PROP(void, cranelift_nan_canonicalization, bool)

/**
 * \brief Configures Cranelift's optimization level for JIT code.
 *
 * This setting in #WASMTIME_OPT_LEVEL_SPEED by default.
 */
WASMTIME_CONFIG_PROP(void, cranelift_opt_level, wasmtime_opt_level_t)

#endif // WASMTIME_FEATURE_COMPILER

/**
 * \brief Configures the profiling strategy used for JIT code.
 *
 * This setting in #WASMTIME_PROFILING_STRATEGY_NONE by default.
 */
WASMTIME_CONFIG_PROP(void, profiler, wasmtime_profiling_strategy_t)

/**
 * \brief Configures whether `memory_reservation` is the maximal size of linear
 * memory.
 *
 * This setting is `false` by default.
 *
 * For more information see the Rust documentation at
 * https://bytecodealliance.github.io/wasmtime/api/wasmtime/struct.Config.html#method.memory_may_move.
 */
WASMTIME_CONFIG_PROP(void, memory_may_move, bool)

/**
 * \brief Configures the size, in bytes, of initial memory reservation size for
 * linear memories.
 *
 * For more information see the Rust documentation at
 * https://bytecodealliance.github.io/wasmtime/api/wasmtime/struct.Config.html#method.memory_reservation.
 */
WASMTIME_CONFIG_PROP(void, memory_reservation, uint64_t)

/**
 * \brief Configures the guard region size for linear memory.
 *
 * For more information see the Rust documentation at
 * https://bytecodealliance.github.io/wasmtime/api/wasmtime/struct.Config.html#method.memory_guard_size.
 */
WASMTIME_CONFIG_PROP(void, memory_guard_size, uint64_t)

/**
 * \brief Configures the size, in bytes, of the extra virtual memory space
 * reserved for memories to grow into after being relocated.
 *
 * For more information see the Rust documentation at
 * https://docs.wasmtime.dev/api/wasmtime/struct.Config.html#method.memory_reservation_for_growth
 */
WASMTIME_CONFIG_PROP(void, memory_reservation_for_growth, uint64_t)

/**
 * \brief Configures whether to generate native unwind information (e.g.
 * .eh_frame on Linux).
 *
 * This option defaults to true.
 *
 * For more information see the Rust documentation at
 * https://docs.wasmtime.dev/api/wasmtime/struct.Config.html#method.native_unwind_info
 */
WASMTIME_CONFIG_PROP(void, native_unwind_info, bool)

#ifdef WASMTIME_FEATURE_CACHE

/**
 * \brief Enables Wasmtime's cache and loads configuration from the specified
 * path.
 *
 * By default the Wasmtime compilation cache is disabled. The configuration path
 * here can be `NULL` to use the default settings, and otherwise the argument
 * here must be a file on the filesystem with TOML configuration -
 * https://bytecodealliance.github.io/wasmtime/cli-cache.html.
 *
 * An error is returned if the cache configuration could not be loaded or if the
 * cache could not be enabled.
 */
WASM_API_EXTERN wasmtime_error_t *
wasmtime_config_cache_config_load(wasm_config_t *, const char *);

#endif // WASMTIME_FEATURE_CACHE

/**
 * \brief Configures the target triple that this configuration will produce
 * machine code for.
 *
 * This option defaults to the native host. Calling this method will
 * additionally disable inference of the native features of the host (e.g.
 * detection of SSE4.2 on x86_64 hosts). Native features can be reenabled with
 * the `cranelift_flag_{set,enable}` properties.
 *
 * For more information see the Rust documentation at
 * https://docs.wasmtime.dev/api/wasmtime/struct.Config.html#method.config
 */
WASMTIME_CONFIG_PROP(wasmtime_error_t *, target, const char *)

#ifdef WASMTIME_FEATURE_COMPILER

/**
 * \brief Enables a target-specific flag in Cranelift.
 *
 * This can be used, for example, to enable SSE4.2 on x86_64 hosts. Settings can
 * be explored with `wasmtime settings` on the CLI.
 *
 * For more information see the Rust documentation at
 * https://docs.wasmtime.dev/api/wasmtime/struct.Config.html#method.cranelift_flag_enable
 */
WASM_API_EXTERN void wasmtime_config_cranelift_flag_enable(wasm_config_t *,
                                                           const char *);

/**
 * \brief Sets a target-specific flag in Cranelift to the specified value.
 *
 * This can be used, for example, to enable SSE4.2 on x86_64 hosts. Settings can
 * be explored with `wasmtime settings` on the CLI.
 *
 * For more information see the Rust documentation at
 * https://docs.wasmtime.dev/api/wasmtime/struct.Config.html#method.cranelift_flag_set
 */
WASM_API_EXTERN void wasmtime_config_cranelift_flag_set(wasm_config_t *,
                                                        const char *key,
                                                        const char *value);

#endif // WASMTIME_FEATURE_COMPILER

/**
 * \brief Configures whether, when on macOS, Mach ports are used for exception
 * handling instead of traditional Unix-based signal handling.
 *
 * This option defaults to true, using Mach ports by default.
 *
 * For more information see the Rust documentation at
 * https://docs.wasmtime.dev/api/wasmtime/struct.Config.html#method.macos_use_mach_ports
 */
WASMTIME_CONFIG_PROP(void, macos_use_mach_ports, bool)

/**
 * Return the data from a LinearMemory instance.
 *
 * The size in bytes as well as the maximum number of bytes that can be
 * allocated should be returned as well.
 *
 * For more information about see the Rust documentation at
 * https://docs.wasmtime.dev/api/wasmtime/trait.LinearMemory.html
 */
typedef uint8_t *(*wasmtime_memory_get_callback_t)(void *env, size_t *byte_size,
                                                   size_t *byte_capacity);

/**
 * Grow the memory to the `new_size` in bytes.
 *
 * For more information about the parameters see the Rust documentation at
 * https://docs.wasmtime.dev/api/wasmtime/trait.LinearMemory.html#tymethod.grow_to
 */
typedef wasmtime_error_t *(*wasmtime_memory_grow_callback_t)(void *env,
                                                             size_t new_size);

/**
 * A LinearMemory instance created from a #wasmtime_new_memory_callback_t.
 *
 * For more information see the Rust documentation at
 * https://docs.wasmtime.dev/api/wasmtime/trait.LinearMemory.html
 */
typedef struct wasmtime_linear_memory {
  /// User provided value to be passed to get_memory and grow_memory
  void *env;
  /// Callback to get the memory and size of this LinearMemory
  wasmtime_memory_get_callback_t get_memory;
  /// Callback to request growing the memory
  wasmtime_memory_grow_callback_t grow_memory;
  /// An optional finalizer for env
  void (*finalizer)(void *);
} wasmtime_linear_memory_t;

/**
 * A callback to create a new LinearMemory from the specified parameters.
 *
 * The result should be written to `memory_ret` and wasmtime will own the values
 * written into that struct.
 *
 * This callback must be thread-safe.
 *
 * For more information about the parameters see the Rust documentation at
 * https://docs.wasmtime.dev/api/wasmtime/trait.MemoryCreator.html#tymethod.new_memory
 */
typedef wasmtime_error_t *(*wasmtime_new_memory_callback_t)(
    void *env, const wasm_memorytype_t *ty, size_t minimum, size_t maximum,
    size_t reserved_size_in_bytes, size_t guard_size_in_bytes,
    wasmtime_linear_memory_t *memory_ret);

/**
 * A representation of custom memory creator and methods for an instance of
 * LinearMemory.
 *
 * For more information see the Rust documentation at
 * https://docs.wasmtime.dev/api/wasmtime/trait.MemoryCreator.html
 */
typedef struct wasmtime_memory_creator {
  /// User provided value to be passed to new_memory
  void *env;
  /// The callback to create new memory, must be thread safe
  wasmtime_new_memory_callback_t new_memory;
  /// An optional finalizer for env.
  void (*finalizer)(void *);
} wasmtime_memory_creator_t;

/**
 * Sets a custom memory creator.
 *
 * Custom memory creators are used when creating host Memory objects or when
 * creating instance linear memories for the on-demand instance allocation
 * strategy.
 *
 * The config does **not** take ownership of the #wasmtime_memory_creator_t
 * passed in, but instead copies all the values in the struct.
 *
 * For more information see the Rust documentation at
 * https://docs.wasmtime.dev/api/wasmtime/struct.Config.html#method.with_host_memory
 */
WASM_API_EXTERN void
wasmtime_config_host_memory_creator_set(wasm_config_t *,
                                        wasmtime_memory_creator_t *);

/**
 * \brief Configures whether copy-on-write memory-mapped data is used to
 * initialize a linear memory.
 *
 * Initializing linear memory via a copy-on-write mapping can drastically
 * improve instantiation costs of a WebAssembly module because copying memory is
 * deferred. Additionally if a page of memory is only ever read from WebAssembly
 * and never written too then the same underlying page of data will be reused
 * between all instantiations of a module meaning that if a module is
 * instantiated many times this can lower the overall memory required needed to
 * run that module.
 *
 * This option defaults to true.
 *
 * For more information see the Rust documentation at
 * https://docs.wasmtime.dev/api/wasmtime/struct.Config.html#method.memory_init_cow
 */
WASMTIME_CONFIG_PROP(void, memory_init_cow, bool)

#ifdef WASMTIME_FEATURE_POOLING_ALLOCATOR

/**
 * \brief A type containing configuration options for the pooling allocator.
 *
 * For more information see the Rust documentation at
 * https://docs.wasmtime.dev/api/wasmtime/struct.PoolingAllocationConfig.html
 */
typedef struct wasmtime_pooling_allocation_config_t
    wasmtime_pooling_allocation_config_t;

/**
 * \brief Creates a new pooling allocation configuration object. Must be
 * deallocated with a call to wasmtime_pooling_allocation_config_delete.
 */
WASM_API_EXTERN wasmtime_pooling_allocation_config_t *
wasmtime_pooling_allocation_config_new();

/**
 * \brief Deallocates a pooling allocation configuration object created with a
 * call to wasmtime_pooling_allocation_config_new.
 */
WASM_API_EXTERN void wasmtime_pooling_allocation_config_delete(
    wasmtime_pooling_allocation_config_t *);

#define WASMTIME_POOLING_ALLOCATION_CONFIG_PROP(name, ty)                      \
  WASM_API_EXTERN void wasmtime_pooling_allocation_config_##name##_set(        \
      wasmtime_pooling_allocation_config_t *, ty);

/**
 * \brief Configures the maximum number of “unused warm slots” to retain in the
 * pooling allocator.
 *
 * For more information see the Rust documentation at
 * https://docs.wasmtime.dev/api/wasmtime/struct.PoolingAllocationConfig.html#method.max_unused_warm_slots.
 */
WASMTIME_POOLING_ALLOCATION_CONFIG_PROP(max_unused_warm_slots, uint32_t)

/**
 * \brief The target number of decommits to do per batch.
 *
 * For more information see the Rust documentation at
 * https://docs.wasmtime.dev/api/wasmtime/struct.PoolingAllocationConfig.html#method.decommit_batch_size.
 */
WASMTIME_POOLING_ALLOCATION_CONFIG_PROP(decommit_batch_size, size_t)

#ifdef WASMTIME_FEATURE_ASYNC
/**
 * \brief How much memory, in bytes, to keep resident for async stacks allocated
 * with the pooling allocator.
 *
 * For more information see the Rust documentation at
 * https://docs.wasmtime.dev/api/wasmtime/struct.PoolingAllocationConfig.html#method.async_stack_keep_resident.
 */
WASMTIME_POOLING_ALLOCATION_CONFIG_PROP(async_stack_keep_resident, size_t)
#endif

/**
 * \brief How much memory, in bytes, to keep resident for each linear memory
 * after deallocation.
 *
 * For more information see the Rust documentation at
 * https://docs.wasmtime.dev/api/wasmtime/struct.PoolingAllocationConfig.html#method.linear_memory_keep_resident.
 */
WASMTIME_POOLING_ALLOCATION_CONFIG_PROP(linear_memory_keep_resident, size_t)

/**
 * \brief How much memory, in bytes, to keep resident for each table after
 * deallocation.
 *
 * For more information see the Rust documentation at
 * https://docs.wasmtime.dev/api/wasmtime/struct.PoolingAllocationConfig.html#method.table_keep_resident.
 */
WASMTIME_POOLING_ALLOCATION_CONFIG_PROP(table_keep_resident, size_t)

/**
 * \brief The maximum number of concurrent component instances supported
 * (default is 1000).
 *
 * For more information see the Rust documentation at
 * https://docs.wasmtime.dev/api/wasmtime/struct.PoolingAllocationConfig.html#method.total_component_instances.
 */
WASMTIME_POOLING_ALLOCATION_CONFIG_PROP(total_component_instances, uint32_t)

/**
 * \brief The maximum size, in bytes, allocated for a component instance’s
 * VMComponentContext metadata.
 *
 * For more information see the Rust documentation at
 * https://docs.wasmtime.dev/api/wasmtime/struct.PoolingAllocationConfig.html#method.max_component_instance_size.
 */
WASMTIME_POOLING_ALLOCATION_CONFIG_PROP(max_component_instance_size, size_t)

/**
 * \brief The maximum number of core instances a single component may contain
 * (default is unlimited).
 *
 * For more information see the Rust documentation at
 * https://docs.wasmtime.dev/api/wasmtime/struct.PoolingAllocationConfig.html#method.max_core_instances_per_component.
 */
WASMTIME_POOLING_ALLOCATION_CONFIG_PROP(max_core_instances_per_component,
                                        uint32_t)

/**
 * \brief The maximum number of Wasm linear memories that a single component may
 * transitively contain (default is unlimited).
 *
 * For more information see the Rust documentation at
 * https://docs.wasmtime.dev/api/wasmtime/struct.PoolingAllocationConfig.html#method.max_memories_per_component.
 */
WASMTIME_POOLING_ALLOCATION_CONFIG_PROP(max_memories_per_component, uint32_t)

/**
 * \brief The maximum number of tables that a single component may transitively
 * contain (default is unlimited).
 *
 * For more information see the Rust documentation at
 * https://docs.wasmtime.dev/api/wasmtime/struct.PoolingAllocationConfig.html#method.max_tables_per_component.
 */
WASMTIME_POOLING_ALLOCATION_CONFIG_PROP(max_tables_per_component, uint32_t)

/**
 * \brief The maximum number of concurrent Wasm linear memories supported
 * (default is 1000).
 *
 * For more information see the Rust documentation at
 * https://docs.wasmtime.dev/api/wasmtime/struct.PoolingAllocationConfig.html#method.total_memories.
 */
WASMTIME_POOLING_ALLOCATION_CONFIG_PROP(total_memories, uint32_t)

/**
 * \brief The maximum number of concurrent tables supported (default is 1000).
 *
 * For more information see the Rust documentation at
 * https://docs.wasmtime.dev/api/wasmtime/struct.PoolingAllocationConfig.html#method.total_tables.
 */
WASMTIME_POOLING_ALLOCATION_CONFIG_PROP(total_tables, uint32_t)

#ifdef WASMTIME_FEATURE_ASYNC
/**
 * \brief The maximum number of execution stacks allowed for asynchronous
 * execution, when enabled (default is 1000).
 *
 * For more information see the Rust documentation at
 * https://docs.wasmtime.dev/api/wasmtime/struct.PoolingAllocationConfig.html#method.total_stacks.
 */
WASMTIME_POOLING_ALLOCATION_CONFIG_PROP(total_stacks, uint32_t)
#endif

/**
 * \brief The maximum number of concurrent core instances supported (default is
 * 1000).
 *
 * For more information see the Rust documentation at
 * https://docs.wasmtime.dev/api/wasmtime/struct.PoolingAllocationConfig.html#method.total_core_instances.
 */
WASMTIME_POOLING_ALLOCATION_CONFIG_PROP(total_core_instances, uint32_t)

/**
 * \brief The maximum size, in bytes, allocated for a core instance’s VMContext
 * metadata.
 *
 * For more information see the Rust documentation at
 * https://docs.wasmtime.dev/api/wasmtime/struct.PoolingAllocationConfig.html#method.max_core_instance_size.
 */
WASMTIME_POOLING_ALLOCATION_CONFIG_PROP(max_core_instance_size, size_t)

/**
 * \brief The maximum number of defined tables for a core module (default is 1).
 *
 * For more information see the Rust documentation at
 * https://docs.wasmtime.dev/api/wasmtime/struct.PoolingAllocationConfig.html#method.max_tables_per_module.
 */
WASMTIME_POOLING_ALLOCATION_CONFIG_PROP(max_tables_per_module, uint32_t)

/**
 * \brief The maximum table elements for any table defined in a module (default
 * is 20000).
 *
 * For more information see the Rust documentation at
 * https://docs.wasmtime.dev/api/wasmtime/struct.PoolingAllocationConfig.html#method.table_elements.
 */
WASMTIME_POOLING_ALLOCATION_CONFIG_PROP(table_elements, size_t)

/**
 * \brief The maximum number of defined linear memories for a module (default is
 * 1).
 *
 * For more information see the Rust documentation at
 * https://docs.wasmtime.dev/api/wasmtime/struct.PoolingAllocationConfig.html#method.max_memories_per_module.
 */
WASMTIME_POOLING_ALLOCATION_CONFIG_PROP(max_memories_per_module, uint32_t)

/**
 * \brief The maximum byte size that any WebAssembly linear memory may grow to.
 *
 * For more information see the Rust documentation at
 * https://docs.wasmtime.dev/api/wasmtime/struct.PoolingAllocationConfig.html#method.max_memory_size.
 */
WASMTIME_POOLING_ALLOCATION_CONFIG_PROP(max_memory_size, size_t)

/**
 * \brief The maximum number of concurrent GC heaps supported (default is 1000).
 *
 * For more information see the Rust documentation at
 * https://docs.wasmtime.dev/api/wasmtime/struct.PoolingAllocationConfig.html#method.total_gc_heaps.
 */
WASMTIME_POOLING_ALLOCATION_CONFIG_PROP(total_gc_heaps, uint32_t)

/**
 * \brief Sets the Wasmtime allocation strategy to use the pooling allocator. It
 * does not take ownership of the pooling allocation configuration object, which
 * must be deleted with a call to wasmtime_pooling_allocation_config_delete.
 *
 * For more information see the Rust documentation at
 * https://docs.wasmtime.dev/api/wasmtime/struct.Config.html#method.allocation_strategy.
 */
WASM_API_EXTERN void wasmtime_pooling_allocation_strategy_set(
    wasm_config_t *, const wasmtime_pooling_allocation_config_t *);

#endif // WASMTIME_FEATURE_POOLING_ALLOCATOR

#ifdef WASMTIME_FEATURE_COMPONENT_MODEL

/**
 * \brief Configures whether the WebAssembly component-model proposal will be
 * enabled for compilation.
 *
 * For more information see the Rust documentation at
 * https://docs.wasmtime.dev/api/wasmtime/struct.Config.html#method.wasm_component_model.
 */
WASMTIME_CONFIG_PROP(void, wasm_component_model, bool)

#endif // WASMTIME_FEATURE_COMPONENT_MODEL

#ifdef __cplusplus
} // extern "C"
#endif

#endif // WASMTIME_CONFIG_H
