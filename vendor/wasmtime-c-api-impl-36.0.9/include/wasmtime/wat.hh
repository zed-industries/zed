/**
 * \file wasmtime/wat.hh
 */

#ifndef WASMTIME_WAT_HH
#define WASMTIME_WAT_HH

#include <string_view>
#include <vector>
#include <wasmtime/conf.h>
#include <wasmtime/error.hh>
#include <wasmtime/span.hh>
#include <wasmtime/wat.h>

namespace wasmtime {

#ifdef WASMTIME_FEATURE_WAT

/**
 * \brief Converts the WebAssembly text format into the WebAssembly binary
 * format.
 *
 * This will parse the text format and attempt to translate it to the binary
 * format. Note that the text parser assumes that all WebAssembly features are
 * enabled and will parse syntax of future proposals. The exact syntax here
 * parsed may be tweaked over time.
 *
 * Returns either an error if parsing failed or the wasm binary.
 */
inline Result<std::vector<uint8_t>> wat2wasm(std::string_view wat) {
  wasm_byte_vec_t ret;
  auto *error = wasmtime_wat2wasm(wat.data(), wat.size(), &ret);
  if (error != nullptr) {
    return Error(error);
  }
  std::vector<uint8_t> vec;
  // NOLINTNEXTLINE TODO can this be done without triggering lints?
  Span<uint8_t> raw(reinterpret_cast<uint8_t *>(ret.data), ret.size);
  vec.assign(raw.begin(), raw.end());
  wasm_byte_vec_delete(&ret);
  return vec;
}

#endif // WASMTIME_FEATURE_WAT

} // namespace wasmtime

#endif // WASMTIME_WAT_HH
