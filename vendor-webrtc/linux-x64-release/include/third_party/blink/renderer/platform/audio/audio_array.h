/*
 * Copyright (C) 2010 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 * 3.  Neither the name of Apple Computer, Inc. ("Apple") nor the names of
 *     its contributors may be used to endorse or promote products derived
 *     from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE AND ITS CONTRIBUTORS "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE OR ITS CONTRIBUTORS BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_AUDIO_ARRAY_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_AUDIO_ARRAY_H_

#include <string.h>

#include "base/check_op.h"
#include "base/containers/span.h"
#include "base/memory/raw_ptr.h"
#include "base/numerics/checked_math.h"
#include "build/build_config.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/allocator/partitions.h"
#include "third_party/blink/renderer/platform/wtf/assertions.h"

namespace blink {

template <typename T>
class AudioArray final {
  USING_FAST_MALLOC(AudioArray);

 public:
  AudioArray() : allocation_(nullptr), aligned_data_(nullptr), size_(0) {}
  explicit AudioArray(size_t n)
      : allocation_(nullptr), aligned_data_(nullptr), size_(0) {
    Allocate(n);
  }
  AudioArray(const AudioArray&) = delete;
  AudioArray& operator=(const AudioArray&) = delete;

  ~AudioArray() { WTF::Partitions::FastFree(allocation_); }

  // It's OK to call Allocate() multiple times, but data will *not* be copied
  // from an initial allocation if re-allocated. Allocations are
  // zero-initialized.
  void Allocate(size_t n) {
    // Although n is a size_t, its true limit is max unsigned because we use
    // unsigned in zeroRange() and copyToRange(). Also check for integer
    // overflow.
    CHECK_LE(n, std::numeric_limits<unsigned>::max() / sizeof(T));
    uint32_t initial_size = static_cast<uint32_t>(sizeof(T) * n);

    // Minimmum alignment requirements for arrays so that we can use
    // SIMD.
#if defined(ARCH_CPU_X86_FAMILY)
    const unsigned kAlignment = 32;
#else
    const unsigned kAlignment = 16;
#endif

    if (allocation_) {
      WTF::Partitions::FastFree(allocation_);
    }

    // Always allocate extra space so that we are guaranteed to get
    // the desired alignment.  Some memory is wasted, but it should be
    // small since most arrays are probably at least 128 floats (or
    // doubles).
    unsigned total = base::CheckAdd(initial_size, kAlignment).ValueOrDie();
    allocation_ = static_cast<T*>(WTF::Partitions::FastZeroedMalloc(
        total, WTF_HEAP_PROFILER_TYPE_NAME(AudioArray<T>)));
    CHECK(allocation_);

    aligned_data_ = AlignedAddress(allocation_.get(), kAlignment);
    size_ = static_cast<uint32_t>(n);
  }

  T* Data() { return aligned_data_; }
  const T* Data() const { return aligned_data_; }
  uint32_t size() const { return size_; }
  base::span<T> as_span() {
    // SAFETY: Allocate() ensures `aligned_data_` and `size_` are safe.
    return UNSAFE_BUFFERS(base::span(aligned_data_.get(), size_));
  }
  base::span<const T> as_span() const {
    // SAFETY: Allocate() ensures `aligned_data_` and `size_` are safe.
    return UNSAFE_BUFFERS(base::span(aligned_data_.get(), size_));
  }

  T& at(size_t i) {
    // Note that although it is a size_t, m_size is now guaranteed to be
    // no greater than max unsigned. This guarantee is enforced in Allocate().
    SECURITY_DCHECK(i < size());
    return as_span()[i];
  }

  T& operator[](size_t i) { return at(i); }

  void Zero() {
    // This multiplication is made safe by the check in Allocate().
    std::ranges::fill(as_span(), 0);
  }

  void ZeroRange(unsigned start, unsigned end) {
    bool is_safe = (start <= end) && (end <= size());
    DCHECK(is_safe);
    if (!is_safe) {
      return;
    }

    // This expression cannot overflow because end - start cannot be
    // greater than m_size, which is safe due to the check in Allocate().
    std::ranges::fill(as_span().subspan(start, end - start), 0);
  }

  void CopyToRange(const T* source_data, unsigned start, unsigned end) {
    bool is_safe = (start <= end) && (end <= size());
    DCHECK(is_safe);
    if (!is_safe) {
      return;
    }

    // This expression cannot overflow because end - start cannot be
    // greater than m_size, which is safe due to the check in Allocate().
    as_span()
        .subspan(start, end - start)
        .copy_from(
            // SAFETY: `is_safe` ensures `source_data` and `end - start` are
            // safe.
            UNSAFE_BUFFERS(base::span(source_data, end - start)));
  }

 private:
  // Return an address that is aligned to an |alignment| boundary.
  // |alignment| MUST be a power of two!
  static T* AlignedAddress(T* address, intptr_t alignment) {
    intptr_t value = reinterpret_cast<intptr_t>(address);
    return reinterpret_cast<T*>((value + alignment - 1) & ~(alignment - 1));
  }

  raw_ptr<T, DanglingUntriaged> allocation_;
  raw_ptr<T, DanglingUntriaged> aligned_data_;
  uint32_t size_;
};

typedef AudioArray<float> AudioFloatArray;
typedef AudioArray<double> AudioDoubleArray;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_AUDIO_ARRAY_H_
