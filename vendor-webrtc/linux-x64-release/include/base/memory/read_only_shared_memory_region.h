// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MEMORY_READ_ONLY_SHARED_MEMORY_REGION_H_
#define BASE_MEMORY_READ_ONLY_SHARED_MEMORY_REGION_H_

#include <stdint.h>

#include "base/base_export.h"
#include "base/check.h"
#include "base/check_op.h"
#include "base/compiler_specific.h"
#include "base/memory/platform_shared_memory_region.h"
#include "base/memory/shared_memory_mapping.h"

namespace base {

struct MappedReadOnlyRegion;

// Scoped move-only handle to a region of platform shared memory. The instance
// owns the platform handle it wraps. Mappings created by this region are
// read-only. These mappings remain valid even after the region handle is moved
// or destroyed.
class BASE_EXPORT ReadOnlySharedMemoryRegion {
 public:
  using MappingType = ReadOnlySharedMemoryMapping;
  // Creates a new ReadOnlySharedMemoryRegion instance of a given size along
  // with the WritableSharedMemoryMapping which provides the only way to modify
  // the content of the newly created region. The returned region and mapping
  // are guaranteed to either be both valid or both invalid. Use
  // |MappedReadOnlyRegion::IsValid()| as a shortcut for checking creation
  // success.
  //
  // This means that the caller's process is the only process that can modify
  // the region content. If you need to pass write access to another process,
  // consider using WritableSharedMemoryRegion or UnsafeSharedMemoryRegion.
  static MappedReadOnlyRegion Create(size_t size,
                                     SharedMemoryMapper* mapper = nullptr);
  using CreateFunction = decltype(Create);

  // Returns a ReadOnlySharedMemoryRegion built from a platform-specific handle
  // that was taken from another ReadOnlySharedMemoryRegion instance. Returns an
  // invalid region iff the |handle| is invalid. CHECK-fails if the |handle|
  // isn't read-only.
  // This should be used only by the code passing handles across process
  // boundaries.
  static ReadOnlySharedMemoryRegion Deserialize(
      subtle::PlatformSharedMemoryRegion handle);

  // Extracts a platform handle from the region. Ownership is transferred to the
  // returned region object.
  // This should be used only for sending the handle from the current process to
  // another.
  static subtle::PlatformSharedMemoryRegion TakeHandleForSerialization(
      ReadOnlySharedMemoryRegion region);

  // Default constructor initializes an invalid instance.
  ReadOnlySharedMemoryRegion();

  // Move operations are allowed.
  ReadOnlySharedMemoryRegion(ReadOnlySharedMemoryRegion&&);
  ReadOnlySharedMemoryRegion& operator=(ReadOnlySharedMemoryRegion&&);

  ReadOnlySharedMemoryRegion(const ReadOnlySharedMemoryRegion&) = delete;
  ReadOnlySharedMemoryRegion& operator=(const ReadOnlySharedMemoryRegion&) =
      delete;

  // Destructor closes shared memory region if valid.
  // All created mappings will remain valid.
  ~ReadOnlySharedMemoryRegion();

  // Duplicates the underlying platform handle and creates a new
  // ReadOnlySharedMemoryRegion instance that owns this handle. Returns a valid
  // ReadOnlySharedMemoryRegion on success, invalid otherwise. The current
  // region instance remains valid in any case.
  ReadOnlySharedMemoryRegion Duplicate() const;

  // Maps the shared memory region into the caller's address space with
  // read-only access. The mapped address is guaranteed to have an alignment of
  // at least |subtle::PlatformSharedMemoryRegion::kMapMinimumAlignment|.
  // Returns a valid ReadOnlySharedMemoryMapping instance on success, invalid
  // otherwise. A custom |SharedMemoryMapper| for mapping (and later unmapping)
  // the region can be provided using the optional |mapper| parameter.
  ReadOnlySharedMemoryMapping Map(SharedMemoryMapper* mapper = nullptr) const;

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
  ReadOnlySharedMemoryMapping MapAt(uint64_t offset,
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

  explicit ReadOnlySharedMemoryRegion(
      subtle::PlatformSharedMemoryRegion handle);

  static void set_create_hook(CreateFunction* hook) { create_hook_ = hook; }

  static CreateFunction* create_hook_;

  subtle::PlatformSharedMemoryRegion handle_;
};

// Helper struct for return value of ReadOnlySharedMemoryRegion::Create().
struct MappedReadOnlyRegion {
  ReadOnlySharedMemoryRegion region;
  WritableSharedMemoryMapping mapping;
  // Helper function to check return value of
  // ReadOnlySharedMemoryRegion::Create(). |region| and |mapping| either both
  // valid or invalid.
  bool IsValid() const {
    DCHECK_EQ(region.IsValid(), mapping.IsValid());
    return region.IsValid() && mapping.IsValid();
  }
};

}  // namespace base

#endif  // BASE_MEMORY_READ_ONLY_SHARED_MEMORY_REGION_H_
