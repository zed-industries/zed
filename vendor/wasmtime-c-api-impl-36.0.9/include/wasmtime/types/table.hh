/**
 * \file wasmtime/types/table.hh
 */

#ifndef WASMTIME_TYPES_TABLE_HH
#define WASMTIME_TYPES_TABLE_HH

#include <optional>
#include <wasmtime/types/val.hh>

namespace wasmtime {

/**
 * \brief Type information about a WebAssembly table.
 */
class TableType {
  friend class Table;

  struct deleter {
    void operator()(wasm_tabletype_t *p) const { wasm_tabletype_delete(p); }
  };

  std::unique_ptr<wasm_tabletype_t, deleter> ptr;

public:
  /// Non-owning reference to a `TableType`, must not be used after the original
  /// owner is deleted.
  class Ref {
    friend class TableType;

    const wasm_tabletype_t *ptr;

  public:
    /// Creates a reference from the raw underlying C API representation.
    Ref(const wasm_tabletype_t *ptr) : ptr(ptr) {}
    /// Creates a reference to the provided `TableType`.
    Ref(const TableType &ty) : Ref(ty.ptr.get()) {}

    /// Returns the minimum size of this table type, in elements.
    uint32_t min() const { return wasm_tabletype_limits(ptr)->min; }

    /// Returns the maximum size of this table type, in elements, if present.
    std::optional<uint32_t> max() const {
      const auto *limits = wasm_tabletype_limits(ptr);
      if (limits->max == wasm_limits_max_default) {
        return std::nullopt;
      }
      return limits->max;
    }

    /// Returns the type of value that is stored in this table.
    ValType::Ref element() const { return wasm_tabletype_element(ptr); }
  };

private:
  Ref ref;
  TableType(wasm_tabletype_t *ptr) : ptr(ptr), ref(ptr) {}

public:
  /// Creates a new table type from the specified value type and minimum size.
  /// The returned table will have no maximum size.
  TableType(ValType ty, uint32_t min) : ptr(nullptr), ref(nullptr) {
    wasm_limits_t limits;
    limits.min = min;
    limits.max = wasm_limits_max_default;
    ptr.reset(wasm_tabletype_new(ty.ptr.release(), &limits));
    ref = ptr.get();
  }
  /// Creates a new table type from the specified value type, minimum size, and
  /// maximum size.
  TableType(ValType ty, uint32_t min, uint32_t max) // NOLINT
      : ptr(nullptr), ref(nullptr) {
    wasm_limits_t limits;
    limits.min = min;
    limits.max = max;
    ptr.reset(wasm_tabletype_new(ty.ptr.release(), &limits));
    ref = ptr.get();
  }
  /// Clones the given reference into a new table type.
  TableType(Ref other) : TableType(wasm_tabletype_copy(other.ptr)) {}
  /// Copies another table type into this one.
  TableType(const TableType &other)
      : TableType(wasm_tabletype_copy(other.ptr.get())) {}
  /// Copies another table type into this one.
  TableType &operator=(const TableType &other) {
    ptr.reset(wasm_tabletype_copy(other.ptr.get()));
    return *this;
  }
  ~TableType() = default;
  /// Moves the table type resources from another type to this one.
  TableType(TableType &&other) = default;
  /// Moves the table type resources from another type to this one.
  TableType &operator=(TableType &&other) = default;

  /// \brief Returns the underlying `Ref`, a non-owning reference pointing to
  /// this instance.
  Ref *operator->() { return &ref; }
  /// \brief Returns the underlying `Ref`, a non-owning reference pointing to
  /// this instance.
  Ref *operator*() { return &ref; }
};

}; // namespace wasmtime

#endif // WASMTIME_TYPES_TABLE_HH
