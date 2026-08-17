/**
 * \file wasmtime/val.h
 *
 * APIs for interacting with WebAssembly values in Wasmtime.
 */

#ifndef WASMTIME_VAL_H
#define WASMTIME_VAL_H

#include <stdalign.h>
#include <wasm.h>
#include <wasmtime/extern.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \typedef wasmtime_anyref_t
 * \brief Convenience alias for #wasmtime_anyref
 *
 * \struct wasmtime_anyref
 * \brief A WebAssembly value in the `any` hierarchy of GC types.
 *
 * This structure represents an `anyref` that WebAssembly can create and
 * pass back to the host. The host can also create values to pass to a guest.
 *
 * Note that this structure does not itself contain the data that it refers to.
 * Instead to contains metadata to point back within a #wasmtime_context_t, so
 * referencing the internal data requires using a `wasmtime_context_t`.
 *
 * Anyref values are required to be explicitly unrooted via
 * #wasmtime_anyref_unroot to enable them to be garbage-collected.
 *
 * Null anyref values are represented by this structure and can be tested and
 * created with the `wasmtime_anyref_is_null` and `wasmtime_anyref_set_null`
 * functions.
 */
typedef struct wasmtime_anyref {
  /// Internal metadata tracking within the store, embedders should not
  /// configure or modify these fields.
  uint64_t store_id;
  /// Internal to Wasmtime.
  uint32_t __private1;
  /// Internal to Wasmtime.
  uint32_t __private2;
} wasmtime_anyref_t;

/// \brief Helper function to initialize the `ref` provided to a null anyref
/// value.
static inline void wasmtime_anyref_set_null(wasmtime_anyref_t *ref) {
  ref->store_id = 0;
}

/// \brief Helper function to return whether the provided `ref` points to a null
/// `anyref` value.
///
/// Note that `ref` itself should not be null as null is represented internally
/// within a #wasmtime_anyref_t value.
static inline bool wasmtime_anyref_is_null(const wasmtime_anyref_t *ref) {
  return ref->store_id == 0;
}

/**
 * \brief Creates a new reference pointing to the same data that `anyref`
 * points to (depending on the configured collector this might increase a
 * reference count or create a new GC root).
 *
 * The returned reference is stored in `out`.
 */
WASM_API_EXTERN void wasmtime_anyref_clone(wasmtime_context_t *context,
                                           const wasmtime_anyref_t *anyref,
                                           wasmtime_anyref_t *out);

/**
 * \brief Unroots the `ref` provided within the `context`.
 *
 * This API is required to enable the `ref` value provided to be
 * garbage-collected. This API itself does not necessarily garbage-collect the
 * value, but it's possible to collect it in the future after this.
 *
 * This may modify `ref` and the contents of `ref` are left in an undefined
 * state after this API is called and it should no longer be used.
 *
 * Note that null or i32 anyref values do not need to be unrooted but are still
 * valid to pass to this function.
 */
WASM_API_EXTERN void wasmtime_anyref_unroot(wasmtime_context_t *context,
                                            wasmtime_anyref_t *ref);

/**
 * \brief Converts a raw `anyref` value coming from #wasmtime_val_raw_t into
 * a #wasmtime_anyref_t.
 *
 * The provided `out` pointer is filled in with a reference converted from
 * `raw`.
 */
WASM_API_EXTERN void wasmtime_anyref_from_raw(wasmtime_context_t *context,
                                              uint32_t raw,
                                              wasmtime_anyref_t *out);

/**
 * \brief Converts a #wasmtime_anyref_t to a raw value suitable for storing
 * into a #wasmtime_val_raw_t.
 *
 * Note that the returned underlying value is not tracked by Wasmtime's garbage
 * collector until it enters WebAssembly. This means that a GC may release the
 * context's reference to the raw value, making the raw value invalid within the
 * context of the store. Do not perform a GC between calling this function and
 * passing it to WebAssembly.
 */
WASM_API_EXTERN uint32_t wasmtime_anyref_to_raw(wasmtime_context_t *context,
                                                const wasmtime_anyref_t *ref);

/**
 * \brief Create a new `i31ref` value.
 *
 * Creates a new `i31ref` value (which is a subtype of `anyref`) and returns a
 * pointer to it.
 *
 * If `i31val` does not fit in 31 bits, it is wrapped.
 */
WASM_API_EXTERN void wasmtime_anyref_from_i31(wasmtime_context_t *context,
                                              uint32_t i31val,
                                              wasmtime_anyref_t *out);

/**
 * \brief Get the `anyref`'s underlying `i31ref` value, zero extended, if any.
 *
 * If the given `anyref` is an instance of `i31ref`, then its value is zero
 * extended to 32 bits, written to `dst`, and `true` is returned.
 *
 * If the given `anyref` is not an instance of `i31ref`, then `false` is
 * returned and `dst` is left unmodified.
 */
