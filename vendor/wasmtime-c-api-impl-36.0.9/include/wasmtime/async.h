/**
 * \file wasmtime/async.h
 *
 * \brief Wasmtime async functionality
 *
 * Async functionality in Wasmtime is well documented here:
 * https://docs.wasmtime.dev/api/wasmtime/struct.Config.html#method.async_support
 *
 * All WebAssembly executes synchronously, but an async support enables the Wasm
 * code be executed on a separate stack, so it can be paused and resumed. There
 * are three mechanisms for yielding control from wasm to the caller: fuel,
 * epochs, and async host functions.
 *
 * When WebAssembly is executed, a `wasmtime_call_future_t` is returned. This
 * struct represents the state of the execution and each call to
 * `wasmtime_call_future_poll` will execute the WebAssembly code on a separate
 * stack until the function returns or yields control back to the caller.
 *
 * It's expected these futures are pulled in a loop until completed, at which
 * point the future should be deleted. Functions that return a
 * `wasmtime_call_future_t` are special in that all parameters to that function
 * should not be modified in any way and must be kept alive until the future is
 * deleted. This includes concurrent calls for a single store - another function
 * on a store should not be called while there is a `wasmtime_call_future_t`
 * alive.
 *
 * As for asynchronous host calls - the reverse contract is upheld. Wasmtime
 * will keep all parameters to the function alive and unmodified until the
 * `wasmtime_func_async_continuation_callback_t` returns true.
 *
 */

#ifndef WASMTIME_ASYNC_H
#define WASMTIME_ASYNC_H

#include <wasm.h>
#include <wasmtime/conf.h>
#include <wasmtime/config.h>
#include <wasmtime/error.h>
#include <wasmtime/func.h>
#include <wasmtime/linker.h>
#include <wasmtime/store.h>

#ifdef WASMTIME_FEATURE_ASYNC

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \brief Whether or not to enable support for asynchronous functions in
 * Wasmtime.
 *
 * When enabled, the config can optionally define host functions with async.
 * Instances created and functions called with this Config must be called
 * through their asynchronous APIs, however. For example using
 * wasmtime_func_call will panic when used with this config.
 *
 * For more information see the Rust documentation at
 * https://docs.wasmtime.dev/api/wasmtime/struct.Config.html#method.async_support
 */
WASMTIME_CONFIG_PROP(void, async_support, bool)

/**
 * \brief Configures the size of the stacks used for asynchronous execution.
 *
 * This setting configures the size of the stacks that are allocated for
 * asynchronous execution.
 *
 * The value cannot be less than max_wasm_stack.
 *
 * The amount of stack space guaranteed for host functions is async_stack_size -
 * max_wasm_stack, so take care not to set these two values close to one
 * another; doing so may cause host functions to overflow the stack and abort
 * the process.
 *
 * By default this option is 2 MiB.
 *
 * For more information see the Rust documentation at
 * https://docs.wasmtime.dev/api/wasmtime/struct.Config.html#method.async_stack_size
 */
WASMTIME_CONFIG_PROP(void, async_stack_size, uint64_t)

/**
 * \brief Configures a Store to yield execution of async WebAssembly code
 * periodically.
 *
 * When a Store is configured to consume fuel with
 * `wasmtime_config_consume_fuel` this method will configure what happens when
 * fuel runs out. Specifically executing WebAssembly will be suspended and
 * control will be yielded back to the caller.
 *
 * This is only suitable with use of a store associated with an async config
 * because only then are futures used and yields are possible.
 *
 * \param context the context for the store to configure.
 * \param interval the amount of fuel at which to yield. A value of 0 will
 *        disable yielding.
 */
WASM_API_EXTERN wasmtime_error_t *
wasmtime_context_fuel_async_yield_interval(wasmtime_context_t *context,
                                           uint64_t interval);

/**
 * \brief Configures epoch-deadline expiration to yield to the async caller and
 * the update the deadline.
 *
 * This is only suitable with use of a store associated with an async config
 * because only then are futures used and yields are possible.
 *
 * See the Rust documentation for more:
 * https://docs.wasmtime.dev/api/wasmtime/struct.Store.html#method.epoch_deadline_async_yield_and_update
 */
WASM_API_EXTERN wasmtime_error_t *
wasmtime_context_epoch_deadline_async_yield_and_update(
    wasmtime_context_t *context, uint64_t delta);

/**
 * The callback to determine a continuation's current state.
 *
 * Return true if the host call has completed, otherwise false will
 * continue to yield WebAssembly execution.
 */
typedef bool (*wasmtime_func_async_continuation_callback_t)(void *env);

/**
 * A continuation for the current state of the host function's execution.
 */
