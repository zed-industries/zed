/**
 * \file wasmtime/profiling.h
 *
 * \brief API for Wasmtime guest profiler
 */

#ifndef WASMTIME_PROFILING_H
#define WASMTIME_PROFILING_H

#include <wasm.h>
#include <wasmtime/conf.h>
#include <wasmtime/error.h>
#include <wasmtime/module.h>
#include <wasmtime/store.h>

#ifdef WASMTIME_FEATURE_PROFILING

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \brief Collects basic profiling data for a single WebAssembly guest.
 *
 * To use this, you’ll need to arrange to call #wasmtime_guestprofiler_sample at
 * regular intervals while the guest is on the stack. The most straightforward
 * way to do that is to call it from a callback registered with
 * #wasmtime_store_epoch_deadline_callback.
 *
 * For more information see the Rust documentation at:
 * https://docs.wasmtime.dev/api/wasmtime/struct.GuestProfiler.html
 */
typedef struct wasmtime_guestprofiler wasmtime_guestprofiler_t;

/**
 * \brief Deletes profiler without finishing it.
 *
 * \param guestprofiler profiler that is being deleted
 */
WASM_API_EXTERN void wasmtime_guestprofiler_delete(
    /* own */ wasmtime_guestprofiler_t *guestprofiler);

/**
 * \typedef wasmtime_guestprofiler_modules_t
 * \brief Alias to #wasmtime_guestprofiler_modules
 *
 * \struct #wasmtime_guestprofiler_modules
 * \brief Tuple of name and module for passing into #wasmtime_guestprofiler_new.
 */
typedef struct wasmtime_guestprofiler_modules {
  const wasm_name_t *name; //!< Name recorded in the profile.
  const wasmtime_module_t
      *mod; //!< Module that is being allowed to appear in captured stack trace.
} wasmtime_guestprofiler_modules_t;

/**
 * \brief Begin profiling a new guest.
 *
 * \param module_name    name recorded in the profile
 * \param interval_nanos intended sampling interval in nanoseconds recorded in
 *                       the profile
 * \param modules        modules and associated names that will appear in
 *                       captured stack traces, pointer to the first element
 * \param modules_len    count of elements in `modules`
 *
 * \return Created profiler that is owned by the caller.
 *
 * This function does not take ownership of the arguments.
 *
 * For more information see the Rust documentation at:
 * https://docs.wasmtime.dev/api/wasmtime/struct.GuestProfiler.html#method.new
 */
WASM_API_EXTERN /* own */ wasmtime_guestprofiler_t *wasmtime_guestprofiler_new(
    const wasm_name_t *module_name, uint64_t interval_nanos,
    const wasmtime_guestprofiler_modules_t *modules, size_t modules_len);

/**
 * \brief Add a sample to the profile.
 *
 * \param guestprofiler the profiler the sample is being added to
 * \param store         store that is being used to collect the backtraces
 * \param delta_nanos   CPU time in nanoseconds that was used by this guest
 *                      since the previous sample
 *
 * Zero can be passed as `delta_nanos` if recording CPU usage information
 * is not needed.
 * This function does not take ownership of the arguments.
 *
 * For more information see the Rust documentation at:
 * https://docs.wasmtime.dev/api/wasmtime/struct.GuestProfiler.html#method.sample
 */
WASM_API_EXTERN void
wasmtime_guestprofiler_sample(wasmtime_guestprofiler_t *guestprofiler,
                              const wasmtime_store_t *store,
                              uint64_t delta_nanos);

/**
 * \brief Writes out the captured profile.
 *
 * \param guestprofiler the profiler which is being finished and deleted
 * \param out           pointer to where #wasm_byte_vec_t containing generated
 *                      file will be written
 *
 * \return Returns #wasmtime_error_t owned by the caller in case of error,
 * `NULL` otherwise.
 *
 * This function takes ownership of `guestprofiler`, even when error is
 * returned.
 * Only when returning without error `out` is filled with #wasm_byte_vec_t owned
 * by the caller.
 *
 * For more information see the Rust documentation at:
 * https://docs.wasmtime.dev/api/wasmtime/struct.GuestProfiler.html#method.finish
 */
WASM_API_EXTERN /* own */ wasmtime_error_t *
wasmtime_guestprofiler_finish(/* own */ wasmtime_guestprofiler_t *guestprofiler,
                              /* own */ wasm_byte_vec_t *out);

#ifdef __cplusplus
} // extern "C"
#endif

#endif // WASMTIME_FEATURE_PROFILING

#endif // WASMTIME_PROFILING_H
