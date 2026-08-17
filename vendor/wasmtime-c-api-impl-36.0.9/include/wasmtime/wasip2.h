/// \file wasmtime/wasip2.h

#ifndef WASMTIME_WASIP2_H
#define WASMTIME_WASIP2_H

#include <wasm.h>
#include <wasmtime/conf.h>

#ifdef WASMTIME_FEATURE_WASI
#ifdef WASMTIME_FEATURE_COMPONENT_MODEL

#ifdef __cplusplus
extern "C" {
#endif

/// Config for the WASIP2 context.
typedef struct wasmtime_wasip2_config_t wasmtime_wasip2_config_t;

/**
 * \brief Create a #wasmtime_wasip2_config_t
 */
WASM_API_EXTERN wasmtime_wasip2_config_t *wasmtime_wasip2_config_new();

/**
 * \brief Configures this context's stdin stream to read the host process's
 * stdin.
 *
 * Note that concurrent reads of stdin can produce surprising results so when
 * using this it's typically best to have a single wasm instance in the process
 * using this.
 */
WASM_API_EXTERN void
wasmtime_wasip2_config_inherit_stdin(wasmtime_wasip2_config_t *config);

/**
 * \brief Configures this context's stdout stream to write to the host process's
 * stdout.
 */
WASM_API_EXTERN void
wasmtime_wasip2_config_inherit_stdout(wasmtime_wasip2_config_t *config);

/**
 * \brief Configures this context's stderr stream to write to the host process's
 * stderr.
 */
WASM_API_EXTERN void
wasmtime_wasip2_config_inherit_stderr(wasmtime_wasip2_config_t *config);

/**
 * \brief Appends a single argument to get passed to wasm.
 */
WASM_API_EXTERN void
wasmtime_wasip2_config_arg(wasmtime_wasip2_config_t *config, const char *arg,
                           size_t arg_len);

/**
 * \brief Delete a #wasmtime_wasip2_config_t
 *
 * \note This is not needed if the config is passed to
 * #wasmtime_component_linker_add_wasip2
 */
WASM_API_EXTERN void
wasmtime_wasip2_config_delete(wasmtime_wasip2_config_t *config);

#ifdef __cplusplus
} // extern "C"
#endif

#endif // WASMTIME_FEATURE_COMPONENT_MODEL
#endif // WASMTIME_FEATURE_WASI

#endif // WASMTIME_WASIP2_H
