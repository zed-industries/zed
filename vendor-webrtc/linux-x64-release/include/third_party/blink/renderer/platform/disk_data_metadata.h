// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_DISK_DATA_METADATA_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_DISK_DATA_METADATA_H_

#include <stddef.h>
#include <stdint.h>

#include <memory>

#include "base/memory/raw_ptr.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

class DiskDataAllocator;

class PLATFORM_EXPORT DiskDataMetadata {
 public:
  int64_t start_offset() const { return start_offset_; }
  size_t size() const { return size_; }

 private:
  DiskDataMetadata(int64_t start_offset, size_t size)
      : start_offset_(start_offset), size_(size) {}
  DiskDataMetadata(const DiskDataMetadata& other) = default;
  DiskDataMetadata(DiskDataMetadata&& other) = default;
  DiskDataMetadata& operator=(const DiskDataMetadata& other) = default;

  int64_t start_offset_;
  size_t size_;

  friend class DiskDataAllocator;
};

class PLATFORM_EXPORT ReservedChunk {
 public:
  ReservedChunk(DiskDataAllocator* allocator,
                std::unique_ptr<DiskDataMetadata> metadata);
  ReservedChunk(const ReservedChunk&) = delete;
  ReservedChunk& operator=(const ReservedChunk&) = delete;
  ~ReservedChunk();

  std::unique_ptr<DiskDataMetadata> Take();

 private:
  raw_ptr<DiskDataAllocator> allocator_;
  std::unique_ptr<DiskDataMetadata> metadata_;
};

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_DISK_DATA_METADATA_H_
