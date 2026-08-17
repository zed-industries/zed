// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MEMORY_WRITABLE_SHARED_MEMORY_REGION_H_
#define BASE_MEMORY_WRITABLE_SHARED_MEMORY_REGION_H_

#include <stdint.h>

#include "base/base_export.h"
#include "base/check.h"
#include "base/compiler_specific.h"
#include "base/memory/platform_shared_memory_region.h"
#include "base/memory/read_only_shared_memory_region.h"
#include "base/memory/shared_memory_mapping.h"
#include "base/memory/unsafe_shared_memory_region.h"
#include "build/build_config.h"

namespace base {

// Scoped move-only handle to a region of platform shared memory. The instance
// owns the platform handle it wraps. Mappings created by this region are
// writable. These mappings remain valid even after the region handle is moved
// or destroyed.
//
// This region can be locked to read-only access by converting it to a
// ReadOnlySharedMemoryRegion. However, unlike ReadOnlySharedMemoryRegion and
// UnsafeSharedMemoryRegion, ownership of this region (while writable) is unique
// and may only be transferred, not duplicated.
//
// Unlike ReadOnlySharedMemoryRegion and UnsafeSharedMemoryRegion,
// WritableSharedMemoryRegion doesn't provide GetPlatformHandle() method to
// ensure that the region is never duplicated while writable.
class BASE_EXPORT WritableSharedMemoryRegion {
 public:
  using MappingType = WritableSharedMemoryMapping;
  // Creates a new WritableSharedMemoryRegion instance of a given
  // size that can be used for mapping writable shared memory into the virtual
  // address space.
  static WritableSharedMemoryRegion Create(size_t size);
  using CreateFunction = decltype(Create);

  // Returns a WritableSharedMemoryRegion built from a platform handle that was
  // taken from another WritableSharedMemoryRegion instance. Returns an invalid
  // region iff the |handle| is invalid. CHECK-fails if the |handle| isn't
  // writable.
  // This should be used only by the code passing handles across process
  // boundaries.
  static WritableSharedMemoryRegion Deserialize(
      subtle::PlatformSharedMemoryRegion handle);

  // Extracts a platform handle from the region. Ownership is transferred to the
  // returned region object.
  // This should be used only for sending the handle from the current
  // process to another.
  static subtle::PlatformSharedMemoryRegion TakeHandleForSerialization(
      WritableSharedMemoryRegion region);

  // Makes the region read-only. No new writable mappings of the region can be
  // created after this call. Returns an invalid region on failure.
  static ReadOnlySharedMemoryRegion ConvertToReadOnly(
      WritableSharedMemoryRegion region);

  // Makes the region unsafe. The region cannot be converted to read-only after
  // this call. Returns an invalid region on failure.
  static UnsafeSharedMemoryRegion ConvertToUnsafe(
      WritableSharedMemoryRegion region);

  // Default constructor initializes an invalid instance.
  WritableSharedMemoryRegion();

  // Move operations are allowed.
  WritableSharedMemoryRegion(WritableSharedMemoryRegion&&);
  WritableSharedMemoryRegion& operator=(WritableSharedMemoryRegion&&);

  WritableSharedMemoryRegion(const WritableSharedMemoryRegion&) = delete;
  WritableSharedMemoryRegion& operator=(const WritableSharedMemoryRegion&) =
      delete;

  // Destructor closes shared memory region if valid.
  // All created mappings will remain valid.
  ~WritableSharedMemoryRegion();

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

  // Whether underlying platform handles are valid.
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

#if BUILDFLAG(IS_WIN)
  // On Windows it is necessary in rare cases to take a writable handle from a
  // region that will be converted to read-only. On this platform it is a safe
  // operation, as the handle returned from this method will remain writable
  // after the region is converted to read-only. However, it breaks chromium's
  // WritableSharedMemoryRegion semantics and so should be use with care.
  HANDLE UnsafeGetPlatformHandle() const { return handle_.GetPlatformHandle(); }
#endif

 private:
  friend class SharedMemoryHooks;

  explicit WritableSharedMemoryRegion(
      subtle::PlatformSharedMemoryRegion handle);

  static void set_create_hook(CreateFunction* hook) { create_hook_ = hook; }

  static CreateFunction* create_hook_;

  subtle::PlatformSharedMemoryRegion handle_;
};

}  // namespace base

#endif  // BASE_MEMORY_WRITABLE_SHARED_MEMORY_REGION_H_
