/**
 * \file wasmtime/trap.hh
 */

#ifndef WASMTIME_TRAP_HH
#define WASMTIME_TRAP_HH

#include <wasmtime/error.hh>
#include <wasmtime/trap.h>

namespace wasmtime {

/**
 * \brief Non-owning reference to a WebAssembly function frame as part of a
 * `Trace`
 *
 * A `FrameRef` represents a WebAssembly function frame on the stack which was
 * collected as part of a trap.
 */
class FrameRef {
  wasm_frame_t *frame;

public:
  /// Returns the WebAssembly function index of this function, in the original
  /// module.
  uint32_t func_index() const { return wasm_frame_func_index(frame); }
  /// Returns the offset, in bytes from the start of the function in the
  /// original module, to this frame's program counter.
  size_t func_offset() const { return wasm_frame_func_offset(frame); }
  /// Returns the offset, in bytes from the start of the original module,
  /// to this frame's program counter.
  size_t module_offset() const { return wasm_frame_module_offset(frame); }

  /// Returns the name, if present, associated with this function.
  ///
  /// Note that this requires that the `name` section is present in the original
  /// WebAssembly binary.
  std::optional<std::string_view> func_name() const {
    const auto *name = wasmtime_frame_func_name(frame);
    if (name != nullptr) {
      return std::string_view(name->data, name->size);
    }
    return std::nullopt;
  }

  /// Returns the name, if present, associated with this function's module.
  ///
  /// Note that this requires that the `name` section is present in the original
  /// WebAssembly binary.
  std::optional<std::string_view> module_name() const {
    const auto *name = wasmtime_frame_module_name(frame);
    if (name != nullptr) {
      return std::string_view(name->data, name->size);
    }
    return std::nullopt;
  }
};

/**
 * \brief An owned vector of `FrameRef` instances representing the WebAssembly
 * call-stack on a trap.
 *
 * This can be used to iterate over the frames of a trap and determine what was
 * running when a trap happened.
 */
class Trace {
  friend class Trap;
  friend class Error;

  wasm_frame_vec_t vec;

  Trace(wasm_frame_vec_t vec) : vec(vec) {}

public:
  ~Trace() { wasm_frame_vec_delete(&vec); }

  Trace(const Trace &other) = delete;
  Trace(Trace &&other) = delete;
  Trace &operator=(const Trace &other) = delete;
  Trace &operator=(Trace &&other) = delete;

  /// Iterator used to iterate over this trace.
  typedef const FrameRef *iterator;

  /// Returns the start of iteration
  iterator begin() const {
    return reinterpret_cast<FrameRef *>(&vec.data[0]); // NOLINT
  }
  /// Returns the end of iteration
  iterator end() const {
    return reinterpret_cast<FrameRef *>(&vec.data[vec.size]); // NOLINT
  }
  /// Returns the size of this trace, or how many frames it contains.
  size_t size() const { return vec.size; }
};

inline Trace Error::trace() const {
  wasm_frame_vec_t frames;
  wasmtime_error_wasm_trace(ptr.get(), &frames);
  return Trace(frames);
}

/**
 * \brief Information about a WebAssembly trap.
 *
 * Traps can happen during normal wasm execution (such as the `unreachable`
 * instruction) but they can also happen in host-provided functions to a host
 * function can simulate raising a trap.
 *
 * Traps have a message associated with them as well as a trace of WebAssembly
 * frames on the stack.
 */
class Trap {
  friend class Linker;
  friend class Instance;
  friend class Func;
  template <typename Params, typename Results> friend class TypedFunc;

  struct deleter {
    void operator()(wasm_trap_t *p) const { wasm_trap_delete(p); }
  };

  std::unique_ptr<wasm_trap_t, deleter> ptr;

  Trap(wasm_trap_t *ptr) : ptr(ptr) {}

public:
  /// Creates a new host-defined trap with the specified message.
  explicit Trap(std::string_view msg)
      : Trap(wasmtime_trap_new(msg.data(), msg.size())) {}

  /// Creates a new trap with the given wasmtime trap code.
  Trap(wasmtime_trap_code_enum code) : Trap(wasmtime_trap_new_code(code)) {}

  /// Returns the descriptive message associated with this trap.
  std::string message() const {
    wasm_byte_vec_t msg;
    wasm_trap_message(ptr.get(), &msg);
    std::string ret(msg.data, msg.size - 1);
    wasm_byte_vec_delete(&msg);
    return ret;
  }

  /// Returns the trace of WebAssembly frames associated with this trap.
  ///
  /// Note that the `trace` cannot outlive this error object.
  Trace trace() const {
    wasm_frame_vec_t frames;
    wasm_trap_trace(ptr.get(), &frames);
    return Trace(frames);
  }

  /// \brief Returns the trap code associated with this trap, or nothing if
  /// it was a manually created trap.
  std::optional<wasmtime_trap_code_t> code() const {
    wasmtime_trap_code_t code;
    bool present = wasmtime_trap_code(ptr.get(), &code);
    if (present)
      return code;
    return std::nullopt;
  }
};

/// Structure used to represent either a `Trap` or an `Error`.
struct TrapError {
  /// Storage for what this trap represents.
  std::variant<Trap, Error> data;

  /// Creates a new `TrapError` from a `Trap`
  TrapError(Trap t) : data(std::move(t)) {}
  /// Creates a new `TrapError` from an `Error`
  TrapError(Error e) : data(std::move(e)) {}

  /// Dispatches internally to return the message associated with this error.
  std::string message() const {
    if (const auto *trap = std::get_if<Trap>(&data)) {
      return trap->message();
    }
    if (const auto *error = std::get_if<Error>(&data)) {
      return std::string(error->message());
    }
    std::abort();
  }
};

/// Result used by functions which can fail because of invariants being violated
/// (such as a type error) as well as because of a WebAssembly trap.
template <typename T> using TrapResult = Result<T, TrapError>;

} // namespace wasmtime

#endif // WASMTIME_TRAP_HH
