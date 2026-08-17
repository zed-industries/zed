// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MEMORY_STRUCTURED_SHARED_MEMORY_H_
#define BASE_MEMORY_STRUCTURED_SHARED_MEMORY_H_

#include <atomic>
#include <memory>
#include <optional>
#include <utility>

#include "base/check.h"
#include "base/compiler_specific.h"
#include "base/containers/span.h"
#include "base/memory/read_only_shared_memory_region.h"
#include "base/memory/shared_memory_mapper.h"
#include "base/memory/shared_memory_mapping.h"
#include "base/memory/shared_memory_safety_checker.h"

namespace base {

// `StructuredSharedMemory` wraps a handle to a shared memory region, and a
// writable mapping of that region sized and aligned to hold a type `T`. Only
// the process that creates the memory region can write to it, but it can pass
// read-only handles to other processes for reading.
//
// The caller must ensure that reads from other processes are synchronized with
// writes to the memory, such as by using a shared lock or storing std::atomic
// types in the memory region. As a convenience, `AtomicSharedMemory<T>` is an
// alias for `StructuredSharedMemory<std::atomic<T>>`.
//
// If `T` is a struct, the caller should ensure that it has no padding that
// could leak information, and that each member is safe to use over shared
// memory. SharedMemorySafetyChecker is helpful for this.
//
// Example of use:
//
// In the browser process:
//
//   optional<AtomicSharedMemory<TimeTicks>> shared_timestamp_memory =
//       AtomicSharedMemory<TimeTicks>::Create(TimeTicks::Now());
//   if (!shared_timestamp_memory) {
//     HandleFailedToMapMemoryError();
//     return;
//   }
//   PassRegionHandleToChild(shared_timestamp_memory->TakeReadOnlyRegion());
//   ...
//   // When an event occurs:
//   shared_timestamp_memory->WritableRef().store(TimeTicks::Now(),
//       std::memory_order_relaxed);
//   ...
//   // Destroying the StructuredSharedMemory will unmap the memory from this
//   // process. The child will still have a mapping.
//   shared_timestamp_memory.reset();
//
// In the child process:
//
//   optional<AtomicSharedMemory<TimeTicks>::ReadOnlyMapping>
//       shared_timestamp_mapping =
//           AtomicSharedMemory<TimeTicks>::MapReadOnlyRegion(region_handle);
//   if (!shared_timestamp_mapping) {
//     HandleFailedToMapMemoryError();
//     return;
//   }
//   ...
//   // Periodically check the timestamp.
//   TimeTicks event_time = shared_timestamp_mapping->ReadOnlyRef().load(
//        std::memory_order_relaxed);
//   ...
//
// TODO(crbug.com/357945779): Find a way to automatically validate struct
// members, or find another way to safely store multiple types in the same
// region.
//
// TODO(crbug.com/357945779): Allow multiple copies of T, with accessors that
// return span<T>.
template <typename T>
class StructuredSharedMemory {
 public:
  class ReadOnlyMapping;

  // Move-only.
  StructuredSharedMemory(const StructuredSharedMemory&) = delete;
  StructuredSharedMemory& operator=(const StructuredSharedMemory&) = delete;
  StructuredSharedMemory(StructuredSharedMemory&&) = default;
  StructuredSharedMemory& operator=(StructuredSharedMemory&&) = default;

  // Creates and maps a default-initialized shared memory region. Returns
  // nullopt if the region couldn't be created or mapped.
  static std::optional<StructuredSharedMemory> Create();

  // Creates and maps a shared memory region initialized with `initial_value`.
  // Returns nullopt if the region couldn't be created or mapped.
  template <typename U>
  static std::optional<StructuredSharedMemory> Create(U&& initial_value);

  // As Create(), but uses `mapper` to map and later unmap the region.
  static std::optional<StructuredSharedMemory> CreateWithCustomMapper(
      SharedMemoryMapper* mapper);

  // As Create<U>(), but uses `mapper` to map and later unmap the region.
  template <typename U>
  static std::optional<StructuredSharedMemory> CreateWithCustomMapper(
      U&& initial_value,
      SharedMemoryMapper* mapper);

  // Returns a read-only view of `region`, or nullopt if `region` couldn't be
  // mapped or can't contain a T. `region` should be a handle returned by
  // TakeReadOnlyRegion() or DuplicateReadOnlyRegion().
  static std::optional<ReadOnlyMapping> MapReadOnlyRegion(
      ReadOnlySharedMemoryRegion region,
      SharedMemoryMapper* mapper = nullptr);

  // Returns a pointer to the object stored in the mapped region.
  T* WritablePtr() {
    CHECK(writable_mapping_.IsValid());
    return writable_mapping_.GetMemoryAs<T>();
  }
  const T* ReadOnlyPtr() const {
    CHECK(writable_mapping_.IsValid());
    return writable_mapping_.GetMemoryAs<const T>();
  }

  // Returns a reference to the object stored in the mapped region.
  T& WritableRef() LIFETIME_BOUND {
    T* ptr = WritablePtr();
    CHECK(ptr);
    return *ptr;
  }
  const T& ReadOnlyRef() const LIFETIME_BOUND {
    const T* ptr = ReadOnlyPtr();
    CHECK(ptr);
    return *ptr;
  }

  // Extracts and returns a read-only handle to the memory region that can be
  // passed to other processes. After calling this, further calls to
  // TakeReadOnlyRegion() or DuplicateReadOnlyRegion() will fail with a CHECK.
  ReadOnlySharedMemoryRegion TakeReadOnlyRegion() {
    CHECK(read_only_region_.IsValid());
    return std::move(read_only_region_);
  }