WASM_API_EXTERN bool wasmtime_anyref_i31_get_u(wasmtime_context_t *context,
                                               const wasmtime_anyref_t *anyref,
                                               uint32_t *dst);

/**
 * \brief Get the `anyref`'s underlying `i31ref` value, sign extended, if any.
 *
 * If the given `anyref` is an instance of `i31ref`, then its value is sign
 * extended to 32 bits, written to `dst`, and `true` is returned.
 *
 * If the given `anyref` is not an instance of `i31ref`, then `false` is
 * returned and `dst` is left unmodified.
 */
WASM_API_EXTERN bool wasmtime_anyref_i31_get_s(wasmtime_context_t *context,
                                               const wasmtime_anyref_t *anyref,
                                               int32_t *dst);

/**
 * \typedef wasmtime_externref_t
 * \brief Convenience alias for #wasmtime_externref
 *
 * \struct wasmtime_externref
 * \brief A host-defined un-forgeable reference to pass into WebAssembly.
 *
 * This structure represents an `externref` that can be passed to WebAssembly.
 * It cannot be forged by WebAssembly itself and is guaranteed to have been
 * created by the host.
 *
 * This structure is similar to #wasmtime_anyref_t but represents the
 * `externref` type in WebAssembly. This can be created on the host from
 * arbitrary host pointers/destructors. Note that this value is itself a
 * reference into a #wasmtime_context_t and must be explicitly unrooted to
 * enable garbage collection.
 *
 * Note that null is represented with this structure and created with
 * `wasmtime_externref_set_null`. Null can be tested for with the
 * `wasmtime_externref_is_null` function.
 */
typedef struct wasmtime_externref {
  /// Internal metadata tracking within the store, embedders should not
  /// configure or modify these fields.
  uint64_t store_id;
  /// Internal to Wasmtime.
  uint32_t __private1;
  /// Internal to Wasmtime.
  uint32_t __private2;
} wasmtime_externref_t;

/// \brief Helper function to initialize the `ref` provided to a null externref
/// value.
static inline void wasmtime_externref_set_null(wasmtime_externref_t *ref) {
  ref->store_id = 0;
}

/// \brief Helper function to return whether the provided `ref` points to a null
/// `externref` value.
///
/// Note that `ref` itself should not be null as null is represented internally
/// within a #wasmtime_externref_t value.
static inline bool wasmtime_externref_is_null(const wasmtime_externref_t *ref) {
  return ref->store_id == 0;
}

/**
 * \brief Create a new `externref` value.
 *
 * Creates a new `externref` value wrapping the provided data, returning whether
 * it was created or not.
 *
 * \param context the store context to allocate this externref within
 * \param data the host-specific data to wrap
 * \param finalizer an optional finalizer for `data`
 * \param out where to store the created value.
 *
 * When the reference is reclaimed, the wrapped data is cleaned up with the
 * provided `finalizer`.
 *
 * If `true` is returned then `out` has been filled in and must be unrooted
 * in the future with #wasmtime_externref_unroot. If `false` is returned then
 * the host wasn't able to create more GC values at this time. Performing a GC
 * may free up enough space to try again.
 */
WASM_API_EXTERN bool wasmtime_externref_new(wasmtime_context_t *context,
                                            void *data,
                                            void (*finalizer)(void *),
                                            wasmtime_externref_t *out);

/**
 * \brief Get an `externref`'s wrapped data
 *
 * Returns the original `data` passed to #wasmtime_externref_new. It is required
 * that `data` is not `NULL`.
 */
WASM_API_EXTERN void *wasmtime_externref_data(wasmtime_context_t *context,
                                              const wasmtime_externref_t *data);

/**
 * \brief Creates a new reference pointing to the same data that `ref` points
 * to (depending on the configured collector this might increase a reference
 * count or create a new GC root).
 *
 * The `out` parameter stores the cloned reference. This reference must
 * eventually be unrooted with #wasmtime_externref_unroot in the future to
 * enable GC'ing it.
 */
WASM_API_EXTERN void wasmtime_externref_clone(wasmtime_context_t *context,
                                              const wasmtime_externref_t *ref,
                                              wasmtime_externref_t *out);

/**
 * \brief Unroots the pointer `ref` from the `context` provided.
 *
 * This function will enable future garbage collection of the value pointed to
 * by `ref` once there are no more references. The `ref` value may be mutated in
 * place by this function and its contents are undefined after this function
 * returns. It should not be used until after re-initializing it.
 *
 * Note that null externref values do not need to be unrooted but are still
 * valid to pass to this function.
 */
