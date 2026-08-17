/**
 * \file wasmtime/types/import.hh
 */

#ifndef WASMTIME_TYPES_IMPORT_HH
#define WASMTIME_TYPES_IMPORT_HH

#include <string_view>
#include <wasm.h>

namespace wasmtime {

/**
 * \brief Type information about a WebAssembly import.
 */
class ImportType {
public:
  /// Non-owning reference to an `ImportType`, must not be used after the
  /// original owner is deleted.
  class Ref {
    friend class ExternType;

    const wasm_importtype_t *ptr;

    // TODO: can this circle be broken another way?
    const wasm_externtype_t *raw_type() { return wasm_importtype_type(ptr); }

  public:
    /// Creates a new reference from the raw underlying C API representation.
    Ref(const wasm_importtype_t *ptr) : ptr(ptr) {}

    /// Returns the module name associated with this import.
    std::string_view module() {
      const auto *name = wasm_importtype_module(ptr);
      return std::string_view(name->data, name->size);
    }

    /// Returns the field name associated with this import.
    std::string_view name() {
      const auto *name = wasm_importtype_name(ptr);
      return std::string_view(name->data, name->size);
    }
  };

  /// An owned list of `ImportType` instances.
  class List {
    friend class Module;
    wasm_importtype_vec_t list;

  public:
    /// Creates an empty list
    List() : list{} {
      list.size = 0;
      list.data = nullptr;
    }
    List(const List &other) = delete;
    /// Moves another list into this one.
    List(List &&other) noexcept : list(other.list) { other.list.size = 0; }
    ~List() {
      if (list.size > 0) {
        wasm_importtype_vec_delete(&list);
      }
    }

    List &operator=(const List &other) = delete;
    /// Moves another list into this one.
    List &operator=(List &&other) noexcept {
      std::swap(list, other.list);
      return *this;
    }

    /// Iterator type, which is a list of non-owning `ImportType::Ref`
    /// instances.
    typedef const Ref *iterator;
    /// Returns the start of iteration.
    iterator begin() const {
      return reinterpret_cast<iterator>(&list.data[0]); // NOLINT
    }
    /// Returns the end of iteration.
    iterator end() const {
      return reinterpret_cast<iterator>(&list.data[list.size]); // NOLINT
    }
    /// Returns the size of this list.
    size_t size() const { return list.size; }
  };
};

}; // namespace wasmtime

#endif // WASMTIME_TYPES_IMPORT_HH
