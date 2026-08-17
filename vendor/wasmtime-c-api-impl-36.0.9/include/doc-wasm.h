/* clang-format off */

/**
 * \file wasm.h
 *
 * Upstream Embedding API for WebAssembly.
 *
 * This API is defined by the upstream wasm-c-api proposal at
 * https://github.com/WebAssembly/wasm-c-api. That proposal is in flux but
 * Wasmtime intends to be active in its development.
 *
 * The documentation for this header file is currently defined in the Wasmtime
 * project, not in the upstream header file. Some behavior here may be
 * Wasmtime-specific and may not be portable to other engines implementing the
 * same C API. Also note that not all functionality from the upstream C API is
 * implemented in Wasmtime. We strive to provide all symbols as to not generate
 * link errors but some functions are unimplemented and will abort the process
 * if called.
 *
 * ### Memory Management
 *
 * Memory management in the wasm C API is intended to be relatively simple. Each
 * individual object is reference counted unless otherwise noted. You can delete
 * any object at any time after you no longer need it. Deletion of an object
 * does not imply that the memory will be deallocated at that time. If another
 * object still internally references the original object then the memory will
 * still be alive.
 *
 * For example you can delete a #wasm_engine_t after you create a #wasm_store_t
 * with #wasm_store_new. The engine, however, is still referenced by the
 * #wasm_store_t so it will not be deallocated. In essence by calling
 * #wasm_engine_delete you're release your own strong reference on the
 * #wasm_engine_t, but that's it.
 *
 * Additionally APIs like #wasm_memory_copy do not actually copy the underlying
 * data. Instead they only increment the reference count and return a new
 * object. You'll need to still call #wasm_memory_delete (or the corresponding
 * `*_delete` function) for each copy of an object you acquire.
 *
 * ### Vectors
 *
 * This API provides a number of `wasm_*_vec_t` type definitions and functions
 * to work with them. Each vector is defined by a pointer and a length.
 * "Ownership" of a vector refers to the data pointer, not the memory holding
 * the data pointer and the length. It is safe, for example to create a
 * #wasm_name_t on the stack and pass it to #wasm_importtype_new. The memory
 * pointed to by #wasm_name_t must be properly initialized, however, and cannot
 * reside on the stack.
 */

/**
 * \typedef byte_t
 * \brief A type definition for a number that occupies a single byte of data.
 *
 * \typedef wasm_byte_t
 * \brief A type definition for a number that occupies a single byte of data.
 *
 * \typedef float32_t
 * \brief A type definition for a 32-bit float.
 *
 * \typedef float64_t
 * \brief A type definition for a 64-bit float.
 *
 * \typedef wasm_name_t
 * \brief Convenience for hinting that an argument only accepts utf-8 input.
 */

/**
 * \typedef wasm_config_t
 * \brief Convenience alias for #wasm_config_t
 *
 * \struct wasm_config_t
 * \brief Global engine configuration
 *
 * This structure represents global configuration used when constructing a
 * #wasm_engine_t. There are now functions to modify this from wasm.h but the
 * wasmtime/config.h header provides a number of Wasmtime-specific functions to
 * tweak configuration options.
 *
 * This object is created with #wasm_config_new.
 *
 * Configuration is safe to share between threads. Typically you'll create a
 * config object and immediately pass it into #wasm_engine_new_with_config,
 * however.
 *
 * For more information about configuration see the Rust documentation as well
 * at
 * https://bytecodealliance.github.io/wasmtime/api/wasmtime/struct.Config.html.
 *
 * \fn wasm_config_t *wasm_config_new(void);
 * \brief Creates a new empty configuration object.
 *
 * The object returned is owned by the caller and will need to be deleted with
 * #wasm_config_delete. May return `NULL` if a configuration object could not be
 * allocated.
 *
 * \fn void wasm_config_delete(wasm_config_t*);
 * \brief Deletes a configuration object.
 */

/**
 * \typedef wasm_engine_t
 * \brief Convenience alias for #wasm_engine_t
 *
 * \struct wasm_engine_t
 * \brief Compilation environment and configuration.
 *
 * An engine is typically global in a program and contains all the configuration
 * necessary for compiling wasm code. From an engine you'll typically create a
 * #wasmtime_store_t. Engines are created with #wasm_engine_new or
 * #wasm_engine_new_with_config.
 *
 * An engine is safe to share between threads. Multiple stores can be created
 * within the same engine with each store living on a separate thread. Typically
 * you'll create one #wasm_engine_t for the lifetime of your program.
 *
 * Engines are reference counted internally so #wasm_engine_delete can be called
 * at any time after a #wasmtime_store_t has been created from one.
 *
 * \fn wasm_engine_t *wasm_engine_new(void);
 * \brief Creates a new engine with the default configuration.
 *
 * The object returned is owned by the caller and will need to be deleted with
 * #wasm_engine_delete. This may return `NULL` if the engine could not be
 * allocated.
 *
 * \fn wasm_engine_t *wasm_engine_new_with_config(wasm_config_t *);
 * \brief Creates a new engine with the specified configuration.
 *
 * This function will take ownership of the configuration specified regardless
 * of the outcome of this function. You do not need to call #wasm_config_delete
 * on the argument. The object returned is owned by the caller and will need to
 * be deleted with #wasm_engine_delete. This may return `NULL` if the engine
 * could not be allocated.
 *
 * \fn void wasm_engine_delete(wasm_engine_t*);
 * \brief Deletes an engine.
 */

/**
 * \typedef wasm_store_t
 * \brief Convenience alias for #wasm_store_t
 *
 * \struct wasm_store_t
 * \brief A collection of instances and wasm global items.
 *
 * A #wasm_store_t corresponds to the concept of an [embedding
 * store](https://webassembly.github.io/spec/core/exec/runtime.html#store)
 *
 * \fn wasm_store_t *wasm_store_new(wasm_engine_t *);
 * \brief Creates a new store within the specified engine.
 *
 * The object returned is owned by the caller and will need to be deleted with
 * #wasm_store_delete. This may return `NULL` if the store could not be
 * allocated.
 *
 * \fn void wasm_store_delete(wasm_store_t *);
 * \brief Deletes the specified store.
 */

/**
 * \struct wasm_byte_vec_t
 * \brief A list of bytes
 *
 * Used to pass data in or pass data out of various functions.  The meaning and
 * ownership of the bytes is defined by each API that operates on this
 * datatype.
 *
 * \var wasm_byte_vec_t::size
 * \brief Length of this vector.
 *
 * \var wasm_byte_vec_t::data
 * \brief Pointer to the base of this vector
 *
 * \typedef wasm_byte_vec_t
 * \brief Convenience alias for #wasm_byte_vec_t
 *
 * \typedef wasm_message_t
 * \brief Alias of #wasm_byte_vec_t which always has a trailing 0-byte.
 *
 * \fn wasm_name
 * \brief Unused by Wasmtime
 *
 * \fn wasm_name_new
 * \brief Convenience alias
 *
 * \fn wasm_name_new_empty
 * \brief Convenience alias
 *
 * \fn wasm_name_new_new_uninitialized
 * \brief Convenience alias
 *
 * \fn wasm_name_new_from_string
 * \brief Create a new name from a C string.
 *
 * \fn wasm_name_new_from_string_nt
 * \brief Create a new name from a C string with null terminator.
 *
 * \fn wasm_name_copy
 * \brief Convenience alias
 *
 * \fn wasm_name_delete
 * \brief Convenience alias
 *
 * \fn void wasm_byte_vec_new_empty(wasm_byte_vec_t *out);
 * \brief Initializes an empty byte vector.
 *
 * \fn void wasm_byte_vec_new_uninitialized(wasm_byte_vec_t *out, size_t);
 * \brief Initializes an byte vector with the specified capacity.
 *
 * This function will initialize the provided vector with capacity to hold the
 * specified number of bytes. The `out` parameter must previously not already be
 * initialized and after this function is called you are then responsible for
 * ensuring #wasm_byte_vec_delete is called.
 *
 * \fn void wasm_byte_vec_new(wasm_byte_vec_t *out, size_t, wasm_byte_t const[]);
 * \brief Copies the specified data into a new byte vector.
 *
 * This function will copy the provided data into this byte vector. The byte
 * vector should not be previously initialized and the caller is responsible for
 * calling #wasm_byte_vec_delete after this function returns.
 *
 * Note that memory of the initialization vector provided to this function
 * must be managed externally. This function will copy the contents to the
 * output vector, but it's up to the caller to properly deallocate the memory.
 *
 * \fn void wasm_byte_vec_copy(wasm_byte_vec_t *out, const wasm_byte_vec_t *);
 * \brief Copies one vector into a new vector.
 *
 * Copies the second argument's data into the first argument. The `out` vector
 * should not be previously initialized and after this function returns you're
 * responsible for calling #wasm_byte_vec_delete.
 *
 * \fn void wasm_byte_vec_delete(wasm_byte_vec_t *);
 * \brief Deletes a byte vector.
 *
 * This function will deallocate the data referenced by the argument provided.
 * This does not deallocate the memory holding the #wasm_byte_vec_t itself, it's
 * expected that memory is owned by the caller.
 */

