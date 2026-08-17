/**
 * \file wasmtime/engine.h
 *
 * Wasmtime-specific extensions to #wasm_engine_t.
 */

#ifndef WASMTIME_ENGINE_H
#define WASMTIME_ENGINE_H

#include <wasm.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \brief Create a new reference to the same underlying engine.
 *
 * This function clones the reference-counted pointer to the internal object,
 * and must be freed using #wasm_engine_delete.
 */
WASM_API_EXTERN wasm_engine_t *wasmtime_engine_clone(wasm_engine_t *engine);

/**
 * \brief Increments the engine-local epoch variable.
 *
 * This function will increment the engine's current epoch which can be used to
 * force WebAssembly code to trap if the current epoch goes beyond the
 * #wasmtime_store_t configured epoch deadline.
 *
 * This function is safe to call from any thread, and it is also
 * async-signal-safe.
 *
 * See also #wasmtime_config_epoch_interruption_set.
 */
WASM_API_EXTERN void wasmtime_engine_increment_epoch(wasm_engine_t *engine);

/**
 * \brief Returns whether this engine is using the Pulley interpreter to execute
 * WebAssembly code.
 */
WASM_API_EXTERN bool wasmtime_engine_is_pulley(wasm_engine_t *engine);

#ifdef __cplusplus
} // extern "C"
#endif

#endif // WASMTIME_ENGINE_H
