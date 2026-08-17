/*
 * Copyright (C) 2009 Apple Inc. All rights reserved.
 * Copyright (C) 2013 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TYPED_ARRAYS_ARRAY_BUFFER_ARRAY_BUFFER_CONTENTS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TYPED_ARRAYS_ARRAY_BUFFER_ARRAY_BUFFER_CONTENTS_H_

#include "base/containers/span.h"
#include "base/memory/platform_shared_memory_region.h"
#include "base/memory/scoped_refptr.h"
#include "partition_alloc/partition_alloc_constants.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/cross_thread_copier.h"
#include "third_party/blink/renderer/platform/wtf/thread_safe_ref_counted.h"
#include "third_party/blink/renderer/platform/wtf/wtf.h"
#include "v8/include/v8.h"

namespace blink {

class CORE_EXPORT ArrayBufferContents {
  DISALLOW_NEW();

 public:
  // Types that need to be used when injecting external memory.
  // v8::BackingStore allows specifying a deleter which will be invoked when
  // v8::BackingStore instance goes out of scope. If the data memory is
  // allocated using ArrayBufferContents::AllocateMemoryOrNull, it is necessary
  // to specify ArrayBufferContents::FreeMemory as the DataDeleter.
  using DataDeleter = void (*)(void* data, size_t length, void* info);

  enum InitializationPolicy { kZeroInitialize, kDontInitialize };

  enum SharingType {
    kNotShared,
    kShared,
  };

  // Behavior of a constructor on memory allocation failure.
  enum class AllocationFailureBehavior {
    // Construct an object for which `!IsValid()`.
    kInvalid,
    // Generate an OOM crash. The cause of the OOM (excessive size, mapping
    // failure, commit failure) can be derived from the crash stack, so this is
    // preferred to having custom logic in the caller to crash if `!IsValid()`.
    kCrash,
  };

  ArrayBufferContents() = default;
  ArrayBufferContents(size_t num_elements,
                      size_t element_byte_size,
                      SharingType is_shared,
                      InitializationPolicy policy,
                      AllocationFailureBehavior allocation_failure_behavior =
                          AllocationFailureBehavior::kInvalid)
      : ArrayBufferContents(num_elements,
                            std::nullopt,
                            element_byte_size,
                            is_shared,
                            policy,
                            allocation_failure_behavior) {}
  // If max_num_elements has a value, a backing store for a resizable
  // ArrayBuffer is created. Otherwise a backing store for a fixed-length
  // ArrayBuffer is created.
  ArrayBufferContents(size_t num_elements,
                      std::optional<size_t> max_num_elements,
                      size_t element_byte_size,
                      SharingType is_shared,
                      InitializationPolicy policy,
                      AllocationFailureBehavior allocation_failure_behavior =
                          AllocationFailureBehavior::kInvalid);

  ArrayBufferContents(
      const base::subtle::PlatformSharedMemoryRegion& shared_memory_region,
      uint64_t offset,
      size_t length);

  ArrayBufferContents(ArrayBufferContents&&) = default;

  ArrayBufferContents(const ArrayBufferContents&) = default;

  explicit ArrayBufferContents(std::shared_ptr<v8::BackingStore> backing_store)
      : backing_store_(std::move(backing_store)) {}

  ~ArrayBufferContents();

  ArrayBufferContents& operator=(const ArrayBufferContents&) = default;
  ArrayBufferContents& operator=(ArrayBufferContents&&) = default;

  void Detach();

  // Resets the internal memory so that the ArrayBufferContents is empty.
  void Reset();

  void* Data() const {
    DCHECK(!IsShared());
    return DataMaybeShared();
  }
  void* DataShared() const {
    DCHECK(IsShared());
    return DataMaybeShared();
  }
  void* DataMaybeShared() const {
    return backing_store_ ? backing_store_->Data() : nullptr;
  }
  size_t DataLength() const {
    return backing_store_ ? backing_store_->ByteLength() : 0;
  }
  size_t MaxDataLength() const {
    return backing_store_ ? backing_store_->MaxByteLength() : 0;
  }
  bool IsShared() const { return backing_store_ && backing_store_->IsShared(); }
  bool IsResizableByUserJavaScript() const {
    return backing_store_ && backing_store_->IsResizableByUserJavaScript();
  }
  bool IsValid() const { return backing_store_ && backing_store_->Data(); }
  base::span<uint8_t> ByteSpan() const {
    // SAFETY: `BackingStore` guarantees that `Data()` points to at least
    // `DataLength()` many bytes.
    return UNSAFE_BUFFERS(
        base::span(static_cast<uint8_t*>(Data()), DataLength()));
  }
  base::span<uint8_t> ByteSpanMaybeShared() const {
    // SAFETY: `BackingStore` guarantees that `Data()` points to at least
    // `DataLength()` many bytes.
    return UNSAFE_BUFFERS(
        base::span(static_cast<uint8_t*>(DataMaybeShared()), DataLength()));
  }

  std::shared_ptr<v8::BackingStore> BackingStore() const {
    return backing_store_;
  }

  void Transfer(ArrayBufferContents& other);
  void ShareWith(ArrayBufferContents& other);
  void ShareNonSharedForInternalUse(ArrayBufferContents& other);
  void CopyTo(ArrayBufferContents& other);

  static void* AllocateMemoryOrNull(size_t, InitializationPolicy);
  static void FreeMemory(void*);

 private:
  template <partition_alloc::AllocFlags flags>
  static void* AllocateMemory(size_t, InitializationPolicy);

  std::shared_ptr<v8::BackingStore> backing_store_;
};

}  // namespace blink

namespace WTF {

template <>
struct CrossThreadCopier<blink::ArrayBufferContents> {
  STATIC_ONLY(CrossThreadCopier);
  using Type = blink::ArrayBufferContents;
  static Type Copy(Type handle) {
    return handle;  // This is in fact a move.
  }
};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TYPED_ARRAYS_ARRAY_BUFFER_ARRAY_BUFFER_CONTENTS_H_
