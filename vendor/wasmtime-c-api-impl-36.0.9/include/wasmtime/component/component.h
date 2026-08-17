/// \file wasmtime/component/component.h

#ifndef WASMTIME_COMPONENT_COMPONENT_H
#define WASMTIME_COMPONENT_COMPONENT_H

#include <wasm.h>
#include <wasmtime/conf.h>
#include <wasmtime/error.h>

#ifdef WASMTIME_FEATURE_COMPONENT_MODEL

#ifdef __cplusplus
extern "C" {
#endif

/// Representation of a component in the component model.
typedef struct wasmtime_component_t wasmtime_component_t;

#ifdef WASMTIME_FEATURE_COMPILER

/**
 * \brief Compiles a WebAssembly binary into a #wasmtime_component_t
 *
 * This function will compile a WebAssembly binary into an owned
 #wasmtime_component_t.
 *
 * It requires a component binary, such as what is produced by Rust `cargo
 component` tooling.
 *
 * This function does not take ownership of any of its arguments, but the
 * returned error and component are owned by the caller.

 * \param engine the #wasm_engine_t that will create the component
 * \param buf the address of the buffer containing the WebAssembly binary
 * \param len the length of the buffer containing the WebAssembly binary
 * \param component_out on success, contains the address of the created
 *        component
 *
 * \return NULL on success, else a #wasmtime_error_t describing the error
 */
WASM_API_EXTERN wasmtime_error_t *
wasmtime_component_new(const wasm_engine_t *engine, const uint8_t *buf,
                       size_t len, wasmtime_component_t **component_out);

/**
 * \brief This function serializes compiled component artifacts as blob data.
 *
 * \param component the component
 * \param ret if the conversion is successful, this byte vector is filled in
 * with the serialized compiled component.
 *
 * \return a non-null error if parsing fails, or returns `NULL`. If parsing
 * fails then `ret` isn't touched.
 *
 * This function does not take ownership of `component`, and the caller is
 * expected to deallocate the returned #wasmtime_error_t and #wasm_byte_vec_t.
 */
WASM_API_EXTERN wasmtime_error_t *
wasmtime_component_serialize(const wasmtime_component_t *component,
                             wasm_byte_vec_t *ret);

#endif // WASMTIME_FEATURE_COMPILER

/**
 * \brief Build a component from serialized data.
 *
 * This function does not take ownership of any of its arguments, but the
 * returned error and component are owned by the caller.
 *
 * This function is not safe to receive arbitrary user input. See the Rust
 * documentation for more information on what inputs are safe to pass in here
 * (e.g. only that of `wasmtime_component_serialize`)
 */
WASM_API_EXTERN wasmtime_error_t *
wasmtime_component_deserialize(const wasm_engine_t *engine, const uint8_t *buf,
                               size_t len,
                               wasmtime_component_t **component_out);

/**
 * \brief Deserialize a component from an on-disk file.
 *
 * This function is the same as #wasmtime_component_deserialize except that it
 * reads the data for the serialized component from the path on disk. This can
 * be faster than the alternative which may require copying the data around.
 *
 * This function does not take ownership of any of its arguments, but the
 * returned error and component are owned by the caller.
 *
 * This function is not safe to receive arbitrary user input. See the Rust
 * documentation for more information on what inputs are safe to pass in here
 * (e.g. only that of `wasmtime_component_serialize`)
 */
WASM_API_EXTERN wasmtime_error_t *
wasmtime_component_deserialize_file(const wasm_engine_t *engine,
                                    const char *path,
                                    wasmtime_component_t **component_out);

/**
 * \brief Creates a shallow clone of the specified component, increasing the
 * internal reference count.
 */
WASM_API_EXTERN wasmtime_component_t *
wasmtime_component_clone(const wasmtime_component_t *component);

/**
 * \brief Deletes a #wasmtime_component_t created by #wasmtime_component_new
 *
 * \param component the component to delete
 */
WASM_API_EXTERN void wasmtime_component_delete(wasmtime_component_t *component);

/// A value which represents a known export of a component.
typedef struct wasmtime_component_export_index_t
    wasmtime_component_export_index_t;

/**
 * \brief Looks up a specific export of this component by \p name optionally
 * nested within the \p instance provided.
 *
 * \param component the component to look up \p name in
 * \param instance_export_index optional (i.e. nullable) instance to look up in
 * \param name the name of the export
 * \param name_len length of \p name in bytes
 * \return export index if found, else NULL
 */
WASM_API_EXTERN wasmtime_component_export_index_t *
wasmtime_component_get_export_index(
    const wasmtime_component_t *component,
    const wasmtime_component_export_index_t *instance_export_index,
    const char *name, size_t name_len);

/**
 * \brief Deletes a #wasmtime_component_export_index_t
 *
 * \param export_index the export index to delete
 */
WASM_API_EXTERN void wasmtime_component_export_index_delete(
    wasmtime_component_export_index_t *export_index);

#ifdef __cplusplus
} // extern "C"
#endif

#endif // WASMTIME_FEATURE_COMPONENT_MODEL

#endif // WASMTIME_COMPONENT_COMPONENT_H
