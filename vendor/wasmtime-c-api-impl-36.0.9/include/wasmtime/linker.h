/**
 * \file wasmtime/linker.h
 *
 * Wasmtime API for a name-based linker used to instantiate modules.
 */

#ifndef WASMTIME_LINKER_H
#define WASMTIME_LINKER_H

#include <wasm.h>
#include <wasmtime/conf.h>
#include <wasmtime/error.h>
#include <wasmtime/extern.h>
#include <wasmtime/func.h>
#include <wasmtime/instance.h>
#include <wasmtime/store.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \typedef wasmtime_linker_t
 * \brief Alias to #wasmtime_linker
 *
 * \struct #wasmtime_linker
 * \brief Object used to conveniently link together and instantiate wasm
 * modules.
 *
 * This type corresponds to the `wasmtime::Linker` type in Rust. This
 * type is intended to make it easier to manage a set of modules that link
 * together, or to make it easier to link WebAssembly modules to WASI.
 *
 * A #wasmtime_linker_t is a higher level way to instantiate a module than
 * #wasm_instance_new since it works at the "string" level of imports rather
 * than requiring 1:1 mappings.
 */
typedef struct wasmtime_linker wasmtime_linker_t;

/**
 * \brief Creates a new linker for the specified engine.
 *
 * This function does not take ownership of the engine argument, and the caller
 * is expected to delete the returned linker.
 */
WASM_API_EXTERN wasmtime_linker_t *wasmtime_linker_new(wasm_engine_t *engine);

/**
 * \brief Clones existing linker.
 *
 * This function does not take ownership of the linker argument, and the caller
 * is expected to delete the returned linker.
 */
WASM_API_EXTERN wasmtime_linker_t *
wasmtime_linker_clone(wasmtime_linker_t *linker);

/**
 * \brief Deletes a linker
 */
WASM_API_EXTERN void wasmtime_linker_delete(wasmtime_linker_t *linker);

/**
 * \brief Configures whether this linker allows later definitions to shadow
 * previous definitions.
 *
 * By default this setting is `false`.
 */
WASM_API_EXTERN void wasmtime_linker_allow_shadowing(wasmtime_linker_t *linker,
                                                     bool allow_shadowing);

/**
 * \brief Defines a new item in this linker.
 *
 * \param linker the linker the name is being defined in.
 * \param store the store that the `item` is owned by.
 * \param module the module name the item is defined under.
 * \param module_len the byte length of `module`
 * \param name the field name the item is defined under
 * \param name_len the byte length of `name`
 * \param item the item that is being defined in this linker.
 *
 * \return On success `NULL` is returned, otherwise an error is returned which
 * describes why the definition failed.
 *
 * For more information about name resolution consult the [Rust
 * documentation](https://bytecodealliance.github.io/wasmtime/api/wasmtime/struct.Linker.html#name-resolution).
 */
WASM_API_EXTERN wasmtime_error_t *
wasmtime_linker_define(wasmtime_linker_t *linker, wasmtime_context_t *store,
                       const char *module, size_t module_len, const char *name,
                       size_t name_len, const wasmtime_extern_t *item);

/**
 * \brief Defines a new function in this linker.
 *
 * \param linker the linker the name is being defined in.
 * \param module the module name the item is defined under.
 * \param module_len the byte length of `module`
 * \param name the field name the item is defined under
 * \param name_len the byte length of `name`
 * \param ty the type of the function that's being defined
 * \param cb the host callback to invoke when the function is called
 * \param data the host-provided data to provide as the first argument to the
 *        callback
 * \param finalizer an optional finalizer for the `data` argument.
 *
 * \return On success `NULL` is returned, otherwise an error is returned which
 * describes why the definition failed.
 *
 * For more information about name resolution consult the [Rust
 * documentation](https://bytecodealliance.github.io/wasmtime/api/wasmtime/struct.Linker.html#name-resolution).
 *
 * Note that this function does not create a #wasmtime_func_t. This creates a
 * store-independent function within the linker, allowing this function
 * definition to be used with multiple stores.
 *
 * For more information about host callbacks see #wasmtime_func_new.
 */
WASM_API_EXTERN wasmtime_error_t *wasmtime_linker_define_func(
    wasmtime_linker_t *linker, const char *module, size_t module_len,
    const char *name, size_t name_len, const wasm_functype_t *ty,
    wasmtime_func_callback_t cb, void *data, void (*finalizer)(void *));

/**
 * \brief Defines a new function in this linker.
 *
 * This is the same as #wasmtime_linker_define_func except that it's the analog
 * of #wasmtime_func_new_unchecked instead of #wasmtime_func_new. Be sure to
 * consult the documentation of #wasmtime_linker_define_func for argument
 * information as well as #wasmtime_func_new_unchecked for why this is an
 * unsafe API.
 */
WASM_API_EXTERN wasmtime_error_t *wasmtime_linker_define_func_unchecked(
    wasmtime_linker_t *linker, const char *module, size_t module_len,
    const char *name, size_t name_len, const wasm_functype_t *ty,
    wasmtime_func_unchecked_callback_t cb, void *data,
    void (*finalizer)(void *));

#ifdef WASMTIME_FEATURE_WASI

/**
 * \brief Defines WASI functions in this linker.
 *
 * \param linker the linker the name is being defined in.
 *
 * \return On success `NULL` is returned, otherwise an error is returned which
 * describes why the definition failed.
 *
 * This function will provide WASI function names in the specified linker. Note
 * that when an instance is created within a store then the store also needs to
 * have its WASI settings configured with #wasmtime_context_set_wasi for WASI
 * functions to work, otherwise an assert will be tripped that will abort the
 * process.
 *
 * For more information about name resolution consult the [Rust
 * documentation](https://bytecodealliance.github.io/wasmtime/api/wasmtime/struct.Linker.html#name-resolution).
 */