/**
 * \struct wasm_valtype_t
 * \brief An object representing the type of a value.
 *
 * \typedef wasm_valtype_t
 * \brief Convenience alias for #wasm_valtype_t
 *
 * \struct wasm_valtype_vec_t
 * \brief A list of #wasm_valtype_t values.
 *
 * \var wasm_valtype_vec_t::size
 * \brief Length of this vector.
 *
 * \var wasm_valtype_vec_t::data
 * \brief Pointer to the base of this vector
 *
 * \typedef wasm_valtype_vec_t
 * \brief Convenience alias for #wasm_valtype_vec_t
 *
 * \fn void wasm_valtype_delete(wasm_valtype_t *);
 * \brief Deletes a type.
 *
 * \fn void wasm_valtype_vec_new_empty(wasm_valtype_vec_t *out);
 * \brief Creates an empty vector.
 *
 * See #wasm_byte_vec_new_empty for more information.
 *
 * \fn void wasm_valtype_vec_new_uninitialized(wasm_valtype_vec_t *out, size_t);
 * \brief Creates a vector with the given capacity.
 *
 * See #wasm_byte_vec_new_uninitialized for more information.
 *
 * \fn void wasm_valtype_vec_new(wasm_valtype_vec_t *out, size_t, wasm_valtype_t *const[]);
 * \brief Creates a vector with the provided contents.
 *
 * See #wasm_byte_vec_new for more information.
 *
 * \fn void wasm_valtype_vec_copy(wasm_valtype_vec_t *out, const wasm_valtype_vec_t *)
 * \brief Copies one vector to another
 *
 * See #wasm_byte_vec_copy for more information.
 *
 * \fn void wasm_valtype_vec_delete(wasm_valtype_vec_t *out)
 * \brief Deallocates memory for a vector.
 *
 * See #wasm_byte_vec_delete for more information.
 *
 * \fn wasm_valtype_t* wasm_valtype_copy(const wasm_valtype_t *)
 * \brief Creates a new value which matches the provided one.
 *
 * The caller is responsible for deleting the returned value.
 *
 * \fn wasm_valtype_t* wasm_valtype_new(wasm_valkind_t);
 * \brief Creates a new value type from the specified kind.
 *
 * The caller is responsible for deleting the returned value.
 *
 * \fn wasm_valkind_t wasm_valtype_kind(const wasm_valtype_t *);
 * \brief Returns the associated kind for this value type.
 */

/**
 * \typedef wasm_valkind_t
 * \brief Different kinds of types supported in wasm.
 */

/**
 * \struct wasm_functype_t
 * \brief An opaque object representing the type of a function.
 *
 * \typedef wasm_functype_t
 * \brief Convenience alias for #wasm_functype_t
 *
 * \struct wasm_functype_vec_t
 * \brief A list of #wasm_functype_t values.
 *
 * \var wasm_functype_vec_t::size
 * \brief Length of this vector.
 *
 * \var wasm_functype_vec_t::data
 * \brief Pointer to the base of this vector
 *
 * \typedef wasm_functype_vec_t
 * \brief Convenience alias for #wasm_functype_vec_t
 *
 * \fn void wasm_functype_delete(wasm_functype_t *);
 * \brief Deletes a type.
 *
 * \fn void wasm_functype_vec_new_empty(wasm_functype_vec_t *out);
 * \brief Creates an empty vector.
 *
 * See #wasm_byte_vec_new_empty for more information.
 *
 * \fn void wasm_functype_vec_new_uninitialized(wasm_functype_vec_t *out, size_t);
 * \brief Creates a vector with the given capacity.
 *
 * See #wasm_byte_vec_new_uninitialized for more information.
 *
 * \fn void wasm_functype_vec_new(wasm_functype_vec_t *out, size_t, wasm_functype_t *const[]);
 * \brief Creates a vector with the provided contents.
 *
 * See #wasm_byte_vec_new for more information.
 *
 * \fn void wasm_functype_vec_copy(wasm_functype_vec_t *out, const wasm_functype_vec_t *)
 * \brief Copies one vector to another
 *
 * See #wasm_byte_vec_copy for more information.
 *
 * \fn void wasm_functype_vec_delete(wasm_functype_vec_t *out)
 * \brief Deallocates memory for a vector.
 *
 * See #wasm_byte_vec_delete for more information.
 *
 * \fn wasm_functype_t* wasm_functype_copy(const wasm_functype_t *)
 * \brief Creates a new value which matches the provided one.
 *
 * The caller is responsible for deleting the returned value.
 *
 * \fn wasm_functype_t* wasm_functype_new(wasm_valtype_vec_t *params, wasm_valtype_vec_t *results);
 * \brief Creates a new function type with the provided parameter and result
 * types.
 *
 * This function takes ownership of the `params` and `results` arguments.
 *
 * The caller is responsible for deleting the returned value.
 *
 * \fn const wasm_valtype_vec_t* wasm_functype_params(const wasm_functype_t *);
 * \brief Returns the list of parameters of this function type.
 *
 * The returned memory is owned by the #wasm_functype_t argument, the caller
 * should not deallocate it.
 *
 * \fn const wasm_valtype_vec_t* wasm_functype_results(const wasm_functype_t *);
 * \brief Returns the list of results of this function type.
 *
 * The returned memory is owned by the #wasm_functype_t argument, the caller
 * should not deallocate it.
 */

/**
 * \struct wasm_globaltype_t
 * \brief An opaque object representing the type of a global.
 *
 * \typedef wasm_globaltype_t
 * \brief Convenience alias for #wasm_globaltype_t
 *
 * \struct wasm_globaltype_vec_t
 * \brief A list of #wasm_globaltype_t values.
 *
 * \var wasm_globaltype_vec_t::size
 * \brief Length of this vector.
 *
 * \var wasm_globaltype_vec_t::data
 * \brief Pointer to the base of this vector
 *
 * \typedef wasm_globaltype_vec_t
 * \brief Convenience alias for #wasm_globaltype_vec_t
 *
 * \fn void wasm_globaltype_delete(wasm_globaltype_t *);
 * \brief Deletes a type.
 *
 * \fn void wasm_globaltype_vec_new_empty(wasm_globaltype_vec_t *out);
 * \brief Creates an empty vector.
 *
 * See #wasm_byte_vec_new_empty for more information.
 *
 * \fn void wasm_globaltype_vec_new_uninitialized(wasm_globaltype_vec_t *out, size_t);
 * \brief Creates a vector with the given capacity.
 *
 * See #wasm_byte_vec_new_uninitialized for more information.
 *
 * \fn void wasm_globaltype_vec_new(wasm_globaltype_vec_t *out, size_t, wasm_globaltype_t *const[]);
 * \brief Creates a vector with the provided contents.
 *
 * See #wasm_byte_vec_new for more information.
 *
 * \fn void wasm_globaltype_vec_copy(wasm_globaltype_vec_t *out, const wasm_globaltype_vec_t *)
 * \brief Copies one vector to another
 *
 * See #wasm_byte_vec_copy for more information.
 *
 * \fn void wasm_globaltype_vec_delete(wasm_globaltype_vec_t *out)
 * \brief Deallocates memory for a vector.
 *
 * See #wasm_byte_vec_delete for more information.
 *
 * \fn wasm_globaltype_t* wasm_globaltype_copy(const wasm_globaltype_t *)
 * \brief Creates a new value which matches the provided one.
 *
 * The caller is responsible for deleting the returned value.
 *
 * \fn wasm_globaltype_t* wasm_globaltype_new(wasm_valtype_t *, wasm_mutability_t)
 * \brief Creates a new global type.
 *
 * This function takes ownership of the #wasm_valtype_t argument.
 *
 * The caller is responsible for deleting the returned value.
 *
 * \fn const wasm_valtype_t* wasm_globaltype_content(const wasm_globaltype_t *);
 * \brief Returns the type of value contained in a global.
 *
 * The returned memory is owned by the provided #wasm_globaltype_t, the caller
 * should not deallocate it.
 *
 * \fn wasm_mutability_t wasm_globaltype_mutability(const wasm_globaltype_t *);
 * \brief Returns whether or not a global is mutable.
 */

/**
 * \typedef wasm_mutability_t
 * \brief Boolean flag for whether a global is mutable or not.
 */

/**
 * \struct wasm_tabletype_t
 * \brief An opaque object representing the type of a table.
 *
 * \typedef wasm_tabletype_t
 * \brief Convenience alias for #wasm_tabletype_t
 *
 * \struct wasm_tabletype_vec_t
 * \brief A list of #wasm_tabletype_t values.
 *
 * \var wasm_tabletype_vec_t::size
 * \brief Length of this vector.
 *
 * \var wasm_tabletype_vec_t::data
 * \brief Pointer to the base of this vector
 *
 * \typedef wasm_tabletype_vec_t
 * \brief Convenience alias for #wasm_tabletype_vec_t
 *
 * \fn void wasm_tabletype_delete(wasm_tabletype_t *);
 * \brief Deletes a type.
 *
 * \fn void wasm_tabletype_vec_new_empty(wasm_tabletype_vec_t *out);
 * \brief Creates an empty vector.
 *
 * See #wasm_byte_vec_new_empty for more information.
 *
 * \fn void wasm_tabletype_vec_new_uninitialized(wasm_tabletype_vec_t *out, size_t);
 * \brief Creates a vector with the given capacity.
 *
 * See #wasm_byte_vec_new_uninitialized for more information.
 *
 * \fn void wasm_tabletype_vec_new(wasm_tabletype_vec_t *out, size_t, wasm_tabletype_t *const[]);
 * \brief Creates a vector with the provided contents.
 *
 * See #wasm_byte_vec_new for more information.
 *
 * \fn void wasm_tabletype_vec_copy(wasm_tabletype_vec_t *out, const wasm_tabletype_vec_t *)
 * \brief Copies one vector to another
 *
 * See #wasm_byte_vec_copy for more information.
 *
 * \fn void wasm_tabletype_vec_delete(wasm_tabletype_vec_t *out)
 * \brief Deallocates memory for a vector.
 *
 * See #wasm_byte_vec_delete for more information.
 *
 * \fn wasm_tabletype_t* wasm_tabletype_copy(const wasm_tabletype_t *)
 * \brief Creates a new value which matches the provided one.
 *
 * The caller is responsible for deleting the returned value.
 *
 * \fn wasm_tabletype_t* wasm_tabletype_new(wasm_valtype_t *, const wasm_limits_t *)h
 * \brief Creates a new table type.
 *
 * This function takes ownership of the #wasm_valtype_t argument, but does not
 * take ownership of the #wasm_limits_t.
 *
 * The caller is responsible for deallocating the returned type.
 *
 * \fn const wasm_valtype_t* wasm_tabletype_element(const wasm_tabletype_t *);
 * \brief Returns the element type of this table.
 *
 * The returned #wasm_valtype_t is owned by the #wasm_tabletype_t parameter, the
 * caller should not deallocate it.
 *
 * \fn const wasm_limits_t* wasm_tabletype_limits(const wasm_tabletype_t *);
 * \brief Returns the limits of this table.
 *
 * The returned #wasm_limits_t is owned by the #wasm_tabletype_t parameter, the
 * caller should not deallocate it.
 */

/**
 * \struct wasm_limits_t
 * \brief Limits for tables/memories in wasm modules
 * \var wasm_limits_t::min
 * The minimum value required.
 * \var wasm_limits_t::max
 * The maximum value required, or `wasm_limits_max_default` if no maximum is
 * specified.
 *
 * \typedef wasm_limits_t
 * \brief A convenience typedef to #wasm_limits_t
 */

