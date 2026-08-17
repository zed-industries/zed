/**
 * \file wasmtime/table.h
 *
 * Wasmtime APIs for interacting with WebAssembly tables.
 */

#ifndef WASMTIME_TABLE_H
#define WASMTIME_TABLE_H

#include <wasm.h>
#include <wasmtime/error.h>
#include <wasmtime/extern.h>
#include <wasmtime/store.h>
#include <wasmtime/val.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \brief Creates a new host-defined wasm table.
 *
 * \param store the store to create the table within
 * \param ty the type of the table to create
 * \param init the initial value for this table's elements
 * \param table where to store the returned table
 *
 * This function does not take ownership of any of its parameters, but yields
 * ownership of returned error. This function may return an error if the `init`
 * value does not match `ty`, for example.
 */
WASM_API_EXTERN wasmtime_error_t *wasmtime_table_new(wasmtime_context_t *store,
                                                     const wasm_tabletype_t *ty,
                                                     const wasmtime_val_t *init,
                                                     wasmtime_table_t *table);

/**
 * \brief Returns the type of this table.
 *
 * The caller has ownership of the returned #wasm_tabletype_t
 */
WASM_API_EXTERN wasm_tabletype_t *
wasmtime_table_type(const wasmtime_context_t *store,
                    const wasmtime_table_t *table);

/**
 * \brief Gets a value in a table.
 *
 * \param store the store that owns `table`
 * \param table the table to access
 * \param index the table index to access
 * \param val where to store the table's value
 *
 * This function will attempt to access a table element. If a nonzero value is
 * returned then `val` is filled in and is owned by the caller. Otherwise zero
 * is returned because the `index` is out-of-bounds.
 */
WASM_API_EXTERN bool wasmtime_table_get(wasmtime_context_t *store,
                                        const wasmtime_table_t *table,
                                        uint64_t index, wasmtime_val_t *val);

/**
 * \brief Sets a value in a table.
 *
 * \param store the store that owns `table`
 * \param table the table to write to
 * \param index the table index to write
 * \param value the value to store.
 *
 * This function will store `value` into the specified index in the table. This
 * does not take ownership of any argument but yields ownership of the error.
 * This function can fail if `value` has the wrong type for the table, or if
 * `index` is out of bounds.
 */
WASM_API_EXTERN wasmtime_error_t *
wasmtime_table_set(wasmtime_context_t *store, const wasmtime_table_t *table,
                   uint64_t index, const wasmtime_val_t *value);

/**
 * \brief Returns the size, in elements, of the specified table
 */
WASM_API_EXTERN uint64_t wasmtime_table_size(const wasmtime_context_t *store,
                                             const wasmtime_table_t *table);

/**
 * \brief Grows a table.
 *
 * \param store the store that owns `table`
 * \param table the table to grow
 * \param delta the number of elements to grow the table by
 * \param init the initial value for new table element slots
 * \param prev_size where to store the previous size of the table before growth
 *
 * This function will attempt to grow the table by `delta` table elements. This
 * can fail if `delta` would exceed the maximum size of the table or if `init`
 * is the wrong type for this table. If growth is successful then `NULL` is
 * returned and `prev_size` is filled in with the previous size of the table, in
 * elements, before the growth happened.
 *
 * This function does not take ownership of any of its arguments.
 */
WASM_API_EXTERN wasmtime_error_t *
wasmtime_table_grow(wasmtime_context_t *store, const wasmtime_table_t *table,
                    uint64_t delta, const wasmtime_val_t *init,
                    uint64_t *prev_size);

#ifdef __cplusplus
} // extern "C"
#endif

#endif // WASMTIME_TABLE_H