WASM_API_EXTERN wasmtime_error_t *
wasmtime_linker_define_wasi(wasmtime_linker_t *linker);

#endif // WASMTIME_FEATURE_WASI

/**
 * \brief Defines an instance under the specified name in this linker.
 *
 * \param linker the linker the name is being defined in.
 * \param store the store that owns `instance`
 * \param name the module name to define `instance` under.
 * \param name_len the byte length of `name`
 * \param instance a previously-created instance.
 *
 * \return On success `NULL` is returned, otherwise an error is returned which
 * describes why the definition failed.
 *
 * This function will take all of the exports of the `instance` provided and
 * defined them under a module called `name` with a field name as the export's
 * own name.
 *
 * For more information about name resolution consult the [Rust
 * documentation](https://bytecodealliance.github.io/wasmtime/api/wasmtime/struct.Linker.html#name-resolution).
 */
WASM_API_EXTERN wasmtime_error_t *wasmtime_linker_define_instance(
    wasmtime_linker_t *linker, wasmtime_context_t *store, const char *name,
    size_t name_len, const wasmtime_instance_t *instance);

/**
 * \brief Instantiates a #wasm_module_t with the items defined in this linker.
 *
 * \param linker the linker used to instantiate the provided module.
 * \param store the store that is used to instantiate within
 * \param module the module that is being instantiated.
 * \param instance the returned instance, if successful.
 * \param trap a trap returned, if the start function traps.
 *
 * \return One of three things can happen as a result of this function. First
 * the module could be successfully instantiated and returned through
 * `instance`, meaning the return value and `trap` are both set to `NULL`.
 * Second the start function may trap, meaning the return value and `instance`
 * are set to `NULL` and `trap` describes the trap that happens. Finally
 * instantiation may fail for another reason, in which case an error is returned
 * and `trap` and `instance` are set to `NULL`.
 *
 * This function will attempt to satisfy all of the imports of the `module`
 * provided with items previously defined in this linker. If any name isn't
 * defined in the linker than an error is returned. (or if the previously
 * defined item is of the wrong type).
 */
WASM_API_EXTERN wasmtime_error_t *
wasmtime_linker_instantiate(const wasmtime_linker_t *linker,
                            wasmtime_context_t *store,
                            const wasmtime_module_t *module,
                            wasmtime_instance_t *instance, wasm_trap_t **trap);

/**
 * \brief Defines automatic instantiations of a #wasm_module_t in this linker.
 *
 * \param linker the linker the module is being added to
 * \param store the store that is used to instantiate `module`
 * \param name the name of the module within the linker
 * \param name_len the byte length of `name`
 * \param module the module that's being instantiated
 *
 * \return An error if the module could not be instantiated or added or `NULL`
 * on success.
 *
 * This function automatically handles [Commands and
 * Reactors](https://github.com/WebAssembly/WASI/blob/master/design/application-abi.md#current-unstable-abi)
 * instantiation and initialization.
 *
 * For more information see the [Rust
 * documentation](https://bytecodealliance.github.io/wasmtime/api/wasmtime/struct.Linker.html#method.module).
 */
WASM_API_EXTERN wasmtime_error_t *
wasmtime_linker_module(wasmtime_linker_t *linker, wasmtime_context_t *store,
                       const char *name, size_t name_len,
                       const wasmtime_module_t *module);

/**
 * \brief Acquires the "default export" of the named module in this linker.
 *
 * \param linker the linker to load from
 * \param store the store to load a function into
 * \param name the name of the module to get the default export for
 * \param name_len the byte length of `name`
 * \param func where to store the extracted default function.
 *
 * \return An error is returned if the default export could not be found, or
 * `NULL` is returned and `func` is filled in otherwise.
 *
 * For more information see the [Rust
 * documentation](https://bytecodealliance.github.io/wasmtime/api/wasmtime/struct.Linker.html#method.get_default).
 */
WASM_API_EXTERN wasmtime_error_t *
wasmtime_linker_get_default(const wasmtime_linker_t *linker,
                            wasmtime_context_t *store, const char *name,
                            size_t name_len, wasmtime_func_t *func);

/**
 * \brief Loads an item by name from this linker.
 *
 * \param linker the linker to load from
 * \param store the store to load the item into
 * \param module the name of the module to get
 * \param module_len the byte length of `module`
 * \param name the name of the field to get
 * \param name_len the byte length of `name`
 * \param item where to store the extracted item
 *
 * \return A nonzero value if the item is defined, in which case `item` is also
 * filled in. Otherwise zero is returned.
 */
WASM_API_EXTERN bool wasmtime_linker_get(const wasmtime_linker_t *linker,
                                         wasmtime_context_t *store,
                                         const char *module, size_t module_len,
                                         const char *name, size_t name_len,
                                         wasmtime_extern_t *item);

/**
 * \brief Perform all the checks for instantiating `module` with the linker,
 *        except that instantiation doesn't actually finish.
 *
 * \param linker the linker used to instantiate the provided module.
 * \param module the module that is being instantiated.
 * \param instance_pre the returned instance_pre, if successful.
 *
 * \return An error or `NULL` if successful.
 *
 * For more information see the Rust documentation at:
 * https://docs.wasmtime.dev/api/wasmtime/struct.Linker.html#method.instantiate_pre
 */
WASM_API_EXTERN wasmtime_error_t *
wasmtime_linker_instantiate_pre(const wasmtime_linker_t *linker,
                                const wasmtime_module_t *module,
                                wasmtime_instance_pre_t **instance_pre);

#ifdef __cplusplus
} // extern "C"
#endif

#endif // WASMTIME_LINKER_H
