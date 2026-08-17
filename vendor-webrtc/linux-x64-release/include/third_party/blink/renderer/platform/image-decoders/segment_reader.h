// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_IMAGE_DECODERS_SEGMENT_READER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_IMAGE_DECODERS_SEGMENT_READER_H_

#include <stdint.h>

#include "base/containers/span.h"
#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/platform/image-decoders/rw_buffer.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/shared_buffer.h"
#include "third_party/blink/renderer/platform/wtf/thread_safe_ref_counted.h"

class SkData;
template <typename T>
class sk_sp;

namespace blink {

// Interface that looks like SharedBuffer. Used by ImageDecoders to use various
// sources of input including:
// - SharedBuffer
//   - for when the caller already has a SharedBuffer
// - SkData
//   - for when the caller already has an SkData
// - ROBuffer
//   - for when the caller wants to read/write in different threads
//
// Unlike SharedBuffer, this is a read-only interface. There is no way to
// modify the underlying data source.
class PLATFORM_EXPORT SegmentReader
    : public ThreadSafeRefCounted<SegmentReader> {
 public:
  // This version is thread-safe so long as no thread is modifying the
  // underlying SharedBuffer. This class does not modify it, so that would
  // mean modifying it in another way.
  static scoped_refptr<SegmentReader> CreateFromSharedBuffer(
      scoped_refptr<const SharedBuffer>);

  // These versions use thread-safe input, so they are always thread-safe.
  static scoped_refptr<SegmentReader> CreateFromSkData(sk_sp<SkData>);
  static scoped_refptr<SegmentReader> CreateFromROBuffer(
      scoped_refptr<ROBuffer>);

  SegmentReader() = default;
  SegmentReader(const SegmentReader&) = delete;
  SegmentReader& operator=(const SegmentReader&) = delete;
  virtual size_t size() const = 0;
  // Returns a span of however much data is left in the segment containing the
  // `position`. If there's no data at the specified `position`, an empty span
  // is returned.
  virtual base::span<const uint8_t> GetSomeData(size_t position) const = 0;
  virtual sk_sp<SkData> GetAsSkData() const = 0;
  virtual void LockData() {}
  virtual void UnlockData() {}

  static sk_sp<SkData> RWBufferCopyAsSkData(RWBuffer::ROIter iter,
                                            size_t available);
  static base::span<const uint8_t> RWBufferGetSomeData(
      RWBuffer::ROIter& iter,
      size_t& position_of_block,
      size_t position);

 protected:
  friend class ThreadSafeRefCounted<SegmentReader>;
  virtual ~SegmentReader() = default;
};

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_IMAGE_DECODERS_SEGMENT_READER_H_
