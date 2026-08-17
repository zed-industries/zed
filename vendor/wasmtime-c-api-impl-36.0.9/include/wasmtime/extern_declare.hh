// A separate "private" header different from `extern.hh` which is used to
// help break the cycle between `extern.hh` and `func.hh`.

#ifndef WASMTIME_EXTERN_DECLARE_HH
#define WASMTIME_EXTERN_DECLARE_HH

#include <variant>

namespace wasmtime {

class Global;
class Func;
class Memory;
class Table;

/// \typedef Extern
/// \brief Representation of an external WebAssembly item
typedef std::variant<Func, Global, Memory, Table> Extern;

} // namespace wasmtime

#endif // WASMTIME_EXTERN_DECLARE_HH
