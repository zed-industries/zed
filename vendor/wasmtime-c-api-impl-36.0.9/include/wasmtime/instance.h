/**
 * \file wasmtime/instance.h
 *
 * Wasmtime APIs for interacting with wasm instances.
 */

#ifndef WASMTIME_INSTANCE_H
#define WASMTIME_INSTANCE_H

#include <wasm.h>
#include <wasmtime/extern.h>
#include <wasmtime/module.h>
#include <wasmtime/store.h>

#ifdef __cplusplus
extern "C" {
#endif

/// \brief Representation of a instance in Wasmtime.
///
/// Instances are represented with a 64-bit identifying integer in Wasmtime.
/// They do not have any destructor associated with them. Instances cannot
/// interoperate between #wasmtime_store_t instances and if the wrong instance
/// is passed to the wrong store then it may trigger an assertion to abort the
/// process.
typedef struct wasmtime_instance {
  /// Internal identifier of what store this belongs to, never zero.
  uint64_t store_id;
  /// Private data for use in Wasmtime.
  size_t __private;
} wasmtime_instance_t;

/**
 * \brief Instantiate a wasm module.
 *
 * This function will instantiate a WebAssembly module with the provided
 * imports, creating a WebAssembly instance. The returned instance can then
 * afterwards be inspected for exports.
 *
 * \param store the store in which to create the instance
 * \param module the module that's being instantiated
 * \param imports the imports provided to the module
 * \param nimports the size of `imports`
 * \param instance where to store the returned instance
 * \param trap where to store the returned trap
 *
 * This function requires that `imports` is the same size as the imports that
 * `module` has. Additionally the `imports` array must be 1:1 lined up with the
 * imports of the `module` specified. This is intended to be relatively low
 * level, and #wasmtime_linker_instantiate is provided for a more ergonomic
 * name-based resolution API.
 *
 * The states of return values from this function are similar to
 * #wasmtime_func_call where an error can be returned meaning something like a
 * link error in this context. A trap can be returned (meaning no error or
 * instance is returned), or an instance can be returned (meaning no error or
 * trap is returned).
 *
 * Note that this function requires that all `imports` specified must be owned
 * by the `store` provided as well.
 *
 * This function does not take ownership of any of its arguments, but all return
 * values are owned by the caller.
 */
WASM_API_EXTERN wasmtime_error_t *
wasmtime_instance_new(wasmtime_context_t *store,
                      const wasmtime_module_t *module,
                      const wasmtime_extern_t *imports, size_t nimports,
                      wasmtime_instance_t *instance, wasm_trap_t **trap);

/**
 * \brief Get an export by name from an instance.
 *
 * \param store the store that owns `instance`
 * \param instance the instance to lookup within
 * \param name the export name to lookup
 * \param name_len the byte length of `name`
 * \param item where to store the returned value
 *
 * Returns nonzero if the export was found, and `item` is filled in. Otherwise
 * returns 0.
 *
 * Doesn't take ownership of any arguments but does return ownership of the
 * #wasmtime_extern_t.
 */
WASM_API_EXTERN bool wasmtime_instance_export_get(
    wasmtime_context_t *store, const wasmtime_instance_t *instance,
    const char *name, size_t name_len, wasmtime_extern_t *item);

/**
 * \brief Get an export by index from an instance.
 *
 * \param store the store that owns `instance`
 * \param instance the instance to lookup within
 * \param index the index to lookup
 * \param name where to store the name of the export
 * \param name_len where to store the byte length of the name
 * \param item where to store the export itself
 *
 * Returns nonzero if the export was found, and `name`, `name_len`, and `item`
 * are filled in. Otherwise returns 0.
 *
 * Doesn't take ownership of any arguments but does return ownership of the
 * #wasmtime_extern_t. The `name` pointer return value is owned by the `store`
 * and must be immediately used before calling any other APIs on
 * #wasmtime_context_t.
 */
WASM_API_EXTERN bool wasmtime_instance_export_nth(
    wasmtime_context_t *store, const wasmtime_instance_t *instance,
    size_t index, char **name, size_t *name_len, wasmtime_extern_t *item);

/**
 * \brief A #wasmtime_instance_t, pre-instantiation, that is ready to be
 * instantiated.
 *
 * Must be deleted using #wasmtime_instance_pre_delete.
 *
 * For more information see the Rust documentation:
 * https://docs.wasmtime.dev/api/wasmtime/struct.InstancePre.html
 */
typedef struct wasmtime_instance_pre wasmtime_instance_pre_t;

/**
 * \brief Delete a previously created wasmtime_instance_pre_t.
 */
WASM_API_EXTERN void
wasmtime_instance_pre_delete(wasmtime_instance_pre_t *instance_pre);

/**
 * \brief Instantiates instance within the given store.
 *
 * This will also run the function's startup function, if there is one.
 *
 * For more information on instantiation see #wasmtime_instance_new.
 *
 * \param instance_pre the pre-initialized instance
 * \param store the store in which to create the instance
 * \param instance where to store the returned instance
 * \param trap_ptr where to store the returned trap
 *
 * \return One of three things can happen as a result of this function. First
 * the module could be successfully instantiated and returned through
 * `instance`, meaning the return value and `trap` are both set to `NULL`.
 * Second the start function may trap, meaning the return value and `instance`
 * are set to `NULL` and `trap` describes the trap that happens. Finally
 * instantiation may fail for another reason, in which case an error is returned
 * and `trap` and `instance` are set to `NULL`.
 *
 * This function does not take ownership of any of its arguments, and all return
 * values are owned by the caller.
 */
WASM_API_EXTERN wasmtime_error_t *wasmtime_instance_pre_instantiate(
    const wasmtime_instance_pre_t *instance_pre, wasmtime_context_t *store,
    wasmtime_instance_t *instance, wasm_trap_t **trap_ptr);

/**
 * \brief Get the module (as a shallow clone) for a instance_pre.
 *
 * The returned module is owned by the caller and the caller **must**
 * delete it via `wasmtime_module_delete`.
 */
WASM_API_EXTERN wasmtime_module_t *
wasmtime_instance_pre_module(wasmtime_instance_pre_t *instance_pre);

#ifdef __cplusplus
} // extern "C"
#endif

#endif // WASMTIME_INSTANCE_H