/**
 * \struct wasm_memorytype_t
 * \brief An opaque object representing the type of a memory.
 *
 * \typedef wasm_memorytype_t
 * \brief Convenience alias for #wasm_memorytype_t
 *
 * \struct wasm_memorytype_vec_t
 * \brief A list of #wasm_memorytype_t values.
 *
 * \var wasm_memorytype_vec_t::size
 * \brief Length of this vector.
 *
 * \var wasm_memorytype_vec_t::data
 * \brief Pointer to the base of this vector
 *
 * \typedef wasm_memorytype_vec_t
 * \brief Convenience alias for #wasm_memorytype_vec_t
 *
 * \fn void wasm_memorytype_delete(wasm_memorytype_t *);
 * \brief Deletes a type.
 *
 * \fn void wasm_memorytype_vec_new_empty(wasm_memorytype_vec_t *out);
 * \brief Creates an empty vector.
 *
 * See #wasm_byte_vec_new_empty for more information.
 *
 * \fn void wasm_memorytype_vec_new_uninitialized(wasm_memorytype_vec_t *out, size_t);
 * \brief Creates a vector with the given capacity.
 *
 * See #wasm_byte_vec_new_uninitialized for more information.
 *
 * \fn void wasm_memorytype_vec_new(wasm_memorytype_vec_t *out, size_t, wasm_memorytype_t *const[]);
 * \brief Creates a vector with the provided contents.
 *
 * See #wasm_byte_vec_new for more information.
 *
 * \fn void wasm_memorytype_vec_copy(wasm_memorytype_vec_t *out, const wasm_memorytype_vec_t *)
 * \brief Copies one vector to another
 *
 * See #wasm_byte_vec_copy for more information.
 *
 * \fn void wasm_memorytype_vec_delete(wasm_memorytype_vec_t *out)
 * \brief Deallocates memory for a vector.
 *
 * See #wasm_byte_vec_delete for more information.
 *
 * \fn wasm_memorytype_t* wasm_memorytype_copy(const wasm_memorytype_t *)
 * \brief Creates a new value which matches the provided one.
 *
 * The caller is responsible for deleting the returned value.
 *
 * \fn wasm_memorytype_t* wasm_memorytype_new(const wasm_limits_t *)h
 * \brief Creates a new memory type.
 *
 * This function takes ownership of the #wasm_valtype_t argument, but does not
 * take ownership of the #wasm_limits_t.
 *
 * The caller is responsible for deallocating the returned type.
 *
 * For compatibility with memory64 it's recommended to use
 * #wasmtime_memorytype_new instead.
 *
 * \fn const wasm_limits_t* wasm_memorytype_limits(const wasm_memorytype_t *);
 * \brief Returns the limits of this memory.
 *
 * The returned #wasm_limits_t is owned by the #wasm_memorytype_t parameter, the
 * caller should not deallocate it.
 *
 * For compatibility with memory64 it's recommended to use
 * #wasmtime_memorytype_maximum or #wasmtime_memorytype_minimum instead.
 */

/**
 * \struct wasm_externtype_t
 * \brief An opaque object representing the type of a external value. Can be
 * seen as a superclass of #wasm_functype_t, #wasm_tabletype_t,
 * #wasm_globaltype_t, and #wasm_memorytype_t.
 *
 * \typedef wasm_externtype_t
 * \brief Convenience alias for #wasm_externtype_t
 *
 * \struct wasm_externtype_vec_t
 * \brief A list of #wasm_externtype_t values.
 *
 * \var wasm_externtype_vec_t::size
 * \brief Length of this vector.
 *
 * \var wasm_externtype_vec_t::data
 * \brief Pointer to the base of this vector
 *
 * \typedef wasm_externtype_vec_t
 * \brief Convenience alias for #wasm_externtype_vec_t
 *
 * \fn void wasm_externtype_delete(wasm_externtype_t *);
 * \brief Deletes a type.
 *
 * \fn void wasm_externtype_vec_new_empty(wasm_externtype_vec_t *out);
 * \brief Creates an empty vector.
 *
 * See #wasm_byte_vec_new_empty for more information.
 *
 * \fn void wasm_externtype_vec_new_uninitialized(wasm_externtype_vec_t *out, size_t);
 * \brief Creates a vector with the given capacity.
 *
 * See #wasm_byte_vec_new_uninitialized for more information.
 *
 * \fn void wasm_externtype_vec_new(wasm_externtype_vec_t *out, size_t, wasm_externtype_t *const[]);
 * \brief Creates a vector with the provided contents.
 *
 * See #wasm_byte_vec_new for more information.
 *
 * \fn void wasm_externtype_vec_copy(wasm_externtype_vec_t *out, const wasm_externtype_vec_t *)
 * \brief Copies one vector to another
 *
 * See #wasm_byte_vec_copy for more information.
 *
 * \fn void wasm_externtype_vec_delete(wasm_externtype_vec_t *out)
 * \brief Deallocates extern for a vector.
 *
 * See #wasm_byte_vec_delete for more information.
 *
 * \fn wasm_externtype_t* wasm_externtype_copy(const wasm_externtype_t *)
 * \brief Creates a new value which matches the provided one.
 *
 * The caller is responsible for deleting the returned value.
 *
 * \fn wasm_externkind_t wasm_externtype_kind(const wasm_externtype_t *)
 * \brief Returns the kind of external item this type represents.
 */

/**
 * \typedef wasm_externkind_t
 * \brief Classifier for #wasm_externtype_t
 *
 * This is returned from #wasm_extern_kind and #wasm_externtype_kind to
 * determine what kind of type is wrapped.
 */

/**
 * \fn wasm_externtype_t* wasm_functype_as_externtype(wasm_functype_t *)
 * \brief Converts a #wasm_functype_t to a #wasm_externtype_t
 *
 * The returned value is owned by the #wasm_functype_t argument and should not
 * be deleted.
 *
 * \fn wasm_externtype_t* wasm_tabletype_as_externtype(wasm_tabletype_t *)
 * \brief Converts a #wasm_tabletype_t to a #wasm_externtype_t
 *
 * The returned value is owned by the #wasm_tabletype_t argument and should not
 * be deleted.
 *
 * \fn wasm_externtype_t* wasm_globaltype_as_externtype(wasm_globaltype_t *)
 * \brief Converts a #wasm_globaltype_t to a #wasm_externtype_t
 *
 * The returned value is owned by the #wasm_globaltype_t argument and should not
 * be deleted.
 *
 * \fn wasm_externtype_t* wasm_memorytype_as_externtype(wasm_memorytype_t *)
 * \brief Converts a #wasm_memorytype_t to a #wasm_externtype_t
 *
 * The returned value is owned by the #wasm_memorytype_t argument and should not
 * be deleted.
 *
 * \fn const wasm_externtype_t* wasm_functype_as_externtype_const(const wasm_functype_t *)
 * \brief Converts a #wasm_functype_t to a #wasm_externtype_t
 *
 * The returned value is owned by the #wasm_functype_t argument and should not
 * be deleted.
 *
 * \fn const wasm_externtype_t* wasm_tabletype_as_externtype_const(const wasm_tabletype_t *)
 * \brief Converts a #wasm_tabletype_t to a #wasm_externtype_t
 *
 * The returned value is owned by the #wasm_tabletype_t argument and should not
 * be deleted.
 *
 * \fn const wasm_externtype_t* wasm_globaltype_as_externtype_const(const wasm_globaltype_t *)
 * \brief Converts a #wasm_globaltype_t to a #wasm_externtype_t
 *
 * The returned value is owned by the #wasm_globaltype_t argument and should not
 * be deleted.
 *
 * \fn const wasm_externtype_t* wasm_memorytype_as_externtype_const(const wasm_memorytype_t *)
 * \brief Converts a #wasm_memorytype_t to a #wasm_externtype_t
 *
 * The returned value is owned by the #wasm_memorytype_t argument and should not
 * be deleted.
 *
 * \fn wasm_functype_t* wasm_externtype_as_functype(wasm_externtype_t *)
 * \brief Attempts to convert a #wasm_externtype_t to a #wasm_functype_t
 *
 * The returned value is owned by the #wasm_functype_t argument and should not
 * be deleted. Returns `NULL` if the provided argument is not a
 * #wasm_functype_t.
 *
 * \fn wasm_tabletype_t* wasm_externtype_as_tabletype(wasm_externtype_t *)
 * \brief Attempts to convert a #wasm_externtype_t to a #wasm_tabletype_t
 *
 * The returned value is owned by the #wasm_tabletype_t argument and should not
 * be deleted. Returns `NULL` if the provided argument is not a
 * #wasm_tabletype_t.
 *
 * \fn wasm_memorytype_t* wasm_externtype_as_memorytype(wasm_externtype_t *)
 * \brief Attempts to convert a #wasm_externtype_t to a #wasm_memorytype_t
 *
 * The returned value is owned by the #wasm_memorytype_t argument and should not
 * be deleted. Returns `NULL` if the provided argument is not a
 * #wasm_memorytype_t.
 *
 * \fn wasm_globaltype_t* wasm_externtype_as_globaltype(wasm_externtype_t *)
 * \brief Attempts to convert a #wasm_externtype_t to a #wasm_globaltype_t
 *
 * The returned value is owned by the #wasm_globaltype_t argument and should not
 * be deleted. Returns `NULL` if the provided argument is not a
 * #wasm_globaltype_t.
 *
 * \fn const wasm_functype_t* wasm_externtype_as_functype_const(const wasm_externtype_t *)
 * \brief Attempts to convert a #wasm_externtype_t to a #wasm_functype_t
 *
 * The returned value is owned by the #wasm_functype_t argument and should not
 * be deleted. Returns `NULL` if the provided argument is not a
 * #wasm_functype_t.
 *
 * \fn const wasm_tabletype_t* wasm_externtype_as_tabletype_const(const wasm_externtype_t *)
 * \brief Attempts to convert a #wasm_externtype_t to a #wasm_tabletype_t
 *
 * The returned value is owned by the #wasm_tabletype_t argument and should not
 * be deleted. Returns `NULL` if the provided argument is not a
 * #wasm_tabletype_t.
 *
 * \fn const wasm_memorytype_t* wasm_externtype_as_memorytype_const(const wasm_externtype_t *)
 * \brief Attempts to convert a #wasm_externtype_t to a #wasm_memorytype_t
 *
 * The returned value is owned by the #wasm_memorytype_t argument and should not
 * be deleted. Returns `NULL` if the provided argument is not a
 * #wasm_memorytype_t.
 *
 * \fn const wasm_globaltype_t* wasm_externtype_as_globaltype_const(const wasm_externtype_t *)
 * \brief Attempts to convert a #wasm_externtype_t to a #wasm_globaltype_t
 *
 * The returned value is owned by the #wasm_globaltype_t argument and should not
 * be deleted. Returns `NULL` if the provided argument is not a
 * #wasm_globaltype_t.
 */

