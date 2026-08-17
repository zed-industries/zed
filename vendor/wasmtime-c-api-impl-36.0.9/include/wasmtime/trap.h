/**
 * \file wasmtime/trap.h
 *
 * Wasmtime APIs for interacting with traps and extensions to #wasm_trap_t.
 */

#ifndef WASMTIME_TRAP_H
#define WASMTIME_TRAP_H

#include <wasm.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \brief Code of an instruction trap.
 *
 * See #wasmtime_trap_code_enum for possible values.
 */
typedef uint8_t wasmtime_trap_code_t;

/**
 * \brief Trap codes for instruction traps.
 */
enum wasmtime_trap_code_enum {
  /// The current stack space was exhausted.
  WASMTIME_TRAP_CODE_STACK_OVERFLOW,
  /// An out-of-bounds memory access.
  WASMTIME_TRAP_CODE_MEMORY_OUT_OF_BOUNDS,
  /// A wasm atomic operation was presented with a not-naturally-aligned
  /// linear-memory address.
  WASMTIME_TRAP_CODE_HEAP_MISALIGNED,
  /// An out-of-bounds access to a table.
  WASMTIME_TRAP_CODE_TABLE_OUT_OF_BOUNDS,
  /// Indirect call to a null table entry.
  WASMTIME_TRAP_CODE_INDIRECT_CALL_TO_NULL,
  /// Signature mismatch on indirect call.
  WASMTIME_TRAP_CODE_BAD_SIGNATURE,
  /// An integer arithmetic operation caused an overflow.
  WASMTIME_TRAP_CODE_INTEGER_OVERFLOW,
  /// An integer division by zero.
  WASMTIME_TRAP_CODE_INTEGER_DIVISION_BY_ZERO,
  /// Failed float-to-int conversion.
  WASMTIME_TRAP_CODE_BAD_CONVERSION_TO_INTEGER,
  /// Code that was supposed to have been unreachable was reached.
  WASMTIME_TRAP_CODE_UNREACHABLE_CODE_REACHED,
  /// Execution has potentially run too long and may be interrupted.
  WASMTIME_TRAP_CODE_INTERRUPT,
  /// When the `component-model` feature is enabled this trap represents a
  /// function that was `canon lift`'d, then `canon lower`'d, then called.
  /// This combination of creation of a function in the component model
  /// generates a function that always traps and, when called, produces this
  /// flavor of trap.
  WASMTIME_TRAP_CODE_ALWAYS_TRAP_ADAPTER,
  /// Execution has run out of the configured fuel amount.
  WASMTIME_TRAP_CODE_OUT_OF_FUEL,
  /// Used to indicate that a trap was raised by atomic wait operations on non
  /// shared memory.
  WASMTIME_TRAP_CODE_ATOMIC_WAIT_NON_SHARED_MEMORY,
  /// Call to a null reference.
  WASMTIME_TRAP_CODE_NULL_REFERENCE,
  /// Attempt to access beyond the bounds of an array.
  WASMTIME_TRAP_CODE_ARRAY_OUT_OF_BOUNDS,
  /// Attempted an allocation that was too large to succeed.
  WASMTIME_TRAP_CODE_ALLOCATION_TOO_LARGE,
  /// Attempted to cast a reference to a type that it is not an instance of.
  WASMTIME_TRAP_CODE_CAST_FAILURE,
  /// When the `component-model` feature is enabled this trap represents a
  /// scenario where one component tried to call another component but it
  /// would have violated the reentrance rules of the component model,
  /// triggering a trap instead.
  WASMTIME_TRAP_CODE_CANNOT_ENTER_COMPONENT,
  /// Async-lifted export failed to produce a result by calling `task.return`
  /// before returning `STATUS_DONE` and/or after all host tasks completed.
  WASMTIME_TRAP_CODE_NO_ASYNC_RESULT,
  /// A Pulley opcode was executed at runtime when the opcode was disabled at
  /// compile time.
  WASMTIME_TRAP_CODE_DISABLED_OPCODE,
};

/**
 * \brief Creates a new trap with the given message.
 *
 * \param msg the message to associate with this trap
 * \param msg_len the byte length of `msg`
 *
 * The #wasm_trap_t returned is owned by the caller.
 */
WASM_API_EXTERN wasm_trap_t *wasmtime_trap_new(const char *msg, size_t msg_len);

/**
 * \brief Creates a new trap from the given trap code.
 *
 * \param code the trap code to associate with this trap
 *
 * The #wasm_trap_t returned is owned by the caller.
 */
WASM_API_EXTERN wasm_trap_t *wasmtime_trap_new_code(wasmtime_trap_code_t code);

/**
 * \brief Attempts to extract the trap code from this trap.
 *
 * Returns `true` if the trap is an instruction trap triggered while
 * executing Wasm. If `true` is returned then the trap code is returned
 * through the `code` pointer. If `false` is returned then this is not
 * an instruction trap -- traps can also be created using wasm_trap_new,
 * or occur with WASI modules exiting with a certain exit code.
 */
WASM_API_EXTERN bool wasmtime_trap_code(const wasm_trap_t *,
                                        wasmtime_trap_code_t *code);

/**
 * \brief Returns a human-readable name for this frame's function.
 *
 * This function will attempt to load a human-readable name for function this
 * frame points to. This function may return `NULL`.
 *
 * The lifetime of the returned name is the same as the #wasm_frame_t itself.
 */
WASM_API_EXTERN const wasm_name_t *
wasmtime_frame_func_name(const wasm_frame_t *);

/**
 * \brief Returns a human-readable name for this frame's module.
 *
 * This function will attempt to load a human-readable name for module this
 * frame points to. This function may return `NULL`.
 *
 * The lifetime of the returned name is the same as the #wasm_frame_t itself.
 */
WASM_API_EXTERN const wasm_name_t *
wasmtime_frame_module_name(const wasm_frame_t *);

#ifdef __cplusplus
} // extern "C"
#endif

#endif // WASMTIME_TRAP_H
