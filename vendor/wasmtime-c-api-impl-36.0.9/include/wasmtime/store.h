/**
 * \file wasmtime/store.h
 *
 * Wasmtime definition of a "store".
 */

#ifndef WASMTIME_STORE_H
#define WASMTIME_STORE_H

#include <wasi.h>
#include <wasm.h>
#include <wasmtime/conf.h>
#include <wasmtime/error.h>
#include <wasmtime/wasip2.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \typedef wasmtime_store_t
 * \brief Convenience alias for #wasmtime_store_t
 *
 * \struct wasmtime_store
 * \brief Storage of WebAssembly objects
 *
 * A store is the unit of isolation between WebAssembly instances in an
 * embedding of Wasmtime. Values in one #wasmtime_store_t cannot flow into
 * another #wasmtime_store_t. Stores are cheap to create and cheap to dispose.
 * It's expected that one-off stores are common in embeddings.
 *
 * Objects stored within a #wasmtime_store_t are referenced with integer handles
 * rather than interior pointers. This means that most APIs require that the
 * store be explicitly passed in, which is done via #wasmtime_context_t. It is
 * safe to move a #wasmtime_store_t to any thread at any time. A store generally
 * cannot be concurrently used, however.
 */
typedef struct wasmtime_store wasmtime_store_t;

/**
 * \typedef wasmtime_context_t
 * \brief Convenience alias for #wasmtime_context
 *
 * \struct wasmtime_context
 * \brief An interior pointer into a #wasmtime_store_t which is used as
 * "context" for many functions.
 *
 * This context pointer is used pervasively throughout Wasmtime's API. This can
 * be acquired from #wasmtime_store_context or #wasmtime_caller_context. The
 * context pointer for a store is the same for the entire lifetime of a store,
 * so it can safely be stored adjacent to a #wasmtime_store_t itself.
 *
 * Usage of a #wasmtime_context_t must not outlive the original
 * #wasmtime_store_t. Additionally #wasmtime_context_t can only be used in
 * situations where it has explicitly been granted access to doing so. For
 * example finalizers cannot use #wasmtime_context_t because they are not given
 * access to it.
 */
typedef struct wasmtime_context wasmtime_context_t;

/**
 * \brief Creates a new store within the specified engine.
 *
 * \param engine the compilation environment with configuration this store is
 * connected to
 * \param data user-provided data to store, can later be acquired with
 * #wasmtime_context_get_data.
 * \param finalizer an optional finalizer for `data`
 *
 * This function creates a fresh store with the provided configuration settings.
 * The returned store must be deleted with #wasmtime_store_delete.
 */
WASM_API_EXTERN wasmtime_store_t *wasmtime_store_new(wasm_engine_t *engine,
                                                     void *data,
                                                     void (*finalizer)(void *));

/**
 * \brief Returns the interior #wasmtime_context_t pointer to this store
 */
WASM_API_EXTERN wasmtime_context_t *
wasmtime_store_context(wasmtime_store_t *store);

/**
 * \brief Provides limits for a store. Used by hosts to limit resource
 * consumption of instances. Use negative value to keep the default value
 * for the limit.
 *
 * \param store store where the limits should be set.
 * \param memory_size the maximum number of bytes a linear memory can grow to.
 * Growing a linear memory beyond this limit will fail. By default,
 * linear memory will not be limited.
 * \param table_elements the maximum number of elements in a table.
 * Growing a table beyond this limit will fail. By default, table elements
 * will not be limited.
 * \param instances the maximum number of instances that can be created
 * for a Store. Module instantiation will fail if this limit is exceeded.
 * This value defaults to 10,000.
 * \param tables the maximum number of tables that can be created for a Store.
 * Module instantiation will fail if this limit is exceeded. This value
 * defaults to 10,000.
 * \param memories the maximum number of linear memories that can be created
 * for a Store. Instantiation will fail with an error if this limit is exceeded.
 * This value defaults to 10,000.
 *
 * Use any negative value for the parameters that should be kept on
 * the default values.
 *
 * Note that the limits are only used to limit the creation/growth of
 * resources in the future, this does not retroactively attempt to apply
 * limits to the store.
 */
WASM_API_EXTERN void wasmtime_store_limiter(wasmtime_store_t *store,
                                            int64_t memory_size,
                                            int64_t table_elements,
                                            int64_t instances, int64_t tables,
                                            int64_t memories);

/**
 * \brief Deletes a store.
 */
WASM_API_EXTERN void wasmtime_store_delete(wasmtime_store_t *store);

/**
 * \brief Returns the user-specified data associated with the specified store
 */
WASM_API_EXTERN void *
wasmtime_context_get_data(const wasmtime_context_t *context);

/**
 * \brief Overwrites the user-specified data associated with this store.
 *
 * Note that this does not execute the original finalizer for the provided data,
 * and the original finalizer will be executed for the provided data when the
 * store is deleted.
 */
WASM_API_EXTERN void wasmtime_context_set_data(wasmtime_context_t *context,
                                               void *data);

