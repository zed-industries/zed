/**
 * \file wasmtime/types/extern.hh
 */

#ifndef WASMTIME_TYPES_EXTERN_HH
#define WASMTIME_TYPES_EXTERN_HH

#include <variant>
#include <wasm.h>
#include <wasmtime/types/export.hh>
#include <wasmtime/types/func.hh>
#include <wasmtime/types/global.hh>
#include <wasmtime/types/import.hh>
#include <wasmtime/types/memory.hh>
#include <wasmtime/types/table.hh>

namespace wasmtime {

/**
 * \brief Generic type of a WebAssembly item.
 */
class ExternType {
  friend class ExportType;
  friend class ImportType;

public:
  /// \typedef Ref
  /// \brief Non-owning reference to an item's type
  ///
  /// This cannot be used after the original owner has been deleted, and
  /// otherwise this is used to determine what the actual type of the outer item
  /// is.
  typedef std::variant<FuncType::Ref, GlobalType::Ref, TableType::Ref,
                       MemoryType::Ref>
      Ref;

  /// Extract the type of the item imported by the provided type.
  static Ref from_import(ImportType::Ref ty) {
    // TODO: this would ideally be some sort of implicit constructor, unsure how
    // to do that though...
    return ref_from_c(ty.raw_type());
  }

  /// Extract the type of the item exported by the provided type.
  static Ref from_export(ExportType::Ref ty) {
    // TODO: this would ideally be some sort of implicit constructor, unsure how
    // to do that though...
    return ref_from_c(ty.raw_type());
  }

private:
  static Ref ref_from_c(const wasm_externtype_t *ptr) {
    switch (wasm_externtype_kind(ptr)) {
    case WASM_EXTERN_FUNC:
      return wasm_externtype_as_functype_const(ptr);
    case WASM_EXTERN_GLOBAL:
      return wasm_externtype_as_globaltype_const(ptr);
    case WASM_EXTERN_TABLE:
      return wasm_externtype_as_tabletype_const(ptr);
    case WASM_EXTERN_MEMORY:
      return wasm_externtype_as_memorytype_const(ptr);
    }
    std::abort();
  }
};

}; // namespace wasmtime

#endif // WASMTIME_TYPES_EXTERN_HH