/**
 * \struct wasm_importtype_t
 * \brief An opaque object representing the type of an import.
 *
 * \typedef wasm_importtype_t
 * \brief Convenience alias for #wasm_importtype_t
 *
 * \struct wasm_importtype_vec_t
 * \brief A list of #wasm_importtype_t values.
 *
 * \var wasm_importtype_vec_t::size
 * \brief Length of this vector.
 *
 * \var wasm_importtype_vec_t::data
 * \brief Pointer to the base of this vector
 *
 * \typedef wasm_importtype_vec_t
 * \brief Convenience alias for #wasm_importtype_vec_t
 *
 * \fn void wasm_importtype_delete(wasm_importtype_t *);
 * \brief Deletes a type.
 *
 * \fn void wasm_importtype_vec_new_empty(wasm_importtype_vec_t *out);
 * \brief Creates an empty vector.
 *
 * See #wasm_byte_vec_new_empty for more information.
 *
 * \fn void wasm_importtype_vec_new_uninitialized(wasm_importtype_vec_t *out, size_t);
 * \brief Creates a vector with the given capacity.
 *
 * See #wasm_byte_vec_new_uninitialized for more information.
 *
 * \fn void wasm_importtype_vec_new(wasm_importtype_vec_t *out, size_t, wasm_importtype_t *const[]);
 * \brief Creates a vector with the provided contents.
 *
 * See #wasm_byte_vec_new for more information.
 *
 * \fn void wasm_importtype_vec_copy(wasm_importtype_vec_t *out, const wasm_importtype_vec_t *)
 * \brief Copies one vector to another
 *
 * See #wasm_byte_vec_copy for more information.
 *
 * \fn void wasm_importtype_vec_delete(wasm_importtype_vec_t *out)
 * \brief Deallocates import for a vector.
 *
 * See #wasm_byte_vec_delete for more information.
 *
 * \fn wasm_importtype_t* wasm_importtype_copy(const wasm_importtype_t *)
 * \brief Creates a new value which matches the provided one.
 *
 * The caller is responsible for deleting the returned value.
 *
 * \fn wasm_importtype_t* wasm_importtype_new(wasm_name_t *module, wasm_name_t *name, wasm_externtype_t *)
 * \brief Creates a new import type.
 *
 * This function takes ownership of the `module`, `name`, and
 * #wasm_externtype_t arguments. The caller is responsible for deleting the
 * returned value. Note that `name` can be `NULL` where in the module linking
 * proposal the import name can be omitted.
 *
 * \fn const wasm_name_t* wasm_importtype_module(const wasm_importtype_t *);
 * \brief Returns the module this import is importing from.
 *
 * The returned memory is owned by the #wasm_importtype_t argument, the caller
 * should not deallocate it.
 *
 * \fn const wasm_name_t* wasm_importtype_name(const wasm_importtype_t *);
 * \brief Returns the name this import is importing from.
 *
 * The returned memory is owned by the #wasm_importtype_t argument, the caller
 * should not deallocate it. Note that `NULL` can be returned which means
 * that the import name is not provided. This is for imports with the module
 * linking proposal that only have the module specified.
 *
 * \fn const wasm_externtype_t* wasm_importtype_type(const wasm_importtype_t *);
 * \brief Returns the type of item this import is importing.
 *
 * The returned memory is owned by the #wasm_importtype_t argument, the caller
 * should not deallocate it.
 */

/**
 * \struct wasm_exporttype_t
 * \brief An opaque object representing the type of an export.
 *
 * \typedef wasm_exporttype_t
 * \brief Convenience alias for #wasm_exporttype_t
 *
 * \struct wasm_exporttype_vec_t
 * \brief A list of #wasm_exporttype_t values.
 *
 * \var wasm_exporttype_vec_t::size
 * \brief Length of this vector.
 *
 * \var wasm_exporttype_vec_t::data
 * \brief Pointer to the base of this vector
 *
 * \typedef wasm_exporttype_vec_t
 * \brief Convenience alias for #wasm_exporttype_vec_t
 *
 * \fn void wasm_exporttype_delete(wasm_exporttype_t *);
 * \brief Deletes a type.
 *
 * \fn void wasm_exporttype_vec_new_empty(wasm_exporttype_vec_t *out);
 * \brief Creates an empty vector.
 *
 * See #wasm_byte_vec_new_empty for more information.
 *
 * \fn void wasm_exporttype_vec_new_uninitialized(wasm_exporttype_vec_t *out, size_t);
 * \brief Creates a vector with the given capacity.
 *
 * See #wasm_byte_vec_new_uninitialized for more information.
 *
 * \fn void wasm_exporttype_vec_new(wasm_exporttype_vec_t *out, size_t, wasm_exporttype_t *const[]);
 * \brief Creates a vector with the provided contents.
 *
 * See #wasm_byte_vec_new for more information.
 *
 * \fn void wasm_exporttype_vec_copy(wasm_exporttype_vec_t *out, const wasm_exporttype_vec_t *)
 * \brief Copies one vector to another
 *
 * See #wasm_byte_vec_copy for more information.
 *
 * \fn void wasm_exporttype_vec_delete(wasm_exporttype_vec_t *out)
 * \brief Deallocates export for a vector.
 *
 * See #wasm_byte_vec_delete for more information.
 *
 * \fn wasm_exporttype_t* wasm_exporttype_copy(const wasm_exporttype_t *)
 * \brief Creates a new value which matches the provided one.
 *
 * The caller is responsible for deleting the returned value.
 *
 * \fn wasm_exporttype_t* wasm_exporttype_new(wasm_name_t *name, wasm_externtype_t *)
 * \brief Creates a new export type.
 *
 * This function takes ownership of the `name` and
 * #wasm_externtype_t arguments. The caller is responsible for deleting the
 * returned value.
 *
 * \fn const wasm_name_t* wasm_exporttype_name(const wasm_exporttype_t *);
 * \brief Returns the name of this export.
 *
 * The returned memory is owned by the #wasm_exporttype_t argument, the caller
 * should not deallocate it.
 *
 * \fn const wasm_externtype_t* wasm_exporttype_type(const wasm_exporttype_t *);
 * \brief Returns the type of this export.
 *
 * The returned memory is owned by the #wasm_exporttype_t argument, the caller
 * should not deallocate it.
 */

/**
 * \struct wasm_val_t
 * \brief Representation of a WebAssembly value.
 *
 * Note that this structure is intended to represent the way to communicate
 * values from the embedder to the engine. This type is not actually the
 * internal representation in JIT code, for example.
 *
 * Also note that this is an owned value, notably the `ref` field. The
 * #wasm_val_delete function does not delete the memory holding the #wasm_val_t
 * itself, but only the memory pointed to by #wasm_val_t.
 *
 * \var wasm_val_t::kind
 * \brief The kind of this value, or which of the fields in the `of` payload
 * contains the actual value.
 *
 * \var wasm_val_t::of
 * \brief The actual value of this #wasm_val_t. Only one field of this
 * anonymous union is valid, and which field is valid is defined by the `kind`
 * field.
 *
 * \var wasm_val_t::@0::i32
 * \brief value for the `WASM_I32` type
 *
 * \var wasm_val_t::@0::i64
 * \brief value for the `WASM_I64` type
 *
 * \var wasm_val_t::@0::f32
 * \brief value for the `WASM_F32` type
 *
 * \var wasm_val_t::@0::f64
 * \brief value for the `WASM_F64` type
 *
 * \var wasm_val_t::@0::ref
 * \brief Unused by Wasmtime.
 *
 * \typedef wasm_val_t
 * \brief Convenience alias for #wasm_val_t
 *
 * \struct wasm_val_vec_t
 * \brief A list of #wasm_val_t values.
 *
 * \var wasm_val_vec_t::size
 * \brief Length of this vector.
 *
 * \var wasm_val_vec_t::data
 * \brief Pointer to the base of this vector
 *
 * \typedef wasm_val_vec_t
 * \brief Convenience alias for #wasm_val_vec_t
 *
 * \fn void wasm_val_delete(wasm_val_t *v);
 * \brief Deletes a type.
 *
 * This does not delete the memory pointed to by `v`, so it's safe for `v` to
 * reside on the stack. Instead this only deletes the memory referenced by `v`,
 * such as the `ref` variant of #wasm_val_t.
 *
 * \fn void wasm_val_vec_new_empty(wasm_val_vec_t *out);
 * \brief Creates an empty vector.
 *
 * See #wasm_byte_vec_new_empty for more information.
 *
 * \fn void wasm_val_vec_new_uninitialized(wasm_val_vec_t *out, size_t);
 * \brief Creates a vector with the given capacity.
 *
 * See #wasm_byte_vec_new_uninitialized for more information.
 *
 * \fn void wasm_val_vec_new(wasm_val_vec_t *out, size_t, wasm_val_t const[]);
 * \brief Creates a vector with the provided contents.
 *
 * See #wasm_byte_vec_new for more information.
 *
 * \fn void wasm_val_vec_copy(wasm_val_vec_t *out, const wasm_val_vec_t *)
 * \brief Copies one vector to another
 *
 * See #wasm_byte_vec_copy for more information.
 *
 * \fn void wasm_val_vec_delete(wasm_val_vec_t *out)
 * \brief Deallocates export for a vector.
 *
 * See #wasm_byte_vec_delete for more information.
 *
 * \fn void wasm_val_copy(wasm_val_t *out, const wasm_val_t *)
 * \brief Copies a #wasm_val_t to a new one.
 *
 * The second argument to this function is copied to the first. The caller is
 * responsible for calling #wasm_val_delete on the first argument after this
 * function. The `out` parameter is assumed uninitialized by this function and
 * the previous contents will not be deallocated.
 */

