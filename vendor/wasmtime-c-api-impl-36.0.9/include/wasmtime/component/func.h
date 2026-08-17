/// \file wasmtime/component/func.h

#ifndef WASMTIME_COMPONENT_FUNC_H
#define WASMTIME_COMPONENT_FUNC_H

#include <wasmtime/component/val.h>
#include <wasmtime/conf.h>
#include <wasmtime/error.h>
#include <wasmtime/store.h>

#ifdef WASMTIME_FEATURE_COMPONENT_MODEL

#ifdef __cplusplus
extern "C" {
#endif

/// \brief Representation of a function in Wasmtime.
///
/// Functions in Wasmtime are represented as an index into a store and don't
/// have any data or destructor associated with the value. Functions cannot
/// interoperate between #wasmtime_store_t instances and if the wrong function
/// is passed to the wrong store then it may trigger an assertion to abort the
/// process.
typedef struct wasmtime_component_func {
  struct {
    /// Internal identifier of what store this belongs to, never zero.
    uint64_t store_id;
    /// Private internal wasmtime information.
    uint32_t __private1;
  };

  /// Private internal wasmtime information.
  uint32_t __private2;
} wasmtime_component_func_t;

/**
 * \brief Invokes \p func with the \p args given and returns the result.
 *
 * The \p args provided must match the parameters that this function takes in
 * terms of their types and the number of parameters. Results will be written to
 * the \p results provided if the call completes successfully. The initial types
 * of the values in \p results are ignored and values are overwritten to write
 * the result. It's required that the \p results_size exactly matches the number
 * of results that this function produces.
 */
WASM_API_EXTERN wasmtime_error_t *wasmtime_component_func_call(
    const wasmtime_component_func_t *func, wasmtime_context_t *context,
    const wasmtime_component_val_t *args, size_t args_size,
    wasmtime_component_val_t *results, size_t results_size);

/**
 * \brief Invokes the `post-return` canonical ABI option, if specified, after a
 * #wasmtime_component_func_call has finished.
 *
 * This function is a required method call after a #wasmtime_component_func_call
 * completes successfully. After the embedder has finished processing the return
 * value then this function must be invoked.
 */
WASM_API_EXTERN wasmtime_error_t *
wasmtime_component_func_post_return(const wasmtime_component_func_t *func,
                                    wasmtime_context_t *context);

#ifdef __cplusplus
} // extern "C"
#endif

#endif // WASMTIME_FEATURE_COMPONENT_MODEL

#endif // WASMTIME_COMPONENT_FUNC_H
