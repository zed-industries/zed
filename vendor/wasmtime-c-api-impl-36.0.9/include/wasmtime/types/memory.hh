/**
 * \file wasmtime/types/memory.hh
 */

#ifndef WASMTIME_TYPES_MEMORY_HH
#define WASMTIME_TYPES_MEMORY_HH

#include <memory>
#include <optional>
#include <wasm.h>
#include <wasmtime/memory.h>

namespace wasmtime {

/**
 * \brief Type information about a WebAssembly linear memory
 */
class MemoryType {
  friend class Memory;

  struct deleter {
    void operator()(wasm_memorytype_t *p) const { wasm_memorytype_delete(p); }
  };

  std::unique_ptr<wasm_memorytype_t, deleter> ptr;

public:
  /// \brief Non-owning reference to a `MemoryType`, must not be used after the
  /// original owner has been deleted.
  class Ref {
    friend class MemoryType;

    const wasm_memorytype_t *ptr;

  public:
    /// Creates a reference from the raw C API representation.
    Ref(const wasm_memorytype_t *ptr) : ptr(ptr) {}
    /// Creates a reference from an original `MemoryType`.
    Ref(const MemoryType &ty) : Ref(ty.ptr.get()) {}

    /// Returns the minimum size, in WebAssembly pages, of this memory.
    uint64_t min() const { return wasmtime_memorytype_minimum(ptr); }

    /// Returns the maximum size, in WebAssembly pages, of this memory, if
    /// specified.
    std::optional<uint64_t> max() const {
      uint64_t max = 0;
      auto present = wasmtime_memorytype_maximum(ptr, &max);
      if (present) {
        return max;
      }
      return std::nullopt;
    }

    /// Returns whether or not this is a 64-bit memory type.
    bool is_64() const { return wasmtime_memorytype_is64(ptr); }

    /// Returns whether or not this is a shared memory type.
    bool is_shared() const { return wasmtime_memorytype_isshared(ptr); }
  };

private:
  Ref ref;
  MemoryType(wasm_memorytype_t *ptr) : ptr(ptr), ref(ptr) {}

public:
  /// Creates a new 32-bit wasm memory type with the specified minimum number of
  /// pages for the minimum size. The created type will have no maximum memory
  /// size.
  explicit MemoryType(uint32_t min)
      : MemoryType(wasmtime_memorytype_new(min, false, 0, false, false)) {}
  /// Creates a new 32-bit wasm memory type with the specified minimum number of
  /// pages for the minimum size, and maximum number of pages for the max size.
  MemoryType(uint32_t min, uint32_t max)
      : MemoryType(wasmtime_memorytype_new(min, true, max, false, false)) {}

  /// Same as the `MemoryType` constructor, except creates a 64-bit memory.
  static MemoryType New64(uint64_t min) {
    return MemoryType(wasmtime_memorytype_new(min, false, 0, true, false));
  }

  /// Same as the `MemoryType` constructor, except creates a 64-bit memory.
  static MemoryType New64(uint64_t min, uint64_t max) {
    return MemoryType(wasmtime_memorytype_new(min, true, max, true, false));
  }

  /// Creates a new wasm memory type from the specified ref, making a fresh
  /// owned value.
  MemoryType(Ref other) : MemoryType(wasm_memorytype_copy(other.ptr)) {}
  /// Copies the provided type into a new type.
  MemoryType(const MemoryType &other)
      : MemoryType(wasm_memorytype_copy(other.ptr.get())) {}
  /// Copies the provided type into a new type.
  MemoryType &operator=(const MemoryType &other) {
    ptr.reset(wasm_memorytype_copy(other.ptr.get()));
    return *this;
  }
  ~MemoryType() = default;
  /// Moves the type information from another type into this one.
  MemoryType(MemoryType &&other) = default;
  /// Moves the type information from another type into this one.
  MemoryType &operator=(MemoryType &&other) = default;

  /// \brief Returns the underlying `Ref`, a non-owning reference pointing to
  /// this instance.
  Ref *operator->() { return &ref; }
  /// \brief Returns the underlying `Ref`, a non-owning reference pointing to
  /// this instance.
  Ref *operator*() { return &ref; }

  /// \brief Helper class to build a `MemoryType`.
  class Builder {
    uint64_t _min;
    std::optional<uint64_t> _max;
    bool _memory64;
    bool _shared;

  public:
    /// \brief Default constructor for a memory type with 0 initial size.
    Builder() : _min(0), _memory64(false), _shared(false) {}

    /// \brief Configure the minimum size, in pages, of linear memory.
    Builder &min(uint64_t min) {
      _min = min;
      return *this;
    }

    /// \brief Configure the maximal size, in pages, of linear memory.
    Builder &max(std::optional<uint64_t> max) {
      _max = max;
      return *this;
    }

    /// \brief Configure whether this is a 64-bit linear memory.
    Builder &memory64(bool enable) {
      _memory64 = enable;
      return *this;
    }

    /// \brief Configure whether this is a shared linear memory.
    Builder &shared(bool enable) {
      _shared = enable;
      return *this;
    }

    /// \brief Construct the final `MemoryType` value.
    MemoryType build() const {
      return MemoryType(wasmtime_memorytype_new(_min, _max.has_value(),
                                                _max.has_value() ? *_max : 0,
                                                _memory64, _shared));
    }
  };
};

}; // namespace wasmtime

#endif // WASMTIME_TYPES_MEMORY_HH