/**
 * \struct wasm_ref_t
 * \brief A reference type: either a funcref or an externref.
 *
 * \typedef wasm_ref_t
 * \brief Convenience alias for #wasm_ref_t
 *
 * \fn void wasm_ref_delete(wasm_ref_t *v);
 * \brief Delete a reference.
 *
 * \fn wasm_ref_t *wasm_ref_copy(const wasm_ref_t *)
 * \brief Copy a reference.
 *
 * \fn bool wasm_ref_same(const wasm_ref_t *, const wasm_ref_t *)
 * \brief Are the given references pointing to the same externref?
 *
 * > Note: Wasmtime does not support checking funcrefs for equality, and this
 * > function will always return false for funcrefs.
 *
 * \fn void* wasm_ref_get_host_info(const wasm_ref_t *);
 * \brief Unimplemented in Wasmtime, always returns `NULL`.
 *
 * \fn void wasm_ref_set_host_info(wasm_ref_t *, void *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn void wasm_ref_set_host_info_with_finalizer(wasm_ref_t *, void *, void(*)(void*));
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 */

/**
 * \struct wasm_frame_t
 * \brief Opaque struct representing a frame of a wasm stack trace.
 *
 * \typedef wasm_frame_t
 * \brief Convenience alias for #wasm_frame_t
 *
 * \struct wasm_frame_vec_t
 * \brief A list of #wasm_frame_t frameues.
 *
 * \var wasm_frame_vec_t::size
 * \brief Length of this vector.
 *
 * \var wasm_frame_vec_t::data
 * \brief Pointer to the base of this vector
 *
 * \typedef wasm_frame_vec_t
 * \brief Convenience alias for #wasm_frame_vec_t
 *
 * \fn void wasm_frame_delete(wasm_frame_t *v);
 * \brief Deletes a frame.
 *
 * \fn void wasm_frame_vec_new_empty(wasm_frame_vec_t *out);
 * \brief Creates an empty vector.
 *
 * See #wasm_byte_vec_new_empty for more information.
 *
 * \fn void wasm_frame_vec_new_uninitialized(wasm_frame_vec_t *out, size_t);
 * \brief Creates a vector with the given capacity.
 *
 * See #wasm_byte_vec_new_uninitialized for more information.
 *
 * \fn void wasm_frame_vec_new(wasm_frame_vec_t *out, size_t, wasm_frame_t *const[]);
 * \brief Creates a vector with the provided contents.
 *
 * See #wasm_byte_vec_new for more information.
 *
 * \fn void wasm_frame_vec_copy(wasm_frame_vec_t *out, const wasm_frame_vec_t *)
 * \brief Copies one vector to another
 *
 * See #wasm_byte_vec_copy for more information.
 *
 * \fn void wasm_frame_vec_delete(wasm_frame_vec_t *out)
 * \brief Deallocates export for a vector.
 *
 * See #wasm_byte_vec_delete for more information.
 *
 * \fn wasm_frame_t *wasm_frame_copy(const wasm_frame_t *)
 * \brief Returns a copy of the provided frame.
 *
 * The caller is expected to call #wasm_frame_delete on the returned frame.
 *
 * \fn wasm_instance_t *wasm_frame_instance(const wasm_frame_t *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn uint32_t wasm_frame_func_index(const wasm_frame_t *);
 * \brief Returns the function index in the original wasm module that this frame
 * corresponds to.
 *
 * \fn uint32_t wasm_frame_func_offset(const wasm_frame_t *);
 * \brief Returns the byte offset from the beginning of the function in the
 * original wasm file to the instruction this frame points to.
 *
 * \fn uint32_t wasm_frame_module_offset(const wasm_frame_t *);
 * \brief Returns the byte offset from the beginning of the original wasm file
 * to the instruction this frame points to.
 */

/**
 * \struct wasm_trap_t
 * \brief Opaque struct representing a wasm trap.
 *
 * \typedef wasm_trap_t
 * \brief Convenience alias for #wasm_trap_t
 *
 * \fn void wasm_trap_delete(wasm_trap_t *v);
 * \brief Deletes a trap.
 *
 * \fn wasm_trap_t *wasm_trap_copy(const wasm_trap_t *)
 * \brief Copies a #wasm_trap_t to a new one.
 *
 * The caller is responsible for deleting the returned #wasm_trap_t.
 *
 * \fn void wasm_trap_same(const wasm_trap_t *, const wasm_trap_t *)
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn void* wasm_trap_get_host_info(const wasm_trap_t *);
 * \brief Unimplemented in Wasmtime, always returns `NULL`.
 *
 * \fn void wasm_trap_set_host_info(wasm_trap_t *, void *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn void wasm_trap_set_host_info_with_finalizer(wasm_trap_t *, void *, void(*)(void*));
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn wasm_ref_t *wasm_trap_as_ref(wasm_trap_t *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn wasm_trap_t *wasm_ref_as_trap(wasm_ref_t *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn const wasm_ref_t *wasm_trap_as_ref_const(const wasm_trap_t *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn const wasm_trap_t *wasm_ref_as_trap_const(const wasm_ref_t *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn wasm_trap_t *wasm_trap_new(wasm_store_t *store, const wasm_message_t *);
 * \brief Creates a new #wasm_trap_t with the provided message.
 *
 * This function will create a new trap within the given #wasm_store_t with the
 * provided message. This will also capture the backtrace, if any, of wasm
 * frames on the stack.
 *
 * Note that the #wasm_message_t argument is expected to have a 0-byte at the
 * end of the message, and the length should include the trailing 0-byte.
 *
 * This function does not take ownership of either argument.
 *
 * The caller is responsible for deallocating the trap returned.
 *
 * \fn void wasm_trap_message(const wasm_trap_t *, wasm_message_t *out);
 * \brief Retrieves the message associated with this trap.
 *
 * The caller takes ownership of the returned `out` value and is responsible for
 * calling #wasm_byte_vec_delete on it.
 *
 * \fn wasm_frame_t* wasm_trap_origin(const wasm_trap_t *);
 * \brief Returns the top frame of the wasm stack responsible for this trap.
 *
 * The caller is responsible for deallocating the returned frame. This function
 * may return `NULL`, for example, for traps created when there wasn't anything
 * on the wasm stack.
 *
 * \fn void wasm_trap_trace(const wasm_trap_t *, wasm_frame_vec_t *out);
 * \brief Returns the trace of wasm frames for this trap.
 *
 * The caller is responsible for deallocating the returned list of frames.
 * Frames are listed in order of increasing depth, with the most recently called
 * function at the front of the list and the base function on the stack at the
 * end.
 */

/**
 * \struct wasm_foreign_t
 * \brief Unimplemented in Wasmtime
 *
 * \typedef wasm_foreign_t
 * \brief Convenience alias for #wasm_foreign_t
 *
 * \fn void wasm_foreign_delete(wasm_foreign_t *v);
 * \brief Unimplemented in Wasmtime, aborts the process if called
 *
 * \fn wasm_foreign_t *wasm_foreign_copy(const wasm_foreign_t *)
 * \brief Unimplemented in Wasmtime, aborts the process if called
 *
 * \fn void wasm_foreign_same(const wasm_foreign_t *, const wasm_foreign_t *)
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn void* wasm_foreign_get_host_info(const wasm_foreign_t *);
 * \brief Unimplemented in Wasmtime, always returns `NULL`.
 *
 * \fn void wasm_foreign_set_host_info(wasm_foreign_t *, void *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn void wasm_foreign_set_host_info_with_finalizer(wasm_foreign_t *, void *, void(*)(void*));
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn wasm_ref_t *wasm_foreign_as_ref(wasm_foreign_t *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn wasm_foreign_t *wasm_ref_as_foreign(wasm_ref_t *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn const wasm_ref_t *wasm_foreign_as_ref_const(const wasm_foreign_t *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn const wasm_foreign_t *wasm_ref_as_foreign_const(const wasm_ref_t *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn wasm_foreign_t *wasm_foreign_new(wasm_store_t *store);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 */