WASM_API_EXTERN void wasmtime_externref_unroot(wasmtime_context_t *context,
                                               wasmtime_externref_t *ref);

/**
 * \brief Converts a raw `externref` value coming from #wasmtime_val_raw_t into
 * a #wasmtime_externref_t.
 *
 * The `out` reference is filled in with the non-raw version of this externref.
 * It must eventually be unrooted with #wasmtime_externref_unroot.
 */
WASM_API_EXTERN void wasmtime_externref_from_raw(wasmtime_context_t *context,
                                                 uint32_t raw,
                                                 wasmtime_externref_t *out);

/**
 * \brief Converts a #wasmtime_externref_t to a raw value suitable for storing
 * into a #wasmtime_val_raw_t.
 *
 * Note that the returned underlying value is not tracked by Wasmtime's garbage
 * collector until it enters WebAssembly. This means that a GC may release the
 * context's reference to the raw value, making the raw value invalid within the
 * context of the store. Do not perform a GC between calling this function and
 * passing it to WebAssembly.
 */
WASM_API_EXTERN uint32_t wasmtime_externref_to_raw(
    wasmtime_context_t *context, const wasmtime_externref_t *ref);

/// \brief Discriminant stored in #wasmtime_val::kind
typedef uint8_t wasmtime_valkind_t;
/// \brief Value of #wasmtime_valkind_t meaning that #wasmtime_val_t is an i32
#define WASMTIME_I32 0
/// \brief Value of #wasmtime_valkind_t meaning that #wasmtime_val_t is an i64
#define WASMTIME_I64 1
/// \brief Value of #wasmtime_valkind_t meaning that #wasmtime_val_t is a f32
#define WASMTIME_F32 2
/// \brief Value of #wasmtime_valkind_t meaning that #wasmtime_val_t is a f64
#define WASMTIME_F64 3
/// \brief Value of #wasmtime_valkind_t meaning that #wasmtime_val_t is a v128
#define WASMTIME_V128 4
/// \brief Value of #wasmtime_valkind_t meaning that #wasmtime_val_t is a
/// funcref
#define WASMTIME_FUNCREF 5
/// \brief Value of #wasmtime_valkind_t meaning that #wasmtime_val_t is an
/// externref
#define WASMTIME_EXTERNREF 6
/// \brief Value of #wasmtime_valkind_t meaning that #wasmtime_val_t is an
/// anyref
#define WASMTIME_ANYREF 7

/// \brief A 128-bit value representing the WebAssembly `v128` type. Bytes are
/// stored in little-endian order.
typedef uint8_t wasmtime_v128[16];

/**
 * \typedef wasmtime_valunion_t
 * \brief Convenience alias for #wasmtime_valunion
 *
 * \union wasmtime_valunion
 * \brief Container for different kinds of wasm values.
 *
 * This type is contained in #wasmtime_val_t and contains the payload for the
 * various kinds of items a value can be.
 */
typedef union wasmtime_valunion {
  /// Field used if #wasmtime_val_t::kind is #WASMTIME_I32
  int32_t i32;
  /// Field used if #wasmtime_val_t::kind is #WASMTIME_I64
  int64_t i64;
  /// Field used if #wasmtime_val_t::kind is #WASMTIME_F32
  float32_t f32;
  /// Field used if #wasmtime_val_t::kind is #WASMTIME_F64
  float64_t f64;
  /// Field used if #wasmtime_val_t::kind is #WASMTIME_ANYREF
  wasmtime_anyref_t anyref;
  /// Field used if #wasmtime_val_t::kind is #WASMTIME_EXTERNREF
  wasmtime_externref_t externref;
  /// Field used if #wasmtime_val_t::kind is #WASMTIME_FUNCREF
  ///
  /// Use `wasmtime_funcref_is_null` to test whether this is a null function
  /// reference.
  wasmtime_func_t funcref;
  /// Field used if #wasmtime_val_t::kind is #WASMTIME_V128
  wasmtime_v128 v128;
} wasmtime_valunion_t;

/// \brief Initialize a `wasmtime_func_t` value as a null function reference.
///
/// This function will initialize the `func` provided to be a null function
/// reference. Used in conjunction with #wasmtime_val_t and
/// #wasmtime_valunion_t.
static inline void wasmtime_funcref_set_null(wasmtime_func_t *func) {
  func->store_id = 0;
}

/// \brief Helper function to test whether the `func` provided is a null
/// function reference.
///
/// This function is used with #wasmtime_val_t and #wasmtime_valunion_t and its
/// `funcref` field. This will test whether the field represents a null funcref.
static inline bool wasmtime_funcref_is_null(const wasmtime_func_t *func) {
  return func->store_id == 0;
}