typedef struct wasmtime_async_continuation_t {
  /// Callback for if the async function has completed.
  wasmtime_func_async_continuation_callback_t callback;
  /// User-provided argument to pass to the callback.
  void *env;
  /// A finalizer for the user-provided *env
  void (*finalizer)(void *);
} wasmtime_async_continuation_t;

/**
 * \brief Callback signature for #wasmtime_linker_define_async_func.
 *
 * This is a host function that returns a continuation to be called later.
 *
 * All the arguments to this function will be kept alive until the continuation
 * returns that it has errored or has completed.
 *
 * \param env user-provided argument passed to
 *        #wasmtime_linker_define_async_func
 * \param caller a temporary object that can only be used during this function
 *        call. Used to acquire #wasmtime_context_t or caller's state
 * \param args the arguments provided to this function invocation
 * \param nargs how many arguments are provided
 * \param results where to write the results of this function
 * \param nresults how many results must be produced
 * \param trap_ret if assigned a not `NULL` value then the called
 *        function will trap with the returned error. Note that ownership of
 *        trap is transferred to wasmtime.
 * \param continuation_ret the returned continuation
 *        that determines when the async function has completed executing.
 *
 * Only supported for async stores.
 *
 * See #wasmtime_func_callback_t for more information.
 */
typedef void (*wasmtime_func_async_callback_t)(
    void *env, wasmtime_caller_t *caller, const wasmtime_val_t *args,
    size_t nargs, wasmtime_val_t *results, size_t nresults,
    wasm_trap_t **trap_ret, wasmtime_async_continuation_t *continuation_ret);

/**
 * \brief The structure representing a asynchronously running function.
 *
 * This structure is always owned by the caller and must be deleted using
 * #wasmtime_call_future_delete.
 *
 * Functions that return this type require that the parameters to the function
 * are unmodified until this future is destroyed.
 */
typedef struct wasmtime_call_future wasmtime_call_future_t;

/**
 * \brief Executes WebAssembly in the function.
 *
 * Returns true if the function call has completed. After this function returns
 * true, it should *not* be called again for a given future.
 *
 * This function returns false if execution has yielded either due to being out
 * of fuel (see wasmtime_context_fuel_async_yield_interval), or the epoch has
 * been incremented enough (see
 * wasmtime_context_epoch_deadline_async_yield_and_update). The function may
 * also return false if asynchronous host functions have been called, which then
 * calling this  function will call the continuation from the async host
 * function.
 *
 * For more see the information at
 * https://docs.wasmtime.dev/api/wasmtime/struct.Config.html#asynchronous-wasm
 *
 */
WASM_API_EXTERN bool wasmtime_call_future_poll(wasmtime_call_future_t *future);

/**
 * /brief Frees the underlying memory for a future.
 *
 * All wasmtime_call_future_t are owned by the caller and should be deleted
 * using this function.
 */
WASM_API_EXTERN void
wasmtime_call_future_delete(wasmtime_call_future_t *future);

/**
 * \brief Invokes this function with the params given, returning the results
 * asynchronously.
 *
 * This function is the same as wasmtime_func_call except that it is
 * asynchronous. This is only compatible with stores associated with an
 * asynchronous config.
 *
 * The result is a future that is owned by the caller and must be deleted via
 * #wasmtime_call_future_delete.
 *
 * The `args` and `results` pointers may be `NULL` if the corresponding length
 * is zero. The `trap_ret` and `error_ret` pointers may *not* be `NULL`.
 *
 * Does not take ownership of #wasmtime_val_t arguments or #wasmtime_val_t
 * results, and all parameters to this function must be kept alive and not
 * modified until the returned #wasmtime_call_future_t is deleted. This includes
 * the context and store parameters. Only a single future can be alive for a
 * given store at a single time (meaning only call this function after the
 * previous call's future was deleted).
 *
 * See the header documentation for for more information.
 *
 * For more information see the Rust documentation at
 * https://docs.wasmtime.dev/api/wasmtime/struct.Func.html#method.call_async
 */
WASM_API_EXTERN wasmtime_call_future_t *wasmtime_func_call_async(
    wasmtime_context_t *context, const wasmtime_func_t *func,
    const wasmtime_val_t *args, size_t nargs, wasmtime_val_t *results,
    size_t nresults, wasm_trap_t **trap_ret, wasmtime_error_t **error_ret);

/**
 * \brief Defines a new async function in this linker.
 *
 * This function behaves similar to #wasmtime_linker_define_func, except it
 * supports async callbacks.
 *
 * The callback `cb` will be invoked on another stack (fiber for Windows).
 */
WASM_API_EXTERN wasmtime_error_t *wasmtime_linker_define_async_func(
    wasmtime_linker_t *linker, const char *module, size_t module_len,
    const char *name, size_t name_len, const wasm_functype_t *ty,
    wasmtime_func_async_callback_t cb, void *data, void (*finalizer)(void *));

