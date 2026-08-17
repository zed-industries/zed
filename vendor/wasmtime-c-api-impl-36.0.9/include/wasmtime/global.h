/**
 * \file wasmtime/global.h
 *
 * Wasmtime APIs for interacting with WebAssembly globals.
 */

#ifndef WASMTIME_GLOBAL_H
#define WASMTIME_GLOBAL_H

#include <wasm.h>
#include <wasmtime/extern.h>
#include <wasmtime/store.h>
#include <wasmtime/val.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \brief Creates a new global value.
 *
 * Creates a new host-defined global value within the provided `store`
 *
 * \param store the store in which to create the global
 * \param type the wasm type of the global being created
 * \param val the initial value of the global
 * \param ret a return pointer for the created global.
 *
 * This function may return an error if the `val` argument does not match the
 * specified type of the global, or if `val` comes from a different store than
 * the one provided.
 *
 * This function does not take ownership of any of its arguments but error is
 * owned by the caller.
 */
WASM_API_EXTERN wasmtime_error_t *
wasmtime_global_new(wasmtime_context_t *store, const wasm_globaltype_t *type,
                    const wasmtime_val_t *val, wasmtime_global_t *ret);

/**
 * \brief Returns the wasm type of the specified global.
 *
 * The returned #wasm_globaltype_t is owned by the caller.
 */
WASM_API_EXTERN wasm_globaltype_t *
wasmtime_global_type(const wasmtime_context_t *store,
                     const wasmtime_global_t *global);

/**
 * \brief Get the value of the specified global.
 *
 * \param store the store that owns `global`
 * \param global the global to get
 * \param out where to store the value in this global.
 *
 * This function returns ownership of the contents of `out`, so
 * #wasmtime_val_unroot may need to be called on the value.
 */
WASM_API_EXTERN void wasmtime_global_get(wasmtime_context_t *store,
                                         const wasmtime_global_t *global,
                                         wasmtime_val_t *out);

/**
 * \brief Sets a global to a new value.
 *
 * \param store the store that owns `global`
 * \param global the global to set
 * \param val the value to store in the global
 *
 * This function may return an error if `global` is not mutable or if `val` has
 * the wrong type for `global`.
 *
 * THis does not take ownership of any argument but returns ownership of the
 * error.
 */
WASM_API_EXTERN wasmtime_error_t *
wasmtime_global_set(wasmtime_context_t *store, const wasmtime_global_t *global,
                    const wasmtime_val_t *val);

#ifdef __cplusplus
} // extern "C"
#endif

#endif // WASMTIME_GLOBAL_H