/**
 * \brief Perform garbage collection within the given context.
 *
 * Garbage collects `externref`s that are used within this store. Any
 * `externref`s that are discovered to be unreachable by other code or objects
 * will have their finalizers run.
 *
 * The `context` argument must not be NULL.
 */
WASM_API_EXTERN void wasmtime_context_gc(wasmtime_context_t *context);

/**
 * \brief Set fuel to this context's store for wasm to consume while executing.
 *
 * For this method to work fuel consumption must be enabled via
 * #wasmtime_config_consume_fuel_set. By default a store starts with 0 fuel
 * for wasm to execute with (meaning it will immediately trap).
 * This function must be called for the store to have
 * some fuel to allow WebAssembly to execute.
 *
 * Note that when fuel is entirely consumed it will cause wasm to trap.
 *
 * If fuel is not enabled within this store then an error is returned. If fuel
 * is successfully added then NULL is returned.
 */
WASM_API_EXTERN wasmtime_error_t *
wasmtime_context_set_fuel(wasmtime_context_t *store, uint64_t fuel);

/**
 * \brief Returns the amount of fuel remaining in this context's store.
 *
 * If fuel consumption is not enabled via #wasmtime_config_consume_fuel_set
 * then this function will return an error. Otherwise `NULL` is returned and the
 * fuel parameter is filled in with fuel consumed so far.
 *
 * Also note that fuel, if enabled, must be originally configured via
 * #wasmtime_context_set_fuel.
 */
WASM_API_EXTERN wasmtime_error_t *
wasmtime_context_get_fuel(const wasmtime_context_t *context, uint64_t *fuel);

#ifdef WASMTIME_FEATURE_WASI

/**
 * \brief Configures WASI state within the specified store.
 *
 * This function is required if #wasmtime_linker_define_wasi is called. This
 * will configure the WASI state for instances defined within this store to the
 * configuration specified.
 *
 * This function does not take ownership of `context` but it does take ownership
 * of `wasi`. The caller should no longer use `wasi` after calling this function
 * (even if an error is returned).
 */
WASM_API_EXTERN wasmtime_error_t *
wasmtime_context_set_wasi(wasmtime_context_t *context, wasi_config_t *wasi);

#ifdef WASMTIME_FEATURE_COMPONENT_MODEL

/**
 * \brief Set the WASIP2 config for this store.
 *
 * This function is required if #wasmtime_component_linker_add_wasip2 is called.
 *
 * This function takes ownership of \p config. The caller should no longer use
 * \p config after calling this function.
 */
WASM_API_EXTERN void
wasmtime_context_set_wasip2(wasmtime_context_t *context,
                            wasmtime_wasip2_config_t *config);

#endif // WASMTIME_FEATURE_COMPONENT_MODEL

#endif // WASMTIME_FEATURE_WASI

/**
 * \brief Configures the relative deadline at which point WebAssembly code will
 * trap or invoke the callback function.
 *
 * This function configures the store-local epoch deadline after which point
 * WebAssembly code will trap or invoke the callback function.
 *
 * See also #wasmtime_config_epoch_interruption_set and
 * #wasmtime_store_epoch_deadline_callback.
 */
WASM_API_EXTERN void
wasmtime_context_set_epoch_deadline(wasmtime_context_t *context,
                                    uint64_t ticks_beyond_current);

/// \brief An enum for the behavior before extending the epoch deadline.
typedef uint8_t wasmtime_update_deadline_kind_t;
/// \brief Directly continue to updating the deadline and executing WebAssembly.
#define WASMTIME_UPDATE_DEADLINE_CONTINUE 0
/// \brief Yield control (via async support) then update the deadline.
#define WASMTIME_UPDATE_DEADLINE_YIELD 1

/**
 * \brief Configures epoch deadline callback to C function.
 *
 * This function configures a store-local callback function that will be
 * called when the running WebAssembly function has exceeded its epoch
 * deadline. That function can:
 * - return a #wasmtime_error_t to terminate the function
 * - set the delta argument and return NULL to update the
 *   epoch deadline delta and resume function execution.
 * - set the delta argument, update the epoch deadline,
 *   set update_kind to WASMTIME_UPDATE_DEADLINE_YIELD,
 *   and return NULL to yield (via async support) and
 *   resume function execution.
 *
 * To use WASMTIME_UPDATE_DEADLINE_YIELD async support must be enabled
 * for this store.
 *
 * See also #wasmtime_config_epoch_interruption_set and
 * #wasmtime_context_set_epoch_deadline.
 */
WASM_API_EXTERN void wasmtime_store_epoch_deadline_callback(
    wasmtime_store_t *store,
    wasmtime_error_t *(*func)(wasmtime_context_t *context, void *data,
                              uint64_t *epoch_deadline_delta,
                              wasmtime_update_deadline_kind_t *update_kind),
    void *data, void (*finalizer)(void *));

#ifdef __cplusplus
} // extern "C"
#endif

#endif // WASMTIME_STORE_H
