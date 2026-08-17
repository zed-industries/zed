/**
 * \file wasmtime/table.hh
 */

#ifndef WASMTIME_TABLE_HH
#define WASMTIME_TABLE_HH

#include <optional>

#include <wasmtime/error.hh>
#include <wasmtime/store.hh>
#include <wasmtime/table.h>
#include <wasmtime/types/table.hh>
#include <wasmtime/types/val.hh>
#include <wasmtime/val.hh>

namespace wasmtime {

/**
 * \brief A WebAssembly table.
 *
 * This class represents a WebAssembly table, either created through
 * instantiating a module or a host table. Tables are contiguous vectors of
 * WebAssembly reference types, currently either `externref` or `funcref`.
 *
 * Note that this type does not itself own any resources. It points to resources
 * owned within a `Store` and the `Store` must be passed in as the first
 * argument to the functions defined on `Table`. Note that if the wrong `Store`
 * is passed in then the process will be aborted.
 */
class Table {
  friend class Instance;
  wasmtime_table_t table;

public:
  /// Creates a new table from the raw underlying C API representation.
  Table(wasmtime_table_t table) : table(table) {}

  /**
   * \brief Creates a new host-defined table.
   *
   * \param cx the store in which to create the table.
   * \param ty the type of the table to be created
   * \param init the initial value for all table slots.
   *
   * Returns an error if `init` has the wrong value for the `ty` specified.
   */
  static Result<Table> create(Store::Context cx, const TableType &ty,
                              const Val &init) {
    wasmtime_table_t table;
    auto *error = wasmtime_table_new(cx.ptr, ty.ptr.get(), &init.val, &table);
    if (error != nullptr) {
      return Error(error);
    }
    return Table(table);
  }

  /// Returns the type of this table.
  TableType type(Store::Context cx) const {
    return wasmtime_table_type(cx.ptr, &table);
  }

  /// Returns the size, in elements, that the table currently has.
  uint64_t size(Store::Context cx) const {
    return wasmtime_table_size(cx.ptr, &table);
  }

  /// Loads a value from the specified index in this table.
  ///
  /// Returns `std::nullopt` if `idx` is out of bounds.
  std::optional<Val> get(Store::Context cx, uint64_t idx) const {
    Val val;
    if (wasmtime_table_get(cx.ptr, &table, idx, &val.val)) {
      return std::optional(std::move(val));
    }
    return std::nullopt;
  }

  /// Stores a value into the specified index in this table.
  ///
  /// Returns an error if `idx` is out of bounds or if `val` has the wrong type.
  Result<std::monostate> set(Store::Context cx, uint64_t idx,
                             const Val &val) const {
    auto *error = wasmtime_table_set(cx.ptr, &table, idx, &val.val);
    if (error != nullptr) {
      return Error(error);
    }
    return std::monostate();
  }

  /// Grow this table.
  ///
  /// \param cx the store that owns this table.
  /// \param delta the number of new elements to be added to this table.
  /// \param init the initial value of all new elements in this table.
  ///
  /// Returns an error if `init` has the wrong type for this table. Otherwise
  /// returns the previous size of the table before growth.
  Result<uint64_t> grow(Store::Context cx, uint64_t delta,
                        const Val &init) const {
    uint64_t prev = 0;
    auto *error = wasmtime_table_grow(cx.ptr, &table, delta, &init.val, &prev);
    if (error != nullptr) {
      return Error(error);
    }
    return prev;
  }

  /// Returns the raw underlying C API table this is using.
  const wasmtime_table_t &capi() const { return table; }
};

} // namespace wasmtime

#endif // WASMTIME_TABLE_HH