/**
 * \brief Instantiates a #wasm_module_t with the items defined in this linker
 * for an async store.
 *
 * This is the same as #wasmtime_linker_instantiate but used for async stores
 * (which requires functions are called asynchronously). The returning
 * #wasmtime_call_future_t must be polled using #wasmtime_call_future_poll, and
 * is owned and must be deleted using #wasmtime_call_future_delete.
 *
 * The `trap_ret` and `error_ret` pointers may *not* be `NULL` and the returned
 * memory is owned by the caller.
 *
 * All arguments to this function must outlive the returned future and be
 * unmodified until the future is deleted.
 */
WASM_API_EXTERN wasmtime_call_future_t *wasmtime_linker_instantiate_async(
    const wasmtime_linker_t *linker, wasmtime_context_t *store,
    const wasmtime_module_t *module, wasmtime_instance_t *instance,
    wasm_trap_t **trap_ret, wasmtime_error_t **error_ret);

/**
 * \brief Instantiates instance within the given store.
 *
 * This will also run the function's startup function, if there is one.
 *
 * For more information on async instantiation see
 * #wasmtime_linker_instantiate_async.
 *
 * \param instance_pre the pre-initialized instance
 * \param store the store in which to create the instance
 * \param instance where to store the returned instance
 * \param trap_ret where to store the returned trap
 * \param error_ret where to store the returned trap
 *
 * The `trap_ret` and `error_ret` pointers may *not* be `NULL` and the returned
 * memory is owned by the caller.
 *
 * All arguments to this function must outlive the returned future and be
 * unmodified until the future is deleted.
 */
WASM_API_EXTERN wasmtime_call_future_t *wasmtime_instance_pre_instantiate_async(
    const wasmtime_instance_pre_t *instance_pre, wasmtime_context_t *store,
    wasmtime_instance_t *instance, wasm_trap_t **trap_ret,
    wasmtime_error_t **error_ret);

/**
 * A callback to get the top of the stack address and the length of the stack,
 * excluding guard pages.
 *
 * For more information about the parameters see the Rust documentation at
 * https://docs.wasmtime.dev/api/wasmtime/trait.StackMemory.html
 */
typedef uint8_t *(*wasmtime_stack_memory_get_callback_t)(void *env,
                                                         size_t *out_len);

/**
 * A Stack instance created from a #wasmtime_new_stack_memory_callback_t.
 *
 * For more information see the Rust documentation at
 * https://docs.wasmtime.dev/api/wasmtime/trait.StackMemory.html
 */
typedef struct {
  /// User provided value to be passed to get_memory and grow_memory
  void *env;
  /// Callback to get the memory and size of this LinearMemory
  wasmtime_stack_memory_get_callback_t get_stack_memory;
  /// An optional finalizer for env
  void (*finalizer)(void *);
} wasmtime_stack_memory_t;

/**
 * A callback to create a new StackMemory from the specified parameters.
 *
 * The result should be written to `stack_ret` and wasmtime will own the values
 * written into that struct.
 *
 * This callback must be thread-safe.
 *
 * For more information about the parameters see the Rust documentation at
 * https://docs.wasmtime.dev/api/wasmtime/trait.StackCreator.html#tymethod.new_stack
 */
typedef wasmtime_error_t *(*wasmtime_new_stack_memory_callback_t)(
    void *env, size_t size, bool zeroed, wasmtime_stack_memory_t *stack_ret);

/**
 * A representation of custom stack creator.
 *
 * For more information see the Rust documentation at
 * https://docs.wasmtime.dev/api/wasmtime/trait.StackCreator.html
 */
typedef struct {
  /// User provided value to be passed to new_stack
  void *env;
  /// The callback to create a new stack, must be thread safe
  wasmtime_new_stack_memory_callback_t new_stack;
  /// An optional finalizer for env.
  void (*finalizer)(void *);
} wasmtime_stack_creator_t;

/**
 * Sets a custom stack creator.
 *
 * Custom memory creators are used when creating creating async instance stacks
 * for the on-demand instance allocation strategy.
 *
 * The config does **not** take ownership of the #wasmtime_stack_creator_t
 * passed in, but instead copies all the values in the struct.
 *
 * For more information see the Rust documentation at
 * https://docs.wasmtime.dev/api/wasmtime/struct.Config.html#method.with_host_stack
 */
WASM_API_EXTERN void
wasmtime_config_host_stack_creator_set(wasm_config_t *,
                                       wasmtime_stack_creator_t *);

#ifdef __cplusplus
} // extern "C"
#endif

#endif // WASMTIME_FEATURE_ASYNC

#endif // WASMTIME_ASYNC_H