/**
 * \struct wasm_module_t
 * \brief Opaque struct representing a compiled wasm module.
 *
 * This structure is safe to send across threads in Wasmtime.
 *
 * \typedef wasm_module_t
 * \brief Convenience alias for #wasm_module_t
 *
 * \struct wasm_shared_module_t
 * \brief Opaque struct representing module that can be sent between threads.
 *
 * This structure is safe to send across threads in Wasmtime. Note that in
 * Wasmtime #wasm_module_t is also safe to share across threads.
 *
 * \typedef wasm_shared_module_t
 * \brief Convenience alias for #wasm_shared_module_t
 *
 * \fn void wasm_module_delete(wasm_module_t *v);
 * \brief Deletes a module.
 *
 * \fn wasm_module_t *wasm_module_copy(const wasm_module_t *)
 * \brief Copies a #wasm_module_t to a new one.
 *
 * The caller is responsible for deleting the returned #wasm_module_t.
 *
 * \fn void wasm_module_same(const wasm_module_t *, const wasm_module_t *)
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn void* wasm_module_get_host_info(const wasm_module_t *);
 * \brief Unimplemented in Wasmtime, always returns `NULL`.
 *
 * \fn void wasm_module_set_host_info(wasm_module_t *, void *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn void wasm_module_set_host_info_with_finalizer(wasm_module_t *, void *, void(*)(void*));
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn wasm_ref_t *wasm_module_as_ref(wasm_module_t *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn wasm_module_t *wasm_ref_as_module(wasm_ref_t *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn const wasm_ref_t *wasm_module_as_ref_const(const wasm_module_t *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn const wasm_module_t *wasm_ref_as_module_const(const wasm_ref_t *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn wasm_ref_as_module_const(const wasm_ref_t *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn void wasm_shared_module_delete(wasm_shared_module_t *);
 * \brief Deletes the provided module.
 *
 * \fn wasm_shared_module_t *wasm_module_share(const wasm_module_t *);
 * \brief Creates a shareable module from the provided module.
 *
 * > Note that this API is not necessary in Wasmtime because #wasm_module_t can
 * > be shared across threads. This is implemented for compatibility, however.
 *
 * This function does not take ownership of the argument, but the caller is
 * expected to deallocate the returned #wasm_shared_module_t.
 *
 * \fn wasm_module_t *wasm_module_obtain(wasm_store_t *, const wasm_shared_module_t *);
 * \brief Attempts to create a #wasm_module_t from the shareable module.
 *
 * > Note that this API is not necessary in Wasmtime because #wasm_module_t can
 * > be shared across threads. This is implemented for compatibility, however.
 *
 * This function does not take ownership of its arguments, but the caller is
 * expected to deallocate the returned #wasm_module_t.
 *
 * This function may fail if the engines associated with the #wasm_store_t or
 * #wasm_shared_module_t are different.
 *
 * \fn wasm_module_t *wasm_module_new(wasm_store_t *, const wasm_byte_vec_t *binary)
 * \brief Compiles a raw WebAssembly binary to a #wasm_module_t.
 *
 * This function will validate and compile the provided binary. The returned
 * #wasm_module_t is ready for instantiation after this call returns.
 *
 * This function does not take ownership of its arguments, but the caller is
 * expected to deallocate the returned #wasm_module_t.
 *
 * This function may fail if the provided binary is not a WebAssembly binary or
 * if it does not pass validation. In these cases this function returns `NULL`.
 *
 * \fn bool wasm_module_validate(wasm_store_t *, const wasm_byte_vec_t *binary);
 * \brief Validates whether a provided byte sequence is a valid wasm binary.
 *
 * This function will perform any internal validation necessary to determine if
 * `binary` is a valid WebAssembly binary according to the configuration of the
 * #wasm_store_t provided.
 *
 * \fn void wasm_module_imports(const wasm_module_t *, wasm_importtype_vec_t *out);
 * \brief Returns the list of imports that this module expects.
 *
 * The list of imports returned are the types of items expected to be passed to
 * #wasm_instance_new. You can use #wasm_importtype_type to learn about the
 * expected type of each import.
 *
 * This function does not take ownership of the provided module but ownership of
 * `out` is passed to the caller. Note that `out` is treated as uninitialized
 * when passed to this function.
 *
 * \fn void wasm_module_exports(const wasm_module_t *, wasm_exporttype_vec_t *out);
 * \brief Returns the list of exports that this module provides.
 *
 * The list of exports returned are in the same order as the items returned by
 * #wasm_instance_exports.
 *
 * This function does not take ownership of the provided module but ownership
 * of `out` is passed to the caller. Note that `out` is treated as
 * uninitialized when passed to this function.
 *
 * \fn void wasm_module_serialize(const wasm_module_t *, wasm_byte_vec_t *out);
 * \brief Serializes the provided module to a byte vector.
 *
 * Does not take ownership of the input module but expects the caller will
 * deallocate the `out` vector. The byte vector can later be deserialized
 * through #wasm_module_deserialize.
 *
 * \fn wasm_module_t *wasm_module_deserialize(wasm_store_t *, const wasm_byte_vec_t *);
 * \brief Deserializes a previously-serialized module.
 *
 * The input bytes must have been created from a previous call to
 * #wasm_module_serialize.
 */

/**
 * \struct wasm_func_t
 * \brief Opaque struct representing a compiled wasm function.
 *
 * \typedef wasm_func_t
 * \brief Convenience alias for #wasm_func_t
 *
 * \typedef wasm_func_callback_t
 * \brief Type definition for functions passed to #wasm_func_new.
 *
 * This is the type signature of a host function created with #wasm_func_new.
 * This function takes two parameters, the first of which is the list of
 * parameters to the function and the second of which is where to write the
 * results. This function can optionally return a #wasm_trap_t and does not have
 * to fill in the results in that case.
 *
 * It is guaranteed that this function will be called with the appropriate
 * number and types of arguments according to the function type passed to
 * #wasm_func_new. It is required that this function produces the correct number
 * and types of results as the original type signature. It is undefined behavior
 * to return other types or different numbers of values.
 *
 * Ownership of the results and the trap returned, if any, is passed to the
 * caller of this function.
 *
 * \typedef wasm_func_callback_with_env_t
 * \brief Type definition for functions passed to #wasm_func_new_with_env
 *
 * The semantics of this function are the same as those of
 * #wasm_func_callback_t, except the first argument is the same `void*` argument
 * passed to #wasm_func_new_with_env.
 *
 * \fn void wasm_func_delete(wasm_func_t *v);
 * \brief Deletes a func.
 *
 * \fn wasm_func_t *wasm_func_copy(const wasm_func_t *)
 * \brief Copies a #wasm_func_t to a new one.
 *
 * The caller is responsible for deleting the returned #wasm_func_t.
 *
 * \fn void wasm_func_same(const wasm_func_t *, const wasm_func_t *)
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn void* wasm_func_get_host_info(const wasm_func_t *);
 * \brief Unimplemented in Wasmtime, always returns `NULL`.
 *
 * \fn void wasm_func_set_host_info(wasm_func_t *, void *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn void wasm_func_set_host_info_with_finalizer(wasm_func_t *, void *, void(*)(void*));
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn wasm_ref_t *wasm_func_as_ref(wasm_func_t *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn wasm_func_t *wasm_ref_as_func(wasm_ref_t *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn const wasm_ref_t *wasm_func_as_ref_const(const wasm_func_t *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn const wasm_func_t *wasm_ref_as_func_const(const wasm_ref_t *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn wasm_ref_as_func_const(const wasm_ref_t *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn wasm_func_t *wasm_func_new(wasm_store_t *, const wasm_functype_t *, wasm_func_callback_t);
 * \brief Creates a new WebAssembly function with host functionality.
 *
 * This function creates a new #wasm_func_t from a host-provided function. The
 * host provided function must implement the type signature matching the
 * #wasm_functype_t provided here.
 *
 * The returned #wasm_func_t is expected to be deleted by the caller. This
 * function does not take ownership of its arguments.
 *
 * \fn wasm_func_t *wasm_func_new_with_env(
 *    wasm_store_t *,
 *    const wasm_functype_t *type,
 *    wasm_func_callback_with_env_t,
 *    void *env,
 *    void (*finalizer)(void *));
 * \brief Creates a new WebAssembly function with host functionality.
 *
 * This function is the same as #wasm_func_new except that it the host-provided
 * `env` argument is passed to each invocation of the callback provided. This
 * provides a means of attaching host information to this #wasm_func_t.
 *
 * The `finalizer` argument will be invoked to deallocate `env` when the
 * #wasm_func_t is deallocated. If this argument is `NULL` then the data
 * provided will not be finalized.
 *
 * This function only takes ownership of the `env` argument (which is later
 * deallocated automatically by calling `finalizer`). This function yields
 * ownership of the returned #wasm_func_t to the caller.
 *
 * \fn wasm_functype_t *wasm_func_type(const wasm_func_t *);
 * \brief Returns the type of this function.
 *
 * The returned #wasm_functype_t is expected to be deallocated by the caller.
 *
 * \fn size_t wasm_func_param_arity(const wasm_func_t *);
 * \brief Returns the number of arguments expected by this function.
 *
 * \fn size_t wasm_func_result_arity(const wasm_func_t *);
 * \brief Returns the number of results returned by this function.
 *
* \fn wasm_trap_t *wasm_func_call(const wasm_func_t *, const wasm_val_vec_t *args, wasm_val_vec_t *results);
 * \brief Calls the provided function with the arguments given.
 *
 * This function is used to call WebAssembly from the host. The parameter array
 * provided must be valid for #wasm_func_param_arity number of arguments, and
 * the result array must be valid for #wasm_func_result_arity number of results.
 * Providing not enough space is undefined behavior.
 *
 * If any of the arguments do not have the correct type then a trap is returned.
 * Additionally if any of the arguments come from a different store than
 * the #wasm_func_t provided a trap is returned.
 *
 * When no trap happens and no errors are detected then `NULL` is returned. The
 * `results` array is guaranteed to be filled in with values appropriate for
 * this function's type signature.
 *
 * If a trap happens during execution or some other error then a non-`NULL` trap
 * is returned. In this situation the `results` are is unmodified.
 *
 * Does not take ownership of `wasm_val_t` arguments. Gives ownership of
 * `wasm_val_t` results.
 */

/**
 * \struct wasm_global_t
 * \brief Opaque struct representing a wasm global.
 *
 * \typedef wasm_global_t
 * \brief Convenience alias for #wasm_global_t
 *
 * \fn void wasm_global_delete(wasm_global_t *v);
 * \brief Deletes a global.
 *
 * \fn wasm_global_t *wasm_global_copy(const wasm_global_t *)
 * \brief Copies a #wasm_global_t to a new one.
 *
 * The caller is responsible for deleting the returned #wasm_global_t.
 *
 * \fn void wasm_global_same(const wasm_global_t *, const wasm_global_t *)
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn void* wasm_global_get_host_info(const wasm_global_t *);
 * \brief Unimplemented in Wasmtime, always returns `NULL`.
 *
 * \fn void wasm_global_set_host_info(wasm_global_t *, void *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn void wasm_global_set_host_info_with_finalizer(wasm_global_t *, void *, void(*)(void*));
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn wasm_ref_t *wasm_global_as_ref(wasm_global_t *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn wasm_global_t *wasm_ref_as_global(wasm_ref_t *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn const wasm_ref_t *wasm_global_as_ref_const(const wasm_global_t *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn const wasm_global_t *wasm_ref_as_global_const(const wasm_ref_t *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn wasm_ref_as_global_const(const wasm_ref_t *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn wasm_global_t *wasm_global_new(wasm_store_t *, const wasm_globaltype_t *, const wasm_val_t *);
 * \brief Creates a new WebAssembly global.
 *
 * This function is used to create a wasm global from the host, typically to
 * provide as the import of a module. The type of the global is specified along
 * with the initial value.
 *
 * This function will return `NULL` on errors. Errors include:
 *
 * * The type of the global doesn't match the type of the value specified.
 * * The initialization value does not come from the provided #wasm_store_t.
 *
 * This function does not take ownership of any of its arguments. The caller is
 * expected to deallocate the returned value.
 *
 * \fn wasm_globaltype_t *wasm_global_type(const wasm_global_t *);
 * \brief Returns the type of this global.
 *
 * The caller is expected to deallocate the returned #wasm_globaltype_t.
 *
 * \fn void wasm_global_get(const wasm_global_t *, wasm_val_t *out);
 * \brief Gets the value of this global.
 *
 * The caller is expected to deallocate the returned #wasm_val_t. The provided
 * `out` argument is treated as uninitialized on input.
 *
 * \fn void wasm_global_set(wasm_global_t *, const wasm_val_t *);
 * \brief Sets the value of this global.
 *
 * This function will set the value of a global to a new value. This function
 * does nothing if the global is not mutable, if the #wasm_val_t argument has
 * the wrong type, or if the provided value comes from a different store as the
 * #wasm_global_t.
 *
 * This function does not take ownership of its arguments.
 */

