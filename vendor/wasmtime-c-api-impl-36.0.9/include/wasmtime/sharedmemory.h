/**
 * \file wasmtime/sharedmemory.h
 *
 * Wasmtime API for interacting with wasm shared memories.
 */

#ifndef WASMTIME_SHAREDMEMORY_H
#define WASMTIME_SHAREDMEMORY_H

#include <wasm.h>
#include <wasmtime/conf.h>
#include <wasmtime/error.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \brief Interface for shared memories.
 *
 * For more information see the Rust documentation at:
 * https://docs.wasmtime.dev/api/wasmtime/struct.SharedMemory.html
 */
typedef struct wasmtime_sharedmemory wasmtime_sharedmemory_t;

#ifdef WASMTIME_FEATURE_THREADS

/**
 * \brief Creates a new WebAssembly shared linear memory
 *
 * \param engine engine that created shared memory is associated with
 * \param ty the type of the memory to create
 * \param ret where to store the returned memory
 *
 * If an error happens when creating the memory it's returned and owned by the
 * caller. If an error happens then `ret` is not filled in.
 */
WASM_API_EXTERN wasmtime_error_t *
wasmtime_sharedmemory_new(const wasm_engine_t *engine,
                          const wasm_memorytype_t *ty,
                          wasmtime_sharedmemory_t **ret);

#endif // WASMTIME_FEATURE_THREADS

/**
 * \brief Deletes shared linear memory
 *
 * \param memory memory to be deleted
 */
WASM_API_EXTERN void
wasmtime_sharedmemory_delete(wasmtime_sharedmemory_t *memory);

/**
 * \brief Clones shared linear memory
 *
 * \param memory memory to be cloned
 *
 * This function makes shallow clone, ie. copy of reference counted
 * memory handle.
 */
WASM_API_EXTERN wasmtime_sharedmemory_t *
wasmtime_sharedmemory_clone(const wasmtime_sharedmemory_t *memory);

/**
 * \brief Returns the type of the shared memory specified
 */
WASM_API_EXTERN wasm_memorytype_t *
wasmtime_sharedmemory_type(const wasmtime_sharedmemory_t *memory);

/**
 * \brief Returns the base pointer in memory where
          the shared linear memory starts.
 */
WASM_API_EXTERN uint8_t *
wasmtime_sharedmemory_data(const wasmtime_sharedmemory_t *memory);

/**
 * \brief Returns the byte length of this shared linear memory.
 */
WASM_API_EXTERN size_t
wasmtime_sharedmemory_data_size(const wasmtime_sharedmemory_t *memory);

/**
 * \brief Returns the length, in WebAssembly pages, of this shared linear memory
 */
WASM_API_EXTERN uint64_t
wasmtime_sharedmemory_size(const wasmtime_sharedmemory_t *memory);

/**
 * \brief Attempts to grow the specified shared memory by `delta` pages.
 *
 * \param memory the memory to grow
 * \param delta the number of pages to grow by
 * \param prev_size where to store the previous size of memory
 *
 * If memory cannot be grown then `prev_size` is left unchanged and an error is
 * returned. Otherwise `prev_size` is set to the previous size of the memory, in
 * WebAssembly pages, and `NULL` is returned.
 */
WASM_API_EXTERN wasmtime_error_t *
wasmtime_sharedmemory_grow(const wasmtime_sharedmemory_t *memory,
                           uint64_t delta, uint64_t *prev_size);

#ifdef __cplusplus
} // extern "C"
#endif

#endif // WASMTIME_SHAREDMEMORY_H
