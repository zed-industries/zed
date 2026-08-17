// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_PROFILER_STACK_BUFFER_H_
#define BASE_PROFILER_STACK_BUFFER_H_

#include <stddef.h>
#include <stdint.h>

#include <memory>

#include "base/base_export.h"
#include "base/memory/aligned_memory.h"
#include "build/build_config.h"

namespace base {

// This class contains a buffer for stack copies that can be shared across
// multiple instances of StackSampler.
class BASE_EXPORT StackBuffer {
 public:
  // The expected alignment of the stack on the current platform. Windows and
  // System V AMD64 ABIs on x86, x64, and ARM require the stack to be aligned
  // to twice the pointer size. Excepted from this requirement is code setting
  // up the stack during function calls (between pushing the return address
  // and the end of the function prologue). The profiler will sometimes
  // encounter this exceptional case for leaf frames.
  static constexpr size_t kPlatformStackAlignment = 2 * sizeof(uintptr_t);

  explicit StackBuffer(size_t buffer_size);

  StackBuffer(const StackBuffer&) = delete;
  StackBuffer& operator=(const StackBuffer&) = delete;

  ~StackBuffer();

  // Returns a kPlatformStackAlignment-aligned pointer to the stack buffer.
  uintptr_t* buffer() const {
    // Aligned during allocation.
    return buffer_.get();
  }

  // Size in bytes.
  size_t size() const { return size_; }

#if BUILDFLAG(IS_CHROMEOS)
  // Tell the kernel that we no longer need the data currently in the upper
  // parts of the buffer and that the kernel may discard it to free up space.
  // Specifically, the bytes from |(uint8_t*)buffer + retained_bytes| to the
  // end of the buffer may be discarded, while the bytes from |buffer| to
  // |(uint8_t*)buffer + retained_bytes - 1| will not be affected.
  // The program can still write to that part of the buffer, but should not read
  // from that part of buffer until after the next write. (The contents of that
  // part of the buffer are undefined.)
  // After calling this function, there may be a page fault on the next write to
  // that area, so it should only be called when parts of the buffer were
  // written to and will probably not be written to again soon.
  void MarkUpperBufferContentsAsUnneeded(size_t retained_bytes);
#endif

 private:
  // The size in bytes of the requested buffer allocation.
  const size_t size_;

  // The buffer to store the stack. Already aligned.
  const std::unique_ptr<uintptr_t, AlignedFreeDeleter> buffer_;
};

}  // namespace base

#endif  // BASE_PROFILER_STACK_BUFFER_H_