/**
 * \struct wasm_table_t
 * \brief Opaque struct representing a wasm table.
 *
 * \typedef wasm_table_t
 * \brief Convenience alias for #wasm_table_t
 *
 * \typedef wasm_table_size_t
 * \brief Typedef for indices and sizes of wasm tables.
 *
 * \fn void wasm_table_delete(wasm_table_t *v);
 * \brief Deletes a table.
 *
 * \fn wasm_table_t *wasm_table_copy(const wasm_table_t *)
 * \brief Copies a #wasm_table_t to a new one.
 *
 * The caller is responsible for deleting the returned #wasm_table_t.
 *
 * \fn void wasm_table_same(const wasm_table_t *, const wasm_table_t *)
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn void* wasm_table_get_host_info(const wasm_table_t *);
 * \brief Unimplemented in Wasmtime, always returns `NULL`.
 *
 * \fn void wasm_table_set_host_info(wasm_table_t *, void *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn void wasm_table_set_host_info_with_finalizer(wasm_table_t *, void *, void(*)(void*));
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn wasm_ref_t *wasm_table_as_ref(wasm_table_t *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn wasm_table_t *wasm_ref_as_table(wasm_ref_t *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn const wasm_ref_t *wasm_table_as_ref_const(const wasm_table_t *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn const wasm_table_t *wasm_ref_as_table_const(const wasm_ref_t *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn wasm_ref_as_table_const(const wasm_ref_t *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn wasm_table_t *wasm_table_new(wasm_store_t *, const wasm_tabletype_t *, wasm_ref_t *init);
 * \brief Creates a new WebAssembly table.
 *
 * Creates a new host-defined table of values. This table has the type provided
 * and is filled with the provided initial value (which can be `NULL`).
 *
 * Returns an error if the #wasm_ref_t does not match the element type of the
 * table provided or if it comes from a different store than the one provided.
 *
 * Does not take ownship of the `init` value.
 *
 * \fn wasm_tabletype_t *wasm_table_type(const wasm_table_t *);
 * \brief Returns the type of this table.
 *
 * The caller is expected to deallocate the returned #wasm_tabletype_t.
 *
 * \fn wasm_ref_t *wasm_table_get(const wasm_table_t *, wasm_table_size_t index);
 * \brief Gets an element from this table.
 *
 * Attempts to get a value at an index in this table. This function returns
 * `NULL` if the index is out of bounds.
 *
 * Gives ownership of the resulting `wasm_ref_t*`.
 *
 * \fn void wasm_table_set(wasm_table_t *, wasm_table_size_t index, wasm_ref_t *);
 * \brief Sets an element in this table.
 *
 * Attempts to set a value at an index in this table. This function does nothing
 * in erroneous situations such as:
 *
 * * The index is out of bounds.
 * * The #wasm_ref_t comes from a different store than the table provided.
 * * The #wasm_ref_t does not have an appropriate type to store in this table.
 *
 * Does not take ownership of the given `wasm_ref_t*`.
 *
 * \fn wasm_table_size_t wasm_table_size(const wasm_table_t *);
 * \brief Gets the current size, in elements, of this table.
 *
 * \fn bool wasm_table_grow(wasm_table_t *, wasm_table_size_t delta, wasm_ref_t *init);
 * \brief Attempts to grow this table by `delta` elements.
 *
 * This function will grow the table by `delta` elements, initializing all new
 * elements to the `init` value provided.
 *
 * If growth happens successfully, then `true` is returned. Otherwise `false` is
 * returned and indicates one possible form of failure:
 *
 * * The table's limits do not allow growth by `delta`.
 * * The #wasm_ref_t comes from a different store than the table provided.
 * * The #wasm_ref_t does not have an appropriate type to store in this table.
 *
 * Does not take ownership of the given `init` value.
 */

/**
 * \struct wasm_memory_t
 * \brief Opaque struct representing a wasm memory.
 *
 * \typedef wasm_memory_t
 * \brief Convenience alias for #wasm_memory_t
 *
 * \typedef wasm_memory_pages_t
 * \brief Unsigned integer to hold the number of pages a memory has.
 *
 * \fn void wasm_memory_delete(wasm_memory_t *v);
 * \brief Deletes a memory.
 *
 * \fn wasm_memory_t *wasm_memory_copy(const wasm_memory_t *)
 * \brief Copies a #wasm_memory_t to a new one.
 *
 * The caller is responsible for deleting the returned #wasm_memory_t.
 *
 * \fn void wasm_memory_same(const wasm_memory_t *, const wasm_memory_t *)
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn void* wasm_memory_get_host_info(const wasm_memory_t *);
 * \brief Unimplemented in Wasmtime, always returns `NULL`.
 *
 * \fn void wasm_memory_set_host_info(wasm_memory_t *, void *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn void wasm_memory_set_host_info_with_finalizer(wasm_memory_t *, void *, void(*)(void*));
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn wasm_ref_t *wasm_memory_as_ref(wasm_memory_t *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn wasm_memory_t *wasm_ref_as_memory(wasm_ref_t *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn const wasm_ref_t *wasm_memory_as_ref_const(const wasm_memory_t *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn const wasm_memory_t *wasm_ref_as_memory_const(const wasm_ref_t *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn wasm_ref_as_memory_const(const wasm_ref_t *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn wasm_memory_t *wasm_memory_new(wasm_store_t *, const wasm_memorytype_t *);
 * \brief Creates a new WebAssembly memory.
 *
 * \fn wasm_memorytype_t *wasm_memory_type(const wasm_memory_t *);
 * \brief Returns the type of this memory.
 *
 * The caller is expected to deallocate the returned #wasm_memorytype_t.
 *
 * \fn byte_t *wasm_memory_data(wasm_memory_t *);
 * \brief Returns the base address, in memory, where this memory is located.
 *
 * Note that the returned address may change over time when growth happens. The
 * returned pointer is only valid until the memory is next grown (which could
 * happen in wasm itself).
 *
 * \fn size_t wasm_memory_data_size(const wasm_memory_t *);
 * \brief Returns the size, in bytes, of this memory.
 *
 * \fn wasm_memory_pages_t wasm_memory_size(const wasm_memory_t *);
 * \brief Returns the size, in wasm pages, of this memory.
 *
 * \fn bool wasm_memory_grow(wasm_memory_t *, wasm_memory_pages_t delta);
 * \brief Attempts to grow this memory by `delta` wasm pages.
 *
 * This function is similar to the `memory.grow` instruction in wasm itself. It
 * will attempt to grow the memory by `delta` wasm pages. If growth fails then
 * `false` is returned, otherwise `true` is returned.
 */

/**
 * \struct wasm_extern_t
 * \brief Opaque struct representing a wasm external value.
 *
 * \typedef wasm_extern_t
 * \brief Convenience alias for #wasm_extern_t
 *
 * \struct wasm_extern_vec_t
 * \brief A list of #wasm_extern_t values.
 *
 * \var wasm_extern_vec_t::size
 * \brief Length of this vector.
 *
 * \var wasm_extern_vec_t::data
 * \brief Pointer to the base of this vector
 *
 * \typedef wasm_extern_vec_t
 * \brief Convenience alias for #wasm_extern_vec_t
 *
 * \fn void wasm_extern_delete(wasm_extern_t *v);
 * \brief Deletes a extern.
 *
 * \fn void wasm_extern_vec_new_empty(wasm_extern_vec_t *out);
 * \brief Creates an empty vector.
 *
 * See #wasm_byte_vec_new_empty for more information.
 *
 * \fn void wasm_extern_vec_new_uninitialized(wasm_extern_vec_t *out, size_t);
 * \brief Creates a vector with the given capacity.
 *
 * See #wasm_byte_vec_new_uninitialized for more information.
 *
 * \fn void wasm_extern_vec_new(wasm_extern_vec_t *out, size_t, wasm_extern_t *const[]);
 * \brief Creates a vector with the provided contents.
 *
 * See #wasm_byte_vec_new for more information.
 *
 * \fn void wasm_extern_vec_copy(wasm_extern_vec_t *out, const wasm_extern_vec_t *)
 * \brief Copies one vector to another
 *
 * See #wasm_byte_vec_copy for more information.
 *
 * \fn void wasm_extern_vec_delete(wasm_extern_vec_t *out)
 * \brief Deallocates import for a vector.
 *
 * See #wasm_byte_vec_delete for more information.
 *
 * \fn wasm_extern_t *wasm_extern_copy(const wasm_extern_t *)
 * \brief Copies a #wasm_extern_t to a new one.
 *
 * The caller is responsible for deleting the returned #wasm_extern_t.
 *
 * \fn void wasm_extern_same(const wasm_extern_t *, const wasm_extern_t *)
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn void* wasm_extern_get_host_info(const wasm_extern_t *);
 * \brief Unimplemented in Wasmtime, always returns `NULL`.
 *
 * \fn void wasm_extern_set_host_info(wasm_extern_t *, void *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn void wasm_extern_set_host_info_with_finalizer(wasm_extern_t *, void *, void(*)(void*));
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn wasm_ref_t *wasm_extern_as_ref(wasm_extern_t *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn wasm_extern_t *wasm_ref_as_extern(wasm_ref_t *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn const wasm_ref_t *wasm_extern_as_ref_const(const wasm_extern_t *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn const wasm_extern_t *wasm_ref_as_extern_const(const wasm_ref_t *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn wasm_ref_as_extern_const(const wasm_ref_t *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn wasm_externkind_t *wasm_extern_kind(const wasm_extern_t *);
 * \brief Returns the kind of this extern, indicating what it will downcast as.
 *
 * \fn wasm_externtype_t *wasm_extern_type(const wasm_extern_t *);
 * \brief Returns the type of this extern.
 *
 * The caller is expected to deallocate the returned #wasm_externtype_t.
 */

