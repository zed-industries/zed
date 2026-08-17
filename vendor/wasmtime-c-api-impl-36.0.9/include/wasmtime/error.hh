/**
 * \file wasmtime/error.hh
 */

#ifndef WASMTIME_ERROR_HH
#define WASMTIME_ERROR_HH

#include <memory>
#include <optional>
#include <ostream>
#include <string>
#include <variant>
#include <wasmtime/error.h>

namespace wasmtime {

class Trace;

/**
 * \brief Errors coming from Wasmtime
 *
 * This class represents an error that came from Wasmtime and contains a textual
 * description of the error that occurred.
 */
class Error {
  struct deleter {
    void operator()(wasmtime_error_t *p) const { wasmtime_error_delete(p); }
  };

  std::unique_ptr<wasmtime_error_t, deleter> ptr;

public:
  /// \brief Creates an error from the raw C API representation
  ///
  /// Takes ownership of the provided `error`.
  Error(wasmtime_error_t *error) : ptr(error) {}

  /// \brief Creates an error with the provided message.
  Error(const std::string &s) : ptr(wasmtime_error_new(s.c_str())) {}

  /// \brief Returns the error message associated with this error.
  std::string message() const {
    wasm_byte_vec_t msg_bytes;
    wasmtime_error_message(ptr.get(), &msg_bytes);
    auto ret = std::string(msg_bytes.data, msg_bytes.size);
    wasm_byte_vec_delete(&msg_bytes);
    return ret;
  }

  /// If this trap represents a call to `exit` for WASI, this will return the
  /// optional error code associated with the exit trap.
  std::optional<int32_t> i32_exit() const {
    int32_t status = 0;
    if (wasmtime_error_exit_status(ptr.get(), &status)) {
      return status;
    }
    return std::nullopt;
  }

  /// Returns the trace of WebAssembly frames associated with this error.
  ///
  /// Note that the `trace` cannot outlive this error object.
  Trace trace() const;

  /// Release ownership of this error, acquiring the underlying C raw pointer.
  wasmtime_error_t *release() { return ptr.release(); }
};

/// \brief Used to print an error.
inline std::ostream &operator<<(std::ostream &os, const Error &e) {
  os << e.message();
  return os;
}

/**
 * \brief Fallible result type used for Wasmtime.
 *
 * This type is used as the return value of many methods in the Wasmtime API.
 * This behaves similarly to Rust's `Result<T, E>` and will be replaced with a
 * C++ standard when it exists.
 */
template <typename T, typename E = Error> class [[nodiscard]] Result {
  std::variant<T, E> data;

public:
  /// \brief Creates a `Result` from its successful value.
  Result(T t) : data(std::move(t)) {}
  /// \brief Creates a `Result` from an error value.
  Result(E e) : data(std::move(e)) {}

  /// \brief Returns `true` if this result is a success, `false` if it's an
  /// error
  explicit operator bool() const { return data.index() == 0; }

  /// \brief Returns the error, if present, aborts if this is not an error.
  E &&err() { return std::get<E>(std::move(data)); }
  /// \brief Returns the error, if present, aborts if this is not an error.
  const E &&err() const { return std::get<E>(std::move(data)); }

  /// \brief Returns the success, if present, aborts if this is an error.
  T &&ok() { return std::get<T>(std::move(data)); }
  /// \brief Returns the success, if present, aborts if this is an error.
  const T &&ok() const { return std::get<T>(std::move(data)); }

  /// \brief Returns the success, if present, aborts if this is an error.
  T unwrap() {
    if (*this) {
      return this->ok();
    }
    unwrap_failed();
  }

private:
  [[noreturn]] void unwrap_failed() {
    fprintf(stderr, "error: %s\n", this->err().message().c_str()); // NOLINT
    std::abort();
  }
};

} // namespace wasmtime

#endif // WASMTIME_ERROR_HH