/**
 * \typedef wasmtime_val_raw_t
 * \brief Convenience alias for #wasmtime_val_raw
 *
 * \union wasmtime_val_raw
 * \brief Container for possible wasm values.
 *
 * This type is used on conjunction with #wasmtime_func_new_unchecked as well
 * as #wasmtime_func_call_unchecked. Instances of this type do not have type
 * information associated with them, it's up to the embedder to figure out
 * how to interpret the bits contained within, often using some other channel
 * to determine the type.
 */
typedef union wasmtime_val_raw {
  /// Field for when this val is a WebAssembly `i32` value.
  ///
  /// Note that this field is always stored in a little-endian format.
  int32_t i32;
  /// Field for when this val is a WebAssembly `i64` value.
  ///
  /// Note that this field is always stored in a little-endian format.
  int64_t i64;
  /// Field for when this val is a WebAssembly `f32` value.
  ///
  /// Note that this field is always stored in a little-endian format.
  float32_t f32;
  /// Field for when this val is a WebAssembly `f64` value.
  ///
  /// Note that this field is always stored in a little-endian format.
  float64_t f64;
  /// Field for when this val is a WebAssembly `v128` value.
  ///
  /// Note that this field is always stored in a little-endian format.
  wasmtime_v128 v128;
  /// Field for when this val is a WebAssembly `anyref` value.
  ///
  /// If this is set to 0 then it's a null anyref, otherwise this must be
  /// passed to `wasmtime_anyref_from_raw` to determine the
  /// `wasmtime_anyref_t`.
  ///
  /// Note that this field is always stored in a little-endian format.
  uint32_t anyref;
  /// Field for when this val is a WebAssembly `externref` value.
  ///
  /// If this is set to 0 then it's a null externref, otherwise this must be
  /// passed to `wasmtime_externref_from_raw` to determine the
  /// `wasmtime_externref_t`.
  ///
  /// Note that this field is always stored in a little-endian format.
  uint32_t externref;
  /// Field for when this val is a WebAssembly `funcref` value.
  ///
  /// If this is set to 0 then it's a null funcref, otherwise this must be
  /// passed to `wasmtime_func_from_raw` to determine the `wasmtime_func_t`.
  ///
  /// Note that this field is always stored in a little-endian format.
  void *funcref;
} wasmtime_val_raw_t;

// Assert that the shape of this type is as expected since it needs to match
// Rust.
static inline void __wasmtime_val_assertions() {
  static_assert(sizeof(wasmtime_valunion_t) == 16, "should be 16-bytes large");
  static_assert(__alignof(wasmtime_valunion_t) == 8,
                "should be 8-byte aligned");
  static_assert(sizeof(wasmtime_val_raw_t) == 16, "should be 16 bytes large");
  static_assert(__alignof(wasmtime_val_raw_t) == 8, "should be 8-byte aligned");
}

/**
 * \typedef wasmtime_val_t
 * \brief Convenience alias for #wasmtime_val_t
 *
 * \union wasmtime_val
 * \brief Container for different kinds of wasm values.
 *
 * Note that this structure may contain an owned value, namely rooted GC
 * references, depending on the context in which this is used. APIs which
 * consume a #wasmtime_val_t do not take ownership, but APIs that return
 * #wasmtime_val_t require that #wasmtime_val_unroot is called to clean up
 * any possible GC roots in the value.
 */
typedef struct wasmtime_val {
  /// Discriminant of which field of #of is valid.
  wasmtime_valkind_t kind;
  /// Container for the extern item's value.
  wasmtime_valunion_t of;
} wasmtime_val_t;

/**
 * \brief Unroot the value contained by `val`.
 *
 * This function will unroot any GC references that `val` points to, for
 * example if it has the `WASMTIME_EXTERNREF` or `WASMTIME_ANYREF` kinds. This
 * function leaves `val` in an undefined state and it should not be used again
 * without re-initializing.
 *
 * This method does not need to be called for integers, floats, v128, or
 * funcref values.
 */
WASM_API_EXTERN void wasmtime_val_unroot(wasmtime_context_t *context,
                                         wasmtime_val_t *val);

/**
 * \brief Clones the value pointed to by `src` into the `dst` provided.
 *
 * This function will clone any rooted GC values in `src` and have them
 * newly rooted inside of `dst`. When using this API the `dst` should be
 * later unrooted with #wasmtime_val_unroot if it contains GC values.
 */
WASM_API_EXTERN void wasmtime_val_clone(wasmtime_context_t *context,
                                        const wasmtime_val_t *src,
                                        wasmtime_val_t *dst);

#ifdef __cplusplus
} // extern "C"
#endif

#endif // WASMTIME_VAL_H
