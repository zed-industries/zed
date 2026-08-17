/**
 * \file wasmtime/extern.hh
 */

#ifndef WASMTIME_EXTERN_HH
#define WASMTIME_EXTERN_HH

#include <wasmtime/extern.h>
#include <wasmtime/extern_declare.hh>
#include <wasmtime/func.hh>
#include <wasmtime/global.hh>
#include <wasmtime/memory.hh>
#include <wasmtime/table.hh>

namespace wasmtime {

// Internal helpers for converting between `Extern`, a `std::variant`, and
// `wasmtime_extern_t`.
namespace detail {
static Extern cvt_extern(wasmtime_extern_t &e) {
  switch (e.kind) {
  case WASMTIME_EXTERN_FUNC:
    return Func(e.of.func);
  case WASMTIME_EXTERN_GLOBAL:
    return Global(e.of.global);
  case WASMTIME_EXTERN_MEMORY:
    return Memory(e.of.memory);
  case WASMTIME_EXTERN_TABLE:
    return Table(e.of.table);
  }
  std::abort();
}

static void cvt_extern(const Extern &e, wasmtime_extern_t &raw) {
  if (const auto *func = std::get_if<Func>(&e)) {
    raw.kind = WASMTIME_EXTERN_FUNC;
    raw.of.func = func->capi();
  } else if (const auto *global = std::get_if<Global>(&e)) {
    raw.kind = WASMTIME_EXTERN_GLOBAL;
    raw.of.global = global->capi();
  } else if (const auto *table = std::get_if<Table>(&e)) {
    raw.kind = WASMTIME_EXTERN_TABLE;
    raw.of.table = table->capi();
  } else if (const auto *memory = std::get_if<Memory>(&e)) {
    raw.kind = WASMTIME_EXTERN_MEMORY;
    raw.of.memory = memory->capi();
  } else {
    std::abort();
  }
}
} // namespace detail

inline std::optional<Extern> Caller::get_export(std::string_view name) {
  wasmtime_extern_t item;
  if (wasmtime_caller_export_get(ptr, name.data(), name.size(), &item)) {
    return detail::cvt_extern(item);
  }
  return std::nullopt;
}

} // namespace wasmtime

#endif // WASMTIME_EXTERN_HH
