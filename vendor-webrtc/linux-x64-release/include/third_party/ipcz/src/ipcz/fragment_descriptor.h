// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_IPCZ_FRAGMENT_DESCRIPTOR_H_
#define IPCZ_SRC_IPCZ_FRAGMENT_DESCRIPTOR_H_

#include <cstdint>
#include <tuple>
#include <type_traits>

#include "ipcz/buffer_id.h"
#include "ipcz/ipcz.h"

namespace ipcz {

// Represents a span of memory within the shared memory regions owned by a
// BufferPool. A FragmentDescriptor can be resolved to a concrete Fragment
// by passing it to GetFragment() on an appropriate BufferPool object.
//
// NOTE: This is a wire structure which must remain stable over time.
struct IPCZ_ALIGN(8) FragmentDescriptor {
  // Constructs a null descriptor. Null descriptors always resolve to null
  // fragments.
  constexpr FragmentDescriptor() = default;

  // Constructs a descriptor for a span of memory `size` bytes long, starting
  // at byte `offset` within the buffer identified by `buffer_id` within some
  // BufferPool.
  constexpr FragmentDescriptor(BufferId buffer_id,
                               uint32_t offset,
                               uint32_t size)
      : buffer_id_(buffer_id), offset_(offset), size_(size) {}

  bool is_null() const { return buffer_id_ == kInvalidBufferId; }

  BufferId buffer_id() const { return buffer_id_; }
  uint32_t offset() const { return offset_; }
  uint32_t size() const { return size_; }

 private:
  // Identifies the shared memory buffer in which the memory resides. This ID is
  // scoped to a specific BufferPool (and therefore to a specific NodeLink).
  BufferId buffer_id_ = kInvalidBufferId;

  // The byte offset from the start of the identified shared memory buffer where
  // this fragment begins.
  uint32_t offset_ = 0;

  // The size of this fragment in bytes.
  uint32_t size_ = 0;
};
static_assert(std::is_trivially_copyable_v<FragmentDescriptor>,
              "FragmentDescriptor must be trivially copyable");

}  // namespace ipcz

#endif  // IPCZ_SRC_IPCZ_FRAGMENT_DESCRIPTOR_H_
