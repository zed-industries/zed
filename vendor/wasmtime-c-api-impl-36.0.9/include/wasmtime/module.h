/**
 * \file wasmtime/module.h
 *
 * APIs for interacting with modules in Wasmtime
 */

#ifndef WASMTIME_MODULE_H
#define WASMTIME_MODULE_H

#include <wasm.h>
#include <wasmtime/conf.h>
#include <wasmtime/error.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \typedef wasmtime_module_t
 * \brief Convenience alias for #wasmtime_module
 *
 * \struct wasmtime_module
 * \brief A compiled Wasmtime module.
 *
 * This type represents a compiled WebAssembly module. The compiled module is
 * ready to be instantiated and can be inspected for imports/exports. It is safe
 * to use a module across multiple threads simultaneously.
 */
typedef struct wasmtime_module wasmtime_module_t;

#ifdef WASMTIME_FEATURE_COMPILER

/**
 * \brief Compiles a WebAssembly binary into a #wasmtime_module_t
 *
 * This function will compile a WebAssembly binary into an owned #wasm_module_t.
 * This performs the same as #wasm_module_new except that it returns a
 * #wasmtime_error_t type to get richer error information.
 *
 * On success the returned #wasmtime_error_t is `NULL` and the `ret` pointer is
 * filled in with a #wasm_module_t. On failure the #wasmtime_error_t is
 * non-`NULL` and the `ret` pointer is unmodified.
 *
 * This function does not take ownership of any of its arguments, but the
 * returned error and module are owned by the caller.
 */
WASM_API_EXTERN wasmtime_error_t *wasmtime_module_new(wasm_engine_t *engine,
                                                      const uint8_t *wasm,
                                                      size_t wasm_len,
                                                      wasmtime_module_t **ret);

#endif // WASMTIME_FEATURE_COMPILER

/**
 * \brief Deletes a module.
 */
WASM_API_EXTERN void wasmtime_module_delete(wasmtime_module_t *m);

/**
 * \brief Creates a shallow clone of the specified module, increasing the
 * internal reference count.
 */
WASM_API_EXTERN wasmtime_module_t *wasmtime_module_clone(wasmtime_module_t *m);

/**
 * \brief Same as #wasm_module_imports, but for #wasmtime_module_t.
 */
WASM_API_EXTERN void wasmtime_module_imports(const wasmtime_module_t *module,
                                             wasm_importtype_vec_t *out);

/**
 * \brief Same as #wasm_module_exports, but for #wasmtime_module_t.
 */
WASM_API_EXTERN void wasmtime_module_exports(const wasmtime_module_t *module,
                                             wasm_exporttype_vec_t *out);

#ifdef WASMTIME_FEATURE_COMPILER

/**
 * \brief Validate a WebAssembly binary.
 *
 * This function will validate the provided byte sequence to determine if it is
 * a valid WebAssembly binary within the context of the engine provided.
 *
 * This function does not take ownership of its arguments but the caller is
 * expected to deallocate the returned error if it is non-`NULL`.
 *
 * If the binary validates then `NULL` is returned, otherwise the error returned
 * describes why the binary did not validate.
 */
WASM_API_EXTERN wasmtime_error_t *
wasmtime_module_validate(wasm_engine_t *engine, const uint8_t *wasm,
                         size_t wasm_len);

/**
 * \brief This function serializes compiled module artifacts as blob data.
 *
 * \param module the module
 * \param ret if the conversion is successful, this byte vector is filled in
 * with the serialized compiled module.
 *
 * \return a non-null error if parsing fails, or returns `NULL`. If parsing
 * fails then `ret` isn't touched.
 *
 * This function does not take ownership of `module`, and the caller is
 * expected to deallocate the returned #wasmtime_error_t and #wasm_byte_vec_t.
 */
WASM_API_EXTERN wasmtime_error_t *
wasmtime_module_serialize(wasmtime_module_t *module, wasm_byte_vec_t *ret);

#endif // WASMTIME_FEATURE_COMPILER

/**
 * \brief Build a module from serialized data.
 *
 * This function does not take ownership of any of its arguments, but the
 * returned error and module are owned by the caller.
 *
 * This function is not safe to receive arbitrary user input. See the Rust
 * documentation for more information on what inputs are safe to pass in here
 * (e.g. only that of `wasmtime_module_serialize`)
 */
WASM_API_EXTERN wasmtime_error_t *
wasmtime_module_deserialize(wasm_engine_t *engine, const uint8_t *bytes,
                            size_t bytes_len, wasmtime_module_t **ret);

/**
 * \brief Deserialize a module from an on-disk file.
 *
 * This function is the same as #wasmtime_module_deserialize except that it
 * reads the data for the serialized module from the path on disk. This can be
 * faster than the alternative which may require copying the data around.
 *
 * This function does not take ownership of any of its arguments, but the
 * returned error and module are owned by the caller.
 *
 * This function is not safe to receive arbitrary user input. See the Rust
 * documentation for more information on what inputs are safe to pass in here
 * (e.g. only that of `wasmtime_module_serialize`)
 */
WASM_API_EXTERN wasmtime_error_t *
wasmtime_module_deserialize_file(wasm_engine_t *engine, const char *path,
                                 wasmtime_module_t **ret);

/**
 * \brief Returns the range of bytes in memory where this module’s compilation
 * image resides.
 *
 * The compilation image for a module contains executable code, data, debug
 * information, etc. This is roughly the same as the wasmtime_module_serialize
 * but not the exact same.
 *
 * For more details see:
 * https://docs.wasmtime.dev/api/wasmtime/struct.Module.html#method.image_range
 */
WASM_API_EXTERN void
wasmtime_module_image_range(const wasmtime_module_t *module, void **start,
                            void **end);

#ifdef __cplusplus
} // extern "C"
#endif

#endif // WASMTIME_MODULE_H
