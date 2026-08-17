// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MEMORY_UNSAFE_SHARED_MEMORY_REGION_H_
#define BASE_MEMORY_UNSAFE_SHARED_MEMORY_REGION_H_

#include <stdint.h>

#include "base/base_export.h"
#include "base/check.h"
#include "base/compiler_specific.h"
#include "base/memory/platform_shared_memory_region.h"
#include "base/memory/shared_memory_mapping.h"

namespace base {

// Scoped move-only handle to a region of platform shared memory. The instance
// owns the platform handle it wraps. Mappings created by this region are
// writable. These mappings remain valid even after the region handle is moved
// or destroyed.
//
// NOTE: UnsafeSharedMemoryRegion cannot be converted to a read-only region. Use
// with caution as the region will be writable to any process with a handle to
// the region.
//
// Use this if and only if the following is true:
// - You do not need to share the region as read-only, and,
// - You need to have several instances of the region simultaneously, possibly
//   in different processes, that can produce writable mappings.

class BASE_EXPORT UnsafeSharedMemoryRegion {
 public:
  using MappingType = WritableSharedMemoryMapping;
  // Creates a new UnsafeSharedMemoryRegion instance of a given size that can be
  // used for mapping writable shared memory into the virtual address space.
  static UnsafeSharedMemoryRegion Create(size_t size);
  using CreateFunction = decltype(Create);

  // Returns an UnsafeSharedMemoryRegion built from a platform-specific handle
  // that was taken from another UnsafeSharedMemoryRegion instance. Returns an
  // invalid region iff the |handle| is invalid. CHECK-fails if the |handle|
  // isn't unsafe.
  // This should be used only by the code passing a handle across
  // process boundaries.
  static UnsafeSharedMemoryRegion Deserialize(
      subtle::PlatformSharedMemoryRegion handle);

  // Extracts a platform handle from the region. Ownership is transferred to the
  // returned region object.
  // This should be used only for sending the handle from the current
  // process to another.
  static subtle::PlatformSharedMemoryRegion TakeHandleForSerialization(
      UnsafeSharedMemoryRegion region);

  // Default constructor initializes an invalid instance.
  UnsafeSharedMemoryRegion();

  // Move operations are allowed.
  UnsafeSharedMemoryRegion(UnsafeSharedMemoryRegion&&);
  UnsafeSharedMemoryRegion& operator=(UnsafeSharedMemoryRegion&&);

  UnsafeSharedMemoryRegion(const UnsafeSharedMemoryRegion&) = delete;
  UnsafeSharedMemoryRegion& operator=(const UnsafeSharedMemoryRegion&) = delete;

  // Destructor closes shared memory region if valid.
  // All created mappings will remain valid.
  ~UnsafeSharedMemoryRegion();

  // Duplicates the underlying platform handle and creates a new
  // UnsafeSharedMemoryRegion instance that owns the newly created handle.
  // Returns a valid UnsafeSharedMemoryRegion on success, invalid otherwise.
  // The current region instance remains valid in any case.
  UnsafeSharedMemoryRegion Duplicate() const;

  // Maps the shared memory region into the caller's address space with write
  // access. The mapped address is guaranteed to have an alignment of
  // at least |subtle::PlatformSharedMemoryRegion::kMapMinimumAlignment|.
  // Returns a valid WritableSharedMemoryMapping instance on success, invalid
  // otherwise. A custom |SharedMemoryMapper| for mapping (and later unmapping)
  // the region can be provided using the optional |mapper| parameter.
  WritableSharedMemoryMapping Map(SharedMemoryMapper* mapper = nullptr) const;

  // Similar to `Map()`, but maps only `size` bytes of the shared memory block
  // at byte `offset`. Returns an invalid mapping if requested bytes are out of
  // the region limits.
  //
  // `offset` does not need to be aligned; if `offset` is not a multiple of
  // `subtle::PlatformSharedMemoryRegion::kMapMinimumAlignment`, then the
  // returned mapping will not respect alignment either. Internally, `offset`
  // and `size` are still first adjusted to respect alignment when mapping in
  // the shared memory region, but the returned mapping will be "unadjusted" to
  // match the exact `offset` and `size` requested.
  WritableSharedMemoryMapping MapAt(uint64_t offset,
                                    size_t size,
                                    SharedMemoryMapper* mapper = nullptr) const;

  // Whether the underlying platform handle is valid.
  bool IsValid() const;

  // Returns the maximum mapping size that can be created from this region.
  size_t GetSize() const {
    DCHECK(IsValid());
    return handle_.GetSize();
  }

  // Returns 128-bit GUID of the region.
  const UnguessableToken& GetGUID() const LIFETIME_BOUND {
    DCHECK(IsValid());
    return handle_.GetGUID();
  }

  // Returns a platform shared memory handle. |this| remains the owner of the
  // handle.
  subtle::PlatformSharedMemoryHandle GetPlatformHandle() const {
    DCHECK(IsValid());
    return handle_.GetPlatformHandle();
  }

 private:
  friend class SharedMemoryHooks;

  explicit UnsafeSharedMemoryRegion(subtle::PlatformSharedMemoryRegion handle);

  static void set_create_hook(CreateFunction* hook) { create_hook_ = hook; }

  static CreateFunction* create_hook_;

  subtle::PlatformSharedMemoryRegion handle_;
};

}  // namespace base

#endif  // BASE_MEMORY_UNSAFE_SHARED_MEMORY_REGION_H_
