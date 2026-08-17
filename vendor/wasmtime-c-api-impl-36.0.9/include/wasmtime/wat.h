/**
 * \file wasmtime/wat.h
 *
 * APIs for converting the text format to binary
 */

#ifndef WASMTIME_WAT_H
#define WASMTIME_WAT_H

#include <wasmtime/conf.h>
#include <wasmtime/error.h>

#ifdef __cplusplus
extern "C" {
#endif

#ifdef WASMTIME_FEATURE_WAT

/**
 * \brief Converts from the text format of WebAssembly to the binary format.
 *
 * \param wat this it the input pointer with the WebAssembly Text Format inside
 *        of it. This will be parsed and converted to the binary format.
 * \param wat_len this it the length of `wat`, in bytes.
 * \param ret if the conversion is successful, this byte vector is filled in
 *        with the WebAssembly binary format.
 *
 * \return a non-null error if parsing fails, or returns `NULL`. If parsing
 * fails then `ret` isn't touched.
 *
 * This function does not take ownership of `wat`, and the caller is expected to
 * deallocate the returned #wasmtime_error_t and #wasm_byte_vec_t.
 */
WASM_API_EXTERN wasmtime_error_t *
wasmtime_wat2wasm(const char *wat, size_t wat_len, wasm_byte_vec_t *ret);

#endif // WASMTIME_FEATURE_WAT

#ifdef __cplusplus
} // extern "C"
#endif

#endif // WASMTIME_WAT_H
