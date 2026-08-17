/// \file wasmtime/component/instance.h

#ifndef WASMTIME_COMPONENT_INSTANCE_H
#define WASMTIME_COMPONENT_INSTANCE_H

#include <wasmtime/component/component.h>
#include <wasmtime/component/func.h>
#include <wasmtime/conf.h>
#include <wasmtime/store.h>

#ifdef WASMTIME_FEATURE_COMPONENT_MODEL

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
typedef struct wasmtime_component_instance {
  /// Internal identifier of what store this belongs to, never zero.
  uint64_t store_id;
  /// Internal index within the store.
  uint32_t __private;
} wasmtime_component_instance_t;

/**
 * \brief A methods similar to #wasmtime_component_get_export_index except for
 * this instance.
 *
 * \param instance the instance to look up \p name in
 * \param context the context where \p instance lives in
 * \param instance_export_index optional (i.e. nullable) instance to look up in
 * \param name the name of the export
 * \param name_len length of \p name in bytes
 * \return export index if found, else NULL
 */
WASM_API_EXTERN wasmtime_component_export_index_t *
wasmtime_component_instance_get_export_index(
    const wasmtime_component_instance_t *instance, wasmtime_context_t *context,
    const wasmtime_component_export_index_t *instance_export_index,
    const char *name, size_t name_len);

/**
 * \brief Looks up an exported function by name within this
 * #wasmtime_component_instance_t.
 *
 * \param instance the instance to look up this name in
 * \param context the store that \p instance lives in
 * \param export_index the export index of the function
 * \param func_out if found, the function corresponding to \p name
 * \return boolean marking if a function for \p name was found
 */
WASM_API_EXTERN bool wasmtime_component_instance_get_func(
    const wasmtime_component_instance_t *instance, wasmtime_context_t *context,
    const wasmtime_component_export_index_t *export_index,
    wasmtime_component_func_t *func_out);

#ifdef __cplusplus
} // extern "C"
#endif

#endif // WASMTIME_FEATURE_COMPONENT_MODEL

#endif // WASMTIME_COMPONENT_INSTANCE_H