  // Duplicates and returns a read-only handle to the memory region that can be
  // passed to other processes. After calling this, further calls to
  // TakeReadOnlyRegion() or DuplicateReadOnlyRegion() will succeed.
  ReadOnlySharedMemoryRegion DuplicateReadOnlyRegion() const {
    CHECK(read_only_region_.IsValid());
    return read_only_region_.Duplicate();
  }

 private:
  explicit StructuredSharedMemory(MappedReadOnlyRegion mapped_region)
      : read_only_region_(std::move(mapped_region.region)),
        writable_mapping_(std::move(mapped_region.mapping)) {}

  ReadOnlySharedMemoryRegion read_only_region_;
  WritableSharedMemoryMapping writable_mapping_;
};

// A read-only mapping of a shared memory region, sized and aligned to hold a
// list types `T`. This is intended for use with a ReadOnlySharedMemoryRegion
// created by StructuredSharedMemory<T>.
//
// Although this view of the memory is read-only, the memory can be modified by
// the process holding the StructuredSharedMemory at any time. So all reads must
// be synchronized with the writes, such as by using a shared lock or storing
// std::atomic types in the memory region.
template <typename T>
class StructuredSharedMemory<T>::ReadOnlyMapping {
 public:
  // Move-only.
  ReadOnlyMapping(const ReadOnlyMapping&) = delete;
  ReadOnlyMapping& operator=(const ReadOnlyMapping&) = delete;
  ReadOnlyMapping(ReadOnlyMapping&&) = default;
  ReadOnlyMapping& operator=(ReadOnlyMapping&&) = default;

  // Returns a pointer to the object stored in the mapped region.
  const T* ReadOnlyPtr() const {
    CHECK(read_only_mapping_.IsValid());
    return read_only_mapping_.GetMemoryAs<T>();
  }

  // Returns a reference to the object stored in the mapped region.
  const T& ReadOnlyRef() const LIFETIME_BOUND {
    const T* ptr = ReadOnlyPtr();
    CHECK(ptr);
    return *ptr;
  }

 private:
  friend class StructuredSharedMemory<T>;

  explicit ReadOnlyMapping(ReadOnlySharedMemoryMapping read_only_mapping)
      : read_only_mapping_(std::move(read_only_mapping)) {}

  ReadOnlySharedMemoryMapping read_only_mapping_;
};

// Convenience alias for a StructuredSharedMemory region containing a
// std::atomic type.
template <typename T>
using AtomicSharedMemory = StructuredSharedMemory<std::atomic<T>>;

// Implementation.

namespace internal {

// CHECK's that a mapping of length `size` located at `ptr` is aligned correctly
// and large enough to hold a T, and that T is safe to use over shared memory.
template <typename T>
  requires(subtle::AllowedOverSharedMemory<T>)
void AssertSafeToMap(base::span<const uint8_t> mapped_span) {
  // If the mapping is too small, align() returns null.
  void* aligned_ptr = const_cast<uint8_t*>(mapped_span.data());
  size_t size = mapped_span.size_bytes();
  CHECK(std::align(alignof(T), sizeof(T), aligned_ptr, size));
  // align() modifies `aligned_ptr` - if it's already aligned it won't change.
  CHECK(aligned_ptr == mapped_span.data());
}

}  // namespace internal

// static
template <typename T>
std::optional<StructuredSharedMemory<T>> StructuredSharedMemory<T>::Create() {
  return CreateWithCustomMapper(nullptr);
}

// static
template <typename T>
template <typename U>
std::optional<StructuredSharedMemory<T>> StructuredSharedMemory<T>::Create(
    U&& initial_value) {
  return CreateWithCustomMapper(std::forward<U>(initial_value), nullptr);
}

// static
template <typename T>
std::optional<StructuredSharedMemory<T>>
StructuredSharedMemory<T>::CreateWithCustomMapper(SharedMemoryMapper* mapper) {
  MappedReadOnlyRegion mapped_region =
      ReadOnlySharedMemoryRegion::Create(sizeof(T), mapper);
  if (!mapped_region.IsValid()) {
    return std::nullopt;
  }
  internal::AssertSafeToMap<T>(mapped_region.mapping);
  // Placement new to initialize the structured memory contents in place.
  new (mapped_region.mapping.memory()) T;
  return StructuredSharedMemory<T>(std::move(mapped_region));
}

// static
template <typename T>
template <typename U>
std::optional<StructuredSharedMemory<T>>
StructuredSharedMemory<T>::CreateWithCustomMapper(U&& initial_value,
                                                  SharedMemoryMapper* mapper) {
  MappedReadOnlyRegion mapped_region =
      ReadOnlySharedMemoryRegion::Create(sizeof(T), mapper);
  if (!mapped_region.IsValid()) {
    return std::nullopt;
  }
  internal::AssertSafeToMap<T>(mapped_region.mapping);
  // Placement new to initialize the structured memory contents in place.
  new (mapped_region.mapping.memory()) T(std::forward<U>(initial_value));
  return StructuredSharedMemory<T>(std::move(mapped_region));
}

// static
template <typename T>
std::optional<typename StructuredSharedMemory<T>::ReadOnlyMapping>
StructuredSharedMemory<T>::MapReadOnlyRegion(ReadOnlySharedMemoryRegion region,
                                             SharedMemoryMapper* mapper) {
  ReadOnlySharedMemoryMapping mapping = region.Map(mapper);
  if (!mapping.IsValid()) {
    return std::nullopt;
  }
  internal::AssertSafeToMap<T>(mapping);
  return ReadOnlyMapping(std::move(mapping));
}

}  // namespace base

#endif  // BASE_MEMORY_STRUCTURED_SHARED_MEMORY_H_