/**
 * \fn wasm_extern_t *wasm_func_as_extern(wasm_func_t *f);
 * \brief Converts a #wasm_func_t to #wasm_extern_t.
 *
 * The returned #wasm_extern_t is owned by the #wasm_func_t argument. Callers
 * should not delete the returned value, and it only lives as long as the
 * #wasm_func_t argument.
 *
 * \fn wasm_extern_t *wasm_global_as_extern(wasm_global_t *f);
 * \brief Converts a #wasm_global_t to #wasm_extern_t.
 *
 * The returned #wasm_extern_t is owned by the #wasm_global_t argument. Callers
 * should not delete the returned value, and it only lives as long as the
 * #wasm_global_t argument.
 *
 * \fn wasm_extern_t *wasm_memory_as_extern(wasm_memory_t *f);
 * \brief Converts a #wasm_memory_t to #wasm_extern_t.
 *
 * The returned #wasm_extern_t is owned by the #wasm_memory_t argument. Callers
 * should not delete the returned value, and it only lives as long as the
 * #wasm_memory_t argument.
 *
 * \fn wasm_extern_t *wasm_table_as_extern(wasm_table_t *f);
 * \brief Converts a #wasm_table_t to #wasm_extern_t.
 *
 * The returned #wasm_extern_t is owned by the #wasm_table_t argument. Callers
 * should not delete the returned value, and it only lives as long as the
 * #wasm_table_t argument.
 *
 * \fn const wasm_extern_t *wasm_func_as_extern_const(const wasm_func_t *f);
 * \brief Converts a #wasm_func_t to #wasm_extern_t.
 *
 * The returned #wasm_extern_t is owned by the #wasm_func_t argument. Callers
 * should not delete the returned value, and it only lives as long as the
 * #wasm_func_t argument.
 *
 * \fn const wasm_extern_t *wasm_global_as_extern_const(const wasm_global_t *f);
 * \brief Converts a #wasm_global_t to #wasm_extern_t.
 *
 * The returned #wasm_extern_t is owned by the #wasm_global_t argument. Callers
 * should not delete the returned value, and it only lives as long as the
 * #wasm_global_t argument.
 *
 * \fn const wasm_extern_t *wasm_memory_as_extern_const(const wasm_memory_t *f);
 * \brief Converts a #wasm_memory_t to #wasm_extern_t.
 *
 * The returned #wasm_extern_t is owned by the #wasm_memory_t argument. Callers
 * should not delete the returned value, and it only lives as long as the
 * #wasm_memory_t argument.
 *
 * \fn const wasm_extern_t *wasm_table_as_extern_const(const wasm_table_t *f);
 * \brief Converts a #wasm_table_t to #wasm_extern_t.
 *
 * The returned #wasm_extern_t is owned by the #wasm_table_t argument. Callers
 * should not delete the returned value, and it only lives as long as the
 * #wasm_table_t argument.
 *
 * \fn wasm_func_t *wasm_extern_as_func(wasm_extern_t *);
 * \brief Converts a #wasm_extern_t to #wasm_func_t.
 *
 * The returned #wasm_func_t is owned by the #wasm_extern_t argument. Callers
 * should not delete the returned value, and it only lives as long as the
 * #wasm_extern_t argument.
 *
 * If the #wasm_extern_t argument isn't a #wasm_func_t then `NULL` is returned.
 *
 * \fn wasm_table_t *wasm_extern_as_table(wasm_extern_t *);
 * \brief Converts a #wasm_extern_t to #wasm_table_t.
 *
 * The returned #wasm_table_t is owned by the #wasm_extern_t argument. Callers
 * should not delete the returned value, and it only lives as long as the
 * #wasm_extern_t argument.
 *
 * If the #wasm_extern_t argument isn't a #wasm_table_t then `NULL` is returned.
 *
 * \fn wasm_memory_t *wasm_extern_as_memory(wasm_extern_t *);
 * \brief Converts a #wasm_extern_t to #wasm_memory_t.
 *
 * The returned #wasm_memory_t is owned by the #wasm_extern_t argument. Callers
 * should not delete the returned value, and it only lives as long as the
 * #wasm_extern_t argument.
 *
 * If the #wasm_extern_t argument isn't a #wasm_memory_t then `NULL` is returned.
 *
 * \fn wasm_global_t *wasm_extern_as_global(wasm_extern_t *);
 * \brief Converts a #wasm_extern_t to #wasm_global_t.
 *
 * The returned #wasm_global_t is owned by the #wasm_extern_t argument. Callers
 * should not delete the returned value, and it only lives as long as the
 * #wasm_extern_t argument.
 *
 * If the #wasm_extern_t argument isn't a #wasm_global_t then `NULL` is returned.
 *
 * \fn const wasm_func_t *wasm_extern_as_func_const(const wasm_extern_t *);
 * \brief Converts a #wasm_extern_t to #wasm_func_t.
 *
 * The returned #wasm_func_t is owned by the #wasm_extern_t argument. Callers
 * should not delete the returned value, and it only lives as long as the
 * #wasm_extern_t argument.
 *
 * If the #wasm_extern_t argument isn't a #wasm_func_t then `NULL` is returned.
 *
 * \fn const wasm_table_t *wasm_extern_as_table_const(const wasm_extern_t *);
 * \brief Converts a #wasm_extern_t to #wasm_table_t.
 *
 * The returned #wasm_table_t is owned by the #wasm_extern_t argument. Callers
 * should not delete the returned value, and it only lives as long as the
 * #wasm_extern_t argument.
 *
 * If the #wasm_extern_t argument isn't a #wasm_table_t then `NULL` is returned.
 *
 * \fn const wasm_memory_t *wasm_extern_as_memory_const(const wasm_extern_t *);
 * \brief Converts a #wasm_extern_t to #wasm_memory_t.
 *
 * The returned #wasm_memory_t is owned by the #wasm_extern_t argument. Callers
 * should not delete the returned value, and it only lives as long as the
 * #wasm_extern_t argument.
 *
 * If the #wasm_extern_t argument isn't a #wasm_memory_t then `NULL` is returned.
 *
 * \fn const wasm_global_t *wasm_extern_as_global_const(const wasm_extern_t *);
 * \brief Converts a #wasm_extern_t to #wasm_global_t.
 *
 * The returned #wasm_global_t is owned by the #wasm_extern_t argument. Callers
 * should not delete the returned value, and it only lives as long as the
 * #wasm_extern_t argument.
 *
 * If the #wasm_extern_t argument isn't a #wasm_global_t then `NULL` is returned.
 */

/**
 * \struct wasm_instance_t
 * \brief Opaque struct representing a wasm instance.
 *
 * \typedef wasm_instance_t
 * \brief Convenience alias for #wasm_instance_t
 *
 * \fn void wasm_instance_delete(wasm_instance_t *v);
 * \brief Deletes a instance.
 *
 * \fn wasm_instance_t *wasm_instance_copy(const wasm_instance_t *)
 * \brief Copies a #wasm_instance_t to a new one.
 *
 * The caller is responsible for deleting the returned #wasm_instance_t.
 *
 * \fn void wasm_instance_same(const wasm_instance_t *, const wasm_instance_t *)
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn void* wasm_instance_get_host_info(const wasm_instance_t *);
 * \brief Unimplemented in Wasmtime, always returns `NULL`.
 *
 * \fn void wasm_instance_set_host_info(wasm_instance_t *, void *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn void wasm_instance_set_host_info_with_finalizer(wasm_instance_t *, void *, void(*)(void*));
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn wasm_ref_t *wasm_instance_as_ref(wasm_instance_t *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn wasm_instance_t *wasm_ref_as_instance(wasm_ref_t *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn const wasm_ref_t *wasm_instance_as_ref_const(const wasm_instance_t *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn const wasm_instance_t *wasm_ref_as_instance_const(const wasm_ref_t *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn wasm_ref_as_instance_const(const wasm_ref_t *);
 * \brief Unimplemented in Wasmtime, aborts the process if called.
 *
 * \fn wasm_instance_t *wasm_instance_new(wasm_store_t *, const wasm_module_t *, const wasm_extern_vec_t *, wasm_trap_t **);
 * \brief Instantiates a module with the provided imports.
 *
 * This function will instantiate the provided #wasm_module_t into the provided
 * #wasm_store_t. The `imports` specified are used to satisfy the imports of the
 * #wasm_module_t.
 *
 * This function must provide exactly the same number of imports as returned by
 * #wasm_module_imports or this results in undefined behavior.
 *
 * Imports provided are expected to be 1:1 matches against the list returned by
 * #wasm_module_imports.
 *
 * Instantiation includes invoking the `start` function of a wasm module. If
 * that function traps then a trap is returned through the #wasm_trap_t type.
 *
 * This function does not take ownership of any of its arguments, and the
 * returned #wasm_instance_t and #wasm_trap_t are owned by the caller.
 *
 * \fn void wasm_instance_exports(const wasm_instance_t *, wasm_extern_vec_t *out);
 * \brief Returns the exports of an instance.
 *
 * This function returns a list of #wasm_extern_t values, which will be owned by
 * the caller, which are exported from the instance. The `out` list will have
 * the same length as #wasm_module_exports called on the original module. Each
 * element is 1:1 matched with the elements in the list of #wasm_module_exports.
 */

/**
 * \def WASM_EMPTY_VEC
 * \brief Used to initialize an empty vector type.
 *
 * \def WASM_ARRAY_VEC
 * \brief Used to initialize a vector type from a C array.
 *
 * \def WASM_I32_VAL
 * \brief Used to initialize a 32-bit integer wasm_val_t value.
 *
 * \def WASM_I64_VAL
 * \brief Used to initialize a 64-bit integer wasm_val_t value.
 *
 * \def WASM_F32_VAL
 * \brief Used to initialize a 32-bit floating point wasm_val_t value.
 *
 * \def WASM_F64_VAL
 * \brief Used to initialize a 64-bit floating point wasm_val_t value.
 *
 * \def WASM_REF_VAL
 * \brief Used to initialize an externref wasm_val_t value.
 *
 * \def WASM_INIT_VAL
 * \brief Used to initialize a null externref wasm_val_t value.
 */
