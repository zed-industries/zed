// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_IPCZ_FRAGMENT_H_
#define IPCZ_SRC_IPCZ_FRAGMENT_H_

#include <cstdint>

#include "ipcz/buffer_id.h"
#include "ipcz/fragment_descriptor.h"
#include "third_party/abseil-cpp/absl/base/macros.h"
#include "third_party/abseil-cpp/absl/types/span.h"

namespace ipcz {

class DriverMemoryMapping;

// Represents a span of memory located within the shared memory regions owned by
// a NodeLinkMemory, via BufferPool. This is essentially a FragmentDescriptor
// plus the actual mapped address of the given buffer and offset.
struct Fragment {
  constexpr Fragment() = default;

  Fragment(const Fragment&);
  Fragment& operator=(const Fragment&);

  // Returns a new concrete Fragment corresponding to `descriptor` within the
  // context of `mapping`. This validates that the fragment's bounds fall within
  // the bounds of `mapping`. If `descriptor` was null or validation fails, this
  // returns a null Fragment.
  static Fragment MappedFromDescriptor(const FragmentDescriptor& descriptor,
                                       DriverMemoryMapping& mapping);

  // Returns a pending Fragment corresponding to `descriptor`.
  static Fragment PendingFromDescriptor(const FragmentDescriptor& descriptor);

  // Returns a Fragment corresponding to `descriptor`, with the starting address
  // already mapped to `address`.
  static Fragment FromDescriptorUnsafe(const FragmentDescriptor& descriptor,
                                       void* address);

  // A null fragment is a fragment with a null descriptor, meaning it does not
  // reference a valid buffer ID.
  bool is_null() const { return descriptor_.is_null(); }

  // An addressable fragment is a fragment with a non-null mapped address (which
  // also implies a non-null descriptor.)
  bool is_addressable() const { return address_ != nullptr; }

  // A pending fragment is one with a non-null descriptor but a null mapped
  // address. Pending fragments may be resolved later, but are not themselves
  // addressable.
  bool is_pending() const { return !is_null() && !is_addressable(); }

  BufferId buffer_id() const { return descriptor_.buffer_id(); }
  uint32_t offset() const { return descriptor_.offset(); }
  uint32_t size() const { return descriptor_.size(); }
  const FragmentDescriptor& descriptor() const { return descriptor_; }

  // Returns the mapped base address for this fragment. Null unless
  // `is_addressable()` is true.
  void* address() const { return address_; }

  // Returns the span of mapped bytes corresponding to this fragment. Invalid
  // to call unless `is_addressable()` is true.
  absl::Span<const uint8_t> bytes() const {
    ABSL_ASSERT(is_addressable());
    return {static_cast<const uint8_t*>(address_), descriptor_.size()};
  }

  // Returns the span of mapped (mutable) bytes corresponding to this fragment.
  // Invalid to call unless `is_addressable()` is true.
  absl::Span<uint8_t> mutable_bytes() const {
    ABSL_ASSERT(is_addressable());
    return {static_cast<uint8_t*>(address_), descriptor_.size()};
  }

 private:
  // Constructs a new Fragment over `descriptor`, mapped to `address`. If
  // `address` is null, the Fragment is considered "pending" -- it has a
  // potentially valid descriptor, but could not be resolved to a mapped address
  // yet (e.g. because the relevant BufferPool doesn't have the identified
  // buffer mapped yet.)
  Fragment(const FragmentDescriptor& descriptor, void* address);

  FragmentDescriptor descriptor_;

  // The actual mapped address corresponding to `descriptor_`.
  void* address_ = nullptr;
};

}  // namespace ipcz

#endif  // IPCZ_SRC_IPCZ_FRAGMENT_H_
